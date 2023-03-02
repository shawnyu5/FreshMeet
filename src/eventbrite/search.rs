use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{env, error::Error};

// #[derive(Serialize, Deserialize, Debug)]
// pub struct EventBrite {}

/// check to ensure all environment variables for eventbrite was set
fn check_enviroment_vars() -> bool {
    let api_key = env::var("EVENTBRITE_API_KEY");
    let api_address = env::var("EVENTBRITE_API_ADDRESS");
    return api_key.is_ok() && api_address.is_ok();
}

// pub fn new() -> EventBrite {
// return EventBrite {};
// }

// pub fn search() -> Result<String, Box<dyn Error>> {
// let url = "https://www.eventbrite.ca/api/v3/destination/search/";
// let mut body = HashMap::new();
// body.insert("q", "toronto");
// }
