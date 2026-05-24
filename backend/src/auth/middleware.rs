use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};

use crate::auth::jwt::verify_jwt;

pub async fn auth_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok());

    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => {
            header.trim_start_matches("Bearer ")
        }
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    match verify_jwt(token) {
        Ok(claims) => {
            request.extensions_mut().insert(
                claims.sub,
            );

            Ok(next.run(request).await)
        }

        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}