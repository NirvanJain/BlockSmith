pub fn is_admin(
    role: &str,
) -> bool {
    role == "admin"
}

pub fn is_maintainer(
    role: &str,
) -> bool {
    role == "maintainer"
}

pub fn can_manage_platform(
    role: &str,
) -> bool {
    matches!(
        role,
        "admin" | "maintainer"
    )
}