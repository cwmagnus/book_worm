use std::env;
use serenity::model::gateway::GatewayIntents;
use serenity::prelude::*;
use crate::handler::Handler;

mod book;
mod handler;

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_BOT_TOKEN").expect("Expected a token in the environment");

    let intents =
        GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}