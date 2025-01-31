use chrono::{Duration, TimeDelta};
use lazy_static::lazy_static;
use regex::Regex;

pub const SALT_LENGTH: usize = 16;
pub const NONCE_LENGTH: usize = 12;
pub const SESSION_TTL: TimeDelta = Duration::minutes(10);
pub const CLIPBOARD_TTL: TimeDelta = Duration::seconds(10);
pub const ARG_MAX_LEN: usize = 64;
pub const PASSWORD_MAX_LEN: usize = 128;
pub const PASSWORD_MIN_LEN: usize = 8;
pub const UPPERCASE_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const LOWERCASE_CHARS: &str = "abcdefghijklmnopqrstuvwxyz";
pub const DIGIT_CHARS: &str = "0123456789";
pub const SYMBOL_CHARS: &str = "!@#$%^&*()_+[]{};:'\",.<>/?\\|`~- ";
pub const AMBIGUOUS_CHARS: &str = "B8G6I1l0OQDS5Z2";

lazy_static! {
    pub static ref ARG_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_.@+\-]+$").unwrap();
    pub static ref UPPERCASE_REGEX: Regex = Regex::new(r"[A-Z]").unwrap();
    pub static ref LOWERCASE_REGEX: Regex = Regex::new(r"[a-z]").unwrap();
    pub static ref DIGIT_REGEX: Regex = Regex::new(r"\d").unwrap();
    pub static ref SPECIAL_CHAR_REGEX: Regex = Regex::new(r#"[!@#$%^&*()_+=\[\]{};:'",.<>?/\\|`~\-\s]"#).unwrap();
}