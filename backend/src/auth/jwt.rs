use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use serde::{Deserialize, Serialize};

const JWT_SECRET: &[u8] = b"blocksmith_super_secret_key";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub email: Option<String>,
}

pub async fn verify_jwt(token: &str) -> Result<Claims, String> {
    let validation = Validation::new(Algorithm::HS256);

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &validation,
    )
    .map_err(|e| e.to_string())?;

    Ok(token_data.claims)
}

pub fn create_jwt(subject: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims {
        sub: subject.to_string(),
        exp: 2000000000,
        email: None,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
}
