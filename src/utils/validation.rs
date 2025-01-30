use regex::Regex;
use lazy_static::lazy_static;

const ARG_MAX_LEN: usize = 64;
const PASSWORD_MAX_LEN: usize = 128;
const PASSWORD_MIN_LEN: usize = 8;

lazy_static! {
    static ref ARG_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_.@+\-/]+$").unwrap();
    static ref UPPERCASE_REGEX: Regex = Regex::new(r"[A-Z]").unwrap();
    static ref LOWERCASE_REGEX: Regex = Regex::new(r"[a-z]").unwrap();
    static ref DIGIT_REGEX: Regex = Regex::new(r"\d").unwrap();
    static ref SPECIAL_CHAR_REGEX: Regex = Regex::new(r#"[!@#$%^&*()_+=\[\]{};:'",.<>?/\\|`~\-\s]"#).unwrap();
}

fn validate_arg(input: &str, name: &str) -> Result<(), String> {
    if input.len() > ARG_MAX_LEN {
        return Err(format!("Argument too long: {}", name));
    }
    if input.is_empty() {
        return Err(format!("Argument cannot be empty: {}", name));
    }
    // whitelisting validation
    if ARG_REGEX.is_match(input) {
        Ok(())
    } else {
        Err(format!("Invalid argument: {}", name))
    }
}

fn validate_password(password: &str) -> Result<(), &'static str> {
    if password.len() > PASSWORD_MAX_LEN {
        return Err("Password too long");
    }
    if password.is_empty() {
        return Err("Password cannot be empty");
    }
    // blacklisting validation
    if password.chars().any(|c| c.is_control()) {
        return Err("Invalid password: contains control characters");
    }
    Ok(())
}

fn validate_password_strength(password: &str) -> Result<(), &'static str> {
    if password.len() < PASSWORD_MIN_LEN {
        return Err("Password too short");
    }
    if !(UPPERCASE_REGEX.is_match(password)
        && LOWERCASE_REGEX.is_match(password)
        && DIGIT_REGEX.is_match(password)
        && SPECIAL_CHAR_REGEX.is_match(password)
    ) {
        return Err("Password must contain at least one uppercase letter, one lowercase letter, one digit, and one special character");
    }
    Ok(())
}