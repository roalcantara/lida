use lida_core::format::md::parse_markdown;
use lida_core::format::yaml::parse_yaml;
use lida_core::model::Task;
use lida_core::store::{StorageFormat, Store};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_dual_write_multi_mutation_preserves_yaml_metadata_and_comments() {
    let dir = tempdir().unwrap();
    let yaml_path = dir.path().join("TODO.yaml");
    let md_path = dir.path().join("TODO.md");

    // Initialize with both formats
    let mut store = Store::init(dir.path(), StorageFormat::Both, Some("test_proj".to_string())).unwrap();

    // 1. Add a task with comments and meta
    let mut task = Task::new("t001", "Dual-write robustness task");
    task.meta.insert("numeric_id".to_string(), serde_json::json!(5_048_726));
    task.meta
        .insert("string_number".to_string(), serde_json::json!("12345"));
    task.meta.insert("bool_val".to_string(), serde_json::json!(true));
    task.comments.push("c001".to_string());
    store.add_task(task);
    store.doc_mut().comments.push(lida_core::model::Comment {
        id: "c001".to_string(),
        text: "Important comment that must survive multiple mutations".to_string(),
        task_id: "t001".to_string(),
        actors: vec![],
        extra: std::collections::BTreeMap::new(),
    });
    store.save().unwrap();

    // 2. Second mutation: reload and change status
    let mut store2 = Store::load(dir.path()).unwrap();
    let t = store2.find_task_mut("t001").unwrap();
    t.status = lida_core::model::Status::Done;
    store2.save().unwrap();

    // 3. Third mutation: reload and add another task
    let mut store3 = Store::load(dir.path()).unwrap();
    store3.add_task(Task::new("t002", "Second task"));
    store3.save().unwrap();

    // Verify TODO.yaml preserved comments, task comment IDs, and metadata types!
    let yaml_str = fs::read_to_string(&yaml_path).unwrap();
    let doc_from_yaml = parse_yaml(&yaml_str).unwrap();

    assert_eq!(
        doc_from_yaml.comments.len(),
        1,
        "Comments must not be wiped out on subsequent saves!"
    );
    assert_eq!(
        doc_from_yaml.comments[0].text,
        "Important comment that must survive multiple mutations"
    );
    assert_eq!(doc_from_yaml.tasks[0].comments, vec!["c001".to_string()]);
    assert_eq!(
        doc_from_yaml.tasks[0].meta.get("numeric_id").unwrap(),
        &serde_json::json!(5_048_726)
    );
    assert_eq!(
        doc_from_yaml.tasks[0].meta.get("string_number").unwrap(),
        &serde_json::json!("12345")
    );
    assert_eq!(
        doc_from_yaml.tasks[0].meta.get("bool_val").unwrap(),
        &serde_json::json!(true)
    );

    // Verify TODO.md also preserved string types
    let md_str = fs::read_to_string(&md_path).unwrap();
    let doc_from_md = parse_markdown(&md_str).unwrap();
    assert_eq!(
        doc_from_md.tasks[0].meta.get("string_number").unwrap(),
        &serde_json::json!("12345")
    );
}
