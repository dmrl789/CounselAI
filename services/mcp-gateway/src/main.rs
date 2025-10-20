//! Counsel AI — MCP Gateway
//! Acts as the local privacy firewall between GPT-5 and on-device data.
//! Supports both online reasoning (GPT-5) and offline (llama.cpp).

mod routes;
mod model;
mod gpt_client;

use axum::{routing::{get, post}, Router};
use dotenvy::dotenv;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Define API routes
    let app = Router::new()
        .route("/health", get(routes::health))
        .route("/query", post(routes::query))
        .route("/reason", post(routes::reason))
        .route("/reason_local", post(routes::reason_local))
        .route("/verify", post(routes::verify))
        .route("/store", post(routes::store));

    // Bind address
    let addr: SocketAddr = std::env::var("BIND_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:5142".to_string())
        .parse()?;

    tracing::info!("🚀 MCP Gateway running on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
