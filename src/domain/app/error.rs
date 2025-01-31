use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    InvalidCommand,
    InvalidArgument(String),
    MissingArgument(String),
    Other(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::InvalidCommand => write!(f, "Invalid command"),
            AppError::InvalidArgument(arg) => write!(f, "Invalid argument: <{}>", arg),
            AppError::MissingArgument(arg) => write!(f, "Missing argument: <{}>", arg),
            AppError::Other(err) => write!(f, "Error: {}", err),
        }
    }
}
