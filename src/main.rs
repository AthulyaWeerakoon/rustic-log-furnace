// src/main.rs
mod builder;
mod pipeline;
mod parsers;
mod sources;
mod sinks;
mod pattern;
mod config;

use crate::config::Config;
use crate::builder::build_processors;


fn main() {
    let config = Config::load_from_file("../config.toml")
        .expect("Failed to load config");

    let processors = build_processors(config)
        .expect("Failed to build processors");

    for processor in processors {
        processor.run();
    }
}
