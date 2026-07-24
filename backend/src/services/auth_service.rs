use crate::auth::jwt::create_jwt;

pub fn login_user(
    username: &str,
) -> Option<String> {
    create_jwt(username).ok()
}