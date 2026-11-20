//! Markdown parser and serializer for TODO.md.

use super::FormatError;
use crate::model::{Actor, Comment, Priority, ProjectDoc, Status, Task};

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

    enum Section {
        None,
        Tasks,
        Actors,
        Comments,
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

                    if trimmed.starts_with("- **Priority**:") {
                        in_meta_block = false;
                        let val = trimmed["- **Priority**:".len()..].trim();
                        task.priority = match val.to_lowercase().as_str() {
                            "low" => Some(Priority::Low),
                            "medium" => Some(Priority::Medium),
                            "high" => Some(Priority::High),
                            _ => None,
                        };
                    } else if trimmed.starts_with("- **Created**:") {
                        in_meta_block = false;
                        task.created = Some(trimmed["- **Created**:".len()..].trim().to_string());
                    } else if trimmed.starts_with("- **Due**:") {
                        in_meta_block = false;
                        task.due = Some(trimmed["- **Due**:".len()..].trim().to_string());
                    } else if trimmed.starts_with("- **Position**:") {
                        in_meta_block = false;
                        let pos = trimmed["- **Position**:".len()..].trim();
                        let clean = pos.trim_matches('"').trim();
                        task.position = Some(clean.to_string());
                    } else if trimmed.starts_with("- **Status**:") {
                        in_meta_block = false;
                        let st = trimmed["- **Status**:".len()..].trim();
                        if let Ok(status) = serde_json::from_value::<Status>(serde_json::Value::String(st.to_string())) {
                            task.status = status;
                        }
                    } else if trimmed.starts_with("- **Blocked Reason**:") {
                        in_meta_block = false;
                        task.blocked_reason = Some(trimmed["- **Blocked Reason**:".len()..].trim().to_string());
                    } else if trimmed.starts_with("- **Tags**:") {
                        in_meta_block = false;
                        let tags_str = trimmed["- **Tags**:".len()..].trim();
                        task.tags = tags_str.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
                    } else if trimmed == "- **Meta**:" {
                        in_meta_block = true;
                    } else if in_meta_block && trimmed.starts_with("- ") {
                        let item = &trimmed[2..];
                        if let Some((k, v)) = item.split_once(':') {
                            let k = k.trim().to_string();
                            let v_str = v.trim();
                            let val = serde_json::from_str(v_str).unwrap_or_else(|_| serde_json::Value::String(v_str.to_string()));
                            task.meta.insert(k, val);
                        }
                    }
                }
            }
            Section::Actors => {
                if trimmed.starts_with("- ") {
                    let rest = &trimmed[2..];
                    if let Some((id_part, name_part)) = rest.split_once("**-**") {
                        doc.actors.push(Actor {
                            id: id_part.trim().to_string(),
                            pseudo: name_part.trim().to_string(),
                            pic: None,
                            actor_type: None,
                        });
                    }
                }
            }
            Section::Comments => {
                if trimmed.starts_with("- ") {
                    let rest = &trimmed[2..];
                    if let Some((id_part, text_part)) = rest.split_once("**-**") {
                        doc.comments.push(Comment {
                            id: id_part.trim().to_string(),
                            text: text_part.trim().to_string(),
                            task_id: String::new(),
                            actors: Vec::new(),
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
    let mut out = String::new();

    if let Some(project) = &doc.project {
        out.push_str(&format!("<!-- todo-project: {project} -->\n\n"));
    }

    out.push_str("# Tasks\n\n");

    for task in &doc.tasks {
        let box_mark = if task.status == Status::Done { "[x]" } else { "[ ]" };
        out.push_str(&format!("- {box_mark} {} **-** {}\n", task.id, task.description));

        if task.status == Status::InProgress || task.status == Status::Blocked {
            out.push_str(&format!("  - **Status**: {}\n", task.status.as_str().to_lowercase()));
        }
        if let Some(reason) = &task.blocked_reason {
            out.push_str(&format!("  - **Blocked Reason**: {reason}\n"));
        }
        if let Some(priority) = task.priority {
            out.push_str(&format!("  - **Priority**: {}\n", priority.as_str().to_lowercase()));
        }
        if let Some(created) = &task.created {
            out.push_str(&format!("  - **Created**: {created}\n"));
        }
        if let Some(due) = &task.due {
            out.push_str(&format!("  - **Due**: {due}\n"));
        }
        if !task.tags.is_empty() {
            out.push_str(&format!("  - **Tags**: {}\n", task.tags.join(", ")));
        }
        if let Some(position) = &task.position {
            out.push_str(&format!("  - **Position**: \"{position}\"\n"));
        }
        if !task.meta.is_empty() {
            out.push_str("  - **Meta**:\n");
            let mut keys: Vec<_> = task.meta.keys().collect();
            keys.sort();
            for k in keys {
                let v = &task.meta[k];
                if let serde_json::Value::String(s) = v {
                    out.push_str(&format!("    - {k}: {s}\n"));
                } else {
                    out.push_str(&format!("    - {k}: {v}\n"));
                }
            }
        }
        out.push('\n');
    }

    out.push_str("# Actors\n\n");
    for actor in &doc.actors {
        out.push_str(&format!("- {} **-** {}\n", actor.id, actor.pseudo));
    }
    if !doc.actors.is_empty() {
        out.push('\n');
    }

    out.push_str("# Comments\n\n");
    for comment in &doc.comments {
        out.push_str(&format!("- {} **-** {}\n", comment.id, comment.text));
    }

    Ok(out)
}
