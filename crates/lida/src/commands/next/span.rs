//! Styled text spans and 24-bit ANSI SGR rendering.

use std::fmt::Write as _;

/// 24-bit RGB colour triple.
pub type Rgb = (u8, u8, u8);

/// Visual attributes for one span of text.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Style {
    pub fg: Option<Rgb>,
    pub bg: Option<Rgb>,
    pub bold: bool,
    pub underline: bool,
}

impl Style {
    fn is_plain(self) -> bool {
        self.fg.is_none() && self.bg.is_none() && !self.bold && !self.underline
    }
}

/// One piece of text with a uniform style.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Span {
    pub text: String,
    pub style: Style,
}

/// Renders a slice of spans using 24-bit ANSI SGR sequences.
#[must_use]
pub fn ansi(line: &[Span]) -> String {
    let mut out = String::new();
    for span in line {
        if span.text.is_empty() {
            continue;
        }
        if span.style.is_plain() {
            out.push_str(&span.text);
            continue;
        }
        out.push_str("\u{1b}[");
        let mut sep = false;
        if span.style.bold {
            out.push('1');
            sep = true;
        }
        if span.style.underline {
            if sep {
                out.push(';');
            }
            out.push('4');
            sep = true;
        }
        if let Some((r, g, b)) = span.style.fg {
            if sep {
                out.push(';');
            }
            let _ = write!(out, "38;2;{r};{g};{b}");
            sep = true;
        }
        if let Some((r, g, b)) = span.style.bg {
            if sep {
                out.push(';');
            }
            let _ = write!(out, "48;2;{r};{g};{b}");
        }
        out.push('m');
        out.push_str(&span.text);
        out.push_str("\u{1b}[0m");
    }
    out
}

/// Concatenates the plain text of all spans without styling.
#[cfg_attr(not(test), expect(dead_code, reason = "helper for span unit tests"))]
#[must_use]
pub fn plain(line: &[Span]) -> String {
    let mut out = String::new();
    for span in line {
        out.push_str(&span.text);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::next::palette::OVERDUE;

    #[test]
    fn ansi_and_plain_render_styled_span() {
        let line = [Span {
            text: "6d".into(),
            style: Style {
                fg: Some(OVERDUE),
                bold: true,
                ..Default::default()
            },
        }];
        assert_eq!(ansi(&line), "\u{1b}[1;38;2;247;118;142m6d\u{1b}[0m");
        assert_eq!(plain(&line), "6d");
    }
}
