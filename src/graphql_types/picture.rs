use super::{
    crop::{Crop, Point},
    picture_size::PictureSize,
};
use async_graphql::ID;
use serde::Serialize;

#[derive(Debug, Serialize, async_graphql::SimpleObject)]
pub struct Picture {
    pub id: ID,
    pub color: String,
    pub original_size: PictureSize,
    pub crop: Crop,
    #[serde(skip_serializing)]
    #[graphql(skip)]
    pub product_id: Option<String>,
}

impl From<entity::Picture> for Picture {
    fn from(row: entity::Picture) -> Self {
        Self {
            id: ID::from(row.id),
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
    pub fn new(filename: &str, width: i64, height: i64, dominant_color: &str, crop: &Crop) -> Self {
        Self {
            id: ID::from(filename),
            color: dominant_color.to_string(),
            original_size: PictureSize { width, height },
            crop: crop.clone(),
            product_id: None,
        }
    }
}

impl From<&Picture> for entity::Picture {
    fn from(pic: &Picture) -> Self {
        Self {
            id: pic.id.to_string(),
            color: pic.color.clone(),
            original_size_width: pic.original_size.width,
            original_size_height: pic.original_size.height,
            crop_anchor_x: pic.crop.anchor.x,
            crop_anchor_y: pic.crop.anchor.y,
            crop_factor: pic.crop.factor,
            product_id: pic.product_id.clone(),
            idx: None,
        }
    }
}

impl Picture {
    pub async fn get_by_product_id(db: &sqlx::SqlitePool, id: &ID) -> sqlx::Result<Vec<Self>> {
        let rows = sqlx::query_as!(
            entity::Picture,
            "select * from pictures where product_id = ? order by `idx`",
            id.0
        )
        .fetch_all(db)
        .await
        .unwrap()
        .into_iter()
        .map(|row| row.into())
        .collect::<Vec<Self>>();

        Ok(rows)
    }
}
