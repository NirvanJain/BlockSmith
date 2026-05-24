use std::env;

#[derive(Debug, Clone)]
pub struct WebsocketConfig {
    pub enabled: bool,
}

impl WebsocketConfig {
    pub fn from_env() -> Self {
        Self {
            enabled: env::var(
                "WEBSOCKET_ENABLED",
            )
            .unwrap_or(
                "true".to_string(),
            )
            == "true",
        }
    }
}