use std::collections::HashMap;

use reqwest::Method;

use crate::api::{
    ApiClient,
    models::{Message, NewMessageBody, SendMessageResult},
    types::Result,
};

impl ApiClient {
    pub async fn send_message(
        &self,
        message: &NewMessageBody,
        chat_id: Option<i64>,
        user_id: Option<i64>,
    ) -> Result<SendMessageResult> {
        let mut query = HashMap::new();
        if let Some(cid) = chat_id {
            query.insert("chat_id", cid.to_string());
        }
        if let Some(uid) = user_id {
            query.insert("user_id", uid.to_string());
        }

        let response = self
            .send_request(Method::POST, "/messages", Some(&query), Some(message))
            .await?;
        Ok(response.json().await?)
    }

    pub async fn get_message(&self, message_id: &str) -> Result<Message> {
        let path = format!("/messages/{}", message_id);
        let response = self
            .send_request::<()>(Method::GET, &path, None, None)
            .await?;
        Ok(response.json().await?)
    }

    pub async fn edit_message(&self, message_id: &str, message: &NewMessageBody) -> Result<()> {
        let mut query = HashMap::new();
        query.insert("message_id", message_id.to_string());

        self.send_request(Method::PUT, "/messages", Some(&query), Some(message))
            .await?;
        Ok(())
    }

    pub async fn delete_message(&self, message_id: &str) -> Result<()> {
        let mut query = HashMap::new();
        query.insert("message_id", message_id.to_string());

        self.send_request::<()>(Method::DELETE, "/messages", Some(&query), None)
            .await?;
        Ok(())
    }
}
