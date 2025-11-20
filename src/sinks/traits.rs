use std::collections::HashMap;
use std::sync::{Arc, mpsc::Receiver};

pub trait Sink {
    fn run(&self, channel_from_pattern: Receiver<Arc<HashMap<String, String>>>);
}
