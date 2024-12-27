use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthError {
    pub error: String,
    pub error_description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub auth_url: String,
}

impl AuthError {
    pub fn as_string(&self) -> String {
        self.error_description
            .clone()
            .unwrap_or_else(|| self.error.clone())
    }
}

pub struct AuthApi;

impl AuthApi {
    fn get_config() -> Config {
        use_context::<Config>().expect("Config not found in context")
    }

    async fn get_oauth_url(provider: &str) -> Result<String, JsValue> {
        let config = Self::get_config();
        let window = web_sys::window().unwrap();

        let mut opts = RequestInit::new();
        opts.set_method("GET");
        opts.set_mode(RequestMode::Cors);
        opts.set_credentials(web_sys::RequestCredentials::Include);

        let request = Request::new_with_str_and_init(
            &format!("{}/api/auth/{}", config.api_host, provider),
            &opts,
        )?;
        request.headers().set("Accept", "application/json")?;

        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
        let resp: Response = resp_value.dyn_into()?;

        if resp.ok() {
            let json = JsFuture::from(resp.json()?).await?;
            let response: AuthResponse = serde_wasm_bindgen::from_value(json)?;
            Ok(response.auth_url)
        } else {
            let json = JsFuture::from(resp.json()?).await?;
            let error: AuthError = serde_wasm_bindgen::from_value(json)?;
            Err(JsValue::from_str(&error.as_string()))
        }
    }

    pub async fn github_auth() -> Result<(), JsValue> {
        let auth_url = Self::get_oauth_url("github").await?;
        let window = web_sys::window().unwrap();
        let location = window.location();
        location.set_href(&auth_url)?;
        Ok(())
    }

    pub async fn gitlab_auth() -> Result<(), JsValue> {
        let auth_url = Self::get_oauth_url("gitlab").await?;
        let window = web_sys::window().unwrap();
        let location = window.location();
        location.set_href(&auth_url)?;
        Ok(())
    }

    pub async fn bitbucket_auth() -> Result<(), JsValue> {
        let auth_url = Self::get_oauth_url("bitbucket").await?;
        let window = web_sys::window().unwrap();
        let location = window.location();
        location.set_href(&auth_url)?;
        Ok(())
    }

    pub async fn handle_oauth_callback() -> Result<(), JsValue> {
        let config = Self::get_config();
        let window = web_sys::window().unwrap();
        let location = window.location();
        let search = location.search()?;
        let params = web_sys::UrlSearchParams::new_with_str(&search)?;

        // Check for error parameters from OAuth provider
        if let Some(error) = params.get("error") {
            let error_description = params
                .get("error_description")
                .unwrap_or_else(|| "Authentication failed".to_string());

            // Redirect to login page with error
            let error_url = format!(
                "/login?error={}",
                js_sys::encode_uri_component(&error_description)
            );
            location.set_href(&error_url)?;
            return Ok(());
        }

        // Get the code and state parameters
        let code = params
            .get("code")
            .ok_or_else(|| JsValue::from_str("No code parameter found"))?;
        let state = params
            .get("state")
            .ok_or_else(|| JsValue::from_str("No state parameter found"))?;

        // Make request to backend
        let mut opts = RequestInit::new();
        opts.set_method("GET");
        opts.set_mode(RequestMode::Cors);
        opts.set_credentials(web_sys::RequestCredentials::Include);

        // Get the current path to determine which provider to use
        let path = location.pathname()?;
        let callback_url = format!(
            "{}{}?code={}&state={}",
            config.api_host,
            path.replace("/auth", "/api/auth"),
            js_sys::encode_uri_component(&code),
            js_sys::encode_uri_component(&state)
        );

        let request = Request::new_with_str_and_init(&callback_url, &opts)?;
        request.headers().set("Accept", "application/json")?;

        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
        let resp: Response = resp_value.dyn_into()?;

        if !resp.ok() {
            // Try to parse error response
            let error_text = JsFuture::from(resp.text()?).await?;
            let error_str = error_text
                .as_string()
                .unwrap_or_else(|| "Unknown error".to_string());

            // Try to parse as JSON, but if it fails, use the raw error string
            let error_msg = match serde_json::from_str::<AuthError>(&error_str) {
                Ok(error) => error.as_string(),
                Err(_) => error_str,
            };

            // Redirect to login page with error
            let error_url = format!("/login?error={}", js_sys::encode_uri_component(&error_msg));
            location.set_href(&error_url)?;
            return Ok(());
        }

        // On success, redirect to dashboard
        location.set_href("/dashboard")?;
        Ok(())
    }

    pub async fn logout() -> Result<(), JsValue> {
        let config = Self::get_config();
        let window = web_sys::window().unwrap();

        let mut opts = RequestInit::new();
        opts.set_method("POST");
        opts.set_mode(RequestMode::Cors);
        opts.set_credentials(web_sys::RequestCredentials::Include);

        let request =
            Request::new_with_str_and_init(&format!("{}/api/auth/logout", config.api_host), &opts)?;
        request.headers().set("Accept", "application/json")?;

        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
        let resp: Response = resp_value.dyn_into()?;

        if resp.ok() {
            Ok(())
        } else {
            let json = JsFuture::from(resp.json()?).await?;
            let error: serde_json::Value = serde_wasm_bindgen::from_value(json)?;
            Err(JsValue::from_str(
                &error["error"].as_str().unwrap_or("Unknown error"),
            ))
        }
    }
}
