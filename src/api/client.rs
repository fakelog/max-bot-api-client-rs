use reqwest::{Client, Method, Response, Url};
use serde::Serialize;
use std::collections::HashMap;

use super::error::ApiClientError;
use crate::{api::types::Result, models::ApiError};

#[derive(Debug, Clone)]
pub struct ApiClient {
    http_client: Client,
    base_url: Url,
    access_token: String,
}

impl ApiClient {
    pub fn new(access_token: String, base_url: Url) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            access_token,
        }
    }

    pub async fn send_request<T: Serialize>(
        &self,
        method: Method,
        path: &str,
        query: Option<&HashMap<&str, String>>,
        body: Option<&T>,
    ) -> Result<Response> {
        let url = self.base_url.join(path)?;

        let mut query_params = HashMap::new();
        query_params.insert("access_token", self.access_token.clone());

        if let Some(q) = query {
            for (k, v) in q.iter() {
                query_params.insert(*k, v.clone());
            }
        }

        let request = self.http_client.request(method, url).query(&query_params);

        let request = if let Some(b) = body {
            request.json(b)
        } else {
            request
        };

        let response = request.send().await?;

        if !response.status().is_success() {
            let error: ApiError = response.json().await?;
            return Err(ApiClientError::ApiError {
                code: error.code,
                message: error.message,
            });
        }

        Ok(response)
    }
}
