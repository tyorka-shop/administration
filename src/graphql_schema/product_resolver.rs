use async_graphql::{ComplexObject, Context};
use sqlx::{Result, SqlitePool};

use crate::entity::{multi_lang::MultiLang, picture::{self, Picture}, product::Product};

#[ComplexObject]
impl Product {

    // back compatibility
    async fn state(&self) -> Result<String> {
        Ok("DRAFT".into())
    }

    async fn cover(&self, ctx: &Context<'_>) -> Result<Picture> {
        let pool = ctx.data::<SqlitePool>().unwrap();
        let cover = picture::Entity::get_by_id(&pool, &self.cover_id).await?;
        Ok(cover.into())
    }

    async fn pictures(&self, ctx: &Context<'_>) -> Result<Vec<Picture>> {
        let db = ctx.data::<SqlitePool>().unwrap();
        Ok(Picture::get_by_product_id(db, &self.id).await.unwrap())
    }

    #[allow(non_snake_case)]
    async fn description_HTML(&self) -> MultiLang {
        self.description.to_html()
    }

    pub async fn description_text(&self) -> MultiLang {
        self.description.to_text()
    }
}
