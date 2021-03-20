use crate::config::Matcher;
use crate::matchers::TalkMatcher;
use rand::seq::SliceRandom;

pub struct ContainsMatcher {
    matcher: Matcher,
}

impl TalkMatcher for ContainsMatcher {
    fn test(&self, msg: &str) -> bool {
        for pattern in &self.matcher.patterns {
            if msg.contains(pattern) {
                return true;
            }
        }

        return false;
    }

    fn get_msg(&self) -> &str {
        return &self
            .matcher
            .messages
            .choose(&mut rand::thread_rng())
            .unwrap();
    }

    fn get_name(&self) -> &str {
        return &self.matcher.name;
    }
}

impl ContainsMatcher {
    pub fn new(matcher: Matcher) -> Self {
        ContainsMatcher { matcher: matcher }
    }
}
