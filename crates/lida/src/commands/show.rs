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
    if let Some(pos) = &task.position {
        writeln!(out, "  Position: {pos}")?;
    }
    if !task.meta.is_empty() {
        writeln!(out, "  Metadata:")?;
        for (k, v) in &task.meta {
            writeln!(out, "    {k}: {v}")?;
        }
    }
    Ok(())
}
