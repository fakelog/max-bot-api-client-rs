use crate::api::ApiClient;
use crate::api::types::Result;
use crate::models::{BotInfo, BotPatch};

impl ApiClient {
    pub async fn get_my_info(&self) -> Result<BotInfo> {
        let response = self
            .send_request::<()>(reqwest::Method::GET, "/me", None, None)
            .await?;
        Ok(response.json().await?)
    }

    pub async fn edit_my_info(&self, patch: &BotPatch) -> Result<BotInfo> {
        let response = self
            .send_request(reqwest::Method::PATCH, "/me", None, Some(patch))
            .await?;
        Ok(response.json().await?)
    }
}
