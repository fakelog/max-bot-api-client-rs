use std::collections::HashMap;

use reqwest::Method;

use crate::api::{
    ApiClient,
    models::{Chat, ChatList, ChatPatch},
    types::Result,
};

impl ApiClient {
    pub async fn get_chats(&self, count: Option<i32>, marker: Option<i64>) -> Result<ChatList> {
        let mut query = HashMap::new();
        if let Some(c) = count {
            query.insert("count", c.to_string());
        }
        if let Some(m) = marker {
            query.insert("marker", m.to_string());
        }

        let response = self
            .send_request::<()>(Method::GET, "/chats", Some(&query), None)
            .await?;
        Ok(response.json().await?)
    }

    pub async fn get_chat_by_link(&self, chat_link: &str) -> Result<Chat> {
        let path = format!("/chats/{}", chat_link);
        let response = self
            .send_request::<()>(Method::GET, &path, None, None)
            .await?;
        Ok(response.json().await?)
    }

    pub async fn get_chat(&self, chat_id: i64) -> Result<Chat> {
        let path = format!("/chats/{}", chat_id);
        let response = self
            .send_request::<()>(Method::GET, &path, None, None)
            .await?;
        Ok(response.json().await?)
    }

    pub async fn edit_chat(&self, chat_id: i64, patch: &ChatPatch) -> Result<Chat> {
        let path = format!("/chats/{}", chat_id);
        let response = self
            .send_request(Method::PATCH, &path, None, Some(patch))
            .await?;
        Ok(response.json().await?)
    }

    pub async fn delete_chat(&self, chat_id: i64) -> Result<()> {
        let path = format!("/chats/{}", chat_id);
        self.send_request::<()>(Method::DELETE, &path, None, None)
            .await?;
        Ok(())
    }
}
