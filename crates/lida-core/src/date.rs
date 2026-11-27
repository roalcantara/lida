//! Date parsing and due-date classification for tasks.

pub use jiff::civil::{Date, Weekday};

/// Classification of a task's due date relative to a given day.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Due {
    None,
    Overdue { days: i64 },
    Today,
    Soon { weekday: Weekday },
    Later { days: i64 },
    Far { date: Date, same_year: bool },
}

/// Parses a `YYYY-MM-DD` or `YYYY-MM-DD HH:MM` string into a civil date.
#[must_use]
pub fn parse_day(value: &str) -> Option<Date> {
    let trimmed = value.trim();
    if let Ok(day) = trimmed.parse::<Date>() {
        return Some(day);
    }
    let (day_part, time_part) = trimmed.split_once(' ')?;
    let day = day_part.parse::<Date>().ok()?;
    time_part.trim().parse::<jiff::civil::Time>().ok()?;
    Some(day)
}

/// Classifies an optional due-date string against `today`.
#[must_use]
pub fn classify(due: Option<&str>, today: Date) -> Due {
    let Some(due_date) = due.and_then(parse_day) else {
        return Due::None;
    };
    let days = i64::from((due_date - today).get_days());
    match days {
        ..0 => Due::Overdue { days: -days },
        0 => Due::Today,
        1..=6 => Due::Soon {
            weekday: due_date.weekday(),
        },
        7..=59 => Due::Later { days },
        _ => Due::Far {
            date: due_date,
            same_year: due_date.year() == today.year(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::civil::date;

    #[test]
    fn parses_day_and_day_with_time() {
        assert_eq!(parse_day("2026-11-24"), Some(date(2026, 11, 24)));
        assert_eq!(parse_day("2026-11-24 09:00"), Some(date(2026, 11, 24)));
        assert_eq!(parse_day("  2026-11-24 09:00  "), Some(date(2026, 11, 24)));
        assert_eq!(parse_day("2026-11-24 garbage"), None);
        assert_eq!(parse_day("tomorrow"), None);
    }

    #[test]
    fn classifies_every_due_class() {
        let t = date(2026, 12, 3);
        assert_eq!(classify(None, t), Due::None);
        assert_eq!(classify(Some("garbage"), t), Due::None);
        assert_eq!(classify(Some("2026-11-27"), t), Due::Overdue { days: 6 });
        assert_eq!(classify(Some("2026-12-03 09:00"), t), Due::Today);
        assert_eq!(
            classify(Some("2026-12-04"), t),
            Due::Soon {
                weekday: Weekday::Friday
            }
        );
        assert_eq!(
            classify(Some("2026-12-09"), t),
            Due::Soon {
                weekday: Weekday::Wednesday
            }
        );
        assert_eq!(classify(Some("2026-12-10"), t), Due::Later { days: 7 });
        assert_eq!(classify(Some("2027-01-31"), t), Due::Later { days: 59 });
        assert_eq!(
            classify(Some("2027-02-01"), t),
            Due::Far {
                date: date(2027, 2, 1),
                same_year: false
            }
        );
        assert_eq!(
            classify(Some("2026-12-03"), date(2026, 1, 1)),
            Due::Far {
                date: date(2026, 12, 3),
                same_year: true
            }
        );
    }
}
