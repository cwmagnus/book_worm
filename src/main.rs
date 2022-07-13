use serenity::model::gateway::GatewayIntents;
use serenity::prelude::*;
use crate::handler::Handler;

mod book;
mod handler;

const BOT_TOKEN: &str = "OTk2NTEwMDkyNTA3NzU4NjYy.GRh2Mu.V-QDQMa41I7doOCVNTA-0ELivDSUtUEthC2aSM";

#[tokio::main]
async fn main() {
    let intents =
        GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(BOT_TOKEN, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}