use super::attachment::PhotoAttachmentRequestPayload;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BotInfo {
    pub user_id: i64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub is_bot: bool,
    pub last_activity_time: i64,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub full_avatar_url: Option<String>,
    pub commands: Option<Vec<BotCommand>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BotPatch {
    pub name: Option<String>,
    pub description: Option<String>,
    pub commands: Option<Vec<BotCommand>>,
    pub photo: Option<PhotoAttachmentRequestPayload>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BotCommand {
    pub name: String,
    pub description: Option<String>,
}
