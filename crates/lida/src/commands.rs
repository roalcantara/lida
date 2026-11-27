//! One module per subcommand; this file dispatches to them and holds the parsing they share.

mod add;
mod cwi;
mod delete;
mod generate;
mod init;
mod list;
mod meta;
mod next;
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
        Subcommand::Init(args) => {
            let format = init_format(args.yaml, args.md, args.both)?;
            init::run(
                &cwd,
                init::Args {
                    format,
                    project: args.project,
                    force: args.force,
                },
                out,
            )
        }
        Subcommand::Add(args) => add::run(
            &cwd,
            add::Args {
                description: args.description.or(args.task),
                task_id: args.task_id,
                priority: args.priority,
                tags: args.tags,
                due: args.due,
                position: args.position,
                comment: args.comment,
            },
            out,
        ),
        Subcommand::Update(args) => update::run(
            &cwd,
            &args.id,
            update::Args {
                description: args.description,
                priority: args.priority,
                tags: args.tags.or(args.tag),
                due: args.due,
                position: args.position,
            },
            out,
        ),
        Subcommand::List(args) => list::run(
            &cwd,
            &list::Filter {
                status: args.status,
                tag: args.tag,
                priority: args.priority,
            },
            list::Style::from_flags(args.tasks, args.json),
            out,
        ),
        Subcommand::Next(args) => {
            let today = env::var("LIDA_TODAY")
                .ok()
                .and_then(|s| lida_core::date::parse_day(&s))
                .unwrap_or_else(|| jiff::Zoned::now().date());
            let no_color = env::var("NO_COLOR").is_ok_and(|v| !v.is_empty());
            let force_colour = env::var("CLICOLOR_FORCE").is_ok_and(|v| !v.is_empty() && v != "0");
            let columns = env::var("COLUMNS").ok().and_then(|s| s.parse::<usize>().ok());
            let mode = next::mode::mode(
                args.json,
                args.raw,
                crate::output::stdout_is_terminal(),
                no_color,
                force_colour,
                columns,
                crate::output::terminal_width(),
            );
            next::run(&cwd, &args.words, args.long, mode, today, out)
        }
        Subcommand::Done(args) => status::done(&cwd, &args.id, out),
        Subcommand::Start(args) => status::start(&cwd, &args.id, out),
        Subcommand::Status(args) => status::set(&cwd, &args.id, &args.set, args.reason, out),
        Subcommand::Show(args) => show::run(&cwd, &args.id, args.json, out),
        Subcommand::Meta(args) => meta::run(&cwd, &args.id, &args.action, &args.key, args.value, out),
        Subcommand::Tags => tags::run(&cwd, out),
        Subcommand::Delete(args) => delete::run(&cwd, &args.id, out),
        Subcommand::Cwi(args) => cwi::run(&cwd, args.format.as_deref(), out),
        Subcommand::Completion(completion) => generate::completion(&completion, out, &mut io.err),
        Subcommand::Usage => generate::usage(out),
    }
}

/// Bare `lida` lists tasks.
fn default_command() -> Subcommand {
    Subcommand::List(crate::cli::ListArgs {
        tasks: false,
        status: None,
        tag: None,
        priority: None,
        json: false,
    })
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
