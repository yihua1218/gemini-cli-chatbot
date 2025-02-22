// src/config.rs
use std::env;

pub struct Config {
    pub api_key: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            api_key: env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set"),
        }
    }
}
