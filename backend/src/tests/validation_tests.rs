use blocksmith::utils::validators::{
    validate_contribution_type,
    validate_github_username,
    validate_repository,
};

#[test]
fn test_validate_username() {
    assert!(
        validate_github_username(
            "nirvanjain"
        )
    );
}

#[test]
fn test_validate_repository() {
    assert!(
        validate_repository(
            "owner/repo"
        )
    );
}

#[test]
fn test_validate_contribution_type() {
    assert!(
        validate_contribution_type(
            "pull_request"
        )
    );
}