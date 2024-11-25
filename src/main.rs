mod config;
mod error;
use clap::Parser;
use config::SLACK_BOT_TOKEN;
use dotenvy::dotenv;
use reqwest::Client;
use serde::Serialize;
use std::env;
use std::process;

use config::{Args, Commands, RUST_LOG, SLACK_API_URL};
// Re-exports
pub use error::{Error, Result};

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Set the RUST_LOG, if it hasn't been explicitly defined
    if env::var(RUST_LOG).is_err() {
        env::set_var(RUST_LOG, "brix-bot=info")
    }

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let args = Args::parse();

    if let Err(e) = run_command(args).await {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

async fn run_command(args: Args) -> Result<()> {
    match args.command {
        Commands::ListChannels => list_channels().await,
        Commands::PostMessage => post_message().await,
    }
}

#[derive(Debug, Serialize)]
pub struct ChannelMessage {
    pub channel: String,
    pub text: String,
}

async fn list_channels() -> Result<()> {
    Ok(())
}

async fn post_message() -> Result<()> {
    let base_url = env::var(SLACK_API_URL).expect("SLACK_API_URL is not set.");
    let token = env::var(SLACK_BOT_TOKEN).expect("SLACK_BOT_TOKEN is not set.");

    let url = format!("{}/chat.postMessage", base_url);

    println!("Posting message to Slack... {}", url);

    let data = ChannelMessage {
        channel: "foo".to_string(),
        text: "hello world".to_string(),
    };

    let result = Client::new()
        .post(url)
        .bearer_auth(token)
        .json(&data)
        .send()
        .await;

    let Ok(response) = result else {
        return Err("Unable to post message. Try again later.".into());
    };

    match response.text().await {
        Ok(text) => {
            println!("Some response:");
            println!("{}", text);
        }
        Err(e) => {
            let msg = format!("Error: {}", e);
            return Err(msg.into());
        }
    }

    Ok(())
}
