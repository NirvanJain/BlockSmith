use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::Response,
};

pub async fn logging_middleware(
    request: Request<Body>,
    next: Next,
) -> Response {
    println!(
        "{} {}",
        request.method(),
        request.uri()
    );

    next.run(request).await
}