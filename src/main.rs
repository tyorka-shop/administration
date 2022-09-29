mod builder;
mod cached_session_service;
mod graphql_schema;
mod graphql_types;
mod guard;
mod image_storage;
mod web;

#[cfg(test)]
mod test_utils;

use image_storage::ImageStorage;
use sqlx::sqlite::SqlitePoolOptions;

pub const PIC_SIZES: &[u32] = &[200, 600, 2000];

#[tokio::main]
async fn main() {
    env_logger::init();
    let cfg = config::load("tyorka-admin");

    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&cfg.database_uri)
        .await
        .unwrap();
        
    sqlx::migrate!().run(&db).await.unwrap();

    let images = ImageStorage::new(&cfg.images_folder, PIC_SIZES.into()).unwrap();

    let web = web::make_server(cfg.clone(), db, images);

    let result = tokio::join!(web.await);

    println!("result: {:?}", result);
}
