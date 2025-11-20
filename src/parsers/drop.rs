use std::collections::HashMap;
use super::traits::Parser;

pub struct DropFieldsParser {
    fields: Vec<String>,
}

impl DropFieldsParser {
    pub fn new(fields: Vec<&str>) -> Self {
        Self { fields: fields.into_iter().map(String::from).collect() }
    }
}

impl Parser for DropFieldsParser {
    fn parse(&self, mut input: HashMap<String, String>) -> HashMap<String, String> {
        for f in &self.fields {
            input.remove(f);
        }
        input
    }
}
