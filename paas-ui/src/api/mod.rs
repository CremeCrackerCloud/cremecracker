pub mod auth;
pub mod user;

pub const API_BASE_URL: &str = "http://localhost:3000/api";

pub use auth::AuthApi;
pub use user::UserApi;
