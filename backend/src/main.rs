#[macro_use]
extern crate rocket;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Method, Status};
use rocket::response::Responder;
use rocket::{get, routes};
use rocket::{Request, Response};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};

mod eventbrite;
mod meetup;
mod routes;

// lazy_static! {
// pub static ref CACHE: Arc<Cache<String, Result_>> = Arc::new(Cache::<String, Result_>::new());
// }

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, OPTIONS, DELETE",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/")]
fn index() -> &'static str {
    return "Hello";
}

#[options("/meetup/search")]
fn handle_options_request() -> Status {
    // response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
    // response.set_header(Header::new(
    // "Access-Control-Allow-Methods",
    // "GET, POST, OPTIONS",
    // ));
    // response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
    // response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    Status::Ok
}

// fn cors_options() -> Cors {
// let allowed_origins = AllowedOrigins::all(); // You can specify specific origins if needed

// let allowed_methods = vec![Method::Get, Method::Post, Method::Options]
// .into_iter()
// .map(From::from)
// .collect();

// let cors = CorsOptions::default()
// .allowed_origins(AllowedOrigins::all())
// .allowed_methods(
// vec![Method::Get, Method::Post, Method::Patch]
// .into_iter()
// .map(From::from)
// .collect(),
// )
// .allow_credentials(true);

// CorsOptions {
// allowed_origins,
// allowed_methods,
// allowed_headers: AllowedHeaders::all(),
// allow_credentials: true,
// ..Default::default()
// }
// .to_cors()
// .expect("Failed to create CORS")
// }

#[launch]
fn rocket() -> _ {
    println!("Starting on port 8000");

    rocket::build()
        .attach(CORS)
        .mount("/", routes![index, handle_options_request])
        .mount(
            "/meetup",
            routes![routes::meetup::search, routes::meetup::search_post],
        )
}
