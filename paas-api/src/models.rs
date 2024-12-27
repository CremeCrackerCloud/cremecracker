use crate::config::OAuthProvider;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: i64,
    pub provider: String, // Stores the OAuth provider (GitHub, GitLab, Bitbucket)
    pub provider_user_id: String, // User's ID from the OAuth provider
    pub username: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub created_at: String, // Store as string since SQLite doesn't have a native DateTime type
}

impl User {
    pub async fn find_or_create(
        pool: &SqlitePool,
        provider: &OAuthProvider,
        provider_user_id: &str,
        username: &str,
        email: Option<&str>,
        avatar_url: Option<&str>,
    ) -> Result<Self, sqlx::Error> {
        // First, try to find the user by provider and provider user ID
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE provider = ? AND provider_user_id = ?",
        )
        .bind(provider.to_string())
        .bind(provider_user_id)
        .fetch_optional(pool)
        .await?;

        // If user exists, return the existing user
        if let Some(user) = user {
            return Ok(user);
        }

        // If not, create a new user
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (provider, provider_user_id, username, email, avatar_url, created_at) 
             VALUES (?, ?, ?, ?, ?, datetime('now')) 
             RETURNING *"
        )
        .bind(provider.to_string())
        .bind(provider_user_id)
        .bind(username)
        .bind(email)
        .bind(avatar_url)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    #[allow(dead_code)]
    pub async fn update_tokens(
        pool: &SqlitePool,
        provider: &OAuthProvider,
        provider_user_id: &str,
        access_token: &str,
        refresh_token: Option<&str>,
    ) -> Result<Self, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            "UPDATE users 
             SET access_token = ?, refresh_token = ? 
             WHERE provider = ? AND provider_user_id = ? 
             RETURNING *",
        )
        .bind(access_token)
        .bind(refresh_token)
        .bind(provider.to_string())
        .bind(provider_user_id)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }
}

// Implement ToString for OAuthProvider to store in database
impl ToString for OAuthProvider {
    fn to_string(&self) -> String {
        match self {
            OAuthProvider::GitHub => "github".to_string(),
            OAuthProvider::GitLab => "gitlab".to_string(),
            OAuthProvider::Bitbucket => "bitbucket".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitProvider {
    pub id: i64,
    pub user_id: i64,
    pub provider_type: String, // "github", "gitlab", "bitbucket"
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    pub id: i64,
    pub user_id: i64,
    pub provider_id: i64,
    pub name: String,
    pub url: String,
    pub is_private: bool,
    pub last_synced: String,
}
