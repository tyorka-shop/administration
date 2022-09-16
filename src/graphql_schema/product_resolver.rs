use sqlx::{query_as, Result, SqlitePool};

use crate::entity::picture::{Entity, Picture};

impl Picture {
    pub async fn get_by_product_id(db: &SqlitePool, id: &str) -> Result<Vec<Self>> {
        let rows = query_as!(Entity, "select * from pictures where product_id = ?", id)
            .fetch_all(db)
            .await
            .unwrap()
            .into_iter()
            .map(|row| row.into())
            .collect::<Vec<Self>>();

        Ok(rows)
    }
}
