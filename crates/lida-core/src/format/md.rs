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
    let mut doc = ProjectDoc::default();

    // Check project header comment: <!-- todo-project: <name> ... -->
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("<!--") && trimmed.contains("todo-project:") {
            if let Some(after) = trimmed.split("todo-project:").nth(1) {
                let name = after.split_whitespace().next().unwrap_or("").trim();
                let clean_name = name.trim_end_matches("-->").trim();
                if !clean_name.is_empty() {
                    doc.project = Some(clean_name.to_string());
                }
            }
            break;
        }
    }

    let mut section = Section::None;
    let mut current_task: Option<Task> = None;
    let mut in_meta_block = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed == "# Tasks" {
            if let Some(t) = current_task.take() {
                doc.tasks.push(t);
            }
            section = Section::Tasks;
            continue;
        } else if trimmed == "# Actors" {
            if let Some(t) = current_task.take() {
                doc.tasks.push(t);
            }
            section = Section::Actors;
            continue;
        } else if trimmed == "# Comments" {
            if let Some(t) = current_task.take() {
                doc.tasks.push(t);
            }
            section = Section::Comments;
            continue;
        }

        match section {
            Section::Tasks => {
                if trimmed.starts_with("- [ ] ") || trimmed.starts_with("- [x] ") || trimmed.starts_with("- [X] ") {
                    if let Some(t) = current_task.take() {
                        doc.tasks.push(t);
                    }
                    in_meta_block = false;

                    let is_done = trimmed.starts_with("- [x] ") || trimmed.starts_with("- [X] ");
                    let rest = &trimmed[6..];

                    // Format: <id> **-** <description>
                    if let Some((id_part, desc_part)) = rest.split_once("**-**") {
                        let id = id_part.trim();
                        let desc = desc_part.trim();
                        let mut t = Task::new(id, desc);
                        if is_done {
                            t.status = Status::Done;
                        }
                        current_task = Some(t);
                    }
                } else if let Some(task) = current_task.as_mut() {
                    if trimmed.is_empty() {
                        in_meta_block = false;
                        continue;
                    }

                    if let Some(stripped) = trimmed.strip_prefix("- **Priority**:") {
                        in_meta_block = false;
                        let val = stripped.trim();
                        task.priority = match val.to_lowercase().as_str() {
                            "low" => Some(Priority::Low),
                            "medium" => Some(Priority::Medium),
                            "high" => Some(Priority::High),
                            _ => None,
                        };
                    } else if let Some(stripped) = trimmed.strip_prefix("- **Created**:") {
                        in_meta_block = false;
                        task.created = Some(stripped.trim().to_string());
                    } else if let Some(stripped) = trimmed.strip_prefix("- **Due**:") {
                        in_meta_block = false;
                        task.due = Some(stripped.trim().to_string());
                    } else if let Some(stripped) = trimmed.strip_prefix("- **Position**:") {
                        in_meta_block = false;
                        let pos = stripped.trim();
                        let clean = pos.trim_matches('"').trim();
                        task.position = Some(clean.to_string());
                    } else if let Some(stripped) = trimmed.strip_prefix("- **Status**:") {
                        in_meta_block = false;
                        let st = stripped.trim();
                        if let Ok(status) = serde_json::from_value::<Status>(serde_json::Value::String(st.to_string()))
                        {
                            task.status = status;
                        }
                    } else if let Some(stripped) = trimmed.strip_prefix("- **Blocked Reason**:") {
                        in_meta_block = false;
                        task.blocked_reason = Some(stripped.trim().to_string());
                    } else if let Some(stripped) = trimmed.strip_prefix("- **Tags**:") {
                        in_meta_block = false;
                        let tags_str = stripped.trim();
                        task.tags = tags_str
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    } else if trimmed == "- **Meta**:" {
                        in_meta_block = true;
                    } else if in_meta_block {
                        if let Some(item) = trimmed.strip_prefix("- ") {
                            if let Some((k, v)) = item.split_once(':') {
                                let k = k.trim().to_string();
                                let v_str = v.trim();
                                let val = serde_json::from_str(v_str)
                                    .unwrap_or_else(|_| serde_json::Value::String(v_str.to_string()));
                                task.meta.insert(k, val);
                            }
                        }
                    }
                }
            }
            Section::Actors => {
                if let Some(rest) = trimmed.strip_prefix("- ") {
                    if let Some((id_part, name_part)) = rest.split_once("**-**") {
                        doc.actors.push(Actor {
                            id: id_part.trim().to_string(),
                            pseudo: name_part.trim().to_string(),
                            pic: None,
                            actor_type: None,
                            extra: BTreeMap::new(),
                        });
                    }
                }
            }
            Section::Comments => {
                if let Some(rest) = trimmed.strip_prefix("- ") {
                    if let Some((id_part, text_part)) = rest.split_once("**-**") {
                        doc.comments.push(Comment {
                            id: id_part.trim().to_string(),
                            text: text_part.trim().to_string(),
                            task_id: String::new(),
                            actors: Vec::new(),
                            extra: BTreeMap::new(),
                        });
                    }
                }
            }
            Section::None => {}
        }
    }

    if let Some(t) = current_task {
        doc.tasks.push(t);
    }

    Ok(doc)
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
