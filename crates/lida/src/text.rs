//! Terminal-safe text sanitising and Unicode cell-width fitting.

use std::fmt::Write as _;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// Replaces CR, LF, and TAB with spaces and escapes all other control characters.
#[must_use]
pub fn sanitise(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '\n' | '\r' | '\t' => out.push(' '),
            c if c.is_control() => {
                let _ = write!(out, "\\x{:02x}", u32::from(c));
            }
            c => out.push(c),
        }
    }
    out
}

/// Returns the display width of `s` in terminal cells.
#[must_use]
pub fn width(s: &str) -> usize {
    UnicodeWidthStr::width(s)
}

/// Truncates `s` on grapheme boundaries to at most `cells` terminal cells, adding `…` if cut.
#[must_use]
pub fn cut(s: &str, cells: usize) -> String {
    if cells == 0 {
        return String::new();
    }
    if width(s) <= cells {
        return s.to_owned();
    }
    let budget = cells - 1;
    let mut out = String::new();
    let mut used = 0;
    for grapheme in s.graphemes(true) {
        let w = width(grapheme);
        if used + w > budget {
            break;
        }
        out.push_str(grapheme);
        used += w;
    }
    out.push('…');
    out
}

/// Fits `s` into exactly `cells` terminal cells, cutting with `…` or padding with spaces.
#[must_use]
pub fn fit(s: &str, cells: usize) -> String {
    if cells == 0 {
        return String::new();
    }
    let mut out = cut(s, cells);
    let w = width(&out);
    if w < cells {
        out.extend(std::iter::repeat_n(' ', cells - w));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitise_keeps_one_line_and_no_controls() {
        let s = sanitise("a\nb\r\tc\u{1b}[31md\u{9b}e");
        assert!(!s.contains('\n') && !s.contains('\r') && !s.chars().any(char::is_control));
        assert!(s.starts_with("a b  c"));
    }

    #[test]
    fn width_counts_cells() {
        assert_eq!(width("日本語"), 6);
        assert_eq!(width("🚀"), 2);
        assert_eq!(width("re\u{301}sume\u{301}"), 6);
    }

    #[test]
    fn fit_pads_and_cuts_on_graphemes() {
        assert_eq!(fit("abc", 5), "abc  ");
        assert_eq!(fit("abcdef", 4), "abc…");
        assert_eq!(fit("日本語", 4), "日… ");
        assert_eq!(width(&fit("re\u{301}sume\u{301}x", 6)), 6);
        assert_eq!(fit("abc", 0), "");
    }

    #[test]
    fn cut_truncates_without_padding() {
        assert_eq!(cut("abc", 5), "abc");
        assert_eq!(cut("abcdef", 4), "abc…");
        assert_eq!(cut("日本語", 4), "日…");
        assert_eq!(cut("abc", 1), "…");
        assert_eq!(cut("abc", 0), "");
    }
}
