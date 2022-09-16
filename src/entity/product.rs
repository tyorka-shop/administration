use super::multi_lang::MultiLang;
use async_graphql::Result;
use serde::Serialize;

#[derive(macros::Entity)]
#[table_name = "products"]
pub struct Entity {
    pub id: String,
    pub cover_id: Option<String>,
    pub title_en: String,
    pub title_ru: String,
    pub description_en: String,
    pub description_ru: String,
    pub price: Option<i64>,
    pub show_in_gallery: bool,
    pub show_in_shop: bool,
}

#[derive(async_graphql::SimpleObject)]
#[graphql(complex)]
pub struct Product {
    pub id: String,
    pub cover_id: Option<String>,
    pub title: MultiLang,
    pub show_in_gallery: bool,
    pub show_in_shop: bool,
    pub price: Option<i64>,
    pub description: MultiLang,
}

impl From<&Entity> for Product {
    fn from(row: &Entity) -> Self {
        Self {
            id: row.id.clone(),
            cover_id: row.cover_id.clone(),
            description: MultiLang {
                en: row.description_en.clone(),
                ru: row.description_ru.clone(),
            },
            price: row.price,
            show_in_gallery: row.show_in_gallery,
            show_in_shop: row.show_in_shop,
            title: MultiLang {
                en: row.title_en.clone(),
                ru: row.title_ru.clone(),
            },
        }
    }
}

#[derive(async_graphql::InputObject, Serialize, Clone)]
pub struct ProductInput {
    pub id: String,
    pub pictures: Vec<String>,
    pub cover_id: Option<String>,
    pub title: MultiLang,
    pub show_in_gallery: bool,
    pub show_in_shop: bool,
    pub price: Option<i64>,
    pub description: MultiLang,
}

impl From<&ProductInput> for Entity {
    fn from(input: &ProductInput) -> Self {
        Self {
            id: input.id.clone(),
            title_en: input.title.en.clone(),
            title_ru: input.title.ru.clone(),
            description_en: input.description.en.clone(),
            description_ru: input.description.ru.clone(),
            price: input.price,
            show_in_gallery: input.show_in_gallery,
            show_in_shop: input.show_in_shop,
            cover_id: input.cover_id.clone(),
        }
    }
}

#[cfg(test)]
impl ProductInput {
    pub fn mock() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            pictures: vec!["bc1e6f801c0a2657af7eeb23638fd5b8".to_string()],
            cover_id: None,
            title: MultiLang {
                en: "title".to_string(),
                ru: "заголовок".to_string(),
            },
            show_in_gallery: true,
            show_in_shop: false,
            price: None,
            description: MultiLang {
                en: "description".to_string(),
                ru: "описание".to_string(),
            },
        }
    }
}

#[cfg(test)]
impl Entity {
    pub fn mock() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            cover_id: None,
            title_en: "title".to_string(),
            title_ru: "заголовок".to_string(),
            description_en: "description".to_string(),
            description_ru: "описание".to_string(),
            price: None,
            show_in_gallery: true,
            show_in_shop: false,
        }
    }
}
