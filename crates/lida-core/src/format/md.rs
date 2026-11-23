//! Markdown parser and serializer for TODO.md.

use super::FormatError;
use crate::model::{Actor, Comment, Priority, ProjectDoc, Status, Task};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Section {
    None,
    Tasks,
    Actors,
    Comments,
}

/// Parses a Markdown task file (GFM format) into a `ProjectDoc`.
///
/// # Errors
/// Returns `FormatError::Markdown` if the markdown cannot be parsed.
pub fn parse_markdown(content: &str) -> Result<ProjectDoc, FormatError> {
    let mut doc = ProjectDoc {
        project: parse_project_name(content),
        ..ProjectDoc::default()
    };

    let mut section = Section::None;
    let mut current_task: Option<Task> = None;
    let mut in_meta_block = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if let Some(heading) = section_heading(trimmed) {
            doc.tasks.extend(current_task.take());
            section = heading;
            continue;
        }

        match section {
            Section::Tasks => {
                if let Some(header) = trimmed.get(..6).filter(|h| TASK_MARKERS.contains(h)) {
                    doc.tasks.extend(current_task.take());
                    in_meta_block = false;
                    current_task = parse_task_header(header, &trimmed[6..]);
                } else if let Some(task) = current_task.as_mut() {
                    apply_task_line(task, trimmed, &mut in_meta_block);
                }
            }
            Section::Actors => {
                if let Some((id, pseudo)) = parse_entry(trimmed) {
                    doc.actors.push(Actor {
                        id,
                        pseudo,
                        pic: None,
                        actor_type: None,
                        extra: BTreeMap::new(),
                    });
                }
            }
            Section::Comments => {
                if let Some((id, text)) = parse_entry(trimmed) {
                    doc.comments.push(Comment {
                        id,
                        text,
                        task_id: String::new(),
                        actors: Vec::new(),
                        extra: BTreeMap::new(),
                    });
                }
            }
            Section::None => {}
        }
    }

    doc.tasks.extend(current_task);
    Ok(doc)
}

/// Checkbox prefixes that open a task line.
const TASK_MARKERS: [&str; 3] = ["- [ ] ", "- [x] ", "- [X] "];

/// Reads the project name from the first `<!-- todo-project: <name> ... -->` comment.
fn parse_project_name(content: &str) -> Option<String> {
    let line = content
        .lines()
        .map(str::trim)
        .find(|l| l.starts_with("<!--") && l.contains("todo-project:"))?;
    let after = line.split("todo-project:").nth(1)?;
    let name = after.split_whitespace().next().unwrap_or("").trim();
    let clean_name = name.trim_end_matches("-->").trim();
    (!clean_name.is_empty()).then(|| clean_name.to_string())
}

fn section_heading(trimmed: &str) -> Option<Section> {
    match trimmed {
        "# Tasks" => Some(Section::Tasks),
        "# Actors" => Some(Section::Actors),
        "# Comments" => Some(Section::Comments),
        _ => None,
    }
}

/// Builds a task from `<id> **-** <description>`; `header` is its checkbox prefix.
fn parse_task_header(header: &str, rest: &str) -> Option<Task> {
    let (id_part, desc_part) = rest.split_once("**-**")?;
    let mut task = Task::new(id_part.trim(), desc_part.trim());
    if header != "- [ ] " {
        task.status = Status::Done;
    }
    Some(task)
}

/// Splits a `- <id> **-** <text>` list entry.
fn parse_entry(trimmed: &str) -> Option<(String, String)> {
    let (id_part, text_part) = trimmed.strip_prefix("- ")?.split_once("**-**")?;
    Some((id_part.trim().to_string(), text_part.trim().to_string()))
}

/// Applies one detail line under a task header; `in_meta_block` tracks a `- **Meta**:` list.
fn apply_task_line(task: &mut Task, trimmed: &str, in_meta_block: &mut bool) {
    if trimmed == "- **Meta**:" {
        *in_meta_block = true;
        return;
    }
    if trimmed.is_empty() {
        *in_meta_block = false;
        return;
    }
    let Some((field, value)) = trimmed
        .strip_prefix("- **")
        .and_then(|rest| rest.split_once("**:"))
        .filter(|(field, _)| TASK_FIELDS.contains(field))
    else {
        if *in_meta_block {
            apply_meta_item(task, trimmed);
        }
        return;
    };
    *in_meta_block = false;
    let value = value.trim();
    match field {
        "Priority" => {
            task.priority = match value.to_lowercase().as_str() {
                "low" => Some(Priority::Low),
                "medium" => Some(Priority::Medium),
                "high" => Some(Priority::High),
                _ => None,
            };
        }
        "Created" => task.created = Some(value.to_string()),
        "Due" => task.due = Some(value.to_string()),
        "Position" => task.position = Some(value.trim_matches('"').trim().to_string()),
        "Status" => {
            if let Ok(status) = serde_json::from_value::<Status>(serde_json::Value::String(value.to_string())) {
                task.status = status;
            }
        }
        "Blocked Reason" => task.blocked_reason = Some(value.to_string()),
        _ => {
            task.tags = value
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }
    }
}

/// Task detail fields recognised under a task header (`Tags` is the last match arm).
const TASK_FIELDS: [&str; 7] = [
    "Priority",
    "Created",
    "Due",
    "Position",
    "Status",
    "Blocked Reason",
    "Tags",
];

/// Parses a `- key: value` meta item; values are JSON when they parse, strings otherwise.
fn apply_meta_item(task: &mut Task, trimmed: &str) {
    if let Some((k, v)) = trimmed.strip_prefix("- ").and_then(|item| item.split_once(':')) {
        let v_str = v.trim();
        let val = serde_json::from_str(v_str).unwrap_or_else(|_| serde_json::Value::String(v_str.to_string()));
        task.meta.insert(k.trim().to_string(), val);
    }
}

/// Serializes a `ProjectDoc` into GitHub Flavored Markdown matching TODO.md.
///
/// # Errors
/// Always returns Ok.
pub fn serialize_markdown(doc: &ProjectDoc) -> Result<String, FormatError> {
    use std::fmt::Write;
    let mut out = String::new();

    if let Some(project) = &doc.project {
        let _ = writeln!(out, "<!-- todo-project: {project} -->\n");
    }

    out.push_str("# Tasks\n\n");

    for task in &doc.tasks {
        let box_mark = if task.status == Status::Done { "[x]" } else { "[ ]" };
        let _ = writeln!(out, "- {box_mark} {} **-** {}", task.id, task.description);

        if task.status == Status::InProgress || task.status == Status::Blocked {
            let _ = writeln!(out, "  - **Status**: {}", task.status.as_str().to_lowercase());
        }
        if let Some(reason) = &task.blocked_reason {
            let _ = writeln!(out, "  - **Blocked Reason**: {reason}");
        }
        if let Some(priority) = task.priority {
            let _ = writeln!(out, "  - **Priority**: {}", priority.as_str().to_lowercase());
        }
        if let Some(created) = &task.created {
            let _ = writeln!(out, "  - **Created**: {created}");
        }
        if let Some(due) = &task.due {
            let _ = writeln!(out, "  - **Due**: {due}");
        }
        if !task.tags.is_empty() {
            let _ = writeln!(out, "  - **Tags**: {}", task.tags.join(", "));
        }
        if let Some(position) = &task.position {
            let _ = writeln!(out, "  - **Position**: \"{position}\"");
        }
        if !task.meta.is_empty() {
            out.push_str("  - **Meta**:\n");
            for (k, v) in &task.meta {
                let val_str = serde_json::to_string(v).unwrap_or_else(|_| v.to_string());
                let _ = writeln!(out, "    - {k}: {val_str}");
            }
        }
        out.push('\n');
    }

    out.push_str("# Actors\n\n");
    for actor in &doc.actors {
        let _ = writeln!(out, "- {} **-** {}", actor.id, actor.pseudo);
    }
    if !doc.actors.is_empty() {
        out.push('\n');
    }

    out.push_str("# Comments\n\n");
    for comment in &doc.comments {
        let _ = writeln!(out, "- {} **-** {}", comment.id, comment.text);
    }

    Ok(out)
}
