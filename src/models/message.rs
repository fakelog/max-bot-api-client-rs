use super::attachment::{Attachment, MarkupElement};
use super::user::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub sender: Option<User>,
    pub recipient: Recipient,
    pub timestamp: i64,
    pub link: Option<LinkedMessage>,
    pub body: MessageBody,
    pub stat: Option<MessageStat>,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipient {
    pub chat_id: Option<i64>,
    pub chat_type: super::chat::ChatType,
    pub user_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkedMessage {
    #[serde(rename = "type")]
    pub link_type: MessageLinkType,
    pub sender: Option<User>,
    pub chat_id: Option<i64>,
    pub message: MessageBody,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageLinkType {
    Forward,
    Reply,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageBody {
    pub mid: String,
    pub seq: i64,
    pub text: Option<String>,
    pub attachments: Option<Vec<Attachment>>,
    pub markup: Option<Vec<MarkupElement>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageStat {
    pub views: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageList {
    pub messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewMessageBody {
    pub text: Option<String>,
    pub attachments: Option<Vec<super::attachment::AttachmentRequest>>,
    pub link: Option<NewMessageLink>,
    pub notify: Option<bool>,
    pub format: Option<TextFormat>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TextFormat {
    Markdown,
    Html,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageResult {
    pub message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewMessageLink {
    #[serde(rename = "type")]
    pub link_type: MessageLinkType,
    pub mid: String,
}
