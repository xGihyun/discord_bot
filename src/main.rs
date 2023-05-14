// Ignore unused stuff for now
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use reqwest::Error as ReqwestError;
use dotenv::dotenv;
use std::env;
use serde::Serialize;
use serde::Deserialize;
use serenity::{
    prelude::*,
    async_trait,
    client::Context,
    framework::standard::macros::{command, group},
    framework::standard::{CommandResult, StandardFramework, Args},
    model::channel::Message,
    json::json,
};

// #[derive(Debug, Deserialize)]
// struct ChatCompletion {
   
//     choices: Vec<Choice>,
// }

// #[derive(Debug, Deserialize)]
// struct Usage {
//     prompt_tokens: i32,
//     completion_tokens: i32,
//     total_tokens: i32
// }

// #[derive(Debug, Deserialize, Serialize)]
// struct Choice {
//     message: Message,
//     finish_reason: String,
//     index: i32
// }

// #[derive(Debug, Serialize)]
// struct OpenAIRequest {
//     model: String,
//     messages: Vec<OpenAIMessage>,
// }

// #[derive(Debug, Deserialize, Serialize)]
// struct OpenAIMessage {
//     role: String,
//     content: String
// }

#[group]
#[commands(ping, hi, print, econfig)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    dotenv().ok();

    env::set_var("RUST_BACKTRACE", "full");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("/")) // set the bot's prefix to "/"
        .group(&GENERAL_GROUP);

    let token = env::var("DISCORD_TOKEN").expect("Expected a DISCORD_TOKEN environment variable");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

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
async fn hi(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Hello!").await?;

    Ok(())
}

#[command]
async fn print(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let text = args.rest();
    msg.reply(ctx, text).await?;

    Ok(())
}

#[command]
async fn econfig(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let sub = [
        "s", "s", "p", "s", "p", "s", "d", "p", "s", "d", "p", "s", "f", "d", "p", "s", "f", "d",
        "p", "f", "d", "f",
    ];
    let principal_quantum_number: [i32; 22] = [1, 2, 2, 3, 3, 4, 3, 4, 5, 4, 5, 6, 4, 5, 6, 7, 5, 6, 7, 6, 7, 7];
    let diff: i32;
    let mut electrons: [i32; 22] = [0; 22];
    let mut current_electron: i32 = 0;
    let mut l: usize = 0;
    let mut answer = String::new();

    let mut args = args;
    let atomic_number: i32 = args.single().unwrap();

    while atomic_number != current_electron {
        match sub[l] {
            "s" => {
                current_electron += 2;
                electrons[l] += 2;
            }
            "p" => {
                current_electron += 6;
                electrons[l] += 6;
            }
            "d" => {
                current_electron += 10;
                electrons[l] += 10;
            }
            _ => {
                current_electron += 14;
                electrons[l] += 14;
            }
        }

        if current_electron > atomic_number {
            diff = current_electron - atomic_number;
            electrons[l] -= diff;
            break;
        }

        l += 1;
    }

    for i in 0..l + 1 {
        let electrons_string = electrons[i].to_string();
        let principal_quantum_string = principal_quantum_number[i].to_string();

        answer.push_str(&format!("{}{}{} ", principal_quantum_string, sub[i], electrons_string));
    }

    msg.reply(ctx, answer).await?;

    Ok(())
}

// Doesn't work :(
// #[command]
// async fn summarize(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
//     let rapid_api_key = env::var("RAPID_API_KEY").expect("Missing Rapid API key");
//     let text = args.rest().to_string();

//     // // Use GPT 3.5 to summarize
//     let summary_preamble = "Summarize the following: ".to_string();

//     println!("Summarizing...");

//     let ai_summary = chatgpt(&text.to_string(), &summary_preamble, &rapid_api_key).await?;
//     let ai_summary_text = &ai_summary.choices[0].message.content;

//     println!("{:?}", ai_summary_text);

//     msg.reply(ctx, ai_summary_text).await?;

//     Ok(())
// }

// async fn chatgpt(text: &String, _preamble: &String, api_key: &String) -> Result<ChatCompletion, ReqwestError> {
//     let client = reqwest::Client::new();

//     let mut open_ai_headers = reqwest::header::HeaderMap::new();
//     open_ai_headers.insert("Content-Type", "application/json".parse().unwrap());
//     open_ai_headers.insert("X-RapidAPI-Key", api_key.parse().unwrap());
//     open_ai_headers.insert("X-RapidAPI-Host", "openai80.p.rapidapi.com".parse().unwrap());

//     let open_ai_req_opts = json!({
//         "model": "gpt-3.5-turbo",
//         "messages": [
//             {
//                 "role": "user",
//                 "content": text
//             }
//         ]
//     });

//     let open_ai_summary_res = client
//         .post("https://openai80.p.rapidapi.com/chat/completions")
//         .headers(open_ai_headers)
//         .json(&open_ai_req_opts)
//         .send()
//         .await?
//         .text()
//         .await?;

//     let chat_completion: ChatCompletion = serde_json::from_str(&open_ai_summary_res).unwrap();

//     Ok(chat_completion)
// }