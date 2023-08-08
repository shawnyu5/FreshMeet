pub mod meetup;

use axum::{
    http::{self, Method},
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use self::meetup::search;

pub fn app() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers([http::header::CONTENT_TYPE])
        .allow_methods([Method::GET, Method::POST]);
    return Router::new()
        .route("/", get(hello))
        .route("/meetup/search", post(search))
        .layer(cors);
}

async fn hello() -> &'static str {
    return "Hello world!";
}
