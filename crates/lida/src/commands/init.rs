//! `lida init`: create the TODO file(s).

use super::prelude::*;
use lida_core::store::StorageFormat;

pub fn run(cwd: &Path, format: StorageFormat, project: Option<String>, force: bool, out: &mut impl Write) -> CmdResult {
    if !force && (cwd.join("TODO.yaml").exists() || cwd.join("TODO.md").exists()) {
        return Err("TODO file already exists. Use --force to overwrite.".into());
    }
    Store::init(cwd, format, project)?;
    writeln!(
        out,
        "{} Initialized project tasks in {}",
        "✓".bright_green(),
        cwd.display()
    )?;
    Ok(())
}
