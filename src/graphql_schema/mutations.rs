use async_graphql::{Context, Object, Result};
use sqlx::SqlitePool;

use crate::entity::product::{self, ProductInput};

pub struct Mutations;

#[Object]
impl Mutations {
    async fn save_product<'a>(&self, ctx: &Context<'a>, product: ProductInput) -> Result<String> {
        let db = ctx.data::<SqlitePool>().unwrap();
        product::Entity::from(product).insert(db).await?;

        Ok("ok".into())
    }
}
