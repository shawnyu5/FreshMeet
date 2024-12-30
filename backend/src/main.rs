#![allow(clippy::needless_return)]
mod meetup;
mod routes;
mod utils;

use crate::routes::app;
use anyhow::Result;
use common_axum::axum::axum_serve;
use tokio::net::TcpListener;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(Level::INFO)
        .compact()
        .init();

    let addr = "0.0.0.0:8000";
    let listener = TcpListener::bind(addr).await.unwrap();
    info!("Listening on {}", addr);
    return axum_serve(listener, app()).await;
}
