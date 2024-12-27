use crate::handlers;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/auth/github", web::get().to(handlers::github_auth))
            .route("/auth/github/callback", web::get().to(handlers::github_callback))
            .route("/auth/gitlab", web::get().to(handlers::gitlab_auth))
            .route("/auth/gitlab/callback", web::get().to(handlers::gitlab_callback))
            .route("/auth/bitbucket", web::get().to(handlers::bitbucket_auth))
            .route(
                "/auth/bitbucket/callback",
                web::get().to(handlers::bitbucket_callback),
            )
            .route("/auth/logout", web::post().to(handlers::logout))
            .route("/user/me", web::get().to(handlers::get_current_user)),
    );
}
