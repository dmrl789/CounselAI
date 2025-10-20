use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    model::*,
    routes::*,
    health::*,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::query,
        routes::reason,
        routes::reason_local,
        routes::verify,
        routes::store,
        health::health_check,
        health::metrics,
    ),
    components(
        schemas(
            QueryRequest,
            ReasoningRequest,
            ReasoningResponse,
            VerifyResponse,
            LogEntry,
            HealthResponse,
            HealthChecks,
            CheckResult,
        )
    ),
    tags(
        (name = "counsel-ai", description = "Counsel AI Legal Assistant API")
    ),
    info(
        title = "Counsel AI API",
        description = "A comprehensive legal assistant API providing secure, privacy-preserving legal analysis and document generation.",
        version = "1.0.0",
        contact(
            name = "IPPAN Labs",
            email = "support@ippanlabs.com"
        ),
        license(
            name = "Proprietary"
        )
    ),
    servers(
        (url = "http://localhost:5142", description = "Development server"),
        (url = "https://api.counsel-ai.com", description = "Production server")
    )
)]
pub struct ApiDoc;

pub fn create_swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/docs")
        .url("/api-docs/openapi.json", ApiDoc::openapi())
}