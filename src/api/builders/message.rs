use crate::api::models::{NewMessageBody, TextFormat};

impl NewMessageBody {
    pub fn new(text: String) -> Self {
        Self {
            text: Some(text.into()),
            attachments: None,
            link: None,
            notify: Some(false),
            format: Some(TextFormat::Markdown),
        }
    }

    pub fn set_notify(&mut self, notify: bool) {
        self.notify = Some(notify);
    }
}
