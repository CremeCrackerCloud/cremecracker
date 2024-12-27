use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::env;

pub async fn create_pool() -> Result<SqlitePool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").map_err(|_| {
        sqlx::Error::Configuration("DATABASE_URL must be set".into())
    })?;

    if database_url.is_empty() {
        return Err(sqlx::Error::Configuration("DATABASE_URL cannot be empty".into()));
    }

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
