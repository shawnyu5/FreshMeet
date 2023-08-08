mod meetup;
mod routes;
use crate::routes::app;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    axum::Server::bind(&addr)
        .serve(app().into_make_service())
        .await
        .unwrap();
}
