//! `lida meta`: get, set or remove one metadata key on a task.

use super::not_found;
use super::prelude::*;

/// `set` stores the value as JSON when it parses, else as a string; `get` prints strings bare.
pub fn run(cwd: &Path, id: &str, action: &str, key: &str, value: Option<String>, out: &mut impl Write) -> CmdResult {
    let mut store = Store::load(cwd)?;
    let task = store.find_task_mut(id).ok_or_else(|| not_found(id))?;

    match action.to_lowercase().as_str() {
        "set" => {
            let val_str = value.ok_or("value required for `meta set <key> <value>`")?;
            let val = serde_json::from_str(&val_str).unwrap_or(serde_json::Value::String(val_str));
            task.meta.insert(key.to_string(), val);
            store.save()?;
            writeln!(
                out,
                "{} Set metadata {} for task #{}",
                "✓".bright_green(),
                key.bold(),
                id.bold()
            )?;
        }
        "get" => match task.meta.get(key) {
            Some(serde_json::Value::String(s)) => writeln!(out, "{s}")?,
            Some(v) => writeln!(out, "{v}")?,
            None => return Err(format!("metadata key '{key}' not set on task #{id}").into()),
        },
        "unset" | "rm" => {
            task.meta.remove(key);
            store.save()?;
            writeln!(
                out,
                "{} Removed metadata {} from task #{}",
                "✓".bright_green(),
                key.bold(),
                id.bold()
            )?;
        }
        _ => return Err(format!("unknown meta action '{action}'. Use set, get, or unset").into()),
    }
    Ok(())
}
