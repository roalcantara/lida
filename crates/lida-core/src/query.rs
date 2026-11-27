//! Filter-word parsing and matching for `lida next`.

use crate::date::{Date, parse_day};
use crate::model::Task;
use std::collections::BTreeMap;
use std::ops::Range;

/// Reserved virtual due-date tag recognised in `lida next` queries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DueTag {
    Overdue,
    Today,
    Tomorrow,
    Week,
}

impl DueTag {
    /// All supported due tags in canonical order.
    pub const ALL: [Self; 4] = [Self::Overdue, Self::Today, Self::Tomorrow, Self::Week];

    /// Parses an uppercase due tag name (without the leading `+`).
    #[must_use]
    pub fn parse(name: &str) -> Option<Self> {
        match name {
            "OVERDUE" => Some(Self::Overdue),
            "TODAY" => Some(Self::Today),
            "TOMORROW" => Some(Self::Tomorrow),
            "WEEK" => Some(Self::Week),
            _ => None,
        }
    }

    /// Returns the uppercase due tag name (without the leading `+`).
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Overdue => "OVERDUE",
            Self::Today => "TODAY",
            Self::Tomorrow => "TOMORROW",
            Self::Week => "WEEK",
        }
    }

    /// Returns whether `due` falls in this tag's window relative to `today`.
    #[must_use]
    pub fn matches(self, due: Date, today: Date) -> bool {
        let days = (due - today).get_days();
        match self {
            Self::Overdue => days < 0,
            Self::Today => days == 0,
            Self::Tomorrow => days == 1,
            Self::Week => (0..=6).contains(&days),
        }
    }
}

/// Parsed filter query for `lida next`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Query {
    pub words: Vec<String>,
    pub projects: Vec<String>,
    pub tags: Vec<String>,
    pub due_tags: Vec<DueTag>,
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
                if let Some(due_tag) = DueTag::parse(tag) {
                    if !query.due_tags.contains(&due_tag) {
                        query.due_tags.push(due_tag);
                    }
                } else {
                    query.tags.push(tag.to_owned());
                }
            } else if let Some((key, value)) = word.split_once(':') {
                query.meta.entry(key.to_owned()).or_default().push(value.to_owned());
            } else {
                query.words.push(word.to_owned());
            }
        }
        query
    }

    /// Returns whether `task` satisfies every clause in this query as of `today`.
    #[must_use]
    pub fn matches(&self, task: &Task, today: Date) -> bool {
        self.matches_words(&task.description)
            && self.matches_projects(task)
            && self.matches_tags(task)
            && self.matches_due(task, today)
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

    fn matches_due(&self, task: &Task, today: Date) -> bool {
        if self.due_tags.is_empty() {
            return true;
        }
        let Some(due_date) = task.due.as_deref().and_then(parse_day) else {
            return false;
        };
        self.due_tags.iter().any(|tag| tag.matches(due_date, today))
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

#[cfg(test)]
mod tests {
    use super::DueTag;

    #[test]
    fn due_tag_round_trips_and_rejects_lowercase() {
        for tag in DueTag::ALL {
            assert_eq!(DueTag::parse(tag.name()), Some(tag));
        }
        assert_eq!(DueTag::parse("today"), None);
    }
}
