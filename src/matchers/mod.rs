mod talk_matcher;
mod regex_matcher;
mod contains_matcher;

pub use self::{
    talk_matcher::TalkMatcher,
    regex_matcher::RegexMatcher,
    contains_matcher::ContainsMatcher
};