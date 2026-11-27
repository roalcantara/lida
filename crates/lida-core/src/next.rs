//! Selecting, ordering, and annotating open tasks for `lida next`.

use crate::date::{Date, parse_day};
use crate::model::{Priority, ProjectDoc, Status, Task};
use crate::query::Query;
use std::cmp::Ordering;
use std::collections::HashMap;

/// Why an open task is waiting, if at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Waiting {
    No,
    Blocked,
    Depends,
}

/// One open task ready to render in `lida next`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Entry<'a> {
    pub task: &'a Task,
    pub urgency: Option<f64>,
    pub waiting: Waiting,
}

/// Returns open tasks matching `query` as of `today`, ordered by urgency, priority, then ID.
#[must_use]
pub fn next<'a>(doc: &'a ProjectDoc, query: &Query, today: Date) -> Vec<Entry<'a>> {
    let statuses: HashMap<&str, Status> = doc.tasks.iter().map(|task| (task.id.as_str(), task.status)).collect();

    let mut entries: Vec<Entry<'a>> = doc
        .tasks
        .iter()
        .filter(|task| is_open(task, today) && query.matches(task))
        .map(|task| Entry {
            task,
            urgency: task_urgency(task),
            waiting: task_waiting(task, &statuses),
        })
        .collect();

    entries.sort_by(compare_entries);
    entries
}

fn is_open(task: &Task, today: Date) -> bool {
    if task.status == Status::Done {
        return false;
    }
    let wait_day = task
        .extra
        .get("wait")
        .and_then(serde_yaml_ng::Value::as_str)
        .and_then(parse_day);
    wait_day.is_none_or(|day| day <= today)
}

fn task_urgency(task: &Task) -> Option<f64> {
    task.meta
        .get("urgency")
        .and_then(serde_json::Value::as_f64)
        .filter(|v| !v.is_nan())
}

fn task_waiting(task: &Task, statuses: &HashMap<&str, Status>) -> Waiting {
    if task.status == Status::Blocked {
        return Waiting::Blocked;
    }
    let has_open_dep = task
        .extra
        .get("depends")
        .and_then(serde_yaml_ng::Value::as_sequence)
        .is_some_and(|deps| {
            deps.iter()
                .filter_map(serde_yaml_ng::Value::as_str)
                .any(|id| statuses.get(id).is_some_and(|status| *status != Status::Done))
        });
    if has_open_dep { Waiting::Depends } else { Waiting::No }
}

fn compare_entries(a: &Entry<'_>, b: &Entry<'_>) -> Ordering {
    compare_urgency(a.urgency, b.urgency)
        .then_with(|| priority_rank(a.task.priority).cmp(&priority_rank(b.task.priority)))
        .then_with(|| a.task.id.cmp(&b.task.id))
}

fn compare_urgency(a: Option<f64>, b: Option<f64>) -> Ordering {
    match (a, b) {
        (Some(lhs), Some(rhs)) => rhs.total_cmp(&lhs),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}

fn priority_rank(priority: Option<Priority>) -> u8 {
    match priority {
        Some(Priority::High) => 0,
        Some(Priority::Medium) => 1,
        Some(Priority::Low) => 2,
        None => 3,
    }
}
