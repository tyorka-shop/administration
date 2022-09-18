use crate::{
    graphql_types::{BlogPost, Product, User},
    guard::{Role, RoleData},
};
use async_graphql::{Context, Object, Result, ID};
use sqlx::SqlitePool;

pub struct Queries;

#[Object]
impl Queries {
    async fn status(&self) -> Result<String> {
        Ok("Ok".into())
    }

    async fn user(&self, ctx: &Context<'_>) -> Result<User> {
        let role = ctx.data::<Role>().unwrap();
        match role {
            Role::Admin(email) => Ok(User {
                email: email.clone(),
            }),
            _ => Ok(User {
                email: "client".into(),
            }),
        }
    }

    #[graphql(guard = "RoleData::admin()")]
    async fn products(&self, ctx: &Context<'_>) -> Result<Vec<Product>> {
        let db = ctx.data::<SqlitePool>().unwrap();
        Ok(entity::Product::get_all(db)
            .await
            .unwrap()
            .iter()
            .map(Product::from)
            .collect())
    }

    #[graphql(guard = "RoleData::admin()")]
    async fn product(&self, ctx: &Context<'_>, id: ID) -> Result<Product> {
        let db = ctx.data::<SqlitePool>().unwrap();
        let product = entity::Product::get_by_id(db, &id.to_string())
            .await
            .unwrap();
        Ok((&product).into())
    }

    #[graphql(guard = "RoleData::admin()")]
    async fn shop(&self, ctx: &Context<'_>) -> Result<Vec<Product>> {
        let db = ctx.data::<SqlitePool>().unwrap();

        let products = entity::Product::get_shop(db)
            .await
            .unwrap()
            .iter()
            .map(Product::from)
            .collect();
        Ok(products)
    }

    #[graphql(guard = "RoleData::admin()")]
    async fn gallery(&self, ctx: &Context<'_>) -> Result<Vec<Product>> {
        let db = ctx.data::<SqlitePool>().unwrap();

        let products = entity::Product::get_gallery(&db)
            .await
            .unwrap()
            .iter()
            .map(Product::from)
            .collect();
        Ok(products)
    }

    #[graphql(guard = "RoleData::admin()")]
    async fn blog(&self, ctx: &Context<'_>) -> Result<Vec<BlogPost>> {
        let db = ctx.data::<SqlitePool>().unwrap();
        let posts = sqlx::query_as!(entity::BlogPost, "SELECT * FROM blog")
            .fetch_all(db)
            .await
            .unwrap()
            .iter()
            .map(BlogPost::from)
            .collect();
        Ok(posts)
    }
}

#[cfg(test)]
mod role {
    use crate::guard::{Role, RoleData};
    use async_graphql::{EmptyMutation, EmptySubscription, Object, Request, Result};

    struct Queries;

    #[Object]
    impl Queries {
        #[graphql(guard = "RoleData::admin()")]
        async fn only_for_admin(&self) -> Result<String> {
            Ok("Ok".into())
        }
    }

    #[tokio::test]
    pub async fn admin() {
        let request = Request::new("query TestRole { onlyForAdmin }");

        let schema =
            async_graphql::Schema::build(Queries, EmptyMutation, EmptySubscription).finish();

        let response = schema
            .execute(request.data(Role::Admin("admin".to_string())))
            .await;

        assert_eq!(response.errors.first(), None);
    }

    #[tokio::test]
    pub async fn client() {
        let request = Request::new("query TestRole { onlyForAdmin }");

        let schema =
            async_graphql::Schema::build(Queries, EmptyMutation, EmptySubscription).finish();

        let response = schema.execute(request.data(Role::Client)).await;

        assert_eq!(
            response.errors.first().unwrap().message,
            "Permission denied"
        );
    }
}

#[cfg(test)]
mod products {
    use crate::guard::Role;

    use super::Queries;
    use async_graphql::{EmptyMutation, EmptySubscription, Request, Variables};
    use sqlx::SqlitePool;

    async fn request(request: Request, db: &SqlitePool) -> async_graphql::Response {
        let schema =
            async_graphql::Schema::build(Queries, EmptyMutation, EmptySubscription).finish();

        schema
            .execute(
                request
                    .data(db.clone())
                    .data(Role::Admin("admin".to_string())),
            )
            .await
    }

    #[tokio::test]
    pub async fn valid() {
        let db = crate::test_utils::setup_db().await.unwrap();

        let product = entity::Product::new_fixture();

        product.insert(&db).await.unwrap();

        let query = r#"query Products { products { __typename id title { ...Localized } showInGallery showInShop price description { ...Localized } } }
        fragment Localized on MultiLang { en ru }
        "#;

        let result = request(Request::new(query), &db).await;

        assert_eq!(result.errors.first(), None);

        insta::assert_json_snapshot!(result.data.into_json().unwrap());
    }

    #[tokio::test]
    pub async fn pictures() {
        let db = crate::test_utils::setup_db().await.unwrap();

        let product = entity::Product::new_fixture();

        let mut picture = entity::Picture::new_fixture();
        picture.product_id = Some(product.id.clone());

        picture.insert(&db).await.unwrap();
        product.insert(&db).await.unwrap();

        let query = r#"query Products { products { pictures { id } } }"#;

        let result = request(Request::new(query), &db).await;

        assert_eq!(result.errors.first(), None);

        insta::assert_json_snapshot!(result.data.into_json().unwrap());
    }

    #[tokio::test]
    pub async fn cover() {
        let db = crate::test_utils::setup_db().await.unwrap();

        let mut product = entity::Product::new_fixture();

        let picture = entity::Picture::new_fixture();
        product.cover_id = picture.id.clone();

        product.insert(&db).await.unwrap();
        picture.insert(&db).await.unwrap();

        let query = r#"query Products { products { cover { id } } }"#;

        let result = request(Request::new(query), &db).await;

        assert_eq!(result.errors.first(), None);

        insta::assert_json_snapshot!(result.data.into_json().unwrap());
    }

    #[tokio::test]
    pub async fn by_id() {
        let db = crate::test_utils::setup_db().await.unwrap();

        let product = entity::Product::new_fixture();

        product.insert(&db).await.unwrap();

        let query = r#"query Product($id: String!) { product(id: $id) { id } }"#;

        let vars = Variables::from_json(serde_json::json!({ "id": product.id }));

        let result = request(Request::new(query).variables(vars), &db).await;

        assert_eq!(result.errors.first(), None);

        insta::assert_json_snapshot!(result.data.into_json().unwrap());
    }

    #[tokio::test]
    pub async fn description() {
        let db = crate::test_utils::setup_db().await.unwrap();

        let mut product = entity::Product::new_fixture();

        product.description_en = "This was ~~erased~~ *deleted*.".to_string();

        product.insert(&db).await.unwrap();

        let query = r#"query Product($id: String!) { product(id: $id) { descriptionText { en } descriptionHTML { en } } }"#;

        let vars = Variables::from_json(serde_json::json!({ "id": product.id }));

        let result = request(Request::new(query).variables(vars), &db).await;

        assert_eq!(result.errors.first(), None);

        insta::assert_json_snapshot!(result.data.into_json().unwrap());
    }
}

#[cfg(test)]
mod entity_order {
    use async_graphql::{EmptyMutation, EmptySubscription, Request, Response, Result};
    use sqlx::SqlitePool;

    use crate::guard::Role;

    use super::Queries;

    async fn request(query: &str, db: &SqlitePool) -> Response {
        let schema =
            async_graphql::Schema::build(Queries, EmptyMutation, EmptySubscription).finish();

        let r = Request::new(query);

        let response = schema
            .execute(r.data(db.clone()).data(Role::Admin("admin".to_string())))
            .await;
        assert_eq!(response.errors.first(), None);
        response
    }

    async fn setup(entity_type: &str, list: [i32; 2]) -> Result<SqlitePool> {
        let db = crate::test_utils::setup_db().await.unwrap();

        for i in 1..3 {
            let id = i.to_string();
            let mut product = entity::Product::new_fixture_with_id(&id);
            product.show_in_gallery = true;
            product.show_in_shop = true;
            product.price = Some(100);
            product.insert(&db).await.unwrap();

            sqlx::query!(
                "insert into `entity_order` (`type`, `entity_id`, `idx`) values ($1, $2, $3)",
                entity_type,
                id,
                list[i - 1]
            )
            .execute(&db)
            .await
            .unwrap();
        }
        Ok(db)
    }

    #[tokio::test]
    async fn gallery_asc() {
        let db = setup("gallery", [1, 2]).await.unwrap();

        let response = request("query Gallery { gallery { id } }", &db).await;

        insta::assert_json_snapshot!(response.data.into_json().unwrap());
    }

    #[tokio::test]
    async fn gallery_desc() {
        let db = setup("gallery", [2, 1]).await.unwrap();

        let response = request("query Gallery { gallery { id } }", &db).await;

        insta::assert_json_snapshot!(response.data.into_json().unwrap());
    }

    #[tokio::test]
    async fn shop_asc() {
        let db = setup("shop", [1, 2]).await.unwrap();

        let response = request("query Shop { shop { id } }", &db).await;

        insta::assert_json_snapshot!(response.data.into_json().unwrap());
    }

    #[tokio::test]
    async fn shop_desc() {
        let db = setup("shop", [2, 1]).await.unwrap();

        let response = request("query Shop { shop { id } }", &db).await;

        insta::assert_json_snapshot!(response.data.into_json().unwrap());
    }
}
