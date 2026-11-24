use lida_core::model::{Status, Task};
use lida_core::store::{StorageFormat, Store};
use std::fs;
use std::path::Path;
use tempfile::tempdir;

/// Loads the store in `dir`, adds one task, saves, and asserts `path` holds its ID.
fn assert_roundtrip(dir: &Path, path: &Path, format: StorageFormat) -> Result<(), Box<dyn std::error::Error>> {
    let mut store = Store::load(dir)?;
    assert_eq!(store.format(), format);
    assert_eq!(store.doc().tasks.len(), 1);

    store.add_task(Task::new("c3d4", "Task 2"));
    store.save()?;

    let content = fs::read_to_string(path)?;
    assert!(content.contains("c3d4"));
    Ok(())
}

#[test]
fn test_store_single_yaml() {
    let dir = tempdir().unwrap();
    let yaml_path = dir.path().join("TODO.yaml");
    fs::write(
        &yaml_path,
        "tasks:\n- id: a1b2\n  status: Todo\n  description: Task 1\n",
    )
    .unwrap();

    assert_roundtrip(dir.path(), &yaml_path, StorageFormat::YamlOnly).unwrap();
    assert!(!dir.path().join("TODO.md").exists());
}

#[test]
fn test_store_single_markdown() {
    let dir = tempdir().unwrap();
    let md_path = dir.path().join("TODO.md");
    fs::write(&md_path, "# Tasks\n\n- [ ] a1b2 **-** Task 1\n").unwrap();

    assert_roundtrip(dir.path(), &md_path, StorageFormat::MdOnly).unwrap();
    assert!(!dir.path().join("TODO.yaml").exists());
}

#[test]
fn test_store_dual_write_synchronization() {
    let dir = tempdir().unwrap();
    let yaml_path = dir.path().join("TODO.yaml");
    let md_path = dir.path().join("TODO.md");

    fs::write(
        &yaml_path,
        "tasks:\n- id: a1b2\n  status: Todo\n  description: Task 1\n",
    )
    .unwrap();
    fs::write(&md_path, "# Tasks\n\n- [ ] a1b2 **-** Task 1\n").unwrap();

    let mut store = Store::load(dir.path()).unwrap();
    assert_eq!(store.format(), StorageFormat::Both);

    let mut new_task = Task::new("e5f6", "Task Dual");
    new_task.status = Status::Done;
    store.add_task(new_task);
    store.save().unwrap();

    // Verify both files updated
    let yaml_content = fs::read_to_string(&yaml_path).unwrap();
    let md_content = fs::read_to_string(&md_path).unwrap();

    assert!(yaml_content.contains("e5f6"));
    assert!(yaml_content.contains("Task Dual"));

    assert!(md_content.contains("- [x] e5f6 **-** Task Dual"));
}

fn assert_files_exist(dir: &Path, yaml: bool, md: bool) {
    assert_eq!(dir.join("TODO.yaml").exists(), yaml);
    assert_eq!(dir.join("TODO.md").exists(), md);
}

#[test]
fn cwi_to_current_format_deletes_nothing() {
    let dir = tempdir().unwrap();
    let mut store = Store::init(dir.path(), StorageFormat::Both, None).unwrap();
    assert_files_exist(dir.path(), true, true);
    store.switch_format(StorageFormat::Both).unwrap();
    assert_files_exist(dir.path(), true, true);
}

#[test]
fn cwi_both_keeps_both_files() {
    let dir = tempdir().unwrap();
    let mut store = Store::init(dir.path(), StorageFormat::MdOnly, None).unwrap();
    assert_files_exist(dir.path(), false, true);
    store.switch_format(StorageFormat::Both).unwrap();
    assert_files_exist(dir.path(), true, true);
}

#[test]
fn duplicate_ids_refuse_to_load() {
    let dir = tempdir().unwrap();
    let md_path = dir.path().join("TODO.md");
    fs::write(&md_path, "# Tasks\n\n- [ ] aaaa **-** One\n- [ ] aaaa **-** Two\n").unwrap();
    let err = Store::load(dir.path()).unwrap_err();
    match err {
        lida_core::store::StoreError::DuplicateId { id, file } => {
            assert_eq!(id, "aaaa");
            assert_eq!(file, "TODO.md");
        }
        other => panic!("expected DuplicateId, got {other:?}"),
    }
}

#[test]
fn todo_config_is_read_and_written() {
    let dir = tempdir().unwrap();
    let todo_dir = dir.path().join(".todo");
    fs::create_dir_all(&todo_dir).unwrap();
    fs::write(todo_dir.join("config"), "yaml\n").unwrap();
    fs::write(
        dir.path().join("TODO.yaml"),
        "tasks:\n- id: aaaa\n  status: Todo\n  description: One\n",
    )
    .unwrap();

    let mut store = Store::load(dir.path()).unwrap();
    assert_eq!(store.format(), StorageFormat::YamlOnly);
    store.switch_format(StorageFormat::MdOnly).unwrap();

    assert_eq!(fs::read_to_string(todo_dir.join("config")).unwrap().trim(), "md");
    assert!(!dir.path().join(".lida").join("config").exists());
}

#[test]
fn unrecognised_lines_are_kept_and_block_saving() {
    let dir = tempdir().unwrap();
    let md_path = dir.path().join("TODO.md");
    let input = "# Tasks\n\n- [ ] aaaa **-** Task 1\n  Some stray note\n\n# Actors\n\n# Comments\n";
    fs::write(&md_path, input).unwrap();

    let store = Store::load(dir.path()).unwrap();
    assert_eq!(store.doc().unrecognised.len(), 1);
    assert_eq!(store.doc().unrecognised[0].0, 4);
    assert_eq!(store.doc().unrecognised[0].1, "  Some stray note");

    let err = store.save().unwrap_err();
    match err {
        lida_core::store::StoreError::UnrecognisedMarkdown { line, text } => {
            assert_eq!(line, 4);
            assert_eq!(text, "  Some stray note");
        }
        other => panic!("expected UnrecognisedMarkdown, got {other:?}"),
    }
    assert_eq!(fs::read_to_string(&md_path).unwrap(), input);
}

#[test]
fn cwi_md_refuses_what_markdown_cannot_hold() {
    let dir = tempdir().unwrap();
    let blog_yaml = include_str!("fixtures/dots_blog_TODO.yaml");
    fs::write(dir.path().join("TODO.yaml"), blog_yaml).unwrap();

    let mut store = Store::load(dir.path()).unwrap();
    let err = store.switch_format(StorageFormat::MdOnly).unwrap_err();
    match err {
        lida_core::store::StoreError::Unrepresentable { what } => {
            assert!(what.contains("actor types"));
            assert!(what.contains("stay on yaml or both"));
        }
        other => panic!("expected Unrepresentable, got {other:?}"),
    }
    assert!(dir.path().join("TODO.yaml").exists());
    assert!(!dir.path().join("TODO.md").exists());
}

#[test]
fn cwi_yaml_refuses_unrecognised_markdown() {
    let dir = tempdir().unwrap();
    let input = "# Tasks\n\n- [ ] aaaa **-** Task 1\n  Stray text\n\n# Actors\n\n# Comments\n";
    fs::write(dir.path().join("TODO.md"), input).unwrap();

    let mut store = Store::load(dir.path()).unwrap();
    let err = store.switch_format(StorageFormat::YamlOnly).unwrap_err();
    match err {
        lida_core::store::StoreError::UnrecognisedMarkdown { line, text } => {
            assert_eq!(line, 4);
            assert_eq!(text, "  Stray text");
        }
        other => panic!("expected UnrecognisedMarkdown, got {other:?}"),
    }
    assert!(dir.path().join("TODO.md").exists());
    assert!(!dir.path().join("TODO.yaml").exists());
}
