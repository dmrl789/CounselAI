//! Counsel AI - MCP Gateway main entry point
//! Provides secure API endpoints for local legal reasoning.

mod routes;
mod model;
mod gpt_client;

use axum::{routing::get, Router};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let app = Router::new()
        .route("/health", get(routes::health))
        .route("/query", axum::routing::post(routes::query))
        .route("/reason", axum::routing::post(routes::reason))
        .route("/verify", axum::routing::post(routes::verify))
        .route("/store", axum::routing::post(routes::store));

    let addr = SocketAddr::from(([0, 0, 0, 0], 5142));
    tracing::info!("\u{1f680} MCP Gateway running on http://{}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;
    Ok(())
}
