#[cfg(test)]
mod tests {
    use crate::{db, routes};
    use actix_web::test;
    use sqlx::SqlitePool;
    use std::env;

    fn setup_test_env() {
        dotenv::from_path("tests.env").expect("Failed to load tests.env file");
        
        // Set redirect URLs using test environment HOST and PORT
        let host = env::var("HOST").expect("HOST must be set in tests.env");
        let port = env::var("PORT").expect("PORT must be set in tests.env");
        let base_url = format!("http://{}:{}", host, port);

        env::set_var("GITHUB_REDIRECT_URL", format!("{}/api/auth/github/callback", base_url));
        env::set_var("GITLAB_REDIRECT_URL", format!("{}/api/auth/gitlab/callback", base_url));
        env::set_var("BITBUCKET_REDIRECT_URL", format!("{}/api/auth/bitbucket/callback", base_url));
    }

    async fn setup_test_db() -> SqlitePool {
        setup_test_env();
        let pool = SqlitePool::connect("sqlite::memory:")
            .await
            .expect("Failed to create test database");

        db::init_db(&pool)
            .await
            .expect("Failed to initialize test database");
        pool
    }

    #[actix_web::test]
    async fn test_github_auth() {
        let pool = setup_test_db().await;
        let app = test::init_service(
            actix_web::App::new()
                .app_data(actix_web::web::Data::new(pool.clone()))
                .configure(routes::configure),
        )
        .await;

        let resp = test::TestRequest::get()
            .uri("/api/auth/github")
            .send_request(&app)
            .await;

        assert_eq!(resp.status(), 200);

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert!(body.get("auth_url").is_some());
        assert!(body["auth_url"].as_str().unwrap().contains("github.com"));
    }

    #[actix_web::test]
    async fn test_gitlab_auth() {
        let pool = setup_test_db().await;
        let app = test::init_service(
            actix_web::App::new()
                .app_data(actix_web::web::Data::new(pool.clone()))
                .configure(routes::configure),
        )
        .await;

        let resp = test::TestRequest::get()
            .uri("/api/auth/gitlab")
            .send_request(&app)
            .await;

        assert_eq!(resp.status(), 200);

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert!(body.get("auth_url").is_some());
        assert!(body["auth_url"].as_str().unwrap().contains("gitlab.com"));
    }

    #[actix_web::test]
    async fn test_bitbucket_auth() {
        let pool = setup_test_db().await;
        let app = test::init_service(
            actix_web::App::new()
                .app_data(actix_web::web::Data::new(pool.clone()))
                .configure(routes::configure),
        )
        .await;

        let resp = test::TestRequest::get()
            .uri("/api/auth/bitbucket")
            .send_request(&app)
            .await;

        assert_eq!(resp.status(), 200);

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert!(body.get("auth_url").is_some());
        assert!(body["auth_url"].as_str().unwrap().contains("bitbucket.org"));
    }
}
