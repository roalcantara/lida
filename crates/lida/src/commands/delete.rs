//! `lida delete`: remove a task and its comments.

use super::not_found;
use super::prelude::*;

pub fn run(cwd: &Path, id: &str, out: &mut impl Write) -> CmdResult {
    let mut store = Store::load(cwd)?;
    store.doc_mut().comments.retain(|c| c.task_id != id);
    if !store.remove_task(id) {
        return Err(not_found(id).into());
    }
    store.save()?;
    writeln!(out, "{} Deleted task #{}", "✓".bright_green(), id.bold())?;
    Ok(())
}
