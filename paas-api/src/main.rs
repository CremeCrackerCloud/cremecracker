mod auth;
mod config;
mod db;
mod error;
mod handlers;
mod models;
mod routes;
mod tests;

use actix_cors::Cors;
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{http::header, middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use env_logger;
use std::env;
use time::Duration;

pub async fn run() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let pool = db::create_pool()
        .await
        .expect("Failed to create database pool");

    db::init_db(&pool)
        .await
        .expect("Failed to initialize database");

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let bind_address = format!("{}:{}", host, port);

    let secret_key = actix_web::cookie::Key::generate();

    println!("Starting server at http://{}", bind_address);
    HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .session_lifecycle(PersistentSession::default().session_ttl(Duration::days(7)))
                    .cookie_secure(false)
                    .cookie_http_only(true)
                    .build(),
            )
            .wrap(
                Cors::default()
                    .allowed_origin("http://127.0.0.1:8080")
                    .allowed_origin("http://localhost:8080")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![
                        header::AUTHORIZATION,
                        header::ACCEPT,
                        header::CONTENT_TYPE,
                    ])
                    .supports_credentials()
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run().await
}
