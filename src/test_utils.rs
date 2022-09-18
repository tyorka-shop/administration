use sqlx::{Result, SqlitePool, sqlite::SqlitePoolOptions};

pub async fn setup_db() -> Result<SqlitePool> {
    let db = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(":memory:")
        .await
        .unwrap();

    sqlx::migrate!().run(&db).await.unwrap();

    Ok(db)
}
