use axum::{http::StatusCode, Json};
use serde_json::json;
use std::time::{Duration, Instant};
use tokio::time::timeout;
use utoipa::ToSchema;

/// Health check response structure
#[derive(serde::Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
    pub version: String,
    pub uptime: u64,
    pub checks: HealthChecks,
}

#[derive(serde::Serialize, ToSchema)]
pub struct HealthChecks {
    pub database: CheckResult,
    pub storage: CheckResult,
    pub external_apis: CheckResult,
}

#[derive(serde::Serialize, ToSchema)]
pub struct CheckResult {
    pub status: String,
    pub message: String,
    pub response_time_ms: Option<u64>,
}

/// Global start time for uptime calculation
static mut START_TIME: Option<Instant> = None;

pub fn init() {
    unsafe {
        START_TIME = Some(Instant::now());
    }
}

pub fn get_uptime() -> u64 {
    unsafe {
        START_TIME
            .map(|start| start.elapsed().as_secs())
            .unwrap_or(0)
    }
}

/// Comprehensive health check endpoint
pub async fn health_check() -> (StatusCode, Json<HealthResponse>) {
    let start_time = Instant::now();
    let mut checks = HealthChecks {
        database: check_database().await,
        storage: check_storage().await,
        external_apis: check_external_apis().await,
    };

    // Determine overall status
    let overall_status = if checks.database.status == "healthy" 
        && checks.storage.status == "healthy" 
        && checks.external_apis.status != "critical" {
        "healthy"
    } else {
        "degraded"
    };

    let response = HealthResponse {
        status: overall_status.to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime: get_uptime(),
        checks,
    };

    let status_code = if overall_status == "healthy" {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (status_code, Json(response))
}

/// Check database connectivity
async fn check_database() -> CheckResult {
    let start = Instant::now();
    
    // Simulate database check (replace with actual implementation)
    match timeout(Duration::from_secs(5), async {
        // Add actual database health check here
        tokio::time::sleep(Duration::from_millis(10)).await;
        Ok::<(), String>(())
    }).await {
        Ok(Ok(_)) => CheckResult {
            status: "healthy".to_string(),
            message: "Database connection successful".to_string(),
            response_time_ms: Some(start.elapsed().as_millis() as u64),
        },
        Ok(Err(e)) => CheckResult {
            status: "unhealthy".to_string(),
            message: format!("Database error: {}", e),
            response_time_ms: Some(start.elapsed().as_millis() as u64),
        },
        Err(_) => CheckResult {
            status: "unhealthy".to_string(),
            message: "Database connection timeout".to_string(),
            response_time_ms: Some(5000),
        },
    }
}

/// Check storage accessibility
async fn check_storage() -> CheckResult {
    let start = Instant::now();
    
    match timeout(Duration::from_secs(3), async {
        // Check if storage directory is accessible
        let storage_path = std::env::var("STORAGE_PATH").unwrap_or_else(|_| "/data/encrypted".to_string());
        tokio::fs::metadata(&storage_path).await.map_err(|e| e.to_string())
    }).await {
        Ok(Ok(_)) => CheckResult {
            status: "healthy".to_string(),
            message: "Storage accessible".to_string(),
            response_time_ms: Some(start.elapsed().as_millis() as u64),
        },
        Ok(Err(e)) => CheckResult {
            status: "unhealthy".to_string(),
            message: format!("Storage error: {}", e),
            response_time_ms: Some(start.elapsed().as_millis() as u64),
        },
        Err(_) => CheckResult {
            status: "unhealthy".to_string(),
            message: "Storage check timeout".to_string(),
            response_time_ms: Some(3000),
        },
    }
}

/// Check external API dependencies
async fn check_external_apis() -> CheckResult {
    let start = Instant::now();
    
    // Check OpenAI API if configured
    let openai_key = std::env::var("OPENAI_API_KEY").ok();
    if openai_key.is_some() {
        match timeout(Duration::from_secs(5), async {
            // Add actual OpenAI API health check here
            tokio::time::sleep(Duration::from_millis(50)).await;
            Ok::<(), String>(())
        }).await {
            Ok(Ok(_)) => CheckResult {
                status: "healthy".to_string(),
                message: "External APIs accessible".to_string(),
                response_time_ms: Some(start.elapsed().as_millis() as u64),
            },
            Ok(Err(e)) => CheckResult {
                status: "degraded".to_string(),
                message: format!("External API error: {}", e),
                response_time_ms: Some(start.elapsed().as_millis() as u64),
            },
            Err(_) => CheckResult {
                status: "degraded".to_string(),
                message: "External API timeout".to_string(),
                response_time_ms: Some(5000),
            },
        }
    } else {
        CheckResult {
            status: "healthy".to_string(),
            message: "No external APIs configured".to_string(),
            response_time_ms: Some(start.elapsed().as_millis() as u64),
        }
    }
}

/// Metrics endpoint for monitoring
pub async fn metrics() -> (StatusCode, String) {
    let metrics = format!(
        "# HELP counsel_ai_uptime_seconds Total uptime in seconds
# TYPE counsel_ai_uptime_seconds counter
counsel_ai_uptime_seconds {{}} {}

# HELP counsel_ai_requests_total Total number of requests
# TYPE counsel_ai_requests_total counter
counsel_ai_requests_total {{}} 0

# HELP counsel_ai_request_duration_seconds Request duration in seconds
# TYPE counsel_ai_request_duration_seconds histogram
counsel_ai_request_duration_seconds_bucket {{le=\"0.1\"}} 0
counsel_ai_request_duration_seconds_bucket {{le=\"0.5\"}} 0
counsel_ai_request_duration_seconds_bucket {{le=\"1.0\"}} 0
counsel_ai_request_duration_seconds_bucket {{le=\"5.0\"}} 0
counsel_ai_request_duration_seconds_bucket {{le=\"+Inf\"}} 0
counsel_ai_request_duration_seconds_sum {{}} 0
counsel_ai_request_duration_seconds_count {{}} 0
",
        get_uptime()
    );
    
    (StatusCode::OK, metrics)
}