mod config;
mod doc;
mod error;
mod handler;

use std::net::SocketAddr;

use axum::{Router, routing};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

use crate::compiler::config::Config;

fn build() -> Router {
    Router::new()
        .route("/health", routing::get(handler::health))
        .route("/compile", routing::post(handler::compile))
        .merge(doc::build())
        .layer(TraceLayer::new_for_http())
}

pub async fn run() -> color_eyre::Result<()> {
    let cfg = Config::new()?;

    let api = build();
    let listener = TcpListener::bind(SocketAddr::new([0, 0, 0, 0].into(), cfg.port)).await?;

    tracing::info!("Listening on port {}", cfg.port);

    axum::serve(listener, api).await?;

    Ok(())
}
