use serde::Deserialize;
use std::{fmt::Debug, fs};

#[derive(Deserialize, Debug)]
pub struct Config {
    twitch: Option<TwitchConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Config { twitch: None }
    }
}

#[derive(Deserialize, Debug)]
/// Table of config options for a Twitch client
struct TwitchConfig {
    /// Client id used to get access tokens & to set the Client-Id header in API requests
    id: String,
    /// Path to file containing a client secret for use in auth
    secret_path: String,
}

pub fn load_config_file(config_path: &str) -> Config {
    let config = match fs::read_to_string(&config_path) {
        Ok(config_string) => {
            println!("Reading config from file {}", config_path);
            toml::from_str(&config_string).unwrap_or_default() // [debt] Does not alert user when falling back to default
        },
        Err(_) => {
            println!("[Error] reading config from file {}, using default config.", &config_path);
            Config::default()
        },
    };
    config
}
