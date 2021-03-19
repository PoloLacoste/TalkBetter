use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, user::CurrentUser},
    prelude::{Context, EventHandler},
    utils::MessageBuilder,
};

use log::{debug, error, info};

use crate::config::{Config, MatchType};
use crate::matchers::{RegexMatcher, TalkMatcher};

pub struct Handler {
    matchers: Vec<Box<dyn TalkMatcher>>,
}

static mut BOT: Option<CurrentUser> = None;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        // ignore messages from bot
        if msg.author.id.0 == self.get_bot().id.0 {
            return;
        }

        let mut has_match = false;
        let mut message = "Empty message";

        debug!("Analyzing message '{}' from {}", msg.content, msg.author.name);

        for matcher in self.matchers.iter() {
            if matcher.test(&msg.content) {
                debug!("Found matcher {}", matcher.get_name());
                has_match = true;
                message = matcher.get_msg();
                break;
            }
        }

        if has_match {
            debug!("Sending response '{}' to {}", message, msg.author.name);
            let response = MessageBuilder::new().push(message).build();
            if let Err(why) = msg.reply(&context.http, &response).await {
                error!("Error sending message: {:?}", why);
            }
        }
        else {
            debug!("Could't find a matcher on message '{}' from {}", msg.content, msg.author.name)
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
        unsafe {
            BOT = Some(ready.user);
        }
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
                }
                MatchType::Contains => {
                    info!(
                        "Added contains matcher {} => {}",
                        matcher.name, matcher.pattern
                    );
                }
            }
        }

        Handler { matchers: matchers }
    }

    fn get_bot(&self) -> &'static mut CurrentUser {
        unsafe {
            match BOT {
                Some(ref mut x) => x,
                None => panic!(),
            }
        }
    }
}
