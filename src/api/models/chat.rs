use super::attachment::{Image, PhotoAttachmentRequestPayload};
use super::message::Message;
use super::user::UserWithPhoto;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Chat {
    pub chat_id: i64,
    #[serde(rename = "type")]
    pub chat_type: ChatType,
    pub status: ChatStatus,
    pub title: Option<String>,
    pub icon: Option<Image>,
    pub last_event_time: i64,
    pub participants_count: i32,
    pub owner_id: Option<i64>,
    pub participants: Option<HashMap<String, i64>>,
    pub is_public: bool,
    pub link: Option<String>,
    pub description: Option<String>,
    pub dialog_with_user: Option<UserWithPhoto>,
    pub messages_count: Option<i32>,
    pub chat_message_id: Option<String>,
    pub pinned_message: Option<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChatType {
    Dialog,
    Chat,
    Channel,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChatStatus {
    Active,
    Removed,
    Left,
    Closed,
    Suspended,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatList {
    pub chats: Vec<Chat>,
    pub marker: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatPatch {
    pub icon: Option<PhotoAttachmentRequestPayload>,
    pub title: Option<String>,
    pub pin: Option<String>,
    pub notify: Option<bool>,
}
