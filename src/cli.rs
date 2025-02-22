// src/cli.rs
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author = "Your Name", version = "0.1.0", about = "Gemini CLI Chatbot")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(about = "Chat with the Gemini AI model")]
    Chat {
        #[clap(value_parser, help = "The message to send to the Gemini AI model")]
        message: String,
    },
}
