//! `lida update`: change a task's fields.

use super::prelude::*;
use super::{not_found, parse_priority, parse_tags};

/// Fields to change; `None` leaves a field as it is. `tags` is `--tags`, else `--tag`.
#[derive(Debug)]
pub struct Args {
    pub description: Option<String>,
    pub priority: Option<String>,
    pub tags: Option<String>,
    pub due: Option<String>,
    pub position: Option<String>,
}

pub fn run(cwd: &Path, id: &str, args: Args, out: &mut impl Write) -> CmdResult {
    let mut store = Store::load(cwd)?;
    let task = store.find_task_mut(id).ok_or_else(|| not_found(id))?;

    if let Some(desc) = args.description {
        task.description = desc;
    }
    if let Some(p) = args.priority {
        task.priority = parse_priority(&p);
    }
    if let Some(t) = args.tags {
        task.tags = parse_tags(&t);
    }
    if args.due.is_some() {
        task.due = args.due;
    }
    if args.position.is_some() {
        task.position = args.position;
    }

    store.save()?;
    writeln!(out, "{} Updated task #{}", "✓".bright_green(), id.bold())?;
    Ok(())
}
