use crate::config::{Config, MatchType, Matcher};
use log::{debug, error};
use serde_yaml::Value;
use std::fs;
use std::path::Path;

pub struct Parser {
    config_path: String,
}

static MATCHER_CONFIG: &'static [(&'static str, MatchType)] = &[
    ("regex", MatchType::Regex),
    ("contains", MatchType::Contains),
];

impl Parser {
    pub fn new(config_path: &str) -> Self {
        Parser {
            config_path: config_path.to_owned(),
        }
    }

    pub fn parse(&self) -> Result<Config, String> {
        if !Path::new(&self.config_path).exists() {
            return Err(format!("File {} doesn't exists", self.config_path));
        }

        debug!("Parsing configuration...");
        let value: Value = self.read_and_parse_yaml(&self.config_path);
        debug!("Config: {:?}", value);

        let mut config: Config = Config { matchers: vec![] };

        let matchers = value.get("matchers").unwrap_or(&Value::Null);

        if matchers.is_null() {
            return Err(format!("Matchers does not exist in the configuration file !"));
        }

        for matcher in matchers.as_mapping().iter() {
            for (key, value) in matcher.iter() {
                let name = key.as_str().unwrap().to_owned();

                debug!("Parsing matcher {}", name);

                let messages = match self.get_messages(value, &name) {
                    Ok(messages) => messages,
                    Err(why) => {
                        error!("{}", why);
                        continue;
                    }
                };

                for (config_name, match_type) in MATCHER_CONFIG {
                    let val: &Value = value.get(config_name).unwrap_or(&Value::Null);
                    if !val.is_null() {
                        let matcher = Matcher {
                            name: name,
                            messages: messages,
                            match_type: match_type.clone(),
                            patterns: self.get_patterns(val),
                        };
                        config.matchers.push(matcher);
                        break;
                    }
                }
            }
        }

        debug!("Parsing configuration done !");
        return Ok(config);
    }

    fn read_and_parse_yaml(&self, file_path: &str) -> Value {
        let file_str = fs::read_to_string(file_path).unwrap();
        let value: Value= serde_yaml::from_str(&file_str).expect("Failed to parse yaml file");
        return value;
    }

    fn get_messages(&self, value: &Value, name: &str) -> Result<Vec<String>, String> {
        let messages = value.get("messages").unwrap_or(&Value::Null);
        if messages.is_null() {
            return Err(format!("Matcher {} doesn't provide messages or messages file", name));
        }

        let val: Value;

        if messages.is_string() {
            let messages_path = messages.as_str().unwrap();

            if !Path::new(messages_path).exists() {
                return Err(format!("Messages file for matcher {} doesn't exist", name));
            }
            val = self.read_and_parse_yaml(messages_path);
        }
        else if messages.is_sequence() {
            val = messages.clone();
        }
        else {
            return Err(format!("Invalid messages type for matcher {}", name));
        }

        let mut messages: Vec<String> = vec![];

        if val.is_sequence() {
            for message in val.as_sequence().unwrap() {
                messages.push(message.as_str().unwrap().to_owned());
            }
        }

        if messages.len() == 0 {
            return Err(format!("No messages for matcher {}", name));
        }

        return Ok(messages);
    }

    fn get_patterns(&self, value: &Value) -> Vec<String> {
        let mut patterns: Vec<String> = vec![];

        if value.is_sequence() {
            for val in value.as_sequence().unwrap() {
                patterns.push(val.as_str().unwrap().to_owned());
            }
        }
        else if value.is_string(){
            patterns.push(value.as_str().unwrap().to_owned());
        }
        else if value.is_f64(){
            patterns.push(value.as_f64().unwrap().to_string());
        }
        else if value.is_i64(){
            patterns.push(value.as_i64().unwrap().to_string());
        }
        else if value.is_u64(){
            patterns.push(value.as_u64().unwrap().to_string());
        }

        return patterns;
    }
}
