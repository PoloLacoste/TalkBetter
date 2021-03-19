use crate::config::Matcher;
use crate::matchers::TalkMatcher;
use rand::seq::SliceRandom;
use regex::Regex;

pub struct RegexMatcher {
    matcher: Matcher,
    regex: Regex,
}

impl TalkMatcher for RegexMatcher {
    fn test(&self, msg: &str) -> bool {
        return self.regex.is_match(msg);
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

impl RegexMatcher {
    pub fn new(matcher: Matcher) -> Self {
        RegexMatcher {
            regex: Regex::new(&matcher.pattern).unwrap(),
            matcher: matcher,
        }
    }
}
