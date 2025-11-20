use std::collections::HashMap;
use std::sync::{Arc, mpsc::{channel, Sender, Receiver}};
use std::thread;

use regex::Regex;
use crate::parsers::traits::Parser;
use crate::sinks::traits::Sink;

type SharedRecord = Arc<HashMap<String, String>>;

pub struct Pattern {
    name: String,
    compiled_regex: Regex,
    parsers: Vec<Box<dyn Parser + Send>>,
    sinks: Vec<Box<dyn Sink + Send>>
}

impl Pattern {
    pub fn new(name: Option<&str>, regex: &str) -> Result<Self, regex::Error> {
        let compiled_regex = Regex::new(regex)?;

        Ok(Self {
            name: name.unwrap_or("Pattern").to_string(),
            compiled_regex,
            parsers: vec![],
            sinks: vec![],
        })
    }

    pub fn add_parser(&mut self, p: Box<dyn Parser + Send>) {
        self.parsers.push(p);
    }

    pub fn add_sink(&mut self, s: Box<dyn Sink + Send>) {
        self.sinks.push(s);
    }

    pub fn run(self, receiver_from_sources: Receiver<Arc<String>>) {
        let log_prefix = &self.name;
        let mut sink_senders: Vec<Sender<SharedRecord>> = Vec::new();
        let mut sink_handles = vec![];
        
        // Create channels for each sink and run sinks
        for sink in self.sinks {
            let (tx, rx) = channel::<SharedRecord>();

            sink_senders.push(tx);

            let handle = thread::spawn(move || {
                sink.run(rx);
            });
            sink_handles.push(handle);
        }

        println!("[{}] [DEBUG] Pattern started. Ready to process records.", log_prefix);

        // Process and fan out
        for buffer_arc in receiver_from_sources {

            // Capture matches
            let match_iter = self.compiled_regex.find_iter(buffer_arc.as_ref());

            for m in match_iter {
                // Initialize the record with the full match under the key 'line'.
                let mut record = HashMap::from([
                    ("line".to_string(), m.as_str().to_string())
                ]);
                
                // Apply all parsers in sequence to the newly formed record
                for parser in &self.parsers {
                    record = parser.parse(record);
                }

                // Wrap the processed data in an Arc
                let shared_record = Arc::new(record);

                // Fan out to all sinks
                for sink_sender in &sink_senders {
                    let _ = sink_sender.send(shared_record.clone());
                }
            }
        }
        
        drop(sink_senders);
        
        println!("[{}] [DEBUG] Input channel closed. Waiting for sinks to join.", log_prefix);
        
        for handle in sink_handles {
            let _ = handle.join();
        }

        println!("[{}] [DEBUG] Pattern thread shut down.", log_prefix);
    }
}