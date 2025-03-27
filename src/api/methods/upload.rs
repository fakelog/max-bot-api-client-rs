use std::collections::HashMap;

use reqwest::Method;

use crate::{api::ApiClient, api::types::Result, models::UpdateList};

impl ApiClient {
    pub async fn get_updates(
        &self,
        limit: Option<i32>,
        timeout: Option<i32>,
        marker: Option<i64>,
        types: Option<Vec<String>>,
    ) -> Result<UpdateList> {
        let mut query = HashMap::new();

        if let Some(l) = limit {
            query.insert("limit", l.to_string());
        }
        if let Some(t) = timeout {
            query.insert("timeout", t.to_string());
        }
        if let Some(m) = marker {
            query.insert("marker", m.to_string());
        }
        if let Some(t) = types {
            query.insert("types", t.join(","));
        }

        let response = self
            .send_request::<()>(Method::GET, "/updates", Some(&query), None)
            .await?;

        Ok(response.json().await?)
    }
}
