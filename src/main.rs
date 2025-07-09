use std::env;

use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::commands::mobs::{mob_drops, mob_info};

mod api;
mod commands;
mod embeds;
mod response_structs;

#[derive(Deserialize, Serialize)]
struct Gateway {}

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Login with a bot token from the environment
    let token = env::var("DISCORD_KEY").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!mobDrops") {
            let (_command, mob_name) = msg.content.split_at(9);
            mob_drops(&ctx, &msg, mob_name.trim()).await;
        }
        if msg.content.starts_with("!mob") {
            let (_command, mob_name) = msg.content.split_at(4);
            mob_info(&ctx, &msg, mob_name.trim()).await;
        }
    }
}
