use crossbeam_channel::{Sender, Receiver};
use sqlx::{SqlitePool, Result};

#[derive(Clone)]
pub struct PublicationStatus {
  tx: Sender<bool>,
  rx: Receiver<bool>,
}

#[async_trait::async_trait]
pub trait PublicationStatusTrait  {
  fn new() -> Self;

  fn set_draft(&self);

  fn set_published(&self);

  fn recv(&self) -> bool;

  async fn is_draft(db: &SqlitePool) -> Result<bool>;
}
  

#[async_trait::async_trait]
impl PublicationStatusTrait for PublicationStatus {
  fn new() -> Self {
    let (tx, rx) = crossbeam_channel::unbounded();
    Self { tx, rx }
  }

  fn set_draft(&self){
    self.tx.send(true).unwrap();
  }

  fn set_published(&self){
    self.tx.send(false).unwrap();
  }

  fn recv(&self) -> bool {
    self.rx.recv().unwrap()
  }

  async fn is_draft(db: &SqlitePool) -> Result<bool> {
    let row = sqlx::query!(
        r#"
    select count(updated_at) as cnt 
    from (
        select max(updated_at) as updated_at from products 
        union all
        select max(updated_at) as updated_at from entity_order
        union all
        select max(updated_at) as updated_at from product_pictures
    )
    where updated_at > (select max(created_at) from build where status = 'DONE')
    "#
    )
    .fetch_one(db)
    .await?;

    match row.cnt {
        Some(0) | None => Ok(false),
        _ => Ok(true),
    }

  }
}