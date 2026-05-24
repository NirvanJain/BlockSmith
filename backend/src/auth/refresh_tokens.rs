use chrono::{
    Duration,
    Utc,
};

use jsonwebtoken::{
    encode,
    EncodingKey,
    Header,
};

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: String,
    pub exp: usize,
}

const REFRESH_SECRET: &[u8] =
    b"blocksmith_refresh_secret";

pub fn create_refresh_token(
    username: &str,
) -> Result<
    String,
    jsonwebtoken::errors::Error,
> {
    let expiration = Utc::now()
        + Duration::days(30);

    let claims = RefreshClaims {
        sub: username.to_string(),
        exp: expiration.timestamp()
            as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(
            REFRESH_SECRET,
        ),
    )
}