use crate::handlers;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").service(
            web::scope("/auth")
                .route("/logout", web::post().to(handlers::logout))
                .route("/github", web::get().to(handlers::github_auth))
                .route("/github/callback", web::get().to(handlers::github_callback))
                .route("/gitlab", web::get().to(handlers::gitlab_auth))
                .route("/gitlab/callback", web::get().to(handlers::gitlab_callback))
                .route("/bitbucket", web::get().to(handlers::bitbucket_auth))
                .route(
                    "/bitbucket/callback",
                    web::get().to(handlers::bitbucket_callback),
                ),
        ),
    );
}
