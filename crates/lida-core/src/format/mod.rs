//! Serialization and deserialization format drivers for YAML and Markdown.

pub mod md;
pub mod yaml;

use thiserror::Error;

/// Format errors encountered during parsing or serialization.
#[derive(Debug, Error)]
pub enum FormatError {
    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml_ng::Error),
    #[error("Markdown parse error: {0}")]
    Markdown(String),
}
