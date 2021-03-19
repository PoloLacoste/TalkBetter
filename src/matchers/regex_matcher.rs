use crate::matchers::TalkMatcher;
use regex::Regex;
use rand::seq::SliceRandom;

pub struct RegexMatcher {
    messages: Vec<String>,
    regex: Regex
}

impl TalkMatcher for RegexMatcher {
    fn test(&self, msg: &str) -> bool {
        return self.regex.is_match(msg);
    }

    fn get_msg(&self) -> &str {
        return &self.messages.choose(&mut rand::thread_rng()).unwrap();
    }
}

impl RegexMatcher {
    pub fn new(pattern: &str, messages: Vec<String>) -> Self {
        RegexMatcher { 
            regex: Regex::new(pattern).unwrap(),
            messages: messages
        }
    }
}