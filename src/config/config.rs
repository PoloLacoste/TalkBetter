#[derive(Clone)]
pub enum MatchType {
    Null,
    Regex,
    Contains,
}

#[derive(Clone)]
pub struct Matcher {
    pub name: String,
    pub messages: Vec<String>,
    pub match_type: MatchType,
    pub pattern: String,
}

pub struct Config {
    pub matchers: Vec<Matcher>,
}
