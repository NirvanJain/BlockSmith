use crate::auth::{
    permissions::{
        can_manage_users,
        can_verify_contributions,
    },
    roles::UserRole,
};

pub fn require_admin(
    role: &UserRole,
) -> bool {
    can_manage_users(role)
}

pub fn require_maintainer(
    role: &UserRole,
) -> bool {
    can_verify_contributions(role)
}

pub fn require_authenticated(
) -> bool {
    true
}