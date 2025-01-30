use chrono::{Duration, TimeDelta};

pub const SALT_LENGTH: usize = 16;
pub const NONCE_LENGTH: usize = 12;
pub const SESSION_TTL: TimeDelta = Duration::minutes(10);
pub const CLIPBOARD_CLEAR_TIMEOUT: TimeDelta = Duration::seconds(10);