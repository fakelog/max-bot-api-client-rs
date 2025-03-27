use url::Url;

use crate::api::ApiClient;
use crate::api::types::Result;
use crate::error::MaxBotError;
use crate::models::{
    BotInfo, BotPatch, Chat, ChatList, ChatPatch, GetSubscriptionsResult, Message, NewMessageBody,
    SendMessageResult, SubscriptionRequestBody, UpdateList,
};

#[derive(Debug, Clone)]
pub struct MaxBotClient {
    api_client: ApiClient,
}

impl MaxBotClient {
    pub fn new(access_token: String) -> Self {
        let base_url = Url::parse("https://botapi.max.ru").unwrap();
        let api_client = ApiClient::new(access_token.clone(), base_url.clone());

        Self { api_client }
    }

    pub fn with_base_url(
        access_token: String,
        base_url: &str,
    ) -> std::result::Result<Self, MaxBotError> {
        let base_url = Url::parse(base_url).map_err(|e| MaxBotError::InvalidUrl(e.to_string()))?;
        let api_client = ApiClient::new(access_token.clone(), base_url.clone());

        Ok(Self { api_client })
    }

    // Bot methods
    pub async fn get_my_info(&self) -> Result<BotInfo> {
        self.api_client.get_my_info().await
    }

    pub async fn edit_my_info(&self, patch: &BotPatch) -> Result<BotInfo> {
        self.api_client.edit_my_info(patch).await
    }

    // Chat methods
    pub async fn get_chats(&self, count: Option<i32>, marker: Option<i64>) -> Result<ChatList> {
        self.api_client.get_chats(count, marker).await
    }

    pub async fn get_chat_by_link(&self, chat_link: &str) -> Result<Chat> {
        self.api_client.get_chat_by_link(chat_link).await
    }

    pub async fn get_chat(&self, chat_id: i64) -> Result<Chat> {
        self.api_client.get_chat(chat_id).await
    }

    pub async fn edit_chat(&self, chat_id: i64, patch: &ChatPatch) -> Result<Chat> {
        self.api_client.edit_chat(chat_id, patch).await
    }

    pub async fn delete_chat(&self, chat_id: i64) -> Result<()> {
        self.api_client.delete_chat(chat_id).await
    }

    // Message methods
    pub async fn send_message(
        &self,
        message: &NewMessageBody,
        chat_id: Option<i64>,
        user_id: Option<i64>,
    ) -> Result<SendMessageResult> {
        self.api_client
            .send_message(message, chat_id, user_id)
            .await
    }

    pub async fn get_message(&self, message_id: &str) -> Result<Message> {
        self.api_client.get_message(message_id).await
    }

    pub async fn edit_message(&self, message_id: &str, message: &NewMessageBody) -> Result<()> {
        self.api_client.edit_message(message_id, message).await
    }

    pub async fn delete_message(&self, message_id: &str) -> Result<()> {
        self.api_client.delete_message(message_id).await
    }

    // Subscription methods
    pub async fn get_subscriptions(&self) -> Result<GetSubscriptionsResult> {
        self.api_client.get_subscriptions().await
    }

    pub async fn subscribe(&self, subscription: &SubscriptionRequestBody) -> Result<()> {
        self.api_client.subscribe(subscription).await
    }

    pub async fn unsubscribe(&self, url: &str) -> Result<()> {
        self.api_client.unsubscribe(url).await
    }

    // Updates
    pub async fn get_updates(
        &self,
        limit: Option<i32>,
        timeout: Option<i32>,
        marker: Option<i64>,
        types: Option<Vec<String>>,
    ) -> Result<UpdateList> {
        self.api_client
            .get_updates(limit, timeout, marker, types)
            .await
    }
}
