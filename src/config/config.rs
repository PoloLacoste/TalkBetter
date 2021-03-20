#[derive(Clone)]
pub enum MatchType {
    Regex,
    Contains,
}

#[derive(Clone)]
pub struct Matcher {
    pub name: String,
    pub messages: Vec<String>,
    pub match_type: MatchType,
    pub patterns: Vec<String>,
}

pub struct Config {
    pub matchers: Vec<Matcher>,
}
