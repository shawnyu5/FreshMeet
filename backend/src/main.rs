#![allow(clippy::needless_return)]
mod meetup;
mod routes;
mod utils;
use std::fs::File;
use std::io::Write;

use routes::APIDoc;
use tokio::{net::TcpListener, signal};
use tracing::{info, Level};
use utoipa::OpenApi;

use crate::routes::app;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(Level::INFO)
        .compact()
        .init();

    generate_open_api_spec();
    let addr = "0.0.0.0:8000";
    let listener = TcpListener::bind(addr).await.unwrap();
    info!("Listening on {}", addr);
    axum::serve(listener, app())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

/// Create a file and write generated open API spec to it
fn generate_open_api_spec() {
    let api_doc = APIDoc::openapi()
        .to_pretty_json()
        .expect("Failed to generate open API spec");
    let mut file = File::create("open_api.json").expect("Failed to create open API spec file");
    file.write_all(api_doc.as_bytes())
        .expect("Failed to write open api spec to file");
    info!("Created open_api.json")
}
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}
