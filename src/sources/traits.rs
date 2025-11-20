use std::sync::{Arc, mpsc::Sender};

pub trait Source {
    fn run(&self, tx: Sender<Arc<String>>); 
}
