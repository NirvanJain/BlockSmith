use chrono::Utc;

#[derive(Debug, Clone)]
pub struct Session {
    pub user_id: i64,
    pub access_token: String,
    pub refresh_token: String,
    pub created_at: String,
}

impl Session {
    pub fn new(
        user_id: i64,
        access_token: String,
        refresh_token: String,
    ) -> Self {
        Self {
            user_id,
            access_token,
            refresh_token,
            created_at:
                Utc::now()
                    .to_rfc3339(),
        }
    }
}