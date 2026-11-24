//! `lida done`, `lida start` and `lida status`: change a task's status.

use super::prelude::*;
use super::{not_found, parse_status};
use lida_core::model::Status;

pub fn done(cwd: &Path, id: &str, out: &mut impl Write) -> CmdResult {
    mark(cwd, id, Status::Done)?;
    writeln!(out, "{} Task #{} marked as done", "✓".bright_green(), id.bold())?;
    Ok(())
}

pub fn start(cwd: &Path, id: &str, out: &mut impl Write) -> CmdResult {
    mark(cwd, id, Status::InProgress)?;
    writeln!(out, "{} Task #{} marked as in-progress", "✓".bright_green(), id.bold())?;
    Ok(())
}

/// `--set` takes a status or one of its aliases; `--reason` is kept only for `blocked`.
pub fn set(cwd: &Path, id: &str, value: &str, reason: Option<String>, out: &mut impl Write) -> CmdResult {
    let status = parse_status(value)?;
    if let Some(ref r) = reason {
        single_line("reason", r)?;
    }
    let mut store = Store::load(cwd)?;
    let task = store.find_task_mut(id).ok_or_else(|| not_found(id))?;
    task.status = status;
    if status == Status::Blocked {
        task.blocked_reason = reason;
    } else if reason.is_some() {
        task.blocked_reason = None;
    }
    store.save()?;
    writeln!(
        out,
        "{} Task #{} status set to {}",
        "✓".bright_green(),
        id.bold(),
        status.as_str()
    )?;
    Ok(())
}

fn mark(cwd: &Path, id: &str, status: Status) -> CmdResult {
    let mut store = Store::load(cwd)?;
    store.find_task_mut(id).ok_or_else(|| not_found(id))?.status = status;
    store.save()?;
    Ok(())
}
