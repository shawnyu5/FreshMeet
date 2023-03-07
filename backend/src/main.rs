#[macro_use]
extern crate rocket;

mod eventbrite;
mod meetup;

#[get("/search?<query>&<items>")]
async fn search(query: &str, items: Option<i32>) -> String {
    let meetup = meetup::search::Search::default();
    let items = items.unwrap_or(20);
    let result = match meetup.search(query.to_string(), None, Some(items)).await {
        Ok(search) => {
            serde_json::to_string_pretty(&search).unwrap();
            let deseralized = serde_json::to_string_pretty(&search.data.results.edges).unwrap();
            Ok(deseralized)
        }
        Err(e) => Err(e.to_string()),
    };

    // return serde_json::to_string_pretty(&search).unwrap();
    return result.unwrap();
}

#[get("/")]
fn index() -> &'static str {
    return "Hello";
}

#[launch]
fn rocket() -> _ {
    println!("Starting on port 8000");
    rocket::build().mount("/", routes![search, index])
}

// #[launch]
// fn rocket() -> _ {
// rocket::build().mount("/meetup/", routes![search])
// }

// #[tokio::main]
// async fn main() {
// dotenv().ok();
// let meetups = meetup::search::Search::new();
// }
