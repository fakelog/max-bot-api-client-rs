use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{Button, ReplyButton};

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Attachment {
    Image(PhotoAttachment),
    Video(VideoAttachment),
    Audio(AudioAttachment),
    File(FileAttachment),
    Sticker(StickerAttachment),
    Contact(ContactAttachment),
    InlineKeyboard(InlineKeyboardAttachment),
    ReplyKeyboard(ReplyKeyboardAttachment),
    Share(ShareAttachment),
    Location(LocationAttachment),
    Data(DataAttachment),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhotoAttachment {
    pub payload: PhotoAttachmentPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhotoAttachmentPayload {
    pub photo_id: i64,
    pub token: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoAttachment {
    pub payload: MediaAttachmentPayload,
    pub thumbnail: Option<VideoThumbnail>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaAttachmentPayload {
    pub url: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoThumbnail {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileAttachment {
    pub payload: FileAttachmentPayload,
    pub filename: String,
    pub size: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileAttachmentPayload {
    pub url: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StickerAttachment {
    pub payload: StickerAttachmentPayload,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StickerAttachmentPayload {
    pub url: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactAttachment {
    pub payload: ContactAttachmentPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactAttachmentPayload {
    pub vcf_info: Option<String>,
    pub max_info: Option<super::user::User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineKeyboardAttachment {
    pub payload: super::keyboard::Keyboard,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplyKeyboardAttachment {
    pub buttons: Vec<Vec<ReplyButton>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShareAttachment {
    pub payload: ShareAttachmentPayload,
    pub title: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShareAttachmentPayload {
    pub url: Option<String>,
    pub token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationAttachment {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataAttachment {
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AttachmentRequest {
    Image(PhotoAttachmentRequest),
    Video(VideoAttachmentRequest),
    Audio(AudioAttachmentRequest),
    File(FileAttachmentRequest),
    Sticker(StickerAttachmentRequest),
    Contact(ContactAttachmentRequest),
    InlineKeyboard(InlineKeyboardAttachmentRequest),
    ReplyKeyboard(ReplyKeyboardAttachmentRequest),
    Location(LocationAttachmentRequest),
    Share(ShareAttachmentRequest),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhotoAttachmentRequest {
    pub payload: PhotoAttachmentRequestPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhotoAttachmentRequestPayload {
    pub url: Option<String>,
    pub token: Option<String>,
    pub photos: Option<HashMap<String, PhotoToken>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhotoToken {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoAttachmentRequest {
    pub payload: UploadedInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AudioAttachmentRequest {
    pub payload: UploadedInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileAttachmentRequest {
    pub payload: UploadedInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StickerAttachmentRequest {
    pub payload: StickerAttachmentRequestPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StickerAttachmentRequestPayload {
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactAttachmentRequest {
    pub payload: ContactAttachmentRequestPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactAttachmentRequestPayload {
    pub name: Option<String>,
    pub contact_id: Option<i64>,
    pub vcf_info: Option<String>,
    pub vcf_phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineKeyboardAttachmentRequest {
    pub payload: InlineKeyboardAttachmentRequestPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineKeyboardAttachmentRequestPayload {
    pub buttons: Vec<Vec<Button>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplyKeyboardAttachmentRequest {
    pub direct: Option<bool>,
    pub direct_user_id: Option<i64>,
    pub buttons: Vec<Vec<ReplyButton>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationAttachmentRequest {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShareAttachmentRequest {
    pub payload: ShareAttachmentPayload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadedInfo {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadEndpoint {
    pub url: String,
    pub token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UploadType {
    Image,
    Video,
    Audio,
    File,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AudioAttachment {
    pub payload: MediaAttachmentPayload,
    pub transcription: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MarkupElement {
    Strong(StrongMarkup),
    Emphasized(EmphasizedMarkup),
    Monospaced(MonospacedMarkup),
    Link(LinkMarkup),
    Strikethrough(StrikethroughMarkup),
    Underline(UnderlineMarkup),
    UserMention(UserMentionMarkup),
    Heading(HeadingMarkup),
    Highlighted(HighlightedMarkup),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StrongMarkup {
    pub from: i32,
    pub length: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmphasizedMarkup {
    pub from: i32,
    pub length: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonospacedMarkup {
    pub from: i32,
    pub length: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkMarkup {
    pub from: i32,
    pub length: i32,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StrikethroughMarkup {
    pub from: i32,
    pub length: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnderlineMarkup {
    pub from: i32,
    pub length: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserMentionMarkup {
    pub from: i32,
    pub length: i32,
    pub user_link: Option<String>,
    pub user_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeadingMarkup {
    pub from: i32,
    pub length: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HighlightedMarkup {
    pub from: i32,
    pub length: i32,
}
