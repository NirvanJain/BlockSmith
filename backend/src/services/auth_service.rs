use crate::auth::jwt::create_jwt;

pub fn login_user(
    username: &str,
) -> Option<String> {
    match create_jwt(username) {
        Ok(token) => Some(token),
        Err(_) => None,
    }
}