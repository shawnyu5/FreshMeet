mod meetup;
use axum::{
    http::{self, HeaderValue, Method},
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let cors = CorsLayer::new()
        .allow_origin("*".parse::<HeaderValue>().unwrap())
        .allow_headers([http::header::CONTENT_TYPE])
        .allow_methods([Method::GET, Method::POST]);

    let app = Router::new().route("/", get(hello)).layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello() -> &'static str {
    return "Hello world!";
}
