use crate::error::AppError;
use actix_session::{Session, SessionExt};
use actix_web::{dev::ServiceRequest, Error};
use serde::{Deserialize, Serialize};

pub const USER_ID_KEY: &str = "user_id";
pub const ACCESS_TOKEN_KEY: &str = "access_token";
pub const REFRESH_TOKEN_KEY: &str = "refresh_token";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionUser {
    pub id: i64,
    pub username: String,
    pub email: Option<String>,
    pub provider: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
}

#[allow(dead_code)]
pub async fn get_session_user(session: &Session) -> Result<Option<SessionUser>, AppError> {
    if let Some(user_id) = session.get::<i64>(USER_ID_KEY)? {
        Ok(Some(SessionUser {
            id: user_id,
            username: session.get("username")?.unwrap_or_default(),
            email: session.get("email")?,
            provider: session.get("provider")?.unwrap_or_default(),
            access_token: session.get(ACCESS_TOKEN_KEY)?.unwrap_or_default(),
            refresh_token: session.get(REFRESH_TOKEN_KEY)?,
        }))
    } else {
        Ok(None)
    }
}

pub fn set_session_user(session: &Session, user: SessionUser) -> Result<(), AppError> {
    session.insert(USER_ID_KEY, user.id)?;
    session.insert("username", user.username)?;
    session.insert("provider", user.provider)?;
    session.insert(ACCESS_TOKEN_KEY, user.access_token)?;

    if let Some(email) = user.email {
        session.insert("email", email)?;
    }
    if let Some(refresh_token) = user.refresh_token {
        session.insert(REFRESH_TOKEN_KEY, refresh_token)?;
    }

    Ok(())
}

#[allow(dead_code)]
pub async fn validate_auth(req: ServiceRequest) -> Result<ServiceRequest, Error> {
    let session = req.get_session();

    if session.get::<i64>(USER_ID_KEY)?.is_none() {
        return Err(AppError::AuthError("Unauthorized".to_string()).into());
    }

    Ok(req)
}

#[allow(dead_code)]
pub fn clear_session(session: &Session) -> Result<(), AppError> {
    session.purge();
    Ok(())
}
