use crate::config::{Config, MatchType, Matcher};
use log::{debug, error, warn};
use serde_yaml::Value;
use std::fs;
use std::path::Path;
use std::process;

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

    pub fn parse(&self) -> Config {
        if !Path::new(&self.config_path).exists() {
            error!("File {} doesn't exists", self.config_path);
            process::exit(0x1);
        }

        debug!("Parsing configuration...");
        let value: Value = self.read_and_parse_yaml(&self.config_path);
        debug!("Config: {:?}", value);

        let mut config: Config = Config { matchers: vec![] };

        let matchers = value.get("matchers").unwrap_or(&Value::Null);

        if matchers.is_null() {
            error!("Matchers does not exist in the configuration file !");
            process::exit(0x1);
        }

        for matcher in matchers.as_mapping().iter() {
            for (key, value) in matcher.iter() {
                let name = key.as_str().unwrap().to_owned();

                debug!("Parsing matcher {}", name);

                let messages_file = value.get("messages").unwrap_or(&Value::Null);
                if messages_file.is_null() {
                    warn!("Matcher {} doesn't provide a file path for responses", name);
                    break;
                }

                let messages_path = messages_file.as_str().unwrap();

                if !Path::new(messages_path).exists() {
                    warn!("Responses file for matcher {} doesn't exist", name);
                    break;
                }

                let messages = self.parse_messages(messages_path);

                if messages.len() == 0 {
                    warn!("No responses for matcher {}", name);
                    break;
                }

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
        return config;
    }

    fn read_and_parse_yaml(&self, file_path: &str) -> Value {
        let file_str = fs::read_to_string(file_path).unwrap();
        let value: Value = serde_yaml::from_str(&file_str).expect("Failed to parse yaml file");
        return value;
    }

    fn parse_messages(&self, messages_path: &str) -> Vec<String> {
        let mut messages: Vec<String> = vec![];

        let value: Value = self.read_and_parse_yaml(messages_path);

        if value.is_sequence() {
            for response in value.as_sequence().unwrap() {
                messages.push(response.as_str().unwrap().to_owned());
            }
        }

        return messages;
    }

    fn get_patterns(&self, value: &Value) -> Vec<String> {
        let mut patterns: Vec<String> = vec![];

        if value.is_sequence() {
            for val in value.as_sequence().unwrap() {
                patterns.push(val.as_str().unwrap().to_owned());
            }
        }
        else {
            patterns.push(value.as_str().unwrap().to_owned());
        }

        return patterns;
    }
}
