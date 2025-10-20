use axum::{
    extract::Request,
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};
use std::env;

/// Simple API key authentication middleware
pub async fn auth_middleware(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Skip auth for health endpoint
    if req.uri().path() == "/health" {
        return Ok(next.run(req).await);
    }

    // Get API key from environment
    let expected_api_key = env::var("API_KEY").unwrap_or_else(|_| "counsel-ai-dev-key".to_string());
    
    // Extract API key from Authorization header
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Check if it's a Bearer token
    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let api_key = &auth_header[7..]; // Remove "Bearer " prefix
    
    if api_key != expected_api_key {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(req).await)
}

/// Generate a secure API key
pub fn generate_api_key() -> String {
    use uuid::Uuid;
    format!("counsel-ai-{}", Uuid::new_v4().to_string().replace('-', ""))
}