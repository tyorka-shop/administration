use async_graphql::{Context, Object, Result, ID};
use sqlx::SqlitePool;

use crate::{
    graphql_types::{Crop, Picture, Product, ProductInput},
    image_storage::ImageStorage,
};

pub struct Mutations;

#[Object]
impl Mutations {
    async fn save_product<'a>(&self, ctx: &Context<'a>, product: ProductInput) -> Result<Product> {
        let db = ctx.data::<SqlitePool>().unwrap();

        let entity = entity::Product::from(&product);

        if product.pictures.len() == 0 {
            return Err("Product must have at least one picture".into());
        }

        if product.show_in_shop {
            match product.price {
                Some(value) if value <= 0 => {
                    return Err("Product price must be greater than 0".into())
                }
                None => return Err("Product price must be set".into()),
                _ => {}
            }
        }

        entity.insert_or_update(db).await?;

        for (i, picture_id) in product.pictures.iter().enumerate() {
            let index = i.to_string();
            let product_id = product.id.to_string();
            let picture_id = picture_id.to_string();
            sqlx::query!(
                "update pictures set product_id = $1, `idx` = $2 where id = $3",
                product_id,
                index,
                picture_id
            )
            .execute(db)
            .await?;
        }

        Ok(Product::from(&entity))
    }

    async fn save_crop(&self, ctx: &Context<'_>, id: ID, crop: Crop) -> Result<Picture> {
        let db = ctx.data::<SqlitePool>().unwrap();
        let images = ctx.data::<ImageStorage>().unwrap();

        let row = entity::Picture::get_by_id(&db, &id).await.unwrap();

        images.recrop(&row.id, &crop.clone().into()).unwrap();

        let mut pic = Picture::from(row);
        pic.crop = crop.into();
        entity::Picture::from(&pic)
            .insert_or_update(db)
            .await
            .unwrap();

        Ok(pic)
    }

    async fn save_gallery_order(&self, ctx: &Context<'_>, list: Vec<ID>) -> Result<Vec<Product>> {
        let db = ctx.data::<SqlitePool>().unwrap();

        let list = list.iter().map(|id| id.to_string()).collect();

        entity::Product::save_gallery_order(&db, &list)
            .await
            .unwrap();

        let products = entity::Product::get_gallery(&db)
            .await
            .unwrap()
            .iter()
            .map(Product::from)
            .collect();

        Ok(products)
    }

    async fn save_shop_order(&self, ctx: &Context<'_>, list: Vec<ID>) -> Result<Vec<Product>> {
        let db = ctx.data::<SqlitePool>().unwrap();

        let list = list.iter().map(|id| id.to_string()).collect();

        entity::Product::save_shop_order(&db, &list).await.unwrap();

        let products = entity::Product::get_shop(&db)
            .await
            .unwrap()
            .iter()
            .map(Product::from)
            .collect();

        Ok(products)
    }
}

#[cfg(test)]
mod save_product {
    use crate::graphql_types::ProductInput;

    use super::Mutations;
    use async_graphql::{EmptySubscription, InputType, Object, Request, Result, Variables};

    struct Queries;

    #[Object]
    impl Queries {
        async fn status(&self) -> Result<String> {
            Ok("Ok".into())
        }
    }

    fn make_request(product: &ProductInput) -> Request {
        let vars = Variables::from_json(serde_json::json!({ "product": product.to_value() }));

        let mutation = r#"mutation AddProduct($product: ProductInput!) { saveProduct(product: $product) { id } }"#;

        Request::new(mutation).variables(vars)
    }

    async fn request(product: &ProductInput) -> (async_graphql::Response, sqlx::SqlitePool) {
        let schema = async_graphql::Schema::build(Queries, Mutations, EmptySubscription).finish();
        let db = crate::test_utils::setup_db().await.unwrap();

        let request = make_request(product);

        (schema.execute(request.data(db.clone())).await, db)
    }

    #[tokio::test]
    pub async fn valid() {
        let product = ProductInput::new_fixture();
        let (result, _) = request(&product).await;

        assert_eq!(result.errors.first(), None);

        insta::assert_json_snapshot!(result.data.into_json().unwrap());
    }

    #[tokio::test]
    pub async fn empty_pictures() {
        let mut product = ProductInput::new_fixture();
        product.pictures = vec![];

        let (result, _) = request(&product).await;

        assert_eq!(
            result.errors.first().unwrap().message,
            "Product must have at least one picture"
        );
    }

    #[tokio::test]
    pub async fn price_must_be_set() {
        let mut product = ProductInput::new_fixture();
        product.show_in_shop = true;
        product.price = None;

        let (result, _) = request(&product).await;

        assert_eq!(
            result.errors.first().unwrap().message,
            "Product price must be set"
        );
    }

    #[tokio::test]
    pub async fn price_must_be_gt_0() {
        let mut product = ProductInput::new_fixture();
        product.show_in_shop = true;
        product.price = Some(0);

        let (result, _) = request(&product).await;

        assert_eq!(
            result.errors.first().unwrap().message,
            "Product price must be greater than 0"
        );
    }

    #[tokio::test]
    pub async fn update() {
        let schema = async_graphql::Schema::build(Queries, Mutations, EmptySubscription).finish();
        let db = crate::test_utils::setup_db().await.unwrap();

        let mut product = ProductInput::new_fixture();

        let request = make_request(&product).data(db.clone());

        // create
        schema.execute(request).await;

        product.title.en = "New title".into();
        let request = make_request(&product).data(db.clone());

        // update
        let result = schema.execute(request).await;

        assert_eq!(
            serde_json::to_string(&result).unwrap(),
            serde_json::json!({"data":{"saveProduct": {"id": product.id}}}).to_string()
        );

        let row = sqlx::query!("select * from products where id = ? ", product.id.0)
            .fetch_one(&db)
            .await
            .unwrap();

        assert_eq!(row.title_en, "New title");
    }
}

#[cfg(test)]
mod entity_order {
    use async_graphql::{EmptySubscription, Object, Request, Response, Result, Variables};
    use sqlx::SqlitePool;

    use crate::guard::Role;

    use super::Mutations;

    struct Queries;

    #[Object]
    impl Queries {
        async fn status(&self) -> Result<String> {
            Ok("Ok".into())
        }
    }

    async fn request(request: Request, db: &SqlitePool) -> Response {
        let schema = async_graphql::Schema::build(Queries, Mutations, EmptySubscription).finish();

        schema
            .execute(
                request
                    .data(db.clone())
                    .data(Role::Admin("admin".to_string())),
            )
            .await
    }

    async fn save_gallery_order(db: &SqlitePool, list: Vec<String>) -> Result<Response> {
        let vars = Variables::from_json(serde_json::json!({ "list": list }));
        let r = Request::new(
            r#"
            mutation SaveGalleryOrder($list: [ID!]!) {
                saveGalleryOrder(list: $list) {
                    id
                }
            }
            "#,
        )
        .variables(vars);

        let response = request(r, db).await;
        assert_eq!(response.errors.first(), None);

        Ok(response)
    }

    async fn save_shop_order(db: &SqlitePool, list: Vec<String>) -> Result<Response> {
        let vars = Variables::from_json(serde_json::json!({ "list": list }));
        let r = Request::new(
            r#"
            mutation SaveShopOrder($list: [ID!]!) {
                saveShopOrder(list: $list) {
                    id
                }
            }
            "#,
        )
        .variables(vars);

        let response = request(r, db).await;
        assert_eq!(response.errors.first(), None);

        Ok(response)
    }

    async fn setup() -> Result<SqlitePool> {
        let db = crate::test_utils::setup_db().await.unwrap();

        for i in 1..3 {
            let id = i.to_string();
            let mut product = entity::Product::new_fixture_with_id(&id);
            product.show_in_gallery = true;
            product.show_in_shop = true;
            product.price = Some(100);
            product.insert(&db).await.unwrap();
        }

        Ok(db)
    }

    #[derive(serde::Serialize)]
    struct OrderEntity {
        pub entity_id: String,
        pub idx: i64,
    }

    async fn get_entity_order(db: &SqlitePool) -> Result<Vec<OrderEntity>> {
        let rows = sqlx::query_as!(
            OrderEntity,
            "select `entity_id`, `idx` from `entity_order` order by `idx` asc"
        )
        .fetch_all(db)
        .await?;

        Ok(rows)
    }

    #[tokio::test]
    async fn gallery_db_asc() {
        let db = setup().await.unwrap();

        save_gallery_order(&db, vec!["1".into(), "2".into()])
            .await
            .unwrap();

        let rows = get_entity_order(&db).await.unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].entity_id, "1");
        assert_eq!(rows[1].entity_id, "2");
    }

    #[tokio::test]
    async fn gallery_response_asc() {
        let db = setup().await.unwrap();

        let response = save_gallery_order(&db, vec!["1".into(), "2".into()])
            .await
            .unwrap();
        insta::assert_json_snapshot!(response.data.into_json().unwrap());
    }

    #[tokio::test]
    async fn gallery_db_desc() {
        let db = setup().await.unwrap();

        save_gallery_order(&db, vec!["2".into(), "1".into()])
            .await
            .unwrap();

        let rows = get_entity_order(&db).await.unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].entity_id, "2");
        assert_eq!(rows[1].entity_id, "1");
    }

    #[tokio::test]
    async fn gallery_response_desc() {
        let db = setup().await.unwrap();

        let response = save_gallery_order(&db, vec!["2".into(), "1".into()])
            .await
            .unwrap();

        assert_eq!(response.errors.len(), 0);
        insta::assert_json_snapshot!(response.data.into_json().unwrap());
    }

    #[tokio::test]
    async fn shop_db_asc() {
        let db = setup().await.unwrap();

        save_shop_order(&db, vec!["1".into(), "2".into()])
            .await
            .unwrap();

        let rows = get_entity_order(&db).await.unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].entity_id, "1");
        assert_eq!(rows[1].entity_id, "2");
    }

    #[tokio::test]
    async fn shop_response_asc() {
        let db = setup().await.unwrap();

        let response = save_shop_order(&db, vec!["1".into(), "2".into()])
            .await
            .unwrap();
        insta::assert_json_snapshot!(response.data.into_json().unwrap());
    }

    #[tokio::test]
    async fn shop_db_desc() {
        let db = setup().await.unwrap();

        save_shop_order(&db, vec!["2".into(), "1".into()])
            .await
            .unwrap();

        let rows = get_entity_order(&db).await.unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].entity_id, "2");
        assert_eq!(rows[1].entity_id, "1");
    }

    #[tokio::test]
    async fn shop_response_desc() {
        let db = setup().await.unwrap();

        let response = save_gallery_order(&db, vec!["2".into(), "1".into()])
            .await
            .unwrap();

        assert_eq!(response.errors.len(), 0);
        insta::assert_json_snapshot!(response.data.into_json().unwrap());
    }
}
