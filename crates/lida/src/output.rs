//! Where the CLI writes: every command prints through this pair instead of the process streams directly.

use std::io::Write;

/// Result and error streams for one CLI run.
#[derive(Debug)]
pub struct Output<O: Write, E: Write> {
    /// Command results (stdout in production).
    pub out: O,
    /// The error report (stderr in production).
    pub err: E,
}
