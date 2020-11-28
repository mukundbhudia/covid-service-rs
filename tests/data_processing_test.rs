use covid_service_rs::data_processing;

#[test]
fn test_string_hyphenation() {
    assert_eq!(
        data_processing::hyphenate_string("hello world".to_string()),
        "hello-world".to_string()
    );
}

#[test]
fn test_id_key_gen() {
    assert_eq!(
        data_processing::generate_id_key(&Some("Hong Kong".to_string()), &"China".to_string()),
        "china-hong-kong".to_string()
    );
}
