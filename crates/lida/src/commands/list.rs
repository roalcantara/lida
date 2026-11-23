//! `lida list` (and bare `lida`): filter tasks and print them as a table, plain lines or JSON.

use super::prelude::*;
use lida_core::model::{Priority, Status, Task};

/// Filters; each one set must match.
#[derive(Debug)]
pub struct Filter {
    pub status: Option<String>,
    pub tag: Option<String>,
    pub priority: Option<String>,
}

/// How to print the tasks; `--json` wins over `--tasks`.
#[derive(Debug, Clone, Copy)]
pub enum Style {
    Table,
    Plain,
    Json,
}

impl Style {
    pub fn from_flags(tasks: bool, json: bool) -> Self {
        if json {
            Self::Json
        } else if tasks {
            Self::Plain
        } else {
            Self::Table
        }
    }
}

pub fn run(cwd: &Path, filter: &Filter, style: Style, out: &mut impl Write) -> CmdResult {
    let store = Store::load(cwd)?;
    let items = filtered(&store.doc().tasks, filter);
    match style {
        Style::Json => writeln!(out, "{}", serde_json::to_string_pretty(&items)?)?,
        Style::Plain => write_plain(&items, out)?,
        Style::Table => write_table(&items, out)?,
    }
    Ok(())
}

fn filtered<'a>(tasks: &'a [Task], filter: &Filter) -> Vec<&'a Task> {
    let mut items: Vec<&Task> = tasks.iter().collect();
    if let Some(s) = &filter.status {
        let s_lower = s.to_lowercase();
        let normalized = match s_lower.as_str() {
            "inprogress" | "in-progress" | "en-cours" | "encours" | "progress" => "in-progress",
            "done" | "completed" => "done",
            "blocked" | "bloqued" => "blocked",
            "todo" => "todo",
            other => other,
        };
        items.retain(|t| t.status.as_str().eq_ignore_ascii_case(normalized));
    }
    if let Some(tg) = &filter.tag {
        items.retain(|t| t.tags.iter().any(|t_tag| t_tag.eq_ignore_ascii_case(tg)));
    }
    if let Some(p) = &filter.priority {
        items.retain(|t| t.priority.is_some_and(|p_val| p_val.as_str().eq_ignore_ascii_case(p)));
    }
    items
}

/// One line per task, for shell completion: `[Status] [Priority] <id> — <description>`.
fn write_plain(items: &[&Task], out: &mut impl Write) -> std::io::Result<()> {
    for t in items {
        let prio = t.priority.map_or_else(String::new, |p| format!("[{}] ", p.as_str()));
        writeln!(out, "[{}] {prio}{} — {}", t.status.as_str(), t.id, t.description)?;
    }
    Ok(())
}

fn write_table(items: &[&Task], out: &mut impl Write) -> std::io::Result<()> {
    if items.is_empty() {
        return writeln!(out, "{}", "No tasks found.".dimmed());
    }
    writeln!(
        out,
        "{:<6} {:<12} {:<10} {:<30} {}",
        "ID".bold(),
        "STATUS".bold(),
        "PRIORITY".bold(),
        "TITLE".bold(),
        "TAGS".bold()
    )?;
    writeln!(out, "{}", "─".repeat(70).dimmed())?;
    for t in items {
        let tags = if t.tags.is_empty() {
            "-".dimmed().to_string()
        } else {
            t.tags.join(", ").dimmed().to_string()
        };
        writeln!(
            out,
            "#{:<5} {:<12} {:<10} {:<30} {}",
            t.id.bold(),
            status_cell(t.status),
            priority_cell(t.priority),
            t.description,
            tags
        )?;
    }
    Ok(())
}

fn status_cell(status: Status) -> String {
    match status {
        Status::Todo => "Todo".dimmed().to_string(),
        Status::InProgress => "InProgress".bright_yellow().to_string(),
        Status::Done => "Done".bright_green().to_string(),
        Status::Blocked => "Blocked".bright_red().to_string(),
    }
}

fn priority_cell(priority: Option<Priority>) -> String {
    priority.map_or("-".dimmed().to_string(), |p| match p {
        Priority::High => "High".bright_red().to_string(),
        Priority::Medium => "Medium".bright_yellow().to_string(),
        Priority::Low => "Low".dimmed().to_string(),
    })
}
