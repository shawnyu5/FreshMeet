pub mod meetup;

use axum::{
    http::{self, Method},
    routing::{get, post},
    Router,
};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{self, TraceLayer},
};
use tracing::Level;

use self::meetup::{search, suggested_events};

pub fn app() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers([http::header::CONTENT_TYPE])
        .allow_methods([Method::GET, Method::POST]);

    let tracing = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO));

    return Router::new()
        .route("/", get(hello))
        .route("/meetup/search", post(search))
        .route("/meetup/suggested", get(suggested_events))
        .layer(tracing)
        .layer(cors);
}

async fn hello() -> &'static str {
    return "Hello world!";
}
