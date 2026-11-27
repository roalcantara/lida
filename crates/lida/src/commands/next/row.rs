//! Header, styled table rows, and plain `--raw` rows for `lida next`.

use super::layout::{Col, Layout};
use super::palette::{
    CHIP, CHIP_LEFT, CHIP_RIGHT, CHIP_TEXT, HEADER_BG, ID, LATER, MUTED, OVERDUE, PRIORITY_HIGH, PRIORITY_LOW,
    PRIORITY_MEDIUM, SOON, TAGS, TEXT, dimmed, project_colour,
};
use super::span::{Span, Style};
use crate::text::{cut, fit, sanitise, width};
use lida_core::date::{Date, Due, Weekday, classify};
use lida_core::model::{Priority, Status, Task};
use lida_core::next::{Entry, Waiting};
use lida_core::query::{Query, task_project};
use std::fmt::Write as _;
use std::ops::Range;
use unicode_segmentation::UnicodeSegmentation;

/// Builds the full-width header band spans for `layout`.
#[must_use]
pub fn header_row(layout: &Layout) -> Vec<Span> {
    let style = Style {
        fg: Some(MUTED),
        bg: Some(HEADER_BG),
        bold: true,
        underline: false,
    };
    let mut line = String::new();
    for &(col, w) in &layout.cols {
        match col {
            Col::Id => {
                let _ = write!(line, " {}   ", Col::Id.label());
            }
            Col::Urg => {
                let _ = write!(line, "{:>w$}", Col::Urg.label());
            }
            _ => line.push_str(&fit(col.label(), w)),
        }
        line.push_str("  ");
    }
    line.push_str(&fit(Col::Desc.label(), layout.desc));
    vec![Span { text: line, style }]
}

/// Builds the styled spans for one task row in table mode.
#[must_use]
pub fn table_row(entry: &Entry<'_>, query: &Query, layout: &Layout, today: Date) -> Vec<Span> {
    let mut spans = Vec::new();
    for &(col, w) in &layout.cols {
        match col {
            Col::Id => push_id(&mut spans, entry),
            Col::Urg => push_urg(&mut spans, entry.urgency, w),
            Col::Prio => push_prio(&mut spans, entry.task.priority),
            Col::Due => push_due(&mut spans, entry.task.due.as_deref(), today, w),
            Col::Project => push_project(&mut spans, task_project(entry.task), w),
            Col::Tags => push_tags_col(&mut spans, &entry.task.tags, w),
            Col::Why => push_why(&mut spans, entry.task, w),
            Col::Desc => {}
        }
        push_plain(&mut spans, "  ");
    }
    push_desc(&mut spans, entry, query, layout);
    spans
}

/// Formats one task as a tab-separated plain `--raw` line.
#[must_use]
pub fn raw_row(entry: &Entry<'_>, long: bool) -> String {
    let id = sanitise(&entry.task.id);
    let prio = priority_letter(entry.task.priority);
    let due = entry.task.due.as_deref().map(sanitise).unwrap_or_default();
    let project = task_project(entry.task)
        .map(|p| format!("@{}", sanitise(p)))
        .unwrap_or_default();
    let tags = format_tags(&entry.task.tags);
    let state = state_prefix(entry);
    let desc = sanitise(&entry.task.description);

    if long {
        let urg = entry.urgency.map(|u| format!("{u:.1}")).unwrap_or_default();
        let why = ordered_purposes(entry.task).1.join(" ");
        format!("{id}\t{urg}\t{prio}\t{due}\t{project}\t{tags}\t{why}\t{state}{desc}")
    } else if tags.is_empty() {
        format!("{id}\t{prio}\t{due}\t{project}\t{state}{desc}")
    } else {
        format!("{id}\t{prio}\t{due}\t{project}\t{state}{desc} {tags}")
    }
}

fn push_plain(spans: &mut Vec<Span>, text: impl Into<String>) {
    let text = text.into();
    if !text.is_empty() {
        spans.push(Span {
            text,
            style: Style::default(),
        });
    }
}

fn push_id(spans: &mut Vec<Span>, entry: &Entry<'_>) {
    let id = fit(&sanitise(&entry.task.id), 4);
    if entry.task.status == Status::InProgress {
        let edge = Style {
            fg: Some(CHIP),
            ..Style::default()
        };
        spans.push(Span {
            text: CHIP_LEFT.to_string(),
            style: edge,
        });
        spans.push(Span {
            text: id,
            style: Style {
                fg: Some(CHIP_TEXT),
                bg: Some(CHIP),
                bold: true,
                underline: false,
            },
        });
        spans.push(Span {
            text: CHIP_RIGHT.to_string(),
            style: edge,
        });
    } else {
        spans.push(Span {
            text: format!(" {id} "),
            style: Style {
                fg: Some(ID),
                bold: true,
                ..Style::default()
            },
        });
    }
}

fn push_urg(spans: &mut Vec<Span>, urgency: Option<f64>, w: usize) {
    let (raw, fg) = match urgency {
        Some(u) => (format!("{u:.1}"), if u < 0.0 { MUTED } else { TEXT }),
        None => ("-".to_owned(), MUTED),
    };
    let text = if width(&raw) > w {
        cut(&raw, w)
    } else {
        format!("{raw:>w$}")
    };
    spans.push(Span {
        text,
        style: Style {
            fg: Some(fg),
            ..Style::default()
        },
    });
}

fn push_prio(spans: &mut Vec<Span>, priority: Option<Priority>) {
    let fg = match priority {
        Some(Priority::High) => Some(PRIORITY_HIGH),
        Some(Priority::Medium) => Some(PRIORITY_MEDIUM),
        Some(Priority::Low) => Some(PRIORITY_LOW),
        None => None,
    };
    if let Some(colour) = fg {
        spans.push(Span {
            text: priority_letter(priority).to_owned(),
            style: Style {
                fg: Some(colour),
                bold: true,
                ..Style::default()
            },
        });
    } else {
        push_plain(spans, " ");
    }
}

fn push_due(spans: &mut Vec<Span>, due: Option<&str>, today: Date, w: usize) {
    let (label, style) = match classify(due, today) {
        Due::None => (
            "-".to_owned(),
            Style {
                fg: Some(MUTED),
                ..Style::default()
            },
        ),
        Due::Overdue { days } => (
            format!("{days}d"),
            Style {
                fg: Some(OVERDUE),
                bold: true,
                ..Style::default()
            },
        ),
        Due::Today => (
            "today".to_owned(),
            Style {
                fg: Some(SOON),
                bold: true,
                ..Style::default()
            },
        ),
        Due::Soon { weekday } => (
            weekday_short(weekday).to_owned(),
            Style {
                fg: Some(SOON),
                bold: true,
                ..Style::default()
            },
        ),
        Due::Later { days } => (
            format!("{days}d"),
            Style {
                fg: Some(LATER),
                ..Style::default()
            },
        ),
        Due::Far { date, within_year } => {
            let month = month_short(date.month());
            let s = if within_year {
                format!("{month} {}", date.day())
            } else {
                format!("{month}'{:02}", date.year().rem_euclid(100))
            };
            (
                s,
                Style {
                    fg: Some(LATER),
                    ..Style::default()
                },
            )
        }
    };
    spans.push(Span {
        text: fit(&label, w),
        style,
    });
}

fn push_project(spans: &mut Vec<Span>, project: Option<&str>, w: usize) {
    let Some(raw) = project else {
        push_plain(spans, " ".repeat(w));
        return;
    };
    let clean = sanitise(raw);
    let top = clean.split('.').next().unwrap_or(&clean);
    let colour = project_colour(top);
    let full = format!("@{clean}");
    let cut_text = cut(&full, w);
    let pad = w.saturating_sub(width(&cut_text));

    if let Some(dot) = cut_text.find('.') {
        spans.push(Span {
            text: cut_text[..dot].to_owned(),
            style: Style {
                fg: Some(colour),
                bold: true,
                ..Style::default()
            },
        });
        spans.push(Span {
            text: cut_text[dot..].to_owned(),
            style: Style {
                fg: Some(dimmed(colour)),
                ..Style::default()
            },
        });
    } else {
        spans.push(Span {
            text: cut_text,
            style: Style {
                fg: Some(colour),
                bold: true,
                ..Style::default()
            },
        });
    }
    if pad > 0 {
        push_plain(spans, " ".repeat(pad));
    }
}

fn push_tags_col(spans: &mut Vec<Span>, tags: &[String], w: usize) {
    if tags.is_empty() {
        push_plain(spans, " ".repeat(w));
        return;
    }
    let text = fit(&format_tags(tags), w);
    spans.push(Span {
        text,
        style: Style {
            fg: Some(TAGS),
            ..Style::default()
        },
    });
}

fn push_why(spans: &mut Vec<Span>, task: &Task, w: usize) {
    let (lead, words) = ordered_purposes(task);
    if words.is_empty() {
        push_plain(spans, " ".repeat(w));
        return;
    }
    let joined = words.join(" ");
    let cut_text = cut(&joined, w);
    let pad = w.saturating_sub(width(&cut_text));
    let lead_style = Style {
        fg: Some(TEXT),
        bold: true,
        ..Style::default()
    };
    let rest_style = Style {
        fg: Some(MUTED),
        ..Style::default()
    };

    if let Some(lead_word) = lead {
        if cut_text.starts_with(&lead_word)
            && (cut_text.len() == lead_word.len() || cut_text.as_bytes().get(lead_word.len()) == Some(&b' '))
        {
            spans.push(Span {
                text: cut_text[..lead_word.len()].to_owned(),
                style: lead_style,
            });
            if cut_text.len() > lead_word.len() {
                spans.push(Span {
                    text: cut_text[lead_word.len()..].to_owned(),
                    style: rest_style,
                });
            }
        } else {
            spans.push(Span {
                text: cut_text,
                style: lead_style,
            });
        }
    } else {
        spans.push(Span {
            text: cut_text,
            style: rest_style,
        });
    }
    if pad > 0 {
        push_plain(spans, " ".repeat(pad));
    }
}

fn push_desc(spans: &mut Vec<Span>, entry: &Entry<'_>, query: &Query, layout: &Layout) {
    let room = layout.desc;
    let desc = sanitise(&entry.task.description);
    let base = Style {
        fg: Some(if entry.waiting == Waiting::No { TEXT } else { MUTED }),
        ..Style::default()
    };
    let matches = query.search_spans(&desc);

    if layout.tags_after_desc && !entry.task.tags.is_empty() {
        let tags_str = format_tags(&entry.task.tags);
        let tags_w = width(&tags_str);
        let tags_style = Style {
            fg: Some(TAGS),
            bold: true,
            ..Style::default()
        };
        if tags_w + 2 <= room {
            let desc_budget = room - tags_w - 2;
            let (desc_spans, desc_w) = if desc_budget > 0 {
                cut_with_spans(&desc, desc_budget, &matches, base)
            } else {
                (Vec::new(), 0)
            };
            spans.extend(desc_spans);
            if desc_w > 0 {
                push_plain(spans, "  ");
            }
            spans.push(Span {
                text: tags_str,
                style: tags_style,
            });
            let used = if desc_w > 0 { desc_w + 2 + tags_w } else { tags_w };
            let pad = room.saturating_sub(used);
            if pad > 0 {
                push_plain(spans, " ".repeat(pad));
            }
            return;
        }
        let tags_cut = cut(&tags_str, room);
        let pad = room.saturating_sub(width(&tags_cut));
        spans.push(Span {
            text: tags_cut,
            style: tags_style,
        });
        if pad > 0 {
            push_plain(spans, " ".repeat(pad));
        }
        return;
    }

    let (desc_spans, used) = cut_with_spans(&desc, room, &matches, base);
    spans.extend(desc_spans);
    let pad = room.saturating_sub(used);
    if pad > 0 {
        push_plain(spans, " ".repeat(pad));
    }
}

fn cut_with_spans(s: &str, cells: usize, highlights: &[Range<usize>], base: Style) -> (Vec<Span>, usize) {
    if cells == 0 {
        return (Vec::new(), 0);
    }
    let total_w = width(s);
    let (prefix_bytes, used_w, ellipsis) = if total_w <= cells {
        (s.len(), total_w, false)
    } else {
        let budget = cells - 1;
        let mut bytes = 0;
        let mut used = 0;
        for grapheme in s.graphemes(true) {
            let w = width(grapheme);
            if used + w > budget {
                break;
            }
            bytes += grapheme.len();
            used += w;
        }
        (bytes, used + 1, true)
    };

    let mut out = Vec::new();
    let mut cursor = 0;
    let underline_style = Style {
        underline: true,
        ..base
    };
    for range in highlights {
        if range.start >= prefix_bytes {
            break;
        }
        let start = range.start.max(cursor);
        let end = range.end.min(prefix_bytes);
        if start > cursor {
            out.push(Span {
                text: s[cursor..start].to_owned(),
                style: base,
            });
        }
        if end > start {
            out.push(Span {
                text: s[start..end].to_owned(),
                style: underline_style,
            });
            cursor = end;
        }
    }
    if cursor < prefix_bytes {
        out.push(Span {
            text: s[cursor..prefix_bytes].to_owned(),
            style: base,
        });
    }
    if ellipsis {
        out.push(Span {
            text: "…".to_owned(),
            style: base,
        });
    }
    (out, used_w)
}

fn format_tags(tags: &[String]) -> String {
    tags.iter()
        .map(|t| format!("+{}", sanitise(t)))
        .collect::<Vec<_>>()
        .join(" ")
}

fn ordered_purposes(task: &Task) -> (Option<String>, Vec<String>) {
    let lead = task
        .meta
        .get("lead")
        .and_then(serde_json::Value::as_str)
        .map(sanitise)
        .filter(|s| !s.is_empty());

    let mut words = Vec::new();
    if let Some(ref l) = lead {
        words.push(l.clone());
    }
    if let Some(items) = task.meta.get("purposes").and_then(serde_json::Value::as_array) {
        for item in items {
            if let Some(s) = item.as_str() {
                let clean = sanitise(s);
                if !clean.is_empty() && !words.contains(&clean) {
                    words.push(clean);
                }
            }
        }
    }
    (lead, words)
}

fn priority_letter(priority: Option<Priority>) -> &'static str {
    match priority {
        Some(Priority::High) => "H",
        Some(Priority::Medium) => "M",
        Some(Priority::Low) => "L",
        None => "",
    }
}

fn state_prefix(entry: &Entry<'_>) -> &'static str {
    if entry.task.status == Status::InProgress {
        "(doing) "
    } else if entry.waiting == Waiting::Blocked {
        "(blocked) "
    } else if entry.waiting == Waiting::Depends {
        "(waiting) "
    } else {
        ""
    }
}

fn weekday_short(weekday: Weekday) -> &'static str {
    match weekday {
        Weekday::Monday => "Mon",
        Weekday::Tuesday => "Tue",
        Weekday::Wednesday => "Wed",
        Weekday::Thursday => "Thu",
        Weekday::Friday => "Fri",
        Weekday::Saturday => "Sat",
        Weekday::Sunday => "Sun",
    }
}

fn month_short(month: i8) -> &'static str {
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        _ => "Dec",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::next::layout::layout;

    fn render_row(task: &Task, urgency: Option<f64>, waiting: Waiting) -> Vec<Span> {
        let entry = Entry { task, urgency, waiting };
        let lay = layout(120, false, false);
        table_row(&entry, &Query::default(), &lay, Date::constant(2026, 12, 3))
    }

    #[test]
    fn chip_and_priority_spans() {
        let mut task = Task::new("bbbb", "Series: account-wide numbers");
        task.status = Status::InProgress;
        task.priority = Some(Priority::Low);
        let spans = render_row(&task, Some(5.8), Waiting::No);

        assert!(spans.iter().any(|s| s.text == "bbbb" && s.style.bg == Some(CHIP)));

        for (priority, letter, colour) in [
            (Priority::High, "H", PRIORITY_HIGH),
            (Priority::Medium, "M", PRIORITY_MEDIUM),
            (Priority::Low, "L", PRIORITY_LOW),
        ] {
            let mut t = Task::new("aaaa", "Check priority");
            t.priority = Some(priority);
            let row = render_row(&t, Some(1.0), Waiting::No);
            assert!(
                row.iter()
                    .any(|s| s.text == letter && s.style.fg == Some(colour) && s.style.bold),
                "missing bold {letter} span for {priority:?}"
            );
            assert!(
                row.iter()
                    .all(|s| !s.text.chars().any(|c| ('\u{F0000}'..='\u{FFFFF}').contains(&c))),
                "unexpected Nerd Font PUA glyph in {row:?}"
            );
        }

        let none_task = Task::new("cccc", "No priority");
        let none_row = render_row(&none_task, Some(1.0), Waiting::No);
        assert_eq!(none_row[2].text, " ");
        assert_eq!(none_row[2].style, Style::default());
        assert!(
            none_row
                .iter()
                .all(|s| !s.text.chars().any(|c| ('\u{F0000}'..='\u{FFFFF}').contains(&c)))
        );
    }

    #[test]
    fn waiting_is_muted() {
        let task = Task::new("cccc", "Brainstorm strategy rev 5");
        let spans = render_row(&task, Some(18.0), Waiting::Depends);

        assert!(
            spans
                .iter()
                .any(|s| s.text.contains("Brainstorm") && s.style.fg == Some(MUTED))
        );
    }

    #[test]
    fn far_due_formats_same_and_other_year() {
        let mut within = Vec::new();
        push_due(&mut within, Some("2027-02-20"), Date::constant(2026, 12, 3), 6);
        assert_eq!(within[0].text, "Feb 20");
        assert_eq!(within[0].style.fg, Some(LATER));

        let mut beyond = Vec::new();
        push_due(&mut beyond, Some("2028-01-15"), Date::constant(2026, 12, 3), 6);
        assert_eq!(beyond[0].text, "Jan'28");
        assert_eq!(beyond[0].style.fg, Some(LATER));
    }
}
