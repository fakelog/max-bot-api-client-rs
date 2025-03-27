use crate::{
    api,
    api::models::{Message, MessageLinkType, NewMessageBody, NewMessageLink, SendMessageResult},
    client::MaxBotClient,
};

type Result = api::types::Result<SendMessageResult>;

pub trait MessageExt {
    fn answer(
        &self,
        client: MaxBotClient,
        text: &str,
    ) -> impl std::future::Future<Output = Result> + Send;
    fn forward(
        &self,
        client: MaxBotClient,
        text: &str,
    ) -> impl std::future::Future<Output = Result> + Send;
    fn reply(
        &self,
        client: MaxBotClient,
        text: &str,
    ) -> impl std::future::Future<Output = Result> + Send;
}

impl MessageExt for Message {
    async fn answer(&self, client: MaxBotClient, text: &str) -> Result {
        let message = NewMessageBody::new(text.into());

        client
            .api_client
            .send_message(&message, self.recipient.chat_id, None)
            .await
    }

    async fn forward(&self, client: MaxBotClient, text: &str) -> Result {
        let mut message = NewMessageBody::new(text.into());

        message.link = Some(NewMessageLink {
            link_type: MessageLinkType::Forward,
            mid: self.body.mid.clone(),
        });

        client
            .api_client
            .send_message(&message, self.recipient.chat_id, None)
            .await
    }

    async fn reply(&self, client: MaxBotClient, text: &str) -> Result {
        let mut message = NewMessageBody::new(text.into());

        message.link = Some(NewMessageLink {
            link_type: MessageLinkType::Reply,
            mid: self.body.mid.clone(),
        });

        client
            .api_client
            .send_message(&message, self.recipient.chat_id, None)
            .await
    }
}
