#[derive(Debug, Clone)]
pub struct Environment {
    pub api_url: &'static str,
}

pub fn load() -> Environment {
    // dotenv::dotenv().ok();
    Environment {
        api_url: std::option_env!("API_URL").unwrap_or("http://localhost:8000"),
        // api_url: std::env::var("API_URL").unwrap_or("http://localhost:8000".to_string()),
    }
}
