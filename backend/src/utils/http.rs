use axum::http::{
    HeaderMap,
    StatusCode,
};

pub fn get_authorization_header(
    headers: &HeaderMap,
) -> Option<String> {
    headers
        .get("Authorization")
        .and_then(|value| {
            value.to_str().ok()
        })
        .map(|s| s.to_string())
}

pub fn is_success_status(
    status: StatusCode,
) -> bool {
    status.is_success()
}