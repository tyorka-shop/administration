use super::multi_lang::MultiLang;

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

#[derive(async_graphql::InputObject)]
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

impl From<ProductInput> for Entity {
    fn from(input: ProductInput) -> Self {
        Self {
            id: input.id,
            title_en: input.title.en,
            title_ru: input.title.ru,
            description_en: input.description.en,
            description_ru: input.description.ru,
            price: input.price,
            show_in_gallery: input.show_in_gallery,
            show_in_shop: input.show_in_shop,
            cover_id: input.cover_id,
        }
    }
}
