use reqwest::Error as ReqwestError;
use thiserror::Error;
use tokio::sync::mpsc::error::SendError;
use url::ParseError;

use crate::models::Update;

#[derive(Debug, Error)]
pub enum MaxBotError {
    #[error("Request error: {0}")]
    RequestError(#[from] ReqwestError),

    #[error("API error: {message} (code: {code})")]
    ApiError { code: String, message: String },

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("URL parse error: {0}")]
    UrlParseError(#[from] ParseError),

    #[error("Channel send error: {0}")]
    SendError(#[from] SendError<Update>),
}
