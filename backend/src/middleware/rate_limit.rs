use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::Response,
};

pub async fn rate_limit_middleware(
    request: Request<Body>,
    next: Next,
) -> Response {
    next.run(request).await
}