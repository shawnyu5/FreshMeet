#[macro_use]
extern crate rocket;

use lazy_static::lazy_static;
use meetup::search::Search;
use retainer::*;
use std::sync::Arc;
use std::time::Duration;

mod eventbrite;
mod meetup;
mod routes;

lazy_static! {
    pub static ref CACHE: Arc<Cache<String, Search>> = Arc::new(Cache::<String, Search>::new());
}

#[get("/")]
fn index() -> &'static str {
    return "Hello";
}

#[launch]
fn rocket() -> _ {
    println!("Starting on port 8000");
    let cache_clone = CACHE.clone();

    // don't forget to monitor your cache to evict entries
    // let monitor =
    tokio::spawn(async move { cache_clone.monitor(4, 0.25, Duration::from_secs(3)).await });

    rocket::build()
        .mount("/", routes![index])
        .mount("/meetup", routes![routes::meetup::search])
}
