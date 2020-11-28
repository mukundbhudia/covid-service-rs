use covid_service_rs::data_processing;

#[cfg(test)]
mod data_processing_tests {
    use super::*;
    #[test]
    fn general_tests() {
        assert_eq!(
            data_processing::hyphenate_string("hello world".to_string()),
            "hello-world".to_string()
        );
    }
}
