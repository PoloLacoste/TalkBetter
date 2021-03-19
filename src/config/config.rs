pub enum MatchType {
    Null,
    Regex,
    Contains
}

pub struct Matcher {
    pub name: String,
    pub match_type: MatchType,
    pub pattern: Option<String>,
}

pub struct Config {
    pub matchers: Vec<Matcher>
}