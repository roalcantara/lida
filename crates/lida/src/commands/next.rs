//! `lida next`: list open tasks by urgency.

pub(crate) mod layout;
pub(crate) mod mode;
pub(crate) mod palette;
pub(crate) mod row;
pub(crate) mod span;

use super::prelude::*;
use lida_core::date::Date;
use lida_core::next::next;
use lida_core::query::{Query, task_project};
use mode::Mode;

/// Runs `lida next` in read-only mode, writing table, raw, or JSON output to `out`.
pub fn run(cwd: &Path, words: &[String], long: bool, mode: Mode, today: Date, out: &mut impl Write) -> CmdResult {
    let store = Store::load(cwd)?;
    let query = Query::parse(words);
    let entries = next(store.doc(), &query, today);

    match mode {
        Mode::Json => {
            let tasks: Vec<_> = entries.iter().map(|entry| entry.task).collect();
            writeln!(out, "{}", serde_json::to_string_pretty(&tasks)?)?;
        }
        Mode::Raw => {
            if entries.is_empty() {
                writeln!(out, "No tasks found.")?;
            } else {
                for entry in &entries {
                    writeln!(out, "{}", row::raw_row(entry, long))?;
                }
            }
        }
        Mode::Table { width } => {
            if entries.is_empty() {
                writeln!(out, "No tasks found.")?;
            } else {
                let has_project = entries.iter().any(|e| task_project(e.task).is_some());
                let lay = layout::layout(width, long, has_project);
                writeln!(out, "{}", span::ansi(&row::header_row(&lay)))?;
                for entry in &entries {
                    let spans = row::table_row(entry, &query, &lay, today);
                    writeln!(out, "{}", span::ansi(&spans))?;
                }
            }
        }
    }

    Ok(())
}
