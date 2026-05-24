use chrono::{
    Duration,
    Utc,
};

pub fn current_timestamp() -> String {
    Utc::now().to_rfc3339()
}

pub fn add_minutes(
    minutes: i64,
) -> String {
    (Utc::now()
        + Duration::minutes(minutes))
    .to_rfc3339()
}

pub fn add_days(
    days: i64,
) -> String {
    (Utc::now()
        + Duration::days(days))
    .to_rfc3339()
}