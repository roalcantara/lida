//! Filter-word parsing and matching for `lida next`.

use crate::model::Task;
use std::collections::BTreeMap;
use std::ops::Range;

/// Parsed filter query for `lida next`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Query {
    pub words: Vec<String>,
    pub projects: Vec<String>,
    pub tags: Vec<String>,
    pub meta: BTreeMap<String, Vec<String>>,
}

/// Returns the task's own `project` extra field, if present and a string.
#[must_use]
pub fn task_project(task: &Task) -> Option<&str> {
    task.extra.get("project").and_then(serde_yaml_ng::Value::as_str)
}

impl Query {
    /// Parses CLI filter words into structured query clauses.
    #[must_use]
    pub fn parse<S: AsRef<str>>(words: &[S]) -> Self {
        let mut query = Self::default();
        for raw in words {
            let word = raw.as_ref();
            if let Some(project) = word.strip_prefix('@') {
                query.projects.push(project.to_owned());
            } else if let Some(tag) = word.strip_prefix('+') {
                query.tags.push(tag.to_owned());
            } else if let Some((key, value)) = word.split_once(':') {
                query.meta.entry(key.to_owned()).or_default().push(value.to_owned());
            } else {
                query.words.push(word.to_owned());
            }
        }
        query
    }

    /// Returns whether `task` satisfies every clause in this query.
    #[must_use]
    pub fn matches(&self, task: &Task) -> bool {
        self.matches_words(&task.description)
            && self.matches_projects(task)
            && self.matches_tags(task)
            && self.matches_meta(task)
    }

    /// Finds byte ranges in `text` matching any plain search word, ignoring case.
    #[must_use]
    pub fn search_spans(&self, text: &str) -> Vec<Range<usize>> {
        if self.words.is_empty() || text.is_empty() {
            return Vec::new();
        }

        let mut lower = String::with_capacity(text.len());
        let mut byte_map: Vec<(usize, usize)> = Vec::with_capacity(text.len());
        for (orig_start, ch) in text.char_indices() {
            let orig_end = orig_start + ch.len_utf8();
            for lower_ch in ch.to_lowercase() {
                let mut buf = [0u8; 4];
                let encoded = lower_ch.encode_utf8(&mut buf);
                lower.push_str(encoded);
                for _ in 0..encoded.len() {
                    byte_map.push((orig_start, orig_end));
                }
            }
        }

        let mut spans: Vec<Range<usize>> = Vec::new();
        for word in &self.words {
            if word.is_empty() {
                continue;
            }
            let needle = word.to_lowercase();
            if needle.is_empty() {
                continue;
            }
            for (start, matched) in lower.match_indices(&needle) {
                let end_idx = start + matched.len() - 1;
                if let (Some(&(orig_start, _)), Some(&(_, orig_end))) = (byte_map.get(start), byte_map.get(end_idx)) {
                    spans.push(orig_start..orig_end);
                }
            }
        }

        spans.sort_unstable_by_key(|r| (r.start, r.end));
        let mut merged: Vec<Range<usize>> = Vec::with_capacity(spans.len());
        for span in spans {
            if let Some(last) = merged.last_mut()
                && span.start <= last.end
            {
                if span.end > last.end {
                    last.end = span.end;
                }
            } else {
                merged.push(span);
            }
        }
        merged
    }

    fn matches_words(&self, description: &str) -> bool {
        if self.words.is_empty() {
            return true;
        }
        let lower = description.to_lowercase();
        self.words.iter().all(|w| lower.contains(&w.to_lowercase()))
    }

    fn matches_projects(&self, task: &Task) -> bool {
        if self.projects.is_empty() {
            return true;
        }
        let Some(proj) = task_project(task) else {
            return false;
        };
        self.projects
            .iter()
            .any(|p| !p.is_empty() && (proj == p || proj.strip_prefix(p.as_str()).is_some_and(|r| r.starts_with('.'))))
    }

    fn matches_tags(&self, task: &Task) -> bool {
        self.tags
            .iter()
            .all(|tag| !tag.is_empty() && task.tags.iter().any(|t| t == tag))
    }

    fn matches_meta(&self, task: &Task) -> bool {
        self.meta.iter().all(|(key, values)| {
            if key.is_empty() {
                return false;
            }
            let Some(val) = task.meta.get(key) else {
                return false;
            };
            values
                .iter()
                .any(|target| !target.is_empty() && meta_value_matches(val, target))
        })
    }
}

fn meta_value_matches(value: &serde_json::Value, target: &str) -> bool {
    match value {
        serde_json::Value::Array(items) => items.iter().any(|item| scalar_matches(item, target)),
        other => scalar_matches(other, target),
    }
}

fn scalar_matches(value: &serde_json::Value, target: &str) -> bool {
    match value {
        serde_json::Value::String(s) => s == target,
        serde_json::Value::Number(n) => n.to_string() == target,
        serde_json::Value::Bool(b) => (if *b { "true" } else { "false" }) == target,
        serde_json::Value::Null | serde_json::Value::Array(_) | serde_json::Value::Object(_) => false,
    }
}
