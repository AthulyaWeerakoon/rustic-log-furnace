use super::validators::{validate_unique_processor_names};
use serde::{Serialize, Deserialize};
use validator::{Validate};

// Traits
pub(crate) trait NamedComponent{
    fn get_name(&self) -> &Option<String>;
}

// Configuration
#[derive(Debug, Deserialize, Validate)]
pub struct Config {
    #[validate(custom = "validate_unique_processor_names")]
    #[validate(length(min = 1, message = "At least one processor is required"))]
    pub processors: Vec<ProcessorConfig>,
}

// Processor
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ProcessorConfig {
    pub name: Option<String>,
    
    #[validate(length(min = 1, message = "Processor must have at least one source"))]
    pub sources: Vec<SourceConfig>,
    
    #[validate(length(min = 1, message = "Processor must have at least one pattern"))]
    #[validate]
    pub patterns: Vec<PatternConfig>,
}

// Sources
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "protocol")]
pub enum SourceConfig {
    #[serde(rename = "file")]
    File {
        name: Option<String>,
        path: String,
    },

    #[serde(rename = "tcp")]
    TCP {
        name: Option<String>,
        host: String,
        port: u16,
    },
}

// Patterns
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct PatternConfig {
    pub name: Option<String>,
    pub regex: String,
    #[serde(default)]
    pub parsers: Vec<ParserConfig>,
    #[serde(default)]
    pub sinks: Vec<SinkConfig>,
}

// Parsers
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ParserConfig {

    #[serde(rename = "regex_split")]
    RegexSplit {
        order: u32,
        field_in: String,
        // NOTE: regex validity is checked during pipeline build, not config validation
        regex: String,
        #[serde(default)]
        keep_original: bool,
    },

    #[serde(rename = "time_format")]
    TimeSimple {
        order: u32,
        field_in: String,
        #[serde(default)]
        output: Option<String>,
        format_in: String,
        format_out: String,
        #[serde(default)]
        keep_original: bool,
    },

    #[serde(rename = "uppercase")]
    Uppercase {
        order: u32,
        field_in: String,
        #[serde(default)]
        keep_original: bool,
    },

    #[serde(rename = "count_pattern")]
    CountPattern {
        order: u32,
        field_in: String,
        regex: String,
        output: String,
        #[serde(default)]
        keep_original: bool,
    },

    #[serde(rename = "global_count")]
    GlobalCount {
        order: u32,
        field_in: String,
        output: String,
    },

    #[serde(rename = "global_max")]
    GlobalMax {
        order: u32,
        field_in: String,
        output: String,
    },

    #[serde(rename = "global_latest")]
    GlobalLatest {
        order: u32,
        field_in: String,
        output: String,
    },

    #[serde(rename = "drop_fields")]
    DropFields {
        order: u32,
        drop: Vec<String>,
    },

    #[serde(rename = "format_append")]
    FormatAppend {
        order: u32,
        format: String,
        output: String,
        #[serde(default)]
        keep_original: bool,
    },
}

// Sinks
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SinkOperation {
    Append,
    OverwriteWithLast,
}

impl Default for SinkOperation {
    fn default() -> Self { Self::Append }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum SinkConfig {

    #[serde(rename = "json_file")]
    JsonFile {
        name: Option<String>,
        path: String,

        #[serde(default)]
        operation: SinkOperation,
    },
}

// Naming trait implementation for named components
impl NamedComponent for ProcessorConfig {
    fn get_name(&self) -> &Option<String> {
        return &self.name;
    }
}

impl NamedComponent for SourceConfig {
    fn get_name(&self) -> &Option<String> {
        match self {
            SourceConfig::File { name, .. } => name,
            SourceConfig::TCP { name, .. } => name
        }
    }
}

impl NamedComponent for PatternConfig {
    fn get_name(&self) -> &Option<String> {
        return  &self.name;
    }
}

impl NamedComponent for SinkConfig {
    fn get_name(&self) -> &Option<String> {
        match self {
            SinkConfig::JsonFile { name, .. } => name
        }
    }
}
