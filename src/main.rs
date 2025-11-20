// src/main.rs
mod pipeline;
mod parsers;
mod sources;
mod sinks;
mod pattern;

use crate::pipeline::Processor;
use crate::pattern::Pattern;
use crate::sources::FileTailSource;
use crate::parsers::{DropFieldsParser,RegexSplitParser};
use crate::sinks::FileAppendSink;


fn main() {
    let log_line_regex = r"^.+:[ ]*.*$"; 

    // Build the Pattern
    let mut pattern = Pattern::new(None, log_line_regex)
        .expect("Failed to compile regex");

    // Add two parsers to the pattern
    pattern.add_parser(Box::new(RegexSplitParser::new(
        r"^(?P<date>\w{3} \d{1,2}) (?P<time>\d{2}:\d{2}:\d{2}) (?P<host>[\w\-]+) (?P<process>[^ ]+): (?P<msg>.*)$",
        None
    )));
    pattern.add_parser(Box::new(DropFieldsParser::new(vec!["host"])));

    // Add one sink to the pattern
    pattern.add_sink(Box::new(FileAppendSink::new(None, "/tmp/auth_processed.log")));

    // Build the Processor (which manages sources and patterns)
    let mut processor = Processor::new(None);

    // Add one source to the processor
    processor.add_source(Box::new(FileTailSource::new(None, "/var/log/auth.log")));

    // Add the fully configured pattern to the processor
    processor.add_pattern(Box::new(pattern)); 

    // Start processing
    processor.run();
}
