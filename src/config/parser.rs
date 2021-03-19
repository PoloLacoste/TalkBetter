use crate::config::{Matcher, Config, MatchType};
use std::path::Path;
use std::fs;
use std::process;
use log::{error, debug, warn};
use serde_yaml::Value;

pub struct Parser {
    config_path: String
}

impl Parser {
    pub fn new(config_path: &str) -> Self {
        Parser { config_path: config_path.to_owned() }
    }

    pub fn parse(&self) -> Config{
        if !Path::new(&self.config_path).exists() {
            error!("File {} doesn't exists", self.config_path);
            process::exit(0x1);
        }

        debug!("Parsing configuration...");
        
        let value: Value = self.read_and_parse_yaml(&self.config_path);
        debug!("Config: {:?}", value);

        let mut config: Config = Config {
            matchers: vec![]
        };

        let matchers = value.get("matchers").unwrap_or(&Value::Null);

        if matchers.is_null() {
            error!("Matchers does not exist in the configuration file !");
            process::exit(0x1);
        }

        for matcher in matchers.as_mapping().iter() {
            for (key, value) in matcher.iter() {
                
                let name = key.as_str().unwrap().to_owned();

                debug!("Parsing matcher {}", name);

                let mut match_type: MatchType = MatchType::Null;
                let mut pattern: String = "".to_owned();

                let messages_file = value.get("messages_file").unwrap_or(&Value::Null);
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

                let regex: &Value  = value.get("regex").unwrap_or(&Value::Null);
                if !regex.is_null() {
                    match_type = MatchType::Regex;
                    pattern = regex.as_str().unwrap().to_owned();
                }

                let contains: &Value  = value.get("contains").unwrap_or(&Value::Null);
                if !contains.is_null() {
                    match_type = MatchType::Contains;
                    pattern = contains.as_str().unwrap().to_owned();
                }

                let matcher = Matcher {
                    name: name,
                    messages: messages,
                    match_type: match_type,
                    pattern: pattern
                    
                };
                config.matchers.push(matcher);
            }
        }

        debug!("Parsing configuration done !");
        return config;
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

    fn read_and_parse_yaml(&self, file_path: &str) -> Value {
        let file_str = fs::read_to_string(file_path).unwrap();
        let value: Value = serde_yaml::from_str(&file_str).expect("Failed to parse yaml file");
        return value;
    }
}