use crate::{
    entity::product::{self, Product},
    guard::{Role, RoleData},
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
    async fn role(&self, ctx: &Context<'_>) -> Result<String> {
        let role = ctx.data::<Role>().unwrap();
        match role {
            Role::Admin(email) => Ok(format!("admin ({})", &email)),
            _ => Ok("client".into()),
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
    async fn product(&self, ctx: &Context<'_>, id: String) -> Result<Product> {
        let db = ctx.data::<SqlitePool>().unwrap();
        let product = product::Entity::get_by_id(db, &id).await.unwrap();
        Ok((&product).into())
    }
}
