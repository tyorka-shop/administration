mod insta;
mod download;
mod types;

#[tokio::main]
async fn main() {
  env_logger::init();
  let cfg = config::load("tyorka-admin");

  match cfg.insta {
    None => {return ();},
    Some(insta) => {
      
      let db = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&cfg.database_uri)
        .await
        .unwrap();
      
      
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
    }
  }
}