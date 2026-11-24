//! `lida cwi`: show or switch the working format.

use super::prelude::*;
use lida_core::store::StorageFormat;

pub fn run(cwd: &Path, format: Option<&str>, out: &mut impl Write) -> CmdResult {
    let mut store = Store::load(cwd)?;
    let Some(value) = format else {
        writeln!(out, "Current working format: {:?}", store.format())?;
        return Ok(());
    };
    let fmt = match value.to_lowercase().as_str() {
        "both" => StorageFormat::Both,
        "md" => StorageFormat::MdOnly,
        "yaml" => StorageFormat::YamlOnly,
        _ => return Err("format must be 'yaml', 'md', or 'both'".into()),
    };
    store.set_format(fmt)?;
    store.save()?;
    writeln!(out, "{} Switched working format to {fmt:?}", "✓".bright_green())?;
    Ok(())
}
