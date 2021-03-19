use crate::config::{Matcher, Config, MatchType};
use std::path::Path;
use std::fs;
use std::process;
use log::{error, debug};
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
        
        let config_str = fs::read_to_string(&self.config_path).unwrap();
        
        let mut value: Value = serde_yaml::from_str(&config_str).expect("Failed to parse yaml file");
        debug!("Config: {:?}", value);

        let mut config: Config = Config {
            matchers: vec![]
        };

        let matchers = value.get_mut("matchers").unwrap();

        for matcher in matchers.as_mapping().iter() {
            for (key, value) in matcher.iter() {

                let mut match_type: MatchType = MatchType::Null;
                let mut pattern: Option<String> = Option::None;

                let regex: &Value  = value.get("regex").unwrap_or(&Value::Null);
                if !regex.is_null() {
                    match_type = MatchType::Regex;
                    pattern = Some(regex.as_str().unwrap().to_owned());
                }

                let contains: &Value  = value.get("contains").unwrap_or(&Value::Null);
                if !contains.is_null() {
                    match_type = MatchType::Contains;
                    pattern = Some(contains.as_str().unwrap().to_owned());
                }

                let matcher = Matcher {
                    name: key.as_str().unwrap().to_owned(),
                    match_type: match_type,
                    pattern: pattern
                    
                };
                config.matchers.push(matcher);
            }
        }

        debug!("Parsing configuration done !");
        return config;
    }
}