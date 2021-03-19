pub trait TalkMatcher: Send + Sync {
    fn test(&self, msg: &str) -> bool;

    fn get_msg(&self) -> &str;

    fn get_name(&self) -> &str;
}
