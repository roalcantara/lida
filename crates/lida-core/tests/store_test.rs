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
