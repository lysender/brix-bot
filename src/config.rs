use clap::{Parser, Subcommand};

pub const RUST_LOG: &str = "RUST_LOG";
pub const SLACK_BOT_TOKEN: &str = "SLACK_BOT_TOKEN";
pub const SLACK_API_URL: &str = "SLACK_API_URL";

/// brix-bot A slack bot
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List Slack channels
    ListChannels,

    /// Post a message to a Slack channel
    PostMessage,
}
