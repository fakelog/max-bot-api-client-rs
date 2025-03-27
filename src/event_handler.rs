use tokio::sync::mpsc;

use crate::{api::types::Result, client::MaxBotClient, models::Update};

pub struct EventHandler {
    client: MaxBotClient,
    marker: Option<i64>,
    event_sender: mpsc::Sender<Update>,
}

impl EventHandler {
    pub fn new(client: MaxBotClient) -> (Self, mpsc::Receiver<Update>) {
        let (sender, receiver) = mpsc::channel(100);
        (
            Self {
                client,
                marker: None,
                event_sender: sender,
            },
            receiver,
        )
    }

    pub async fn start(&mut self) -> Result<()> {
        loop {
            let updates = self
                .client
                .get_updates(Some(100), Some(30), self.marker, None)
                .await?;

            // Отправляем все обновления через канал
            for update in updates.updates {
                self.event_sender.send(update).await?;
            }

            // Обновляем маркер для следующего запроса
            self.marker = updates.marker;
        }
    }
}
