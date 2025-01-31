
#[cfg(test)]
mod test_input_validations {
    use passman::utils::validation::{validate_arg, validate_password};

    #[test]
    fn validate_arguments() {
        // valid arguments
        assert!(validate_arg("test", "").is_ok());
        assert!(validate_arg("123", "").is_ok());
        assert!(validate_arg("test-123@email.com", "").is_ok());

        // invalid arguments
        assert!(validate_arg(&"A".repeat(100), "").is_err());
        assert!(validate_arg("", "").is_err());
        assert!(validate_arg("../../etc/passwd", "").is_err());
        assert!(validate_arg(";abc", "").is_err());
    }

    #[test]
    fn validate_passwords() {
        // valid passwords
        assert!(validate_password("t3stP@s$wrd!").is_ok());

        // invalid passwords
        assert!(validate_password("abcdef").is_err());
        assert!(validate_password(&"x".repeat(150)).is_err());
        assert!(validate_password("").is_err());
        assert!(validate_password("test\n").is_err());
        assert!(validate_password("test\r").is_err());
        assert!(validate_password("test\t").is_err());
        assert!(validate_password("test\0").is_err());
    }
}