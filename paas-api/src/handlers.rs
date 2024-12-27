use crate::{auth::{self, SessionUser}, config::{self, OAuthProvider}, error::AppError, models};
use actix_session::Session;
use actix_web::{web, HttpResponse};
use oauth2::{AuthorizationCode, CsrfToken, TokenResponse};
use serde::Deserialize;
use serde_json::json;
use sqlx::SqlitePool;
use log::debug;

#[derive(Deserialize)]
pub struct OAuthCallback {
    code: Option<String>,
    #[allow(dead_code)]
    state: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

pub async fn github_auth() -> Result<HttpResponse, AppError> {
    let client = config::github_oauth_client();
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scopes(OAuthProvider::GitHub.get_scopes())
        .url();

    debug!("GitHub auth URL: {}", auth_url);

    Ok(HttpResponse::Ok().json(json!({
        "auth_url": auth_url.to_string()
    })))
}

pub async fn github_callback(
    pool: web::Data<SqlitePool>,
    session: Session,
    params: web::Query<OAuthCallback>,
) -> Result<HttpResponse, AppError> {
    // Check for OAuth error response
    if let Some(error) = &params.error {
        debug!("GitHub OAuth error: {}", error);
        let error_msg = params.error_description
            .as_deref()
            .unwrap_or("OAuth consent was denied");
        return Ok(HttpResponse::Unauthorized()
            .json(json!({ "error": error_msg })));
    }

    let code = params.code.as_ref()
        .ok_or_else(|| AppError::ValidationError("Missing authorization code".to_string()))?;

    debug!("GitHub callback received with code: {}", code);
    let client = config::github_oauth_client();
    
    debug!("Exchanging GitHub code for token...");
    // Exchange the code for an access token
    let token = match client
        .exchange_code(AuthorizationCode::new(code.clone()))
        .request_async(oauth2::reqwest::async_http_client)
        .await {
            Ok(token) => {
                debug!("GitHub token exchange successful");
                token
            },
            Err(e) => {
                debug!("GitHub token exchange error: {:?}", e);
                return Err(AppError::AuthError(format!("Failed to exchange code: {}", e)));
            }
        };

    debug!("Getting GitHub user info...");
    // Get user info from GitHub
    let client = reqwest::Client::new();
    let user_data = client
        .get(OAuthProvider::GitHub.get_user_api_url())
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", token.access_token().secret()),
        )
        .header(reqwest::header::USER_AGENT, "rust-app")
        .send()
        .await
        .map_err(|e| {
            debug!("GitHub user info error: {:?}", e);
            AppError::AuthError(format!("Failed to get user info: {}", e))
        })?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| {
            debug!("GitHub user info parse error: {:?}", e);
            AppError::AuthError(format!("Failed to parse user info: {}", e))
        })?;

    debug!("Creating or updating user in database...");
    // Create or update user in database
    let user = models::User::find_or_create(
        pool.get_ref(),
        &OAuthProvider::GitHub,
        &user_data["id"].to_string(),
        user_data["login"].as_str().unwrap_or(""),
        user_data["email"].as_str().map(|s| s.to_string()).as_deref(),
        user_data["avatar_url"].as_str().map(|s| s.to_string()).as_deref(),
    )
    .await?;

    debug!("Setting session user...");
    // Create session with OAuth tokens
    auth::set_session_user(&session, SessionUser {
        id: user.id,
        username: user.username.clone(),
        email: user.email.clone(),
        provider: "github".to_string(),
        access_token: token.access_token().secret().to_string(),
        refresh_token: token.refresh_token().map(|t| t.secret().to_string()),
    })?;

    debug!("GitHub auth flow completed successfully");
    Ok(HttpResponse::Ok().json(json!({
        "user": {
            "id": user.id,
            "username": user.username,
            "email": user.email
        }
    })))
}

pub async fn gitlab_auth() -> Result<HttpResponse, AppError> {
    let client = config::gitlab_oauth_client();
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scopes(OAuthProvider::GitLab.get_scopes())
        .url();

    debug!("GitLab auth URL: {}", auth_url);

    Ok(HttpResponse::Ok().json(json!({
        "auth_url": auth_url.to_string()
    })))
}

pub async fn gitlab_callback(
    pool: web::Data<SqlitePool>,
    session: Session,
    params: web::Query<OAuthCallback>,
) -> Result<HttpResponse, AppError> {
    // Check for OAuth error response
    if let Some(error) = &params.error {
        debug!("GitLab OAuth error: {}", error);
        let error_msg = params.error_description
            .as_deref()
            .unwrap_or("OAuth consent was denied");
        return Ok(HttpResponse::Unauthorized()
            .json(json!({ "error": error_msg })));
    }

    let code = params.code.as_ref()
        .ok_or_else(|| AppError::ValidationError("Missing authorization code".to_string()))?;

    debug!("GitLab callback received with code: {}", code);
    let client = config::gitlab_oauth_client();
    
    debug!("Exchanging GitLab code for token...");
    // Exchange the code for an access token
    let token = match client
        .exchange_code(AuthorizationCode::new(code.clone()))
        .request_async(oauth2::reqwest::async_http_client)
        .await {
            Ok(token) => {
                debug!("GitLab token exchange successful");
                token
            },
            Err(e) => {
                debug!("GitLab token exchange error: {:?}", e);
                return Err(AppError::AuthError(format!("Failed to exchange code: {}", e)));
            }
        };

    debug!("Getting GitLab user info...");
    // Get user info from GitLab
    let client = reqwest::Client::new();
    let user_data = client
        .get(OAuthProvider::GitLab.get_user_api_url())
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", token.access_token().secret()),
        )
        .send()
        .await
        .map_err(|e| {
            debug!("GitLab user info error: {:?}", e);
            AppError::AuthError(format!("Failed to get user info: {}", e))
        })?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| {
            debug!("GitLab user info parse error: {:?}", e);
            AppError::AuthError(format!("Failed to parse user info: {}", e))
        })?;

    debug!("Creating or updating user in database...");
    // Create or update user in database
    let user = models::User::find_or_create(
        pool.get_ref(),
        &OAuthProvider::GitLab,
        &user_data["id"].to_string(),
        user_data["username"].as_str().unwrap_or(""),
        user_data["email"].as_str().map(|s| s.to_string()).as_deref(),
        user_data["avatar_url"].as_str().map(|s| s.to_string()).as_deref(),
    )
    .await?;

    debug!("Setting session user...");
    // Create session with OAuth tokens
    auth::set_session_user(&session, SessionUser {
        id: user.id,
        username: user.username.clone(),
        email: user.email.clone(),
        provider: "gitlab".to_string(),
        access_token: token.access_token().secret().to_string(),
        refresh_token: token.refresh_token().map(|t| t.secret().to_string()),
    })?;

    debug!("GitLab auth flow completed successfully");
    Ok(HttpResponse::Ok().json(json!({
        "user": {
            "id": user.id,
            "username": user.username,
            "email": user.email
        }
    })))
}

pub async fn bitbucket_auth() -> Result<HttpResponse, AppError> {
    let client = config::bitbucket_oauth_client();
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scopes(OAuthProvider::Bitbucket.get_scopes())
        .url();

    debug!("Bitbucket auth URL: {}", auth_url);

    Ok(HttpResponse::Ok().json(json!({
        "auth_url": auth_url.to_string()
    })))
}

pub async fn bitbucket_callback(
    pool: web::Data<SqlitePool>,
    session: Session,
    params: web::Query<OAuthCallback>,
) -> Result<HttpResponse, AppError> {
    // Check for OAuth error response
    if let Some(error) = &params.error {
        debug!("Bitbucket OAuth error: {}", error);
        let error_msg = params.error_description
            .as_deref()
            .unwrap_or("OAuth consent was denied");
        return Ok(HttpResponse::Unauthorized()
            .json(json!({ "error": error_msg })));
    }

    let code = params.code.as_ref()
        .ok_or_else(|| AppError::ValidationError("Missing authorization code".to_string()))?;

    debug!("Bitbucket callback received with code: {}", code);
    let client = config::bitbucket_oauth_client();
    
    debug!("Exchanging Bitbucket code for token...");
    // Exchange the code for an access token
    let token = match client
        .exchange_code(AuthorizationCode::new(code.clone()))
        .request_async(oauth2::reqwest::async_http_client)
        .await {
            Ok(token) => {
                debug!("Bitbucket token exchange successful");
                token
            },
            Err(e) => {
                debug!("Bitbucket token exchange error: {:?}", e);
                return Err(AppError::AuthError(format!("Failed to exchange code: {}", e)));
            }
        };

    debug!("Getting Bitbucket user info...");
    // Get user info from Bitbucket
    let client = reqwest::Client::new();
    let user_data = client
        .get(OAuthProvider::Bitbucket.get_user_api_url())
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", token.access_token().secret()),
        )
        .send()
        .await
        .map_err(|e| {
            debug!("Bitbucket user info error: {:?}", e);
            AppError::AuthError(format!("Failed to get user info: {}", e))
        })?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| {
            debug!("Bitbucket user info parse error: {:?}", e);
            AppError::AuthError(format!("Failed to parse user info: {}", e))
        })?;

    debug!("Creating or updating user in database...");
    // Create or update user in database
    let user = models::User::find_or_create(
        pool.get_ref(),
        &OAuthProvider::Bitbucket,
        &user_data["uuid"].to_string(),
        user_data["username"].as_str().unwrap_or(""),
        user_data["email"].as_str().map(|s| s.to_string()).as_deref(),
        user_data["links"]["avatar"]["href"].as_str().map(|s| s.to_string()).as_deref(),
    )
    .await?;

    debug!("Setting session user...");
    // Create session with OAuth tokens
    auth::set_session_user(&session, SessionUser {
        id: user.id,
        username: user.username.clone(),
        email: user.email.clone(),
        provider: "bitbucket".to_string(),
        access_token: token.access_token().secret().to_string(),
        refresh_token: token.refresh_token().map(|t| t.secret().to_string()),
    })?;

    debug!("Bitbucket auth flow completed successfully");
    Ok(HttpResponse::Ok().json(json!({
        "user": {
            "id": user.id,
            "username": user.username,
            "email": user.email
        }
    })))
}

pub async fn logout(session: Session) -> Result<HttpResponse, AppError> {
    auth::clear_session(&session)?;
    Ok(HttpResponse::Ok().finish())
}
