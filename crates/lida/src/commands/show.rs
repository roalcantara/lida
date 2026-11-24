//! `lida show`: one task's details, as text or JSON.

use super::not_found;
use super::prelude::*;

pub fn run(cwd: &Path, id: &str, json: bool, out: &mut impl Write) -> CmdResult {
    let store = Store::load(cwd)?;
    let task = store
        .doc()
        .tasks
        .iter()
        .find(|t| t.id == id)
        .ok_or_else(|| not_found(id))?;

    if json {
        writeln!(out, "{}", serde_json::to_string_pretty(task)?)?;
        return Ok(());
    }
    writeln!(out, "Task #{}: {}", task.id.bold(), task.description)?;
    writeln!(out, "  Status: {}", task.status.as_str())?;
    if let Some(p) = task.priority {
        writeln!(out, "  Priority: {}", p.as_str())?;
    }
    if !task.tags.is_empty() {
        writeln!(out, "  Tags: {}", task.tags.join(", "))?;
    }
    if let Some(due) = &task.due {
        writeln!(out, "  Due: {due}")?;
    }
    if let Some(reason) = &task.blocked_reason {
        writeln!(out, "  Blocked reason: {reason}")?;
    }
    if let Some(pos) = &task.position {
        writeln!(out, "  Position: {pos}")?;
    }
    if !task.meta.is_empty() {
        writeln!(out, "  Metadata:")?;
        for (k, v) in &task.meta {
            writeln!(out, "    {k}: {v}")?;
        }
    }
    let comments: Vec<_> = if task.comments.is_empty() {
        store.doc().comments.iter().filter(|c| c.task_id == task.id).collect()
    } else {
        task.comments
            .iter()
            .filter_map(|cid| store.doc().comments.iter().find(|c| &c.id == cid))
            .collect()
    };
    if !comments.is_empty() {
        writeln!(out, "  Comments:")?;
        for comment in comments {
            writeln!(out, "    #{} {}", comment.id, comment.text)?;
        }
    }
    Ok(())
}
