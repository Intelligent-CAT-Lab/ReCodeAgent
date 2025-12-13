use wspace::errors::StatsError;

#[test]
fn test_error_stub() {
    let err = StatsError::new("test error".to_string());
    assert_eq!(err.Error(), "test error", "Error method message didn't match");
    assert_eq!(err.String(), "test error", "String method message didn't match");
}
