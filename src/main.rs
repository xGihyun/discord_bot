// Ignore unused stuff for now
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod commands;

use dotenv::dotenv;
use reqwest::Error as ReqwestError;
use serde::Deserialize;
use serde::Serialize;
use serenity::{
    async_trait,
    client::Context,
    framework::standard::macros::{command, group},
    framework::standard::{Args, CommandResult, StandardFramework},
    json::json,
    model::channel::Message,
    prelude::*,
};
use std::env;

use crate::commands::chemistry::elements::*;

struct Handler;

#[group]
#[commands(ping, print, econfig, element)]
struct General;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("/")) // set the bot's prefix to "/"
        .group(&GENERAL_GROUP);

    let token = env::var("DISCORD_TOKEN").expect("Expected a DISCORD_TOKEN environment variable");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client.");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn print(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let text = args.rest();
    let formatted = format!("```{}```", text);

    msg.reply(ctx, formatted).await?;

    Ok(())
}
