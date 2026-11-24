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

    for (line_idx, raw_line) in content.lines().enumerate() {
        let line_num = line_idx + 1;
        let line = raw_line.strip_suffix('\r').unwrap_or(raw_line);
        let trimmed = line.trim();

        if trimmed.is_empty() {
            in_meta_block = false;
            continue;
        }

        if let Some(heading) = section_heading(trimmed) {
            doc.tasks.extend(current_task.take());
            in_meta_block = false;
            section = heading;
            continue;
        }

        match section {
            Section::None => {
                if !is_project_comment(trimmed) {
                    doc.unrecognised.push((line_num, line.to_string()));
                }
            }
            Section::Tasks => {
                if let Some(header) = trimmed.get(..6).filter(|h| TASK_MARKERS.contains(h)) {
                    if let Some(task) = parse_task_header(header, &trimmed[6..]) {
                        doc.tasks.extend(current_task.take());
                        in_meta_block = false;
                        current_task = Some(task);
                    } else {
                        doc.unrecognised.push((line_num, line.to_string()));
                    }
                } else if let Some(task) = current_task.as_mut() {
                    if !apply_task_line(task, trimmed, &mut in_meta_block) {
                        doc.unrecognised.push((line_num, line.to_string()));
                    }
                } else {
                    doc.unrecognised.push((line_num, line.to_string()));
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
                } else {
                    doc.unrecognised.push((line_num, line.to_string()));
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
                } else {
                    doc.unrecognised.push((line_num, line.to_string()));
                }
            }
        }
    }

    doc.tasks.extend(current_task);

    for task in &doc.tasks {
        for cid in &task.comments {
            if let Some(comment) = doc.comments.iter_mut().find(|c| c.id == *cid) {
                comment.task_id.clone_from(&task.id);
            }
        }
    }

    Ok(doc)
}

/// Returns the first thing in `doc` that `TODO.md` cannot represent.
#[must_use]
pub fn markdown_cannot_hold(doc: &ProjectDoc) -> Option<String> {
    if !doc.extra.is_empty() {
        return Some("TODO.md cannot hold unknown fields: stay on yaml or both".to_string());
    }

    for actor in &doc.actors {
        if actor.actor_type.is_some() {
            return Some(format!(
                "TODO.md cannot hold actor types (actor {}): stay on yaml or both",
                actor.id
            ));
        }
        if actor.pic.is_some() {
            return Some(format!(
                "TODO.md cannot hold actor pictures (actor {}): stay on yaml or both",
                actor.id
            ));
        }
        if !actor.extra.is_empty() {
            return Some(format!(
                "TODO.md cannot hold unknown fields (actor {}): stay on yaml or both",
                actor.id
            ));
        }
    }

    for task in &doc.tasks {
        if !task.actors.is_empty() {
            return Some(format!(
                "TODO.md cannot hold task actors (task #{}): stay on yaml or both",
                task.id
            ));
        }
        if !task.extra.is_empty() {
            return Some(format!(
                "TODO.md cannot hold unknown fields (task #{}): stay on yaml or both",
                task.id
            ));
        }
    }

    for comment in &doc.comments {
        if !comment.actors.is_empty() {
            return Some(format!(
                "TODO.md cannot hold comment actors (comment {}): stay on yaml or both",
                comment.id
            ));
        }
        if !comment.extra.is_empty() {
            return Some(format!(
                "TODO.md cannot hold unknown fields (comment {}): stay on yaml or both",
                comment.id
            ));
        }
    }

    None
}

/// Checkbox prefixes that open a task line.
const TASK_MARKERS: [&str; 3] = ["- [ ] ", "- [x] ", "- [X] "];

fn is_project_comment(trimmed: &str) -> bool {
    trimmed.starts_with("<!--") && trimmed.contains("todo-project:") && trimmed.ends_with("-->")
}

/// Reads the project name from the first `<!-- todo-project: <name> ... -->` comment.
fn parse_project_name(content: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if is_project_comment(trimmed) {
            let (_, after) = trimmed.split_once("todo-project:")?;
            let (name, _) = after.split_once("-->")?;
            let clean = name.trim();
            if !clean.is_empty() {
                return Some(clean.to_string());
            }
        }
    }
    None
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
fn apply_task_line(task: &mut Task, trimmed: &str, in_meta_block: &mut bool) -> bool {
    if trimmed == "- **Meta**:" {
        *in_meta_block = true;
        return true;
    }
    if let Some((field, value)) = trimmed
        .strip_prefix("- **")
        .and_then(|rest| rest.split_once("**:"))
        .filter(|(field, _)| TASK_FIELDS.contains(field))
    {
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
            "Comments" => {
                task.comments = value
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
            _ => {
                task.tags = value
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
        }
        return true;
    }
    if *in_meta_block {
        return apply_meta_item(task, trimmed);
    }
    false
}

/// Task detail fields recognised under a task header (`Tags` is the last match arm).
const TASK_FIELDS: [&str; 8] = [
    "Priority",
    "Created",
    "Due",
    "Position",
    "Status",
    "Blocked Reason",
    "Tags",
    "Comments",
];

/// Parses a `- key: value` meta item; values are JSON when they parse, strings otherwise.
fn apply_meta_item(task: &mut Task, trimmed: &str) -> bool {
    if let Some((k, v)) = trimmed.strip_prefix("- ").and_then(|item| item.split_once(':')) {
        let v_str = v.trim();
        let val = serde_json::from_str(v_str).unwrap_or_else(|_| serde_json::Value::String(v_str.to_string()));
        task.meta.insert(k.trim().to_string(), val);
        true
    } else {
        false
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
        if !task.comments.is_empty() {
            let _ = writeln!(out, "  - **Comments**: {}", task.comments.join(", "));
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
