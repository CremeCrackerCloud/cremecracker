mod auth;
mod config;
mod db;
mod error;
mod handlers;
mod models;
mod routes;
mod tests;

use actix_cors::Cors;
use actix_session::{storage::CookieSessionStore, SessionMiddleware, config::PersistentSession};
use actix_web::{http::header, middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use env_logger;
use std::env;
use time::Duration;

pub async fn run() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

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

    // Generate random key for signing the session cookie
    let secret_key = actix_web::cookie::Key::generate();

    println!("Starting server at http://{}", bind_address);
    HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .session_lifecycle(
                        PersistentSession::default()
                            .session_ttl(Duration::days(7))
                    )
                    .cookie_secure(false) // Set to true in production with HTTPS
                    .cookie_http_only(true)
                    .build(),
            )
            .wrap(
                Cors::default()
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://127.0.0.1")
                    })
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(&[header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .expose_headers(&[header::CONTENT_DISPOSITION])
                    .block_on_origin_mismatch(false)
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::configure)
    })
    .bind(bind_address)?
    .run()
    .await
}

// This allows the crate to be run directly if needed
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run().await
}
