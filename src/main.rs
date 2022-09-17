mod cached_session_service;
mod graphql_types;
mod graphql_schema;
mod guard;
mod web;

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

    init_store(&cfg.images_folder).unwrap();

    let web = web::make_server(cfg.clone(), db);

    let result = tokio::join!(web.await);

    println!("result: {:?}", result);
}

fn init_store(path: &str) -> std::io::Result<()> {
    match std::fs::create_dir_all(path) {
        Ok(_) => Ok(()),
        Err(e) => match e.kind() {
            std::io::ErrorKind::AlreadyExists => Ok(()),
            _ => Err(e),
        },
    }
}
