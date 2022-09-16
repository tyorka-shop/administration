use async_graphql::{ComplexObject, Context};
use sqlx::{Result, SqlitePool};

use crate::entity::{multi_lang::MultiLang, picture::Picture, product::Product};

#[ComplexObject]
impl Product {
    async fn pictures(&self, ctx: &Context<'_>) -> Result<Vec<Picture>> {
        let db = ctx.data::<SqlitePool>().unwrap();
        Ok(Picture::get_by_product_id(db, &self.id).await.unwrap())
    }

    #[allow(non_snake_case)]
    async fn description_HTML(&self) -> MultiLang {
        MultiLang {
            en: markdown::to_html(&self.description.en),
            ru: markdown::to_html(&self.description.ru),
        }
    }

    pub async fn description_text(&self) -> MultiLang {
        MultiLang {
            en: markdown_to_text::convert(&self.description.en),
            ru: markdown_to_text::convert(&self.description.ru),
        }
    }
}
