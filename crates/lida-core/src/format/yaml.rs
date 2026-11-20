//! YAML serialization and deserialization for TODO.yaml.

use super::FormatError;
use crate::model::ProjectDoc;

/// Parses a YAML string into a `ProjectDoc`.
///
/// # Errors
/// Returns `FormatError::Yaml` if the string is not valid YAML or does not match schema.
pub fn parse_yaml(content: &str) -> Result<ProjectDoc, FormatError> {
    let doc: ProjectDoc = serde_yaml_ng::from_str(content)?;
    Ok(doc)
}

/// Serializes a `ProjectDoc` into a clean YAML string.
///
/// # Errors
/// Returns `FormatError::Yaml` if serialization fails.
pub fn serialize_yaml(doc: &ProjectDoc) -> Result<String, FormatError> {
    let mut out = serde_yaml_ng::to_string(doc)?;
    // Ensure clean document layout
    if !out.ends_with('\n') {
        out.push('\n');
    }
    Ok(out)
}
