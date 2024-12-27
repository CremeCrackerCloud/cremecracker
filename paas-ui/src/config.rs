use leptos::*;

#[derive(Clone, Debug)]
pub struct Config {
    pub api_host: String,
}

impl Config {
    pub fn load() -> Self {
        let api_host = option_env!("API_HOST")
            .unwrap_or("http://127.0.0.1:3000")
            .to_string();

        Config { api_host }
    }
}

#[component]
pub fn ConfigProvider(children: Children) -> impl IntoView {
    let config = Config::load();
    provide_context(config);
    children()
}
