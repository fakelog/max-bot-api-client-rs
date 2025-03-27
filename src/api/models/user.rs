use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub user_id: i64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub is_bot: bool,
    pub last_activity_time: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserWithPhoto {
    pub user_id: i64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub is_bot: bool,
    pub last_activity_time: i64,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub full_avatar_url: Option<String>,
}
