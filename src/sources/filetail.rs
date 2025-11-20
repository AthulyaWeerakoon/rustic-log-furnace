use std::{
    fs::File, 
    io::{BufRead, BufReader, Seek, SeekFrom}, 
    sync::{Arc, mpsc::Sender}, 
    thread, 
    time::Duration
};
use super::traits::Source; 

pub struct FileTailSource {
    path: String,
    name: String,
}

impl FileTailSource {
    pub fn new(name: Option<&str>, path: &str) -> Self {
        Self { 
            path: path.to_string(),
            name: name.unwrap_or("FileTailSource").to_string(),
        }
    }
}

impl Source for FileTailSource {
    fn run(&self, sender: Sender<Arc<String>>) {
        let log_prefix = &self.name;

        let path = self.path.clone(); 
        
        let mut last_pos: u64 = 0;
        let sleep_duration = Duration::from_secs(2);

        loop {
            match File::open(&path) {
                Ok(mut file) => {
                    let current_size = file.metadata().map(|m| m.len()).unwrap_or(0);
                    if current_size < last_pos {
                        println!("[{}] [WARN] File '{}' appears to have rotated or shrunk, resetting position to 0.", log_prefix, path);
                        last_pos = 0;
                    }

                    if let Err(e) = file.seek(SeekFrom::Start(last_pos)) {
                        eprintln!("[{}] [ERROR] Failed to seek to position {}: {}", log_prefix, last_pos, e);
                        thread::sleep(sleep_duration);
                        continue;
                    }

                    let mut reader = BufReader::new(file);
                    let mut buffer = String::new();
                    
                    loop {
                        match reader.read_line(&mut buffer) {
                            Ok(0) => break,
                            Ok(bytes_read) => {
                                let line = buffer.trim_end().to_string();
                                if sender.send(Arc::new(line)).is_err() {
                                    eprintln!("[{}] [DEBUG] Receiver (Processor) channel closed, exiting thread.", log_prefix);
                                    return;
                                }
                                
                                last_pos += bytes_read as u64;
                                buffer.clear();
                            }
                            Err(e) => {
                                eprintln!("[{}] [ERROR] Error reading line: {}", log_prefix, e);
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[{}] [ERROR] Could not open file '{}': {}. Retrying in 2s...", log_prefix, path, e);
                }
            }
            
            thread::sleep(sleep_duration);
        }
    }
}