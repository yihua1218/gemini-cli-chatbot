// src/main.rs
use clap::Parser;

mod cli;
mod ai;
mod config;
mod error;
mod utils;

use cli::Cli;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    dotenv::dotenv().ok();

    let config = config::Config::new();

    let cli = Cli::parse();

    match cli.command {
        None => {
            println!("Gemini CLI Chatbot");
            println!("Version: 0.1.0");
            println!("A command-line interface chatbot powered by Google AI's Gemini 2.0 Pro Experimental 02-05 model.");
            println!("Usage: gemini-cli-chatbot --help");
        }
        Some(cli::Commands::Chat { message }) => {
            let response = ai::generate_response(&config, message).await?;
            println!("{}", response);
        }
    }

    Ok(())
}
