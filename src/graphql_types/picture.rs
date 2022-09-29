use super::{
    crop::{Crop, Point},
    picture_size::PictureSize,
};
use async_graphql::ID;
use serde::Serialize;

#[derive(Debug, Serialize, async_graphql::SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Picture {
    pub id: ID,
    pub src: String,
    pub color: String,
    pub original_size: PictureSize,
    pub crop: Crop,
}

impl From<entity::Picture> for Picture {
    fn from(row: entity::Picture) -> Self {
        Self {
            id: ID::from(row.id.clone()),
            src: row.id,
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
        }
    }
}

impl Picture {
    pub fn new(filename: &str, width: i64, height: i64, dominant_color: &str, crop: &Crop) -> Self {
        Self {
            id: ID::from(filename),
            src: filename.to_string(),
            color: dominant_color.to_string(),
            original_size: PictureSize { width, height },
            crop: crop.clone(),
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
        }
    }
}

impl Picture {
    pub async fn get_by_product_id(db: &sqlx::SqlitePool, id: &ID) -> sqlx::Result<Vec<Self>> {
        let rows = sqlx::query_as!(
            entity::Picture,
            "select p.* from pictures p join product_pictures pp on pp.picture_id = p.id where pp.product_id = ? order by pp.`idx`",
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
