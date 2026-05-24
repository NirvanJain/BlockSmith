use crate::auth::roles::UserRole;

pub fn can_verify_contributions(
    role: &UserRole,
) -> bool {
    matches!(
        role,
        UserRole::Admin
            | UserRole::Maintainer
    )
}

pub fn can_manage_users(
    role: &UserRole,
) -> bool {
    matches!(
        role,
        UserRole::Admin
    )
}

pub fn can_create_blocks(
    role: &UserRole,
) -> bool {
    matches!(
        role,
        UserRole::Admin
            | UserRole::Maintainer
            | UserRole::Contributor
    )
}

pub fn can_view_dashboard(
    role: &UserRole,
) -> bool {
    matches!(
        role,
        UserRole::Admin
            | UserRole::Maintainer
            | UserRole::Contributor
            | UserRole::Viewer
    )
}