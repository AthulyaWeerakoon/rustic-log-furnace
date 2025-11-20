use std::{
    collections::HashMap, 
    fs::OpenOptions, 
    io::Write, 
    sync::{Arc, mpsc::Receiver}
};
use super::traits::Sink;

pub struct FileAppendSink {
    path: String,
    name: String,
}

impl FileAppendSink {
    pub fn new(name: Option<&str>, path: &str) -> Self {
        Self { 
            path: path.to_string(),
            name: name.unwrap_or("FileAppendSink").to_string(),
        }
    }

    fn record_to_json(&self, record: &HashMap<String, String>) -> String {
        let mut json_string = String::from("{");
        let mut first = true;
        
        for (key, value) in record {
            if !first {
                json_string.push(',');
            }
            let escaped_value = value.replace('"', "\\\""); 
            json_string.push_str(&format!(r#""{}":"{}""#, key, escaped_value));
            first = false;
        }
        json_string.push('}');
        json_string
    }
}

impl Sink for FileAppendSink {
    fn run(&self, channel_from_pattern: Receiver<Arc<HashMap<String, String>>>) {
        let log_prefix = &self.name;
        println!("[{}] [DEBUG] Starting sink, targeting file: {}", log_prefix, self.path);

        // Continuously monitor the channel until the sender closes it
        for record_arc in channel_from_pattern {
            let record = record_arc.as_ref();

            // Convert to single-line JSON
            let json_line = self.record_to_json(record);

            // Open and write to file
            match OpenOptions::new().create(true).append(true).open(&self.path) {
                Ok(mut file) => {
                    if let Err(e) = writeln!(file, "{}", json_line) {
                        eprintln!("[{}] [ERROR] Failed to write to file '{}': {}", log_prefix, self.path, e);
                    }
                }
                Err(e) => {
                    eprintln!("[{}] [ERROR] Failed to open file '{}': {}", log_prefix, self.path, e);
                }
            }
        }

        println!("[{}] [DEBUG] Channel closed. Shutting down sink.", log_prefix);
    }
}
