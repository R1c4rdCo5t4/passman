use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Invalid command")]
    InvalidCommand,

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Missing required argument")]
    MissingArgument(String),
}
