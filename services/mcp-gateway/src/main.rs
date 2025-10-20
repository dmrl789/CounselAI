//! Counsel AI — MCP Gateway
//! Acts as the local privacy firewall between GPT-5 and on-device data.
//! Supports both online reasoning (GPT-5) and offline (llama.cpp).

mod routes;
mod model;
mod gpt_client;
mod auth;
mod config;
mod health;
mod openapi;

use axum::{
    routing::{get, post},
    Router,
    middleware,
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        Method,
    },
};
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
    compression::CompressionLayer,
    sensitive_headers::SetSensitiveHeadersLayer,
};
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use std::time::Duration;
use dotenvy::dotenv;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    // Load and validate configuration
    let config = config::Config::from_env()?;
    if let Err(errors) = config.validate() {
        eprintln!("Configuration validation failed:");
        for error in errors {
            eprintln!("  - {}", error);
        }
        std::process::exit(1);
    }

    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::new(&config.log_level))
        .init();

    // Initialize health monitoring
    health::init();

    // Rate limiting configuration
    let governor_conf = Box::new(
        GovernorConfigBuilder::default()
            .per_second(config.rate_limit_per_second as u64)
            .burst_size(config.rate_limit_burst_size)
            .finish()
            .unwrap(),
    );

    // Define API routes with middleware
    let app = Router::new()
        .route("/health", get(health::health_check))
        .route("/metrics", get(health::metrics))
        .route("/query", post(routes::query))
        .route("/reason", post(routes::reason))
        .route("/reason_local", post(routes::reason_local))
        .route("/verify", post(routes::verify))
        .route("/store", post(routes::store))
        .merge(openapi::create_swagger_ui())
        .layer(middleware::from_fn(auth::auth_middleware))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                .layer(SetSensitiveHeadersLayer::new(std::iter::once(AUTHORIZATION)))
                .layer(GovernorLayer {
                    config: &governor_conf,
                })
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
                        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
                        .max_age(std::time::Duration::from_secs(3600))
                )
        );

    // Bind address
    let addr: SocketAddr = config.bind_addr.parse()?;

    tracing::info!("🚀 MCP Gateway running on http://{}", addr);
    tracing::info!("Configuration: rate_limit={}/s, burst={}, compression={}, cors={}", 
                   config.rate_limit_per_second, 
                   config.rate_limit_burst_size,
                   config.enable_compression,
                   config.enable_cors);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
