use covid_service_rs::data_processing;

#[test]
fn test_string_hyphenation() {
    assert_eq!(
        data_processing::hyphenate_string("hello world".to_string()),
        "hello-world".to_string()
    );
}
