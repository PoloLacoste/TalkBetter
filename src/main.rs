use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    utils::MessageBuilder,
};

mod matchers;
use matchers::{
    TalkMatcher,
    RegexMatcher
};

use lazy_static::lazy_static;

lazy_static! {
    static ref MATCHERS: Vec<Box<dyn TalkMatcher>> = vec![
        Box::new(RegexMatcher::new("^[^a-z]*$"))
    ];
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {

        let mut has_match = false;
        let mut message = "Empty message";

        for matcher in MATCHERS.iter() {
            if matcher.test(&msg.content) {
                has_match = true;
                message = matcher.get_msg();
                break;
            }
        }

        if has_match {
            let response = MessageBuilder::new()
                .push(message)
                .build();
            
            if let Err(why) = msg.reply(&context.http, &response).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}