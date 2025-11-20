use std::collections::HashMap;

pub trait Parser {
    fn parse(&self, input: HashMap<String, String>) -> HashMap<String, String>;
}
