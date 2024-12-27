use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::env;

pub async fn create_pool() -> Result<SqlitePool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}

pub async fn init_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Run migrations
    sqlx::migrate!("./migrations").run(pool).await?;

    Ok(())
}
