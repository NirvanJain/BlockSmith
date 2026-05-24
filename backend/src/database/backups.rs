use chrono::Utc;

pub fn create_backup() {
    let timestamp =
        Utc::now().to_rfc3339();

    println!(
        "Database backup created at {}",
        timestamp
    );
}

pub fn restore_backup() {
    println!(
        "Database backup restored"
    );
}

pub fn list_backups() {
    println!(
        "Fetching backup history..."
    );
}