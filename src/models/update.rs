use super::chat::Chat;
use super::message::Message;
use super::user::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Callback {
    pub timestamp: i64,
    pub callback_id: String,
    pub payload: Option<String>,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetSubscriptionsResult {
    pub subscriptions: Vec<Subscription>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscription {
    pub url: String,
    pub time: i64,
    pub update_types: Option<Vec<String>>,
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionRequestBody {
    pub url: String,
    pub secret: Option<String>,
    pub update_types: Option<Vec<String>>,
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateList {
    pub updates: Vec<Update>,
    pub marker: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "update_type")]
#[serde(rename_all = "snake_case")]
pub enum Update {
    MessageCreated(MessageCreatedUpdate),
    MessageCallback(MessageCallbackUpdate),
    MessageEdited(MessageEditedUpdate),
    MessageRemoved(MessageRemovedUpdate),
    BotAdded(BotAddedToChatUpdate),
    BotRemoved(BotRemovedFromChatUpdate),
    UserAdded(UserAddedToChatUpdate),
    UserRemoved(UserRemovedFromChatUpdate),
    BotStarted(BotStartedUpdate),
    ChatTitleChanged(ChatTitleChangedUpdate),
    MessageChatCreated(MessageChatCreatedUpdate),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageCreatedUpdate {
    pub timestamp: i64,
    pub message: Message,
    pub user_locale: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageCallbackUpdate {
    pub timestamp: i64,
    pub callback: Callback,
    pub message: Option<Message>,
    pub user_locale: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageEditedUpdate {
    pub timestamp: i64,
    pub message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageRemovedUpdate {
    pub timestamp: i64,
    pub message_id: String,
    pub chat_id: i64,
    pub user_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BotAddedToChatUpdate {
    pub timestamp: i64,
    pub chat_id: i64,
    pub user: User,
    pub is_channel: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BotRemovedFromChatUpdate {
    pub timestamp: i64,
    pub chat_id: i64,
    pub user: User,
    pub is_channel: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAddedToChatUpdate {
    pub timestamp: i64,
    pub chat_id: i64,
    pub user: User,
    pub inviter_id: Option<i64>,
    pub is_channel: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRemovedFromChatUpdate {
    pub timestamp: i64,
    pub chat_id: i64,
    pub user: User,
    pub admin_id: Option<i64>,
    pub is_channel: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BotStartedUpdate {
    pub timestamp: i64,
    pub chat_id: i64,
    pub user: User,
    pub payload: Option<String>,
    pub user_locale: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatTitleChangedUpdate {
    pub timestamp: i64,
    pub chat_id: i64,
    pub user: User,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageChatCreatedUpdate {
    pub timestamp: i64,
    pub chat: Chat,
    pub message_id: String,
    pub start_payload: Option<String>,
}
