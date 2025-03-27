use thiserror::Error;
use reqwest::Error as ReqwestError;
use url::ParseError;

#[derive(Debug, Error)]
pub enum MaxBotError {
    #[error("Request error: {0}")]
    RequestError(#[from] ReqwestError),
    
    #[error("API error: {message} (code: {code})")]
    ApiError {
        code: String,
        message: String,
    },
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("URL parse error: {0}")]
    UrlParseError(#[from] ParseError), 
}
