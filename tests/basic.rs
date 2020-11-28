pub fn my_challenge() -> bool {
    true
}

#[cfg(test)]
mod my_challenge_tests {
    use super::*;
    #[test]
    fn general_tests() {
        assert_eq!(my_challenge(), true);
        assert_eq!(!my_challenge(), false);
    }
}
