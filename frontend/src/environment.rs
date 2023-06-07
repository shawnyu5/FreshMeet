#[derive(Debug, Clone)]
pub struct Environment {
    pub api_url: &'static str,
}

pub fn load() -> Environment {
    // dotenv::dotenv().ok();
    Environment {
        // api_url: "http://localhost:8000",
        api_url: std::option_env!("API_URL").unwrap_or("https://networking-accumulator.fly.dev/"),
    }
}
