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

/// Decides whether ANSI colours should be emitted.
///
/// Rules:
/// - `false` when `NO_COLOR` is set and non-empty.
/// - Else `true` when `CLICOLOR_FORCE` is set, non-empty and not `"0"`.
/// - Else `is_terminal`.
#[must_use]
pub fn colour_enabled(is_terminal: bool, no_color: Option<&str>, clicolor_force: Option<&str>) -> bool {
    if no_color.is_some_and(|nc| !nc.is_empty()) {
        return false;
    }
    if clicolor_force.is_some_and(|cf| !cf.is_empty() && cf != "0") {
        return true;
    }
    is_terminal
}

/// Colorization trait that respects `owo_colors::set_override`.
pub trait OwoColorize: std::fmt::Display + Sized {
    fn bold(&self) -> impl std::fmt::Display + '_ {
        owo_colors::OwoColorize::if_supports_color(self, owo_colors::Stream::Stdout, |v| {
            owo_colors::OwoColorize::bold(v)
        })
    }

    fn dimmed(&self) -> impl std::fmt::Display + '_ {
        owo_colors::OwoColorize::if_supports_color(self, owo_colors::Stream::Stdout, |v| {
            owo_colors::OwoColorize::dimmed(v)
        })
    }

    fn bright_green(&self) -> impl std::fmt::Display + '_ {
        owo_colors::OwoColorize::if_supports_color(self, owo_colors::Stream::Stdout, |v| {
            owo_colors::OwoColorize::bright_green(v)
        })
    }

    fn bright_red(&self) -> impl std::fmt::Display + '_ {
        owo_colors::OwoColorize::if_supports_color(self, owo_colors::Stream::Stdout, |v| {
            owo_colors::OwoColorize::bright_red(v)
        })
    }

    fn bright_yellow(&self) -> impl std::fmt::Display + '_ {
        owo_colors::OwoColorize::if_supports_color(self, owo_colors::Stream::Stdout, |v| {
            owo_colors::OwoColorize::bright_yellow(v)
        })
    }
}

impl<T: std::fmt::Display> OwoColorize for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_color_disables_colour() {
        assert!(!colour_enabled(true, Some("1"), None));
        assert!(!colour_enabled(true, Some("1"), Some("1")));
        assert!(colour_enabled(true, Some(""), None));
    }

    #[test]
    fn clicolor_force_enables_colour_on_non_terminal() {
        assert!(colour_enabled(false, None, Some("1")));
        assert!(!colour_enabled(false, None, Some("0")));
        assert!(!colour_enabled(false, None, Some("")));
    }

    #[test]
    fn terminal_used_when_no_env() {
        assert!(colour_enabled(true, None, None));
        assert!(!colour_enabled(false, None, None));
    }

    #[test]
    fn no_color_beats_clicolor_force() {
        assert!(!colour_enabled(false, Some("1"), Some("1")));
        assert!(!colour_enabled(true, Some("1"), Some("1")));
    }

    #[test]
    fn test_owo_override() {
        owo_colors::set_override(false);
        let s = format!("{}", "hello".bright_red().bold());
        assert_eq!(s, "hello");
        owo_colors::set_override(true);
        let s = format!("{}", "hello".bright_red().bold());
        assert_eq!(s, "\u{1b}[1m\u{1b}[91mhello\u{1b}[39m\u{1b}[0m");
        owo_colors::unset_override();
    }
}
