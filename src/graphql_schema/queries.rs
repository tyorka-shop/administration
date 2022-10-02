use crate::{
    builder::Builder,
    graphql_types::{BlogPost, Build, Picture, Product, User},
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

    #[graphql(guard = "RoleData::admin()")]
    async fn user(&self, ctx: &Context<'_>) -> Result<User> {
        let role = ctx.data::<Role>().unwrap();
        match role {
            Role::Admin(email) => Ok(User {
                email: email.clone(),
            }),
            _ => Err("Unauthorized".into()),
        }
    }

    #[graphql(guard = "RoleData::admin()")]
    async fn products(&self, ctx: &Context<'_>) -> Result<Vec<Product>> {
        let db = ctx.data::<SqlitePool>().unwrap();

        let products = sqlx::query_as!(entity::Product, "select * from `products` order by `created_at` desc")
            .fetch_all(db)
            .await?
            .into_iter()
            .map(Product::from)
            .collect();

        Ok(products)
    }

    #[graphql(guard = "RoleData::admin()")]
    async fn product(&self, ctx: &Context<'_>, id: ID) -> Result<Option<Product>> {
        let db = ctx.data::<SqlitePool>().unwrap();
        let product = entity::Product::get_by_id(db, &id.to_string()).await;
        match product {
            Ok(product) => Ok(Some(Product::from(product))),
            Err(_) => Ok(None),
        }
    }

    #[graphql(guard = "RoleData::admin()")]
    async fn picture(&self, ctx: &Context<'_>, id: ID) -> Result<Option<Picture>> {
        let db = ctx.data::<SqlitePool>().unwrap();
        let pic = entity::Picture::get_by_id(db, &id.to_string()).await;
        match pic {
            Ok(pic) => Ok(Some(Picture::from(pic))),
            Err(_) => Ok(None),
        }
    }

    #[graphql(guard = "RoleData::admin()")]
    async fn shop(&self, ctx: &Context<'_>) -> Result<Vec<Product>> {
        let db = ctx.data::<SqlitePool>().unwrap();

        let products = entity::Product::get_shop(db)
            .await
            .unwrap()
            .into_iter()
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
            .into_iter()
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

    #[graphql(guard = "RoleData::admin()")]
    async fn current_build(&self, ctx: &Context<'_>) -> Result<Option<Build>> {
        let db = ctx.data::<SqlitePool>().unwrap();
        let builder = ctx.data::<Builder>().unwrap();
        let id = builder.current_build();
        match id {
            Some(id) => {
                let build = entity::Build::get_by_id(db, &id).await.unwrap();
                Ok(Some(build.into()))
            }
            None => Ok(None),
        }
    }

    #[graphql(guard = "RoleData::admin()")]
    async fn publications(&self, ctx: &Context<'_>) -> Result<Vec<Build>> {
        let db = ctx.data::<SqlitePool>().unwrap();

        let builds = sqlx::query_as!(entity::Build, "SELECT * FROM `build` order by `created_at` desc")
            .fetch_all(db)
            .await
            .unwrap()
            .into_iter()
            .map(Build::from)
            .collect();
        Ok(builds)
    }

    #[graphql(guard = "RoleData::admin()")]
    async fn publication(&self, ctx: &Context<'_>, id: ID) -> Result<Option<Build>> {
        let db = ctx.data::<SqlitePool>().unwrap();

        let build = entity::Build::get_by_id(&db, &id.to_string()).await;
        match build {
            Ok(build) => Ok(Some(build.into())),
            Err(_) => Ok(None),
        }
    }

    #[graphql(guard = "RoleData::admin()")]
    async fn is_draft(&self, _ctx: &Context<'_>) -> Result<bool> {
        // let db = ctx.data::<SqlitePool>().unwrap();
        Ok(true)
    }
    
    #[graphql(guard = "RoleData::admin()")]
    async fn publication_duration(&self, ctx: &Context<'_>) -> Result<i32> {
        let db = ctx.data::<SqlitePool>().unwrap();
        match sqlx::query!("select `created_at`, `updated_at` from `build` where `status` = 'DONE'").fetch_all(db).await {
            Ok(rows) => {
                let len = rows.len() as i64;
                if len == 0 {
                    return Ok(60_000);
                }
                let sum: i64 = rows.into_iter().map(|row| row.updated_at.timestamp_millis() - row.created_at.timestamp_millis()).sum();
                Ok((sum /  len as i64) as i32)
            },
            Err(_) => Ok(60_000),
        }
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
            "Unauthorized"
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
    pub async fn order() {
        let db = crate::test_utils::setup_db().await.unwrap();

        let mut product = entity::Product::new_fixture();
        product.id = "07d7b72c-5b2e-4a35-a257-158496993dcc".into();
        product.created_at = chrono::NaiveDateTime::from_timestamp(1_000_000_000, 0);
        product.insert_all(&db).await.unwrap();
        
        product = entity::Product::new_fixture();
        product.id = "17d7b72c-5b2e-4a35-a257-158496993dcc".into();
        product.created_at = chrono::NaiveDateTime::from_timestamp(2_000_000_000, 0);
        product.insert_all(&db).await.unwrap();

        let query = r#"query Products { products { id } }"#;

        let result = request(Request::new(query), &db).await;

        assert_eq!(result.errors.first(), None);

        insta::assert_json_snapshot!(result.data.into_json().unwrap());
    }

    #[tokio::test]
    pub async fn pictures() {
        let db = crate::test_utils::setup_db().await.unwrap();

        let product = entity::Product::new_fixture();

        let picture = entity::Picture::new_fixture();

        picture.insert(&db).await.unwrap();
        product.insert(&db).await.unwrap();

        sqlx::query!(
            "INSERT INTO `product_pictures` (`product_id`, `picture_id`, `idx`) VALUES ($1, $2, 0)",
            product.id,
            picture.id
        ).execute(&db).await.unwrap();

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
mod picture_order {
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

    async fn setup(list: [i64; 2]) -> Result<SqlitePool> {
        let db = crate::test_utils::setup_db().await.unwrap();

        let product = entity::Product::new_fixture();
        product.insert(&db).await.unwrap();

        for i in 1..3 {
            let id = i.to_string();
            let mut picture = entity::Picture::new_fixture();
            picture.id = id;

            picture.insert(&db).await.unwrap();

            sqlx::query!(
                r#"
                INSERT INTO `product_pictures` (`product_id`, `picture_id`, `idx`)
                VALUES ($1, $2, $3)
                "#,
                product.id,
                picture.id,
                list[i - 1]
            ).execute(&db).await.unwrap();
        }

        Ok(db)
    }

    #[tokio::test]
    async fn asc() {
        let db = setup([1, 2]).await.unwrap();

        let query = r#"query Products { products { pictures { id } } }"#;

        let result = request(query, &db).await;

        insta::assert_json_snapshot!(result.data.into_json().unwrap());
    }

    #[tokio::test]
    async fn desc() {
        let db = setup([2, 1]).await.unwrap();

        let query = r#"query Products { products { pictures { id } } }"#;

        let result = request(query, &db).await;

        insta::assert_json_snapshot!(result.data.into_json().unwrap());
    }

    #[tokio::test]
    async fn cover_is_first() {
        let db = setup([1, 2]).await.unwrap();

        let mut product = entity::Product::new_fixture();
        product.cover_id = "2".to_string();
        product.insert_or_update(&db).await.unwrap();


        let query = r#"query Products { products { pictures { id } } }"#;

        let result = request(query, &db).await;

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
