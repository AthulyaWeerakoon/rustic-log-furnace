use std::{collections::HashSet};
use std::path::Path;

use crate::config::types::NamedComponent;

use super::types::{ProcessorConfig, SourceConfig, ParserConfig, SinkConfig};
use validator::ValidationError;

pub(crate) fn validate_absolute_path(path: &str) -> Result<(), ValidationError> {
    let p = Path::new(path);
    if p.is_absolute() { Ok(()) } else { Err(ValidationError::new("path_not_absolute")) }
}

pub(crate) fn validate_unique_processor_names(processors: &[ProcessorConfig]) -> Result<(), ValidationError> {
    let mut names = HashSet::new();
    for p in processors {
        if let Some(ref name) = p.name {
            if !names.insert(name) {
                return Err(ValidationError::new("duplicate_processor_name"));
            }
        }
    }
    Ok(())
}

pub(crate) fn validate_unique_internal_names(processor: &ProcessorConfig) -> Result<(), ValidationError> {
    let mut names = HashSet::new();

    // sources
    for s in &processor.sources {
        if let Some(name) = &s.get_name() {
            if !names.insert(name.clone()) {
                return Err(ValidationError::new("duplicate_name_in_processor"));
            }
        }
    }

    // patterns
    for p in &processor.patterns {
        if let Some(name) = &p.name {
            if !names.insert(name.clone()) {
                return Err(ValidationError::new("duplicate_name_in_processor"));
            }
        }

        // sinks inside patterns
        for s in &p.sinks {
            if let Some(name) = &s.get_name() {
                if !names.insert(name.clone()) {
                    return Err(ValidationError::new("duplicate_name_in_processor"));
                }
            }
        }
    }

    Ok(())
}

pub(crate) fn validate_sources(sources: &[SourceConfig]) -> Result<(), ValidationError> {
    for s in sources {
        match s {
            SourceConfig::File { path, .. } => {
                validate_absolute_path(path)?;
            }

            SourceConfig::TCP { port, .. } => {
                if *port == 0 {
                    return Err(ValidationError::new("invalid_port"));
                }
            }
        }
    }

    Ok(())
}

pub(crate) fn validate_parsers(parsers: &[ParserConfig]) -> Result<(), ValidationError> {
    let mut orders = HashSet::new();

    for p in parsers {
        let order = match p {
            ParserConfig::RegexSplit { order, .. }
            | ParserConfig::TimeSimple { order, .. }
            | ParserConfig::Uppercase { order, .. }
            | ParserConfig::CountPattern { order, .. }
            | ParserConfig::GlobalCount { order, .. }
            | ParserConfig::GlobalMax { order, .. }
            | ParserConfig::GlobalLatest { order, .. }
            | ParserConfig::DropFields { order, .. }
            | ParserConfig::FormatAppend { order, .. } => *order,
        };

        if !orders.insert(order) {
            return Err(ValidationError::new("duplicate_parser_order"));
        }
    }

    Ok(())
}

pub(crate) fn validate_sinks(sinks: &[SinkConfig]) -> Result<(), ValidationError> {
    for s in sinks {
        match s {
            SinkConfig::JsonFile { path, .. } => {
                validate_absolute_path(path)?;
            }
        }
    }

    Ok(())
}
