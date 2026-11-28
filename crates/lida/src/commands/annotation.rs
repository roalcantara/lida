//! `-a`/`--annotate`: appends a dated note to a task's `annotations` list in `extra`.

use super::not_found;
use lida_core::date::Date;
use lida_core::model::Task;
use lida_core::store::{StorageFormat, Store};
use serde_yaml_ng::{Mapping, Value};

/// Refuses an annotation the store cannot hold, in the global error order: not found, Markdown-only, not a list.
///
/// `_text` mirrors [`append`]'s signature; only the note's shape is checked by the caller, before the store loads.
pub(super) fn check(store: &Store, id: &str, _text: &str) -> Result<(), String> {
    let task = store
        .doc()
        .tasks
        .iter()
        .find(|task| task.id == id)
        .ok_or_else(|| not_found(id))?;
    if store.format() == StorageFormat::MdOnly {
        return Err(md_refusal(id));
    }
    if task
        .extra
        .get("annotations")
        .is_some_and(|value| !matches!(value, Value::Sequence(_)))
    {
        return Err(not_a_list(id));
    }
    Ok(())
}

/// Appends `{entry, description}` last in `extra["annotations"]`, creating the list when absent.
///
/// Existing entries are untouched; a non-list value is refused without rewriting `extra`.
pub(super) fn append(task: &mut Task, text: &str, today: Date) -> Result<(), String> {
    let mut entry = Mapping::new();
    entry.insert(Value::String("entry".to_string()), Value::String(today.to_string()));
    entry.insert(
        Value::String("description".to_string()),
        Value::String(text.to_string()),
    );
    let entry = Value::Mapping(entry);
    match task.extra.get_mut("annotations") {
        None => {
            task.extra
                .insert("annotations".to_string(), Value::Sequence(vec![entry]));
            Ok(())
        }
        Some(Value::Sequence(entries)) => {
            entries.push(entry);
            Ok(())
        }
        Some(_) => Err(not_a_list(&task.id)),
    }
}

pub(super) fn md_refusal(id: &str) -> String {
    format!("TODO.md cannot hold annotations (task #{id}): stay on yaml or both")
}

fn not_a_list(id: &str) -> String {
    format!("annotations of task #{id} is not a list")
}

#[cfg(test)]
mod tests {
    use super::append;
    use jiff::civil::date;
    use lida_core::model::Task;
    use serde_yaml_ng::Value;

    #[test]
    fn appends_to_absent() {
        let mut task = Task::new("aaaa", "x");
        append(&mut task, "note", date(2026, 11, 28)).expect("append");
        let expected: Value =
            serde_yaml_ng::from_str("- entry: 2026-11-28\n  description: note\n").expect("expected yaml");
        assert_eq!(task.extra["annotations"], expected);
    }

    #[test]
    fn keeps_existing_and_appends_last() {
        let mut task = Task::new("aaaa", "x");
        let original: Value =
            serde_yaml_ng::from_str("- entry: 2026-11-25\n  description: old\n  foo: bar\n").expect("original yaml");
        task.extra.insert("annotations".to_string(), original.clone());
        let first_before = original.as_sequence().expect("sequence")[0].clone();

        append(&mut task, "new", date(2026, 11, 28)).expect("append");

        let entries = task.extra["annotations"].as_sequence().expect("sequence");
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0], first_before);
        let expected: Value =
            serde_yaml_ng::from_str("- entry: 2026-11-28\n  description: new\n").expect("expected yaml");
        assert_eq!(entries[1], expected.as_sequence().expect("sequence")[0]);
    }

    #[test]
    fn refuses_non_list() {
        let mut task = Task::new("aaaa", "x");
        task.extra
            .insert("annotations".to_string(), Value::String("nope".to_string()));
        let before = task.extra.clone();
        assert_eq!(
            append(&mut task, "note", date(2026, 11, 28)),
            Err("annotations of task #aaaa is not a list".to_string())
        );
        assert_eq!(task.extra, before);
    }
}
