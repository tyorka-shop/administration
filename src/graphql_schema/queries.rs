use crate::{
    entity::product::{self, Product},
    guard::RoleData,
};
use async_graphql::{Context, Object, Result};
use sqlx::SqlitePool;

pub struct Queries;

#[Object]
impl Queries {
    async fn status(&self) -> Result<String> {
        Ok("Ok".into())
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
    async fn product(&self, ctx: &Context<'_>, id: String) -> Result<Product> {
        let db = ctx.data::<SqlitePool>().unwrap();
        let product = product::Entity::get_by_id(db, &id).await.unwrap();
        Ok((&product).into())
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
    use assert_json_diff::assert_json_include;

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

        let query = r#"query Products { products { __typename id title { ...Localized } coverId showInGallery showInShop price description { ...Localized } } }
        fragment Localized on MultiLang { en ru }
        "#;

        let r = Request::new(query);

        let result = request(r, &db).await;

        assert_eq!(result.errors.first(), None);

        assert_json_include!(
            expected: result.data.into_json().unwrap(),
            actual: serde_json::json!({
                "products": [
                    {
                        "__typename": "Product",
                        "id": product.id,
                        "title": {
                            "en": product.title_en,
                            "ru": product.title_ru,
                        },
                        "coverId": null,
                        "showInGallery": true,
                        "showInShop": false,
                        "price": null,
                        "description": {
                            "en": product.description_en,
                            "ru": product.description_ru,
                        }
                    }
                ]

            })
        );
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

        let r = Request::new(query);

        let result = request(r, &db).await;

        assert_eq!(result.errors.first(), None);

        assert_json_include!(
            expected: result.data.into_json().unwrap(),
            actual: serde_json::json!({
                "products": [
                    {
                        "pictures": [
                            {
                                "id": picture.id,
                            }
                        ]
                    }
                ]

            })
        );
    }

    #[tokio::test]
    pub async fn by_id() {
        let db = setup_db().await.unwrap();

        let product = Entity::mock();

        product.insert(&db).await.unwrap();

        let query = r#"query Product($id: String!) { product(id: $id) { id } }"#;

        let vars = Variables::from_json(serde_json::json!({ "id": product.id }));
        let r = Request::new(query).variables(vars);

        let result = request(r, &db).await;

        assert_eq!(result.errors.first(), None);

        assert_json_include!(
            expected: result.data.into_json().unwrap(),
            actual: serde_json::json!({
                "product": {"id": product.id}
            })
        );
    }

    #[tokio::test]
    pub async fn description() {
        let db = setup_db().await.unwrap();

        let mut product = Entity::mock();

        product.description_en = "This was ~~erased~~ *deleted*.".to_string();

        product.insert(&db).await.unwrap();

        let query = r#"query Product($id: String!) { product(id: $id) { descriptionText { en } descriptionHTML { en } } }"#;

        let vars = Variables::from_json(serde_json::json!({ "id": product.id }));
        let r = Request::new(query).variables(vars);

        let result = request(r, &db).await;

        assert_eq!(result.errors.first(), None);

        assert_json_include!(
            expected: result.data.into_json().unwrap(),
            actual: serde_json::json!({
                "product": {
                    "descriptionText": {
                        "en": "This was  deleted."
                    },
                    "descriptionHTML": {
                        "en": "<p>This was ~~erased~~ <em>deleted</em>.</p>\n"
                    }
                }
            })
        );
    }

}
