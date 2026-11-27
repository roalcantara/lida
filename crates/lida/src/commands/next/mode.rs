//! Output mode selection for `lida next`.

/// Rendering mode for one `lida next` invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Table { width: usize },
    Raw,
    Json,
}

/// Chooses table, raw, or JSON mode from flags and terminal environment.
#[must_use]
#[expect(
    clippy::fn_params_excessive_bools,
    reason = "matches the independent CLI flags and terminal environment switches"
)]
pub fn mode(
    json: bool,
    raw: bool,
    stdout_terminal: bool,
    no_color: bool,
    force_colour: bool,
    columns: Option<usize>,
    terminal_width: Option<usize>,
) -> Mode {
    if json {
        return Mode::Json;
    }
    if raw || no_color {
        return Mode::Raw;
    }
    if stdout_terminal {
        return Mode::Table {
            width: terminal_width.or(columns).unwrap_or(80),
        };
    }
    if force_colour {
        return Mode::Table {
            width: columns.unwrap_or(80),
        };
    }
    Mode::Raw
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_color_on_terminal_is_raw() {
        assert_eq!(mode(false, false, true, true, false, Some(120), Some(120)), Mode::Raw);
        assert_eq!(mode(false, false, true, true, true, Some(120), Some(120)), Mode::Raw);
    }

    #[test]
    fn flags_and_pipe_select_mode() {
        assert_eq!(mode(true, false, true, false, false, None, Some(120)), Mode::Json);
        assert_eq!(mode(false, true, true, false, false, None, Some(120)), Mode::Raw);
        assert_eq!(
            mode(false, false, true, false, false, Some(100), Some(120)),
            Mode::Table { width: 120 }
        );
        assert_eq!(
            mode(false, false, true, false, false, Some(100), None),
            Mode::Table { width: 100 }
        );
        assert_eq!(mode(false, false, false, false, false, Some(120), None), Mode::Raw);
        assert_eq!(
            mode(false, false, false, false, true, Some(120), None),
            Mode::Table { width: 120 }
        );
        assert_eq!(
            mode(false, false, false, false, true, None, None),
            Mode::Table { width: 80 }
        );
    }
}
