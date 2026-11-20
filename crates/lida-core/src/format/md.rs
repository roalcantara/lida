//! Markdown parser and serializer for TODO.md.

use super::FormatError;
use crate::model::ProjectDoc;

pub fn parse_markdown(_content: &str) -> Result<ProjectDoc, FormatError> {
    Ok(ProjectDoc::default())
}

pub fn serialize_markdown(_doc: &ProjectDoc) -> Result<String, FormatError> {
    Ok(String::new())
}
