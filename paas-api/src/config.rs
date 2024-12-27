use log::debug;
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, Scope, TokenUrl};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub enum OAuthProvider {
    GitHub,
    GitLab,
    Bitbucket,
}

impl OAuthProvider {
    pub fn get_auth_url(&self) -> String {
        match self {
            OAuthProvider::GitHub => env::var("GITHUB_AUTH_URL")
                .unwrap_or_else(|_| "https://github.com/login/oauth/authorize".to_string()),
            OAuthProvider::GitLab => env::var("GITLAB_AUTH_URL")
                .unwrap_or_else(|_| "https://gitlab.com/oauth/authorize".to_string()),
            OAuthProvider::Bitbucket => env::var("BITBUCKET_AUTH_URL")
                .unwrap_or_else(|_| "https://bitbucket.org/site/oauth2/authorize".to_string()),
        }
    }

    pub fn get_token_url(&self) -> String {
        match self {
            OAuthProvider::GitHub => env::var("GITHUB_TOKEN_URL")
                .unwrap_or_else(|_| "https://github.com/login/oauth/access_token".to_string()),
            OAuthProvider::GitLab => env::var("GITLAB_TOKEN_URL")
                .unwrap_or_else(|_| "https://gitlab.com/oauth/token".to_string()),
            OAuthProvider::Bitbucket => env::var("BITBUCKET_TOKEN_URL")
                .unwrap_or_else(|_| "https://bitbucket.org/site/oauth2/access_token".to_string()),
        }
    }

    pub fn get_user_api_url(&self) -> String {
        match self {
            OAuthProvider::GitHub => env::var("GITHUB_API_URL")
                .unwrap_or_else(|_| "https://api.github.com/user".to_string()),
            OAuthProvider::GitLab => env::var("GITLAB_API_URL")
                .unwrap_or_else(|_| "https://gitlab.com/api/v4/user".to_string()),
            OAuthProvider::Bitbucket => env::var("BITBUCKET_API_URL")
                .unwrap_or_else(|_| "https://api.bitbucket.org/2.0/user".to_string()),
        }
    }

    pub fn get_scopes(&self) -> Vec<Scope> {
        match self {
            OAuthProvider::GitHub => vec![
                Scope::new("read:user".to_string()),
                Scope::new("user:email".to_string()),
                Scope::new("repo".to_string()),
            ],
            OAuthProvider::GitLab => vec![
                Scope::new("read_user".to_string()),
                Scope::new("read_repository".to_string()),
            ],
            OAuthProvider::Bitbucket => vec![
                Scope::new("account".to_string()),
                Scope::new("repository".to_string()),
            ],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthUser {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
}

pub fn get_frontend_url() -> String {
    env::var("FRONTEND_URL").unwrap_or_else(|_| "http://127.0.0.1:8080".to_string())
}

pub fn github_oauth_client() -> BasicClient {
    create_oauth_client(&OAuthProvider::GitHub).expect("Failed to create GitHub OAuth client")
}

pub fn gitlab_oauth_client() -> BasicClient {
    create_oauth_client(&OAuthProvider::GitLab).expect("Failed to create GitLab OAuth client")
}

pub fn bitbucket_oauth_client() -> BasicClient {
    create_oauth_client(&OAuthProvider::Bitbucket).expect("Failed to create Bitbucket OAuth client")
}

fn create_oauth_client(
    provider: &OAuthProvider,
) -> Result<BasicClient, oauth2::ConfigurationError> {
    let (client_id_var, redirect_path) = match provider {
        OAuthProvider::GitHub => ("GITHUB", "/api/auth/github/callback"),
        OAuthProvider::GitLab => ("GITLAB", "/api/auth/gitlab/callback"),
        OAuthProvider::Bitbucket => ("BITBUCKET", "/api/auth/bitbucket/callback"),
    };

    let client_id_env = format!("{}_CLIENT_ID", client_id_var);
    let client_secret_env = format!("{}_CLIENT_SECRET", client_id_var);

    let base_url = get_base_url();
    let redirect_url = format!("{}{}", base_url, redirect_path);

    debug!("Creating OAuth client for {:?}", provider);
    debug!("Auth URL: {}", provider.get_auth_url());
    debug!("Token URL: {}", provider.get_token_url());
    debug!("Redirect URL: {}", redirect_url);

    let client = BasicClient::new(
        ClientId::new(
            env::var(&client_id_env)
                .unwrap_or_else(|_| panic!("Missing {} environment variable", client_id_env)),
        ),
        Some(ClientSecret::new(
            env::var(&client_secret_env)
                .unwrap_or_else(|_| panic!("Missing {} environment variable", client_secret_env)),
        )),
        AuthUrl::new(provider.get_auth_url()).expect("Invalid auth URL"),
        Some(TokenUrl::new(provider.get_token_url()).expect("Invalid token URL")),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).expect("Invalid redirect URL"));

    Ok(client)
}

fn get_base_url() -> String {
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let base_url = format!("http://{}:{}", host, port);
    debug!("Base URL: {}", base_url);
    base_url
}
