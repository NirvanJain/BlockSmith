use blocksmith::auth::jwt::{
    create_jwt,
    verify_jwt,
};

#[test]
fn test_invalid_jwt() {
    let invalid_token =
        "invalid.token.here";

    let verified =
        verify_jwt(
            invalid_token,
        );

    assert!(verified.is_err());
}

#[test]
fn test_valid_jwt() {
    let token =
        create_jwt("nirvanjain")
            .unwrap();

    let verified =
        verify_jwt(&token);

    assert!(verified.is_ok());
}