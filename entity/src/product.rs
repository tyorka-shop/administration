use sqlx::SqlitePool;

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
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
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
            created_at: chrono::NaiveDateTime::from_timestamp(0, 0),
            updated_at: chrono::NaiveDateTime::from_timestamp(0, 0),
        }
    }

    pub fn new_fixture_with_id(id: &str) -> Self {
        let mut result = Self::new_fixture();
        result.id = id.into();
        result
    }

    pub async fn get_gallery(db: &SqlitePool) -> Result<Vec<Product>, sqlx::Error> {
        let products = sqlx::query_as!(
            Product,
            r#"
            SELECT `products`.* 
            FROM `products` 
            left join `entity_order` 
                on `entity_order`.`entity_id` = `products`.`id` and `entity_order`.`type` = "gallery"
            WHERE `products`.`show_in_gallery` = 1
            order by `entity_order`.`idx` asc
            "#
        )
        .fetch_all(db)
        .await
        .unwrap();

        Ok(products)
    }

    pub async fn get_shop(db: &SqlitePool) -> Result<Vec<Product>, sqlx::Error> {
        let products = sqlx::query_as!(
            Product,
            r#"
            SELECT `products`.* 
            FROM `products` 
            left join `entity_order` 
                on `entity_order`.`entity_id` = `products`.`id` and `entity_order`.`type` = "shop"
            WHERE `products`.`show_in_shop` = 1
            order by `entity_order`.`idx` asc
            "#
        )
        .fetch_all(db)
        .await
        .unwrap();

        Ok(products)
    }

    pub async fn save_order(
        db: &SqlitePool,
        entity_type: &str,
        list: &Vec<String>,
    ) -> Result<(), sqlx::Error> {
        let mut tx = db.begin().await?;
        let entity_type = entity_type.to_string();
        sqlx::query!(r#"delete from entity_order where `type` = $1"#, entity_type)
            .execute(&mut tx)
            .await
            .unwrap();

        for (i, id) in list.iter().enumerate() {
            let index = i.to_string();
            let id = id.to_string();
            match sqlx::query!(
                r#"insert into `entity_order` (`entity_id`, `type`, `idx`) values ($1, $2, $3)"#,
                id,
                entity_type,
                index
            )
            .execute(&mut tx)
            .await
            {
                Err(e) => {
                    log::error!("{}", e);
                    tx.rollback().await?;
                    return Err(e);
                }
                _ => {}
            };
        }
        tx.commit().await?;

        Ok(())
    }

    pub async fn save_gallery_order(
        db: &SqlitePool,
        list: &Vec<String>,
    ) -> Result<(), sqlx::Error> {
        Self::save_order(db, "gallery", list).await
    }

    pub async fn save_shop_order(db: &SqlitePool, list: &Vec<String>) -> Result<(), sqlx::Error> {
        Self::save_order(db, "shop", list).await
    }
}
