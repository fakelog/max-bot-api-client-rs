use reqwest::{Client, Method, Response};
use serde::Serialize;
use std::collections::HashMap;
use url::Url;

use crate::types::Result;
use crate::{
    error::MaxBotError,
    models::{
        ApiError, BotInfo, BotPatch, Chat, ChatList, ChatPatch, GetSubscriptionsResult, Message,
        NewMessageBody, SendMessageResult, SubscriptionRequestBody, UpdateList,
    },
};

#[derive(Debug, Clone)]
pub struct MaxBotClient {
    client: Client,
    base_url: Url,
    access_token: String,
}

impl MaxBotClient {
    pub fn new(access_token: String) -> Self {
        Self {
            client: Client::new(),
            base_url: Url::parse("https://botapi.max.ru").unwrap(),
            access_token,
        }
    }

    pub fn with_base_url(access_token: String, base_url: &str) -> Result<Self> {
        Ok(Self {
            client: Client::new(),
            base_url: Url::parse(base_url).map_err(|e| MaxBotError::InvalidUrl(e.to_string()))?,
            access_token,
        })
    }

    async fn send_request<T: Serialize>(
        &self,
        method: Method,
        path: &str,
        query: Option<&HashMap<&str, String>>,
        body: Option<&T>,
    ) -> Result<Response> {
        let url: Url = self.base_url.join(path)?;

        // Add access token to query params
        let mut query_params = HashMap::new();
        query_params.insert("access_token", self.access_token.clone());

        if let Some(q) = query {
            for (k, v) in q.iter() {
                query_params.insert(*k, v.clone());
            }
        }

        let request = self.client.request(method, url).query(&query_params);

        let request = if let Some(b) = body {
            request.json(b)
        } else {
            request
        };

        let response = request.send().await?;

        if !response.status().is_success() {
            let error: ApiError = response.json().await?;
            return Err(MaxBotError::ApiError {
                code: error.code,
                message: error.message,
            });
        }

        Ok(response)
    }

    // Bot methods
    pub async fn get_my_info(&self) -> Result<BotInfo> {
        let response = self
            .send_request::<()>(reqwest::Method::GET, "/me", None, None)
            .await?;
        Ok(response.json().await?)
    }

    pub async fn edit_my_info(&self, patch: &BotPatch) -> Result<BotInfo> {
        let response = self
            .send_request(Method::PATCH, "/me", None, Some(patch))
            .await?;
        Ok(response.json().await?)
    }

    // Chat methods
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

    // Message methods
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

    // Subscription methods
    pub async fn get_subscriptions(&self) -> Result<GetSubscriptionsResult> {
        let response = self
            .send_request::<()>(Method::GET, "/subscriptions", None, None)
            .await?;
        Ok(response.json().await?)
    }

    pub async fn subscribe(&self, subscription: &SubscriptionRequestBody) -> Result<()> {
        self.send_request(Method::POST, "/subscriptions", None, Some(subscription))
            .await?;
        Ok(())
    }

    pub async fn unsubscribe(&self, url: &str) -> Result<()> {
        let mut query = HashMap::new();
        query.insert("url", url.to_string());

        self.send_request::<()>(Method::DELETE, "/subscriptions", Some(&query), None)
            .await?;
        Ok(())
    }

    // Updates
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
