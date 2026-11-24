//! `lida tags`: every tag in use with its task count, sorted by name.

use super::prelude::*;
use std::collections::BTreeMap;

pub fn run(cwd: &Path, out: &mut impl Write) -> CmdResult {
    let store = Store::load(cwd)?;
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for tag in store.doc().tasks.iter().flat_map(|t| &t.tags) {
        *counts.entry(tag).or_insert(0) += 1;
    }
    for (tag, count) in counts {
        writeln!(out, "  {tag:<15} {count:>4}")?;
    }
    Ok(())
}
