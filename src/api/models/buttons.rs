use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Button {
    Callback(CallbackButton),
    Link(LinkButton),
    RequestGeoLocation(RequestGeoLocationButton),
    RequestContact(RequestContactButton),
    Chat(ChatButton),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallbackButton {
    pub text: String,
    pub payload: String,
    pub intent: Option<Intent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkButton {
    pub text: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestGeoLocationButton {
    pub text: String,
    pub quick: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestContactButton {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatButton {
    pub text: String,
    pub chat_title: String,
    pub chat_description: Option<String>,
    pub start_payload: Option<String>,
    pub uuid: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Intent {
    Positive,
    Negative,
    Default,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ReplyButton {
    Message(SendMessageButton),
    UserGeoLocation(SendGeoLocationButton),
    UserContact(SendContactButton),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageButton {
    pub text: String,
    pub payload: Option<String>,
    pub intent: Option<Intent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendGeoLocationButton {
    pub text: String,
    pub payload: Option<String>,
    pub quick: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendContactButton {
    pub text: String,
    pub payload: Option<String>,
}
