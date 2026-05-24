#[test]
fn test_mock_github_response() {
    let mock_response = r#"
    {
        "login":"nirvanjain",
        "id":1
    }
    "#;

    assert!(
        mock_response.contains(
            "nirvanjain"
        )
    );
}