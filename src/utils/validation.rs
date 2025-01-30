use regex::Regex;
use lazy_static::lazy_static;
use crate::domain::app::error::AppError;

const ARG_MAX_LEN: usize = 64;
const PASSWORD_MAX_LEN: usize = 128;
const PASSWORD_MIN_LEN: usize = 8;

lazy_static! {
    static ref ARG_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_.@+\-]+$").unwrap();
    static ref UPPERCASE_REGEX: Regex = Regex::new(r"[A-Z]").unwrap();
    static ref LOWERCASE_REGEX: Regex = Regex::new(r"[a-z]").unwrap();
    static ref DIGIT_REGEX: Regex = Regex::new(r"\d").unwrap();
    static ref SPECIAL_CHAR_REGEX: Regex = Regex::new(r#"[!@#$%^&*()_+=\[\]{};:'",.<>?/\\|`~\-\s]"#).unwrap();
}

pub fn validate_arg(input: &str, name: &str) -> Result<(), AppError> {
    let invalid_arg_err = AppError::InvalidArgument(name.to_string());
    if input.len() > ARG_MAX_LEN {
        return Err(invalid_arg_err);
    }
    if input.is_empty() {
        return Err(invalid_arg_err);
    }
    // whitelisting validation
    if ARG_REGEX.is_match(input) {
        Ok(())
    } else {
        Err(invalid_arg_err)
    }
}

pub fn validate_password(password: &str) -> Result<(), AppError> {
    if password.len() > PASSWORD_MAX_LEN {
        return Err(AppError::Other("Password too long".to_string()));
    }
    if password.is_empty() {
        return Err(AppError::Other("Password cannot be empty".to_string()));
    }
    // blacklisting validation
    if password.chars().any(|c| c.is_control()) {
        return Err(AppError::Other("Password cannot contain control characters".to_string()));
    }
    Ok(())
}

pub fn validate_password_strength(password: &str) -> Result<(), AppError> {
    if password.len() < PASSWORD_MIN_LEN {
        return Err(AppError::Other("Password too short".to_string()));
    }
    if !(UPPERCASE_REGEX.is_match(password)
        && LOWERCASE_REGEX.is_match(password)
        && DIGIT_REGEX.is_match(password)
        && SPECIAL_CHAR_REGEX.is_match(password)
    ) {
        return Err(AppError::Other("Password too weak".to_string()));
    }
    Ok(())
}