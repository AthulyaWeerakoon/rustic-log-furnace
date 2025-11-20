use regex::Regex;
use std::collections::HashMap;
use super::traits::Parser;

pub struct RegexSplitParser {
    regex: Regex,
    field_to_split: String,
}

impl RegexSplitParser {
    pub fn new(pattern: &str, field_name: Option<&str>) -> Self {
        Self { 
            regex: Regex::new(pattern).unwrap(),
            field_to_split: field_name.unwrap_or("line").to_string(),
        }
    }
}

impl Parser for RegexSplitParser {
    fn parse(&self, input: HashMap<String, String>) -> HashMap<String, String> {
        let mut output = input.clone();
        
        if let Some(line) = input.get(&self.field_to_split) {
            if let Some(caps) = self.regex.captures(line) {
                for name in self.regex.capture_names().flatten() {
                    if let Some(val) = caps.name(name) {
                        output.insert(name.to_string(), val.as_str().to_string());
                    }
                }
            }
        }
        output
    }
}