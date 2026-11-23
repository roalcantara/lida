//! `lida completion` and `lida usage`: generated shell and spec files.

use super::prelude::*;

pub fn completion(shell: &str, out: &mut impl Write) -> CmdResult {
    if shell.eq_ignore_ascii_case("zsh") {
        write!(out, "{}", include_str!("../../assets/raw_todo_completion.zsh"))?;
    } else {
        writeln!(out, "# Completion for {shell} not implemented yet")?;
    }
    Ok(())
}

pub fn usage(out: &mut impl Write) -> CmdResult {
    writeln!(out, "# lida.usage.kdl")?;
    writeln!(out, "name \"lida\"")?;
    writeln!(out, "bin \"lida\"")?;
    writeln!(out, "version \"0.1.0\"")?;
    writeln!(out, "about \"Dual-format project task engine (YAML · MD)\"")?;
    Ok(())
}
