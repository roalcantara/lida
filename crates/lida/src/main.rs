//! lida: Dual-format project task engine CLI (YAML · MD).

#![forbid(unsafe_code)]

mod cli;
mod commands;
mod output;
#[cfg_attr(not(test), expect(dead_code, reason = "consumed by Task 6 (commands/next)"))]
mod text;

use cli::LidaCli;
use output::{Output, OwoColorize, colour_enabled};
use std::env;
use std::io::{self, IsTerminal, Write};
use std::process::ExitCode;

fn main() -> ExitCode {
    let no_color = env::var("NO_COLOR").ok();
    let clicolor_force = env::var("CLICOLOR_FORCE").ok();

    let stdout_color = colour_enabled(
        io::stdout().is_terminal(),
        no_color.as_deref(),
        clicolor_force.as_deref(),
    );
    owo_colors::set_override(stdout_color);

    let cli = LidaCli::parse();
    let mut io = Output {
        out: io::stdout().lock(),
        err: io::stderr().lock(),
    };

    match commands::run(cli, &mut io) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            let stderr_color = colour_enabled(
                io::stderr().is_terminal(),
                no_color.as_deref(),
                clicolor_force.as_deref(),
            );
            owo_colors::set_override(stderr_color);
            // Nothing is left to report a failed error report to.
            let _ = writeln!(io.err, "{}: {err}", "error".bright_red().bold());
            ExitCode::FAILURE
        }
    }
}
