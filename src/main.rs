use log::{error, info};
use serenity::prelude::Client;
use std::env;

mod config;
mod handler;
mod matchers;

use config::Parser;
use handler::Handler;

fn get_default_env_string(name: &'static str, default: &'static str) -> String {
    return env::var(name).unwrap_or(default.to_owned());
}

#[tokio::main]
async fn main() {
    env_logger::init();

    info!("Starting...");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let config_path = get_default_env_string("CONFIG_PATH", "config/config.yaml");

    let parser = Parser::new(&config_path);
    let config = match parser.parse() {
        Ok(config) => config,
        Err(why) => {
            error!("{}", why);
            return;
        }
    };

    let mut client = Client::builder(&token)
        .event_handler(Handler::new(config))
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
