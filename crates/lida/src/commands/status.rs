//! `lida done`, `lida start`, `lida stop` and `lida status`: change a task's status.

use super::annotation;
use super::prelude::*;
use super::{not_found, parse_status};
use lida_core::date::Date;
use lida_core::model::Status;

pub fn done(cwd: &Path, id: &str, annotate: Option<&str>, today: Date, out: &mut impl Write) -> CmdResult {
    mark(cwd, id, Status::Done, annotate, today)?;
    report(out, id, "done", annotate)
}

pub fn start(cwd: &Path, id: &str, annotate: Option<&str>, today: Date, out: &mut impl Write) -> CmdResult {
    mark(cwd, id, Status::InProgress, annotate, today)?;
    report(out, id, "in-progress", annotate)
}

pub fn stop(cwd: &Path, id: &str, annotate: Option<&str>, today: Date, out: &mut impl Write) -> CmdResult {
    mark(cwd, id, Status::Todo, annotate, today)?;
    report(out, id, "todo", annotate)
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

/// The usual success line, plus `, with annotation` when `-a` was given.
fn report(out: &mut impl Write, id: &str, label: &str, annotate: Option<&str>) -> CmdResult {
    let suffix = if annotate.is_some() { ", with annotation" } else { "" };
    writeln!(
        out,
        "{} Task #{} marked as {label}{suffix}",
        "✓".bright_green(),
        id.bold()
    )?;
    Ok(())
}

/// Refuses before mutating anything; the status change and the annotation are one save.
fn mark(cwd: &Path, id: &str, status: Status, annotate: Option<&str>, today: Date) -> CmdResult {
    if let Some(text) = annotate {
        single_line("annotation", text)?;
    }
    let mut store = Store::load(cwd)?;
    if let Some(text) = annotate {
        annotation::check(&store, id, text)?;
    }
    let task = store.find_task_mut(id).ok_or_else(|| not_found(id))?;
    task.status = status;
    if let Some(text) = annotate {
        annotation::append(task, text, today)?;
    }
    store.save()?;
    Ok(())
}
