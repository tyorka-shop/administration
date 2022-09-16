use serde::Serialize;

#[derive(macros::Entity)]
#[table_name = "pictures"]
pub struct Entity {
    pub id: String,
    pub color: String,
    pub original_size_width: i64,
    pub original_size_height: i64,
    pub crop_anchor_x: f64,
    pub crop_anchor_y: f64,
    pub crop_factor: f64,
    pub product_id: Option<String>,
}

use super::{
    crop::{Crop, Point},
    picture_size::PictureSize,
};

#[derive(Debug, Serialize, async_graphql::SimpleObject)]
pub struct Picture {
    pub id: String,
    pub color: String,
    pub original_size: PictureSize,
    pub crop: Crop,
    #[serde(skip_serializing)]
    #[graphql(skip)]
    pub product_id: Option<String>,
}

impl From<Entity> for Picture {
    fn from(row: Entity) -> Self {
        Self {
            id: row.id,
            color: row.color,
            original_size: PictureSize {
                width: row.original_size_width,
                height: row.original_size_height,
            },
            crop: Crop {
                anchor: Point {
                    x: row.crop_anchor_x,
                    y: row.crop_anchor_y,
                },
                factor: row.crop_factor,
            },
            product_id: row.product_id,
        }
    }
}

impl Picture {
    pub fn new(filename: &str, width: i64, height: i64, dominant_color: &str) -> Self {
        Self {
            id: filename.to_string(),
            color: dominant_color.to_string(),
            original_size: PictureSize { width, height },
            crop: Crop::default_square(width as u32, height as u32),
            product_id: None,
        }
    }
}

impl From<&Picture> for Entity {
    fn from(pic: &Picture) -> Self {
        Self {
            id: pic.id.clone(),
            color: pic.color.clone(),
            original_size_width: pic.original_size.width,
            original_size_height: pic.original_size.height,
            crop_anchor_x: pic.crop.anchor.x,
            crop_anchor_y: pic.crop.anchor.y,
            crop_factor: pic.crop.factor,
            product_id: pic.product_id.clone(),
        }
    }
}

impl Picture {
    pub async fn get_by_product_id(db: &sqlx::SqlitePool, id: &str) -> sqlx::Result<Vec<Self>> {
        let rows = sqlx::query_as!(Entity, "select * from pictures where product_id = ?", id)
            .fetch_all(db)
            .await
            .unwrap()
            .into_iter()
            .map(|row| row.into())
            .collect::<Vec<Self>>();

        Ok(rows)
    }
}



#[cfg(test)]
impl Entity {
    pub fn mock() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            color: "#000000".into(),
            original_size_width: 100,
            original_size_height: 100,
            crop_anchor_x: 0.5,
            crop_anchor_y: 0.5,
            crop_factor: 1.0,
            product_id: None,
        }
    }
}