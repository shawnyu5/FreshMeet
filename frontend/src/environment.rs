use leptos::*;
#[derive(Debug, Clone)]
pub struct Environment {
    pub api_url: &'static str,
}

pub fn load() -> Environment {
    // dotenv::dotenv().ok();
    log!("API_URL: {:?}", std::env::var("API_URL"));
    Environment {
        api_url: std::option_env!("API_URL").unwrap_or("https://networking-accumulator.fly.dev/"),
    }
}
