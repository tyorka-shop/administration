use crate::{
    entity::{
        product::{self, Product},
        user::User,
    },
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
        Ok(product::Entity::get_all(db)
            .await
            .unwrap()
            .iter()
            .map(Product::from)
            .collect())
    }

    #[graphql(guard = "RoleData::admin()")]
    async fn product(&self, ctx: &Context<'_>, id: ID) -> Result<Product> {
        let db = ctx.data::<SqlitePool>().unwrap();
        let product = product::Entity::get_by_id(db, &id.to_string())
            .await
            .unwrap();
        Ok((&product).into())
    }

    #[graphql(guard = "RoleData::admin()")]
    async fn shop(&self, ctx: &Context<'_>) -> Result<Vec<Product>> {
        let db = ctx.data::<SqlitePool>().unwrap();
        let products = sqlx::query_as!(
            product::Entity,
            "SELECT * FROM products WHERE show_in_shop = 1"
        )
        .fetch_all(db)
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
        let products = sqlx::query_as!(
            product::Entity,
            "SELECT * FROM products WHERE show_in_gallery = 1"
        )
        .fetch_all(db)
        .await
        .unwrap()
        .iter()
        .map(Product::from)
        .collect();
        Ok(products)
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
    use crate::{
        entity::{picture, product::Entity},
        guard::Role,
    };

    use super::Queries;
    use async_graphql::{EmptyMutation, EmptySubscription, Request, Result, Variables};
    use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

    async fn setup_db() -> Result<SqlitePool> {
        let db = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(":memory:")
            .await
            .unwrap();

        sqlx::migrate!().run(&db).await.unwrap();

        Ok(db)
    }

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
        let db = setup_db().await.unwrap();

        let product = Entity::mock();

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
        let db = setup_db().await.unwrap();

        let product = Entity::mock();

        let mut picture = picture::Entity::mock();
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
        let db = setup_db().await.unwrap();

        let mut product = Entity::mock();

        let picture = picture::Entity::mock();
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
        let db = setup_db().await.unwrap();

        let product = Entity::mock();

        product.insert(&db).await.unwrap();

        let query = r#"query Product($id: String!) { product(id: $id) { id } }"#;

        let vars = Variables::from_json(serde_json::json!({ "id": product.id }));

        let result = request(Request::new(query).variables(vars), &db).await;

        assert_eq!(result.errors.first(), None);

        insta::assert_json_snapshot!(result.data.into_json().unwrap());
    }

    #[tokio::test]
    pub async fn description() {
        let db = setup_db().await.unwrap();

        let mut product = Entity::mock();

        product.description_en = "This was ~~erased~~ *deleted*.".to_string();

        product.insert(&db).await.unwrap();

        let query = r#"query Product($id: String!) { product(id: $id) { descriptionText { en } descriptionHTML { en } } }"#;

        let vars = Variables::from_json(serde_json::json!({ "id": product.id }));

        let result = request(Request::new(query).variables(vars), &db).await;

        assert_eq!(result.errors.first(), None);

        insta::assert_json_snapshot!(result.data.into_json().unwrap());
    }
}
