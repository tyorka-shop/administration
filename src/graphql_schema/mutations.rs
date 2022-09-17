use async_graphql::{Context, Object, Result};
use sqlx::SqlitePool;

use crate::entity::product::{self, Product, ProductInput};

pub struct Mutations;

#[Object]
impl Mutations {
    async fn save_product<'a>(&self, ctx: &Context<'a>, product: ProductInput) -> Result<Product> {
        let db = ctx.data::<SqlitePool>().unwrap();

        let entity = product::Entity::from(&product);

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
}

#[cfg(test)]
mod save_product {
    use crate::entity::product::ProductInput;

    use super::Mutations;
    use async_graphql::{EmptySubscription, InputType, Object, Request, Result, Variables};
    use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

    struct Queries;

    #[Object]
    impl Queries {
        async fn status(&self) -> Result<String> {
            Ok("Ok".into())
        }
    }

    async fn setup_db() -> Result<SqlitePool> {
        let db = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(":memory:")
            .await
            .unwrap();

        sqlx::migrate!().run(&db).await.unwrap();

        Ok(db)
    }

    fn make_request(product: &ProductInput) -> Request {
        let vars = Variables::from_json(serde_json::json!({ "product": product.to_value() }));

        let mutation = r#"mutation AddProduct($product: ProductInput!) { saveProduct(product: $product) { id } }"#;

        Request::new(mutation).variables(vars)
    }

    async fn request(product: &ProductInput) -> (async_graphql::Response, sqlx::SqlitePool) {
        let schema = async_graphql::Schema::build(Queries, Mutations, EmptySubscription).finish();
        let db = setup_db().await.unwrap();

        let request = make_request(product);

        (schema.execute(request.data(db.clone())).await, db)
    }

    #[tokio::test]
    pub async fn valid() {
        let product = ProductInput::mock();
        let (result, _) = request(&product).await;

        assert_eq!(result.errors.first(), None);

        insta::assert_json_snapshot!(result.data.into_json().unwrap());
    }

    #[tokio::test]
    pub async fn empty_pictures() {
        let mut product = ProductInput::mock();
        product.pictures = vec![];

        let (result, _) = request(&product).await;

        assert_eq!(
            result.errors.first().unwrap().message,
            "Product must have at least one picture"
        );
    }

    #[tokio::test]
    pub async fn price_must_be_set() {
        let mut product = ProductInput::mock();
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
        let mut product = ProductInput::mock();
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
        let db = setup_db().await.unwrap();

        let mut product = ProductInput::mock();

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
