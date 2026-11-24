//! `lida completion` and `lida usage`: generated shell and spec files.

use super::prelude::*;
use usage_rs::completion::Completion;

pub fn completion(completion: &Completion, out: &mut impl Write, err: &mut impl Write) -> CmdResult {
    completion.run_split("lida", out, err)?;
    Ok(())
}

pub fn usage(out: &mut impl Write) -> CmdResult {
    write!(out, "{}", crate::cli::LidaCli::to_kdl())?;
    Ok(())
}
