pub mod auth;
pub mod config;
pub mod db;
pub mod error;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod tests;

pub use crate::auth::*;
pub use crate::config::*;
pub use crate::db::*;
pub use crate::error::*;
pub use crate::handlers::*;
pub use crate::models::*;
pub use crate::routes::*;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

pub async fn run() -> std::io::Result<()> {
    dotenv().ok();

    // Initialize database connection pool
    let pool = db::create_pool()
        .await
        .expect("Failed to create database pool");

    // Initialize database schema
    db::init_db(&pool)
        .await
        .expect("Failed to initialize database");

    // Get host and port from environment variables
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let bind_address = format!("{}:{}", host, port);

    println!("Starting server at http://{}", bind_address);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::configure)
    })
    .bind(bind_address)?
    .run()
    .await
}
