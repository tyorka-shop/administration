mod cached_session_service;
mod entity;
mod feed;
mod graphql_schema;
mod guard;
mod image_processing;
mod web;
mod tasks;

use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() {
    env_logger::init();
    let cfg = config::load("tyorka-admin");

    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&cfg.database_uri)
        .await
        .unwrap();
    image_processing::init_store(&cfg.images_folder).unwrap();

    let web = web::make_server(cfg.clone(), db);

    let tasks = tasks::init();

    let result = tokio::join!(web.await, tasks);

    println!("result: {:?}", result);
}
