use std::env;

/// returns the EventBrite API key
pub fn api_key() -> String {
    return env::var("EVENTBRITE_API_KEY").unwrap();
}

/// return the EventBrite API address
pub fn api_address() -> String {
    return env::var("EVENTBRITE_API_ADDRESS").unwrap();
}
