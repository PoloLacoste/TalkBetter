use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::{Context, EventHandler},
    utils::MessageBuilder,
};

use log::{error, info, warn, debug};

use crate::config::{Config, MatchType};
use crate::matchers::{TalkMatcher, RegexMatcher};

pub struct Handler {
    matchers: Vec<Box<dyn TalkMatcher>>
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {

        let mut has_match = false;
        let mut message = "Empty message";

        debug!("Analyzing message {}", &msg.content);

        for matcher in self.matchers.iter() {
            if matcher.test(&msg.content) {
                debug!("Found matcher {}", matcher.get_name());
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
                error!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

impl Handler {
    pub fn new(config: Config) -> Self {
        let mut matchers: Vec<Box<dyn TalkMatcher>> = vec![];

        for matcher in config.matchers {

            match matcher.match_type {
                MatchType::Regex => {
                    let pattern = &matcher.pattern;
                    matchers.push(Box::new(RegexMatcher::new(matcher.clone())));
                    info!("Added regex matcher {} => {}", matcher.name, pattern);
                },
                MatchType::Contains => {
                    info!("Added contains matcher {} => {}", matcher.name, matcher.pattern);
                }
                MatchType::Null => warn!("Invalid matcher {}", matcher.name)
            }
        }

        Handler { matchers: matchers }
    }
}