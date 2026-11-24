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

const NULL_KEYS: [&str; 5] = ["due", "created", "position", "blocked_reason", "priority"];
const EMPTY_SEQ_KEYS: [&str; 3] = ["actors", "comments", "tags"];

fn prune_task_mapping(task: &mut serde_yaml_ng::Mapping) {
    for key in NULL_KEYS {
        let k = serde_yaml_ng::Value::String(key.to_string());
        if task.get(&k).is_some_and(serde_yaml_ng::Value::is_null) {
            task.remove(&k);
        }
    }
    for key in EMPTY_SEQ_KEYS {
        let k = serde_yaml_ng::Value::String(key.to_string());
        if task
            .get(&k)
            .and_then(serde_yaml_ng::Value::as_sequence)
            .is_some_and(Vec::is_empty)
        {
            task.remove(&k);
        }
    }
}

/// Serializes a `ProjectDoc` into a clean YAML string.
///
/// # Errors
/// Returns `FormatError::Yaml` if serialization fails.
pub fn serialize_yaml(doc: &ProjectDoc) -> Result<String, FormatError> {
    let mut val = serde_yaml_ng::to_value(doc)?;
    if let Some(tasks) = val.get_mut("tasks").and_then(serde_yaml_ng::Value::as_sequence_mut) {
        for task in tasks {
            if let Some(mapping) = task.as_mapping_mut() {
                prune_task_mapping(mapping);
            }
        }
    }
    let mut out = serde_yaml_ng::to_string(&val)?;
    // Ensure clean document layout
    if !out.ends_with('\n') {
        out.push('\n');
    }
    Ok(out)
}
