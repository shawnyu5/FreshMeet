use std::env;

/// returns the EventBrite API key
pub fn api_key() -> String {
    return env::var("EVENTBRITE_API_KEY").unwrap();
}

/// return the EventBrite API address
pub fn api_address() -> String {
    return env::var("EVENTBRITE_API_ADDRESS").unwrap();
}
/// check to ensure all environment variables for eventbrite was set
pub fn check_enviroment_vars() -> bool {
    let api_key = env::var("EVENTBRITE_API_KEY");
    let api_address = env::var("EVENTBRITE_API_ADDRESS");
    return api_key.is_ok() && api_address.is_ok();
}