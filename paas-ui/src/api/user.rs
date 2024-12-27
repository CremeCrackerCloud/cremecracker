use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: Option<String>,
}

pub struct UserApi;

impl UserApi {
    fn get_config() -> Config {
        use_context::<Config>().expect("Config not found in context")
    }

    pub async fn get_current_user() -> Result<User, JsValue> {
        let config = Self::get_config();
        let window = web_sys::window().unwrap();

        let mut opts = RequestInit::new();
        opts.set_method("GET");
        opts.set_mode(RequestMode::Cors);
        opts.set_credentials(web_sys::RequestCredentials::Include);

        let request =
            Request::new_with_str_and_init(&format!("{}/api/user/me", config.api_host), &opts)?;
        request.headers().set("Accept", "application/json")?;

        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
        let resp: Response = resp_value.dyn_into()?;

        if resp.ok() {
            let json = JsFuture::from(resp.json()?).await?;
            Ok(serde_wasm_bindgen::from_value(json)?)
        } else {
            let json = JsFuture::from(resp.json()?).await?;
            let error: serde_json::Value = serde_wasm_bindgen::from_value(json)?;
            Err(JsValue::from_str(
                &error["error"].as_str().unwrap_or("Unknown error"),
            ))
        }
    }
}
