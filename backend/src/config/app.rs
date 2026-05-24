use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub app_name: String,
    pub port: u16,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            app_name: env::var(
                "APP_NAME",
            )
            .unwrap_or(
                "BlockSmith".to_string(),
            ),

            port: env::var("PORT")
                .unwrap_or(
                    "3000".to_string(),
                )
                .parse()
                .unwrap_or(3000),
        }
    }
}