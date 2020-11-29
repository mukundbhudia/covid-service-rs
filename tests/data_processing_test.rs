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
    assert_ne!(
        data_processing::generate_id_key(&Some("Hong Kong".to_string()), &"China".to_string()),
        "china hong kong".to_string()
    );
}

#[test]
fn test_id_key_gen_no_province() {
    assert_eq!(
        data_processing::generate_id_key(&None, &"China".to_string()),
        "china".to_string()
    );
}

#[test]
fn test_id_key_gen_no_country() {
    assert_eq!(
        data_processing::generate_id_key(&Some("Hong Kong".to_string()), &"".to_string()),
        "".to_string()
    );
}
