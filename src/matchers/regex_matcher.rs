use crate::config::Matcher;
use crate::matchers::TalkMatcher;
use rand::seq::SliceRandom;
use regex::Regex;
use log::error;

pub struct RegexMatcher {
    matcher: Matcher,
    regexs: Vec<Regex>,
}

impl TalkMatcher for RegexMatcher {
    fn test(&self, msg: &str) -> bool {
        for regex in &self.regexs {
            if regex.is_match(msg) {
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

impl RegexMatcher {
    pub fn new(matcher: Matcher) -> Self {

        let mut regexs: Vec<Regex> = vec![];

        for pattern in &matcher.patterns {
            let regex = Regex::new(&pattern);
            match regex {
                Ok(re) => regexs.push(re),
                Err(why) => error!("Invalid regex for matcher {} : {}", matcher.name, why)
            }
        }

        RegexMatcher {
            matcher: matcher,
            regexs: regexs
        }
    }
}
