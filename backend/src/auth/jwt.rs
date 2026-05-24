use jsonwebtoken::{
    decode,
    encode,
    DecodingKey,
    EncodingKey,
    Header,
    Validation,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

const SECRET_KEY: &[u8] =
    b"blocksmith_super_secret_key";

pub fn create_jwt(
    username: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims {
        sub: username.to_string(),
        exp: 2000000000,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(
            SECRET_KEY,
        ),
    )
}

pub fn verify_jwt(
    token: &str,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let decoded = decode::<Claims>(
        token,
        &DecodingKey::from_secret(
            SECRET_KEY,
        ),
        &Validation::default(),
    )?;

    Ok(decoded.claims)
}