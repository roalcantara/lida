//! lida: Dual-format project task engine CLI (YAML · MD).

#![forbid(unsafe_code)]

mod cli;
mod commands;
mod output;

use cli::LidaCli;
use output::Output;
use owo_colors::OwoColorize;
use std::io::{self, Write};
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = LidaCli::parse();
    let mut io = Output {
        out: io::stdout().lock(),
        err: io::stderr().lock(),
    };

    match commands::run(cli, &mut io) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            // Nothing is left to report a failed error report to.
            let _ = writeln!(io.err, "{}: {err}", "error".bright_red().bold());
            ExitCode::FAILURE
        }
    }
}
