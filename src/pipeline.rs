use std::sync::{Arc, mpsc};
use std::thread;

use crate::pattern::Pattern;
use crate::sources::traits::Source;

type SourceMessage = Arc<String>; 

pub struct Processor {
    name: String,
    sources: Vec<Box<dyn Source + Send>>,
    patterns: Vec<Box<Pattern>>,
}

impl Processor {
    pub fn new(name: Option<&str>) -> Self {
        Self {
            name: name.unwrap_or("Processor").to_string(),
            sources: vec![],
            patterns: vec![],
        }
    }

    pub fn add_source(&mut self, src: Box<dyn Source + Send>) {
        self.sources.push(src);
    }

    pub fn add_pattern(&mut self, p: Box<Pattern>) {
        self.patterns.push(p);
    }

    pub fn run(self) {
        let log_prefix = &self.name;

        // Set up master channel for sources
        let (tx_master, rx_master) = mpsc::channel::<SourceMessage>();
        
        // Channel senders for fan-out
        let mut pattern_senders: Vec<mpsc::Sender<SourceMessage>> = Vec::new();
        let mut pattern_handles = vec![];

        // Launch all patterns
        println!("[{}] [DEBUG] Launching {} patterns...", log_prefix, self.patterns.len());
        for p_box in self.patterns {
            let pattern = *p_box;
            
            // Create a dedicated channel for this specific pattern
            let (tx_pattern, rx_pattern) = mpsc::channel::<SourceMessage>();
            
            // Store the sender for the fan-out loop
            pattern_senders.push(tx_pattern);

            // Launch the pattern thread, moving ownership of the pattern object and its receiver
            let handle = thread::spawn(move || {
                pattern.run(rx_pattern);
            });
            pattern_handles.push(handle);
        }

        // Launch all sources
        println!("[{}] [DEBUG] Launching {} sources...", log_prefix, self.sources.len());
        for source in self.sources {
            let tx_clone = tx_master.clone();
            thread::spawn(move || {
                source.run(tx_clone);
            });
        }
        
        drop(tx_master);

        println!("[{}] [DEBUG] Processor main thread listening for input...", log_prefix);
        for buffer_arc in rx_master {
            // Fan out the data to all patterns
            for sender in &pattern_senders {
                let _ = sender.send(buffer_arc.clone());
            }
        }
        
        drop(pattern_senders);
        
        for handle in pattern_handles {
            let _ = handle.join();
        }
    }
}