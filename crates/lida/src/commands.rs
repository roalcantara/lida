//! One module per subcommand; this file dispatches to them and holds the parsing they share.

mod add;
mod cwi;
mod delete;
mod generate;
mod init;
mod list;
mod meta;
mod show;
mod status;
mod tags;
mod update;

/// Imports every command module needs.
mod prelude {
    pub use super::CmdResult;
    pub(super) use super::{single_line, validate_task_fields};
    pub use crate::output::OwoColorize;
    pub use lida_core::store::Store;
    pub use std::io::Write;
    pub use std::path::Path;
}

use crate::cli::{LidaCli, Subcommand};
use crate::output::Output;
use lida_core::model::{Priority, Status};
use lida_core::store::StorageFormat;
use std::env;
use std::io::Write;

/// What every command returns; the error is printed once, by `main`.
pub type CmdResult = Result<(), Box<dyn std::error::Error>>;

/// Runs the parsed command line, writing results to `io.out`.
///
/// # Errors
/// Returns the first error a command hits; nothing has been printed for it yet.
pub fn run<O: Write, E: Write>(cli: LidaCli, io: &mut Output<O, E>) -> CmdResult {
    let cwd = env::current_dir()?;
    let out = &mut io.out;

    match cli.command.unwrap_or_else(default_command) {
        Subcommand::Init {
            yaml,
            md,
            both,
            project,
            force,
        } => {
            let format = init_format(yaml, md, both)?;
            init::run(&cwd, init::Args { format, project, force }, out)
        }
        Subcommand::Add {
            description,
            task,
            task_id,
            priority,
            tags,
            due,
            position,
            comment,
        } => add::run(
            &cwd,
            add::Args {
                description: description.or(task),
                task_id,
                priority,
                tags,
                due,
                position,
                comment,
            },
            out,
        ),
        Subcommand::Update {
            id,
            description,
            priority,
            tags,
            tag,
            due,
            position,
        } => update::run(
            &cwd,
            &id,
            update::Args {
                description,
                priority,
                tags: tags.or(tag),
                due,
                position,
            },
            out,
        ),
        Subcommand::List {
            tasks,
            status,
            tag,
            priority,
            json,
        } => list::run(
            &cwd,
            &list::Filter { status, tag, priority },
            list::Style::from_flags(tasks, json),
            out,
        ),
        Subcommand::Done { id } => status::done(&cwd, &id, out),
        Subcommand::Start { id } => status::start(&cwd, &id, out),
        Subcommand::Status { id, set, reason } => status::set(&cwd, &id, &set, reason, out),
        Subcommand::Show { id, json } => show::run(&cwd, &id, json, out),
        Subcommand::Meta { id, action, key, value } => meta::run(&cwd, &id, &action, &key, value, out),
        Subcommand::Tags => tags::run(&cwd, out),
        Subcommand::Delete { id } => delete::run(&cwd, &id, out),
        Subcommand::Cwi { format } => cwi::run(&cwd, format.as_deref(), out),
        Subcommand::Completion { shell } => generate::completion(&shell, out),
        Subcommand::Usage => generate::usage(out),
    }
}

/// Bare `lida` lists tasks.
fn default_command() -> Subcommand {
    Subcommand::List {
        tasks: false,
        status: None,
        tag: None,
        priority: None,
        json: false,
    }
}

fn init_format(yaml: bool, md: bool, both: bool) -> Result<StorageFormat, &'static str> {
    let count = [yaml, md, both].into_iter().filter(|&b| b).count();
    if count > 1 {
        return Err("choose one of --yaml, --md or --both");
    }
    if both {
        Ok(StorageFormat::Both)
    } else if md {
        Ok(StorageFormat::MdOnly)
    } else {
        Ok(StorageFormat::YamlOnly)
    }
}

/// Error text for an unknown task ID.
fn not_found(id: &str) -> String {
    format!("task #{id} not found")
}

pub(super) fn single_line(field: &str, value: &str) -> Result<(), String> {
    if value.contains('\n') || value.contains('\r') {
        Err(format!("{field} must be a single line"))
    } else {
        Ok(())
    }
}

pub(super) fn validate_task_fields(
    position: Option<&str>,
    due: Option<&str>,
    tags: Option<&str>,
) -> Result<(), String> {
    if let Some(p) = position {
        single_line("position", p)?;
    }
    if let Some(d) = due {
        single_line("due", d)?;
    }
    if let Some(t) = tags {
        single_line("tags", t)?;
    }
    Ok(())
}

pub(super) fn parse_status(value: &str) -> Result<Status, String> {
    Status::parse(value)
        .ok_or_else(|| format!("invalid status '{value}'. Valid values: todo, in-progress, done, blocked"))
}

pub(super) fn parse_priority_arg(value: &str) -> Result<Option<Priority>, String> {
    if value.eq_ignore_ascii_case("none") {
        Ok(None)
    } else {
        Priority::parse(value)
            .map(Some)
            .ok_or_else(|| format!("invalid priority '{value}'. Valid values: low, medium, high, none"))
    }
}

/// Comma-separated tags, trimmed, empties dropped.
fn parse_tags(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}
