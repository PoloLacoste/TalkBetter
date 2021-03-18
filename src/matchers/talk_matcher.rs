pub trait TalkMatcher: Send + Sync {
    fn test(&self, msg: &str) -> bool;

    fn get_msg(&self) -> &'static str;
}