use std::fs;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BotConfig {
    pub tokens: Tokens
}

#[derive(Deserialize, Debug)]
pub struct Tokens {
    discord: String
}

impl BotConfig {
    pub fn new(filename: &str) -> Result<BotConfig, String> {
        match fs::read_to_string(filename) {
            Err(_e) => Err(String::from("Failed to open config file")),
            Ok(content) => {
                match toml::from_str(content.as_str()) {
                    Err(e) => Err(e.to_string()),
                    Ok(c) => Ok(c)
                }
            }
        }

    }
}
