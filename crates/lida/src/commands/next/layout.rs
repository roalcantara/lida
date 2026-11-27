//! Column widths, rulers, and narrow-window dropping for `lida next`.

/// One column in the `lida next` table.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Col {
    Id,
    Urg,
    Prio,
    Due,
    Project,
    Tags,
    Why,
    Desc,
}

impl Col {
    /// Column header label.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::Id => "ID",
            Self::Urg => "URG",
            Self::Prio => "P",
            Self::Due => "DUE",
            Self::Project => "PROJECT",
            Self::Tags => "TAGS",
            Self::Why => "WHY",
            Self::Desc => "DESCRIPTION",
        }
    }
}

const ALLEY: usize = 2;
const MIN_LONG_DESC: usize = 30;

/// Computed table layout for a given terminal width.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Layout {
    pub cols: Vec<(Col, usize)>,
    pub desc: usize,
    pub tags_after_desc: bool,
}

/// Computes which columns appear and the description width for `width`.
#[must_use]
pub fn layout(width: usize, long: bool, has_project: bool) -> Layout {
    let mut cols: Vec<(Col, usize)> = Vec::with_capacity(7);
    cols.push((Col::Id, 6));
    if long {
        cols.push((Col::Urg, 5));
    }
    cols.push((Col::Prio, 1));
    cols.push((Col::Due, 6));
    if has_project {
        cols.push((Col::Project, 14));
    }
    if long {
        cols.push((Col::Tags, 17));
        cols.push((Col::Why, 23));
    }

    let mut tags_after_desc = !long;
    if long {
        for drop_col in [Col::Why, Col::Tags, Col::Urg] {
            let used = used_width(&cols);
            if width.saturating_sub(used) >= MIN_LONG_DESC {
                break;
            }
            if let Some(pos) = cols.iter().position(|(c, _)| *c == drop_col) {
                cols.remove(pos);
                if drop_col == Col::Tags {
                    tags_after_desc = true;
                }
            }
        }
    }

    let desc = width.saturating_sub(used_width(&cols)).max(1);
    Layout {
        cols,
        desc,
        tags_after_desc,
    }
}

fn used_width(cols: &[(Col, usize)]) -> usize {
    cols.iter().map(|(_, w)| w + ALLEY).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cols(l: &Layout) -> Vec<Col> {
        l.cols.iter().map(|(c, _)| *c).collect()
    }

    #[test]
    fn default_at_120() {
        let l = layout(120, false, true);
        assert_eq!(cols(&l), [Col::Id, Col::Prio, Col::Due, Col::Project]);
        assert_eq!(l.desc, 85);
        assert!(l.tags_after_desc);
    }

    #[test]
    fn default_at_80() {
        assert_eq!(layout(80, false, true).desc, 45);
    }

    #[test]
    fn long_at_120() {
        let l = layout(120, true, true);
        assert_eq!(
            cols(&l),
            [
                Col::Id,
                Col::Urg,
                Col::Prio,
                Col::Due,
                Col::Project,
                Col::Tags,
                Col::Why,
            ]
        );
        assert!(!l.tags_after_desc);
        assert_eq!(l.desc, 34);
    }

    #[test]
    fn long_at_80_drops_why_then_tags() {
        let l = layout(80, true, true);
        assert_eq!(cols(&l), [Col::Id, Col::Urg, Col::Prio, Col::Due, Col::Project]);
        assert!(l.tags_after_desc);
        assert!(l.desc >= 30);
    }

    #[test]
    fn no_project_column_when_no_task_has_one() {
        assert!(!cols(&layout(120, false, false)).contains(&Col::Project));
    }

    #[test]
    fn tiny_window_never_underflows() {
        assert_eq!(layout(20, true, true).desc, 1);
    }
}
