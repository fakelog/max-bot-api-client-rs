use thiserror::Error;

#[derive(Debug, Error)]
pub enum MaxBotError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
}
