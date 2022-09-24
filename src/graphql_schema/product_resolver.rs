use async_graphql::{ComplexObject, Context};
use sqlx::{Result, SqlitePool};

use crate::graphql_types::{MultiLang, Picture, Product, ProductState};

#[ComplexObject]
impl Product {
    // back compatibility
    async fn state(&self) -> Result<ProductState> {
        Ok(ProductState::Draft)
    }

    async fn cover(&self, ctx: &Context<'_>) -> Result<Picture> {
        let pool = ctx.data::<SqlitePool>().unwrap();
        let cover = entity::Picture::get_by_id(&pool, &self.cover_id).await?;
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
