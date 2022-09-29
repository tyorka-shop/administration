use std::io::Result;

use sqlx::SqlitePool;

mod insta;
mod download;
mod types;

pub async fn sync(cfg: &config::Config, db: &SqlitePool) -> Result<()> {
  match cfg.insta {
    None => {return Ok(());},
    Some(ref insta) => {
      
      let posts = insta::get_posts(
          &insta.access_token,
          &insta.instagram_id,
          12,
          &cfg.images_folder,
      )
      .await
      .unwrap();
  
      entity::BlogPost::clear(&db).await.unwrap();
  
      for post in posts {
          post.insert_or_ignore(&db).await.unwrap();
      }

      Ok(())
    }
  }
}