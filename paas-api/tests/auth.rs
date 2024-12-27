use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, test, web::Data, App, Error};
use env_logger;
use paas_api::{config, models, routes::configure};
use serde_json::json;
use sqlx::SqlitePool;
use std::env;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

fn setup_test_env() {
    env_logger::try_init().ok();
    dotenv::from_filename("tests.env").ok();
    env::set_var("RUST_LOG", "debug");
    env::set_var("HOST", "127.0.0.1");
    env::set_var("PORT", "3000");
    env::set_var("BASE_URL", "http://127.0.0.1:3000");
    env::set_var("FRONTEND_URL", "http://127.0.0.1:8080");
    env::set_var(
        "GITHUB_AUTH_URL",
        "https://github.com/login/oauth/authorize",
    );
    env::set_var(
        "GITHUB_TOKEN_URL",
        "https://github.com/login/oauth/access_token",
    );
    env::set_var("GITHUB_API_URL", "https://api.github.com/user");
    env::set_var("GITLAB_AUTH_URL", "https://gitlab.com/oauth/authorize");
    env::set_var("GITLAB_TOKEN_URL", "https://gitlab.com/oauth/token");
    env::set_var("GITLAB_API_URL", "https://gitlab.com/api/v4/user");
    env::set_var(
        "BITBUCKET_AUTH_URL",
        "https://bitbucket.org/site/oauth2/authorize",
    );
    env::set_var(
        "BITBUCKET_TOKEN_URL",
        "https://bitbucket.org/site/oauth2/access_token",
    );
    env::set_var("BITBUCKET_API_URL", "https://api.bitbucket.org/2.0/user");
}

async fn setup_test_app(
    pool: SqlitePool,
) -> impl actix_web::dev::Service<
    actix_http::Request,
    Response = actix_web::dev::ServiceResponse,
    Error = Error,
> {
    let secret_key = Key::generate();

    test::init_service(
        App::new()
            .app_data(Data::new(pool))
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key)
                    .cookie_secure(false)
                    .build(),
            )
            .configure(configure),
    )
    .await
}

async fn setup_test_db() -> SqlitePool {
    let pool = SqlitePool::connect(":memory:")
        .await
        .expect("Failed to create test database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}

#[actix_web::test]
async fn test_github_auth_flow() {
    setup_test_env();

    let mock_server = MockServer::start().await;

    env::set_var(
        "GITHUB_AUTH_URL",
        format!("{}/login/oauth/authorize", mock_server.uri()),
    );
    env::set_var(
        "GITHUB_TOKEN_URL",
        format!("{}/login/oauth/access_token", mock_server.uri()),
    );
    env::set_var("GITHUB_API_URL", format!("{}/user", mock_server.uri()));

    Mock::given(method("POST"))
        .and(path("/login/oauth/access_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "access_token": "test_access_token",
            "token_type": "bearer",
            "scope": "user:email"
        })))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/user"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 12345,
            "login": "test_user",
            "email": "test@example.com",
            "avatar_url": "https://example.com/avatar.jpg"
        })))
        .mount(&mock_server)
        .await;

    let pool = setup_test_db().await;
    let app = setup_test_app(pool.clone()).await;

    let req = test::TestRequest::get()
        .uri("/api/auth/github")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["auth_url"]
        .as_str()
        .unwrap()
        .contains("/login/oauth/authorize"));

    let req = test::TestRequest::get()
        .uri("/api/auth/github/callback?code=test_code&state=test_state")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_redirection());

    let location = resp.headers().get("Location").unwrap().to_str().unwrap();
    assert!(location.contains(&format!("{}/dashboard", config::get_frontend_url())));

    let user = sqlx::query_as::<_, models::User>("SELECT * FROM users WHERE username = ?")
        .bind("test_user")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(user.username, "test_user");
    assert_eq!(user.email, Some("test@example.com".to_string()));
}

#[actix_web::test]
async fn test_gitlab_auth_flow() {
    setup_test_env();

    let mock_server = MockServer::start().await;

    env::set_var(
        "GITLAB_AUTH_URL",
        format!("{}/oauth/authorize", mock_server.uri()),
    );
    env::set_var(
        "GITLAB_TOKEN_URL",
        format!("{}/oauth/token", mock_server.uri()),
    );
    env::set_var(
        "GITLAB_API_URL",
        format!("{}/api/v4/user", mock_server.uri()),
    );

    Mock::given(method("POST"))
        .and(path("/oauth/token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "access_token": "test_access_token",
            "token_type": "bearer",
            "scope": "read_user"
        })))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/v4/user"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 12345,
            "username": "test_user",
            "email": "test@example.com",
            "avatar_url": "https://example.com/avatar.jpg"
        })))
        .mount(&mock_server)
        .await;

    let pool = setup_test_db().await;
    let app = setup_test_app(pool.clone()).await;

    let req = test::TestRequest::get()
        .uri("/api/auth/gitlab")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["auth_url"]
        .as_str()
        .unwrap()
        .contains("/oauth/authorize"));

    let req = test::TestRequest::get()
        .uri("/api/auth/gitlab/callback?code=test_code&state=test_state")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_redirection());

    let location = resp.headers().get("Location").unwrap().to_str().unwrap();
    assert!(location.contains(&format!("{}/dashboard", config::get_frontend_url())));

    let user = sqlx::query_as::<_, models::User>("SELECT * FROM users WHERE username = ?")
        .bind("test_user")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(user.username, "test_user");
    assert_eq!(user.email, Some("test@example.com".to_string()));
}

#[actix_web::test]
async fn test_bitbucket_auth_flow() {
    setup_test_env();

    let mock_server = MockServer::start().await;

    env::set_var(
        "BITBUCKET_AUTH_URL",
        format!("{}/site/oauth2/authorize", mock_server.uri()),
    );
    env::set_var(
        "BITBUCKET_TOKEN_URL",
        format!("{}/site/oauth2/access_token", mock_server.uri()),
    );
    env::set_var(
        "BITBUCKET_API_URL",
        format!("{}/2.0/user", mock_server.uri()),
    );

    Mock::given(method("POST"))
        .and(path("/site/oauth2/access_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "access_token": "test_access_token",
            "token_type": "bearer",
            "scope": "account"
        })))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/2.0/user"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "uuid": "{12345}",
            "username": "test_user",
            "email": "test@example.com",
            "links": {
                "avatar": {
                    "href": "https://example.com/avatar.jpg"
                }
            }
        })))
        .mount(&mock_server)
        .await;

    let pool = setup_test_db().await;
    let app = setup_test_app(pool.clone()).await;

    let req = test::TestRequest::get()
        .uri("/api/auth/bitbucket")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["auth_url"]
        .as_str()
        .unwrap()
        .contains("/site/oauth2/authorize"));

    let req = test::TestRequest::get()
        .uri("/api/auth/bitbucket/callback?code=test_code&state=test_state")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_redirection());

    let location = resp.headers().get("Location").unwrap().to_str().unwrap();
    assert!(location.contains(&format!("{}/dashboard", config::get_frontend_url())));

    let user = sqlx::query_as::<_, models::User>("SELECT * FROM users WHERE username = ?")
        .bind("test_user")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(user.username, "test_user");
    assert_eq!(user.email, Some("test@example.com".to_string()));
}

#[actix_web::test]
async fn test_logout() {
    setup_test_env();

    let pool = setup_test_db().await;
    let app = setup_test_app(pool.clone()).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/logout")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_github_auth_denied() {
    setup_test_env();

    let pool = setup_test_db().await;
    let app = setup_test_app(pool.clone()).await;

    let req = test::TestRequest::get()
        .uri("/api/auth/github/callback?error=access_denied&error_description=The+user+has+denied+access")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_redirection());

    let location = resp.headers().get("Location").unwrap().to_str().unwrap();
    assert!(location.contains(&format!("{}/login?error=", config::get_frontend_url())));
    assert!(location.contains("denied"));
}

#[actix_web::test]
async fn test_gitlab_auth_denied() {
    setup_test_env();

    let pool = setup_test_db().await;
    let app = setup_test_app(pool.clone()).await;

    let req = test::TestRequest::get()
        .uri("/api/auth/gitlab/callback?error=access_denied&error_description=The+user+has+denied+access")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_redirection());

    let location = resp.headers().get("Location").unwrap().to_str().unwrap();
    assert!(location.contains(&format!("{}/login?error=", config::get_frontend_url())));
    assert!(location.contains("denied"));
}

#[actix_web::test]
async fn test_bitbucket_auth_denied() {
    setup_test_env();

    let pool = setup_test_db().await;
    let app = setup_test_app(pool.clone()).await;

    let req = test::TestRequest::get()
        .uri("/api/auth/bitbucket/callback?error=access_denied&error_description=The+user+has+denied+access")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_redirection());

    let location = resp.headers().get("Location").unwrap().to_str().unwrap();
    assert!(location.contains(&format!("{}/login?error=", config::get_frontend_url())));
    assert!(location.contains("denied"));
}
