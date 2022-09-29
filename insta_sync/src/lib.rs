use std::io::Result;

use config::InstaConfig;
use sqlx::SqlitePool;

mod download;
mod insta;
mod types;

pub async fn sync(cfg: &InstaConfig, images_folder: &str ,db: &SqlitePool) -> Result<()> {
    let posts = insta::get_posts(
        &cfg.access_token,
        &cfg.instagram_id,
        12,
        images_folder,
    )
    .await
    .unwrap();

    entity::BlogPost::clear(&db).await.unwrap();

    for post in posts {
        post.insert_or_ignore(&db).await.unwrap();
    }

    Ok(())
}
