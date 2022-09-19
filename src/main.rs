mod cached_session_service;
mod graphql_types;
mod graphql_schema;
mod guard;
mod web;
mod image_storage;
mod builder;

#[cfg(test)]
mod test_utils;

use sqlx::sqlite::SqlitePoolOptions;
use image_storage::ImageStorage;

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

    let images = ImageStorage::new(&cfg.images_folder, PIC_SIZES.into()).unwrap();

    let web = web::make_server(cfg.clone(), db, images);

    let result = tokio::join!(web.await);

    println!("result: {:?}", result);
}