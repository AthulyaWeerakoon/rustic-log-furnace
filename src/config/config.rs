use anyhow::{Result, Context};
use std::{fs, path::Path};
use tracing::{info};
use validator::Validate;

use super::types::Config;

impl Config {
    pub fn load_from_file<P: AsRef<Path> + std::fmt::Debug>(conf_path: P) -> Result<Self> {
        // Read file
        let config_str = fs::read_to_string(&conf_path)
            .with_context(|| format!("Failed to read config file at: {:?}", conf_path))?;

        info!("Loaded configuration file: {:?}", conf_path);

        // Deserialize
        let config: Config = toml::from_str(&config_str)
            .with_context(|| "Failed to parse TOML configuration")?;

        // Validate (IMPORTANT)
        config.validate()
            .with_context(|| "Configuration validation failed")?;

        // Custom validation layer (your manual validators)
        Self::validate_config(&config)?;

        Ok(config)
    }

    fn validate_config(config: &Config) -> Result<()> {
        for processor in &config.processors {
            // validate sources
            super::validators::validate_sources(&processor.sources)?;
            super::validators::validate_unique_internal_names(&processor)?;

            // validate patterns
            for pattern in &processor.patterns {
                super::validators::validate_parsers(&pattern.parsers)?;
                super::validators::validate_sinks(&pattern.sinks)?;
            }
        }

        Ok(())
    }
}


