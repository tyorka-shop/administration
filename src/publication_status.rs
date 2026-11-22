use crossbeam_channel::{Receiver, Sender};
use sqlx::SqlitePool;

use crate::builder::Builder;

#[derive(Clone)]
pub struct PublicationStatus {
    tx: Sender<bool>,
    rx: Receiver<bool>,
}

#[async_trait::async_trait]
pub trait PublicationStatusTrait {
    fn new() -> Self;

    fn set_draft(&self);

    fn recv(&self) -> bool;
}

#[async_trait::async_trait]
impl PublicationStatusTrait for PublicationStatus {
    fn new() -> Self {
        let (tx, rx) = crossbeam_channel::unbounded();
        Self { tx, rx }
    }

    fn set_draft(&self) {
        self.tx.send(true).unwrap();
    }

    fn recv(&self) -> bool {
        self.rx.recv().unwrap()
    }
}

impl PublicationStatus {
    pub async fn is_draft(db: &SqlitePool, builder: &Builder) -> eyre::Result<bool> {
        let Some(build) = builder.latest_successful_build().await? else {
            return Ok(false);
        };
        let count: i64 = sqlx::query_scalar(
            "select count(*) from (
                select max(updated_at) as updated_at from products
                union all
                select max(updated_at) from entity_order
                union all
                select max(updated_at) from product_pictures
            ) where julianday(updated_at) > julianday($1)",
        )
        .bind(build.started_at.to_rfc3339())
        .fetch_one(db)
        .await?;
        Ok(count > 0)
    }
}
