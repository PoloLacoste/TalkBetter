use lazy_static::lazy_static;
use crate::matchers::TalkMatcher;
use regex::Regex;
use rand::seq::SliceRandom;

pub struct RegexMatcher {
    regex: Regex
}


lazy_static! {
    static ref MESSAGES: Vec<&'static str> = vec![
        "Test response"
    ];
}

impl TalkMatcher for RegexMatcher {
    fn test(&self, msg: &str) -> bool {
        return self.regex.is_match(msg);
    }

    fn get_msg(&self) -> &'static str {
        return MESSAGES.choose(&mut rand::thread_rng()).unwrap();
    }
}

impl RegexMatcher {
    pub fn new(pattern: &str) -> Self {
        RegexMatcher { regex: Regex::new(pattern).unwrap() }
    }
}