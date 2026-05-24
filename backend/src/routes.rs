use axum::{
    routing::get,
    Router,
};

pub fn create_routes() -> Router {
    Router::new()
        .route(
            "/health",
            get(health_check),
        )
        .route(
            "/api/v1/blocks",
            get(get_blocks),
        )
        .route(
            "/api/v1/validate",
            get(validate_chain),
        )
}

async fn health_check() -> &'static str {
    "OK"
}

async fn get_blocks() -> &'static str {
    "Fetching blockchain blocks"
}

async fn validate_chain() -> &'static str {
    "Blockchain validated"
}