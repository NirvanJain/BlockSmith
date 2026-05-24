#[derive(Debug)]
pub struct HealthStatus {
    pub database: bool,
    pub cache: bool,
    pub websocket: bool,
}

pub fn check_health(
) -> HealthStatus {
    HealthStatus {
        database: true,
        cache: true,
        websocket: true,
    }
}

pub fn is_healthy(
    health: &HealthStatus,
) -> bool {
    health.database
        && health.cache
        && health.websocket
}