use anyhow::{Result, Context};

use crate::config::types::*;
use crate::pipeline::Processor;
use crate::pattern::Pattern;

use crate::sources::{FileTailSource};
use crate::sources::traits::Source;

use crate::parsers::{RegexSplitParser, DropFieldsParser};
use crate::parsers::traits::Parser;

use crate::sinks::{FileAppendSink};
use crate::sinks::traits::Sink;


// Main method responsible for building processors based on configurations
pub fn build_processors(config: Config) -> Result<Vec<Processor>> {
    let mut processors = Vec::new();

    for proc_conf in config.processors {
        processors.push(build_processor(proc_conf)?);
    }

    Ok(processors)
}


// Building processors 
fn build_processor(conf: ProcessorConfig) -> Result<Processor> {
    let mut processor = Processor::new(conf.name.as_deref());

    // Sources
    for src_conf in conf.sources {
        processor.add_source(build_source(src_conf)?);
    }

    // Patterns
    for pat_conf in conf.patterns {
        processor.add_pattern(Box::new(build_pattern(pat_conf)?));
    }

    Ok(processor)
}


// Building sources
fn build_source(conf: SourceConfig) -> Result<Box<dyn Source + Send>> {
    match conf {
        SourceConfig::File { name, path } => {
            Ok(Box::new(FileTailSource::new(name.as_deref(), &path)))
        }

        SourceConfig::TCP { name: _, host: _, port: _ } => {
            anyhow::bail!("TCP source not implemented yet");
        }
    }
}


// Building patterns
fn build_pattern(conf: PatternConfig) -> Result<Pattern> {
    let mut pattern = Pattern::new(
        conf.name.as_deref(),
        &conf.regex
    ).context("Invalid pattern regex")?;

    // Order parsers
    let mut parsers = conf.parsers;
    parsers.sort_by_key(|p| get_parser_order(p));

    for p in parsers {
        pattern.add_parser(build_parser(p)?);
    }

    // Add fan-out to sinks
    for s in conf.sinks {
        pattern.add_sink(build_sink(s)?);
    }

    Ok(pattern)
}


// Building all parsers within patterns
fn build_parser(conf: ParserConfig) -> Result<Box<dyn Parser + Send>> {
    match conf {

        ParserConfig::RegexSplit { regex, field_in, .. } => {
            Ok(Box::new(
                RegexSplitParser::new(&regex, Some(&field_in))
                    .context("Failed to build RegexSplitParser")?
            ))
        }

        ParserConfig::DropFields { drop, .. } => {
            Ok(Box::new(
                DropFieldsParser::new(drop.iter().map(|s| s.as_str()).collect())
            ))
        }

        // ---- FUTURE EXTENSIONS ----
        _ => {
            anyhow::bail!("Parser type not implemented yet");
        }
    }
}


// Building all sinks within patterns
fn build_sink(conf: SinkConfig) -> Result<Box<dyn Sink + Send>> {
    match conf {

        SinkConfig::JsonFile { name, path, operation } => {
            match operation.as_str() {
                "append" => Ok(Box::new(
                    FileAppendSink::new(name.as_deref(), &path)
                )),

                "overwrite_with_last" => {
                    anyhow::bail!("overwrite_with_last not implemented yet");
                }

                _ => anyhow::bail!("Invalid sink operation"),
            }
        }
    }
}


// Util method to extract parser order
fn get_parser_order(p: &ParserConfig) -> u32 {
    match p {
        ParserConfig::RegexSplit { order, .. }
        | ParserConfig::TimeSimple { order, .. }
        | ParserConfig::Uppercase { order, .. }
        | ParserConfig::CountPattern { order, .. }
        | ParserConfig::GlobalCount { order, .. }
        | ParserConfig::GlobalMax { order, .. }
        | ParserConfig::GlobalLatest { order, .. }
        | ParserConfig::DropFields { order, .. }
        | ParserConfig::FormatAppend { order, .. } => *order,
    }
}
