//! `lida init`: create the TODO file(s).

use super::prelude::*;
use lida_core::store::StorageFormat;

#[derive(Debug)]
pub struct Args {
    pub format: StorageFormat,
    pub project: Option<String>,
    pub force: bool,
}

pub fn run(cwd: &Path, args: Args, out: &mut impl Write) -> CmdResult {
    if let Some(ref p) = args.project {
        single_line("project", p)?;
        if p.contains("-->") {
            return Err("project name cannot contain '-->'".into());
        }
    }
    if !args.force && (cwd.join("TODO.yaml").exists() || cwd.join("TODO.md").exists()) {
        return Err("TODO file already exists. Use --force to overwrite.".into());
    }
    Store::init(cwd, args.format, args.project)?;
    writeln!(
        out,
        "{} Initialized project tasks in {}",
        "✓".bright_green(),
        cwd.display()
    )?;
    Ok(())
}
