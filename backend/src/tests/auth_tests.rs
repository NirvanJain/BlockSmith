use blocksmith::auth::jwt::{
    create_jwt,
    verify_jwt,
};

#[test]
fn test_jwt_creation() {
    let token =
        create_jwt("nirvanjain");

    assert!(token.is_ok());
}

#[test]
fn test_jwt_verification() {
    let token =
        create_jwt("nirvanjain")
            .unwrap();

    let verified =
        verify_jwt(&token);

    assert!(verified.is_ok());
}