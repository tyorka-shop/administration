#[derive(macros::Entity)]
#[table_name = "products"]
pub struct Product {
    pub id: String,
    pub cover_id: String,
    pub title_en: String,
    pub title_ru: String,
    pub description_en: String,
    pub description_ru: String,
    pub price: Option<i64>,
    pub show_in_gallery: bool,
    pub show_in_shop: bool,
}

impl Product {
    pub fn new_fixture() -> Self {
        Self {
            id: "07d7b72c-5b2e-4a35-a257-158496993dcc".into(),
            cover_id: format!("{:x}", md5::compute("cover_id")),
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
