use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Common models
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleQueryResult {
    pub success: bool,
    pub message: Option<String>,
}

// Bot models
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

// Chat models
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
pub struct Image {
    pub url: String,
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

// Message models
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
pub struct User {
    pub user_id: i64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub is_bot: bool,
    pub last_activity_time: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipient {
    pub chat_id: Option<i64>,
    pub chat_type: ChatType,
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

// Implement all attachment types here...
// (Due to length, I'm showing a couple as examples)

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
    pub attachments: Option<Vec<AttachmentRequest>>,
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

// Subscription models
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

// Update models
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

// Implement all update types here...
// (Example of one update type)

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
pub struct Callback {
    pub timestamp: i64,
    pub callback_id: String,
    pub payload: Option<String>,
    pub user: User,
}

// Дополнения к существующему коду

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
    pub max_info: Option<User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineKeyboardAttachment {
    pub payload: Keyboard,
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

// AttachmentRequest и его варианты
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

// Реализация недостающих Update типов
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

// Дополнительные типы для кнопок
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

// Типы для загрузки файлов
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

// Markup Element и его варианты
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

// Audio Attachment
#[derive(Debug, Serialize, Deserialize)]
pub struct AudioAttachment {
    pub payload: MediaAttachmentPayload,
    pub transcription: Option<String>,
}

// New Message Link
#[derive(Debug, Serialize, Deserialize)]
pub struct NewMessageLink {
    #[serde(rename = "type")]
    pub link_type: MessageLinkType,
    pub mid: String,
}

// Keyboard
#[derive(Debug, Serialize, Deserialize)]
pub struct Keyboard {
    pub buttons: Vec<Vec<Button>>,
}
