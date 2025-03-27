use dotenvy::dotenv;
use std::env;

use max_bot_api_client_rs::api::types::Result;
use max_bot_api_client_rs::models::Update;
use max_bot_api_client_rs::{client::MaxBotClient, event_handler::EventHandler};

async fn handle_update(update: Update) -> Result<()> {
    match update {
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

    let (mut event_handler, mut event_receiver) = EventHandler::new(client);

    let handler_task = tokio::spawn(async move {
        if let Err(e) = event_handler.start().await {
            eprintln!("Ошибка в обработчике событий: {}", e);
        }
    });

    while let Some(update) = event_receiver.recv().await {
        handle_update(update).await?;
    }

    handler_task.await.unwrap();

    Ok(())
}
