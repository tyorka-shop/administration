use super::{multi_lang::MultiLang, ProductState};
use async_graphql::ID;
use serde::Serialize;

#[derive(async_graphql::SimpleObject)]
#[graphql(complex)]
pub struct Product {
    pub id: ID,
    #[graphql(skip)]
    pub cover_id: String,
    pub title: MultiLang,
    pub show_in_gallery: bool,
    pub show_in_shop: bool,
    pub price: Option<i64>,
    pub description: MultiLang,
}

impl From<entity::Product> for Product {
    fn from(row: entity::Product) -> Self {
        Self {
            id: ID::from(&row.id),
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
    pub id: ID,
    pub state: ProductState,
    pub pictures: Vec<ID>,
    pub cover_id: ID,
    pub title: MultiLang,
    pub show_in_gallery: bool,
    pub show_in_shop: bool,
    pub price: Option<i64>,
    pub description: MultiLang,
}

impl From<&ProductInput> for entity::Product {
    fn from(input: &ProductInput) -> Self {
        Self {
            id: input.id.to_string(),
            title_en: input.title.en.clone(),
            title_ru: input.title.ru.clone(),
            description_en: input.description.en.clone(),
            description_ru: input.description.ru.clone(),
            price: input.price,
            show_in_gallery: input.show_in_gallery,
            show_in_shop: input.show_in_shop,
            cover_id: input.cover_id.to_string(),
        }
    }
}

#[cfg(test)]
impl ProductInput {
    pub fn new_fixture() -> Self {
        let pic_id = ID::from(format!("{:x}", md5::compute("cover_id")));
        Self {
            id: "07d7b72c-5b2e-4a35-a257-158496993dcc".into(),
            state: ProductState::Draft,
            pictures: vec![pic_id.clone()],
            cover_id: pic_id,
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
