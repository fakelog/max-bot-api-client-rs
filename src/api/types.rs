use crate::api::error::ApiClientError;

pub type Result<T> = std::result::Result<T, ApiClientError>;
