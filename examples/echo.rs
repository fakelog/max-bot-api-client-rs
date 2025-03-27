use dotenvy::dotenv;
use max_bot_api_client_rs::message::MessageExt;
use std::env;

use max_bot_api_client_rs::api::models::{Message, Update};
use max_bot_api_client_rs::api::types::Result;
use max_bot_api_client_rs::{client::MaxBotClient, event_handler::EventHandler};

async fn handle_message(client: MaxBotClient, message: Message) -> Result<()> {
    let text = match message.body.text {
        Some(ref text) => text,
        None => return Ok(()),
    };

    let _ = message.reply(client, text).await?;

    Ok(())
}

async fn handle_update(client: MaxBotClient, update: Update) -> Result<()> {
    match update {
        Update::MessageCreated(message_update) => {
            let message = message_update.message;

            handle_message(client, message).await?
        }
        _ => {
            println!("Необработанный тип события: {:?}", update);
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let access_token = env::var("ACCESS_TOKEN").expect("TOKEN not set");
    let client = MaxBotClient::new(access_token);

    let (mut event_handler, mut event_receiver) = EventHandler::new(client.clone());

    let handler_task = tokio::spawn(async move {
        if let Err(e) = event_handler.start().await {
            eprintln!("Ошибка в обработчике событий: {}", e);
        }
    });

    while let Some(update) = event_receiver.recv().await {
        if let Err(e) = handle_update(client.clone(), update).await {
            eprintln!("Ошибка обработки обновления: {}", e);
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }

    if let Err(e) = handler_task.await {
        eprintln!("Ошибка в задаче обработчика: {:?}", e);
    }

    Ok(())
}
