use lida_core::format::yaml::{parse_yaml, serialize_yaml};
use lida_core::model::{Priority, Status};

const SAMPLE_YAML: &str = r"tasks:
- id: zOaU
  status: Todo
  description: 'Shell UX: switchable theme'
  due: null
  actors: []
  comments: []
  blocked_reason: null
  tags:
  - adopt
  - theme
  priority: High
  created: 2026-11-17T00:53:26.855898
actors: []
comments: []
project: dots
";

#[test]
fn test_parse_and_serialize_yaml_roundtrip() {
    let doc = parse_yaml(SAMPLE_YAML).unwrap();
    assert_eq!(doc.tasks.len(), 1);
    assert_eq!(doc.tasks[0].id, "zOaU");
    assert_eq!(doc.tasks[0].status, Status::Todo);
    assert_eq!(doc.tasks[0].priority, Some(Priority::High));
    assert_eq!(doc.project, Some("dots".to_string()));

    let serialized = serialize_yaml(&doc).unwrap();
    let reparsed = parse_yaml(&serialized).unwrap();
    assert_eq!(doc, reparsed);
}

#[test]
fn test_yaml_preserves_custom_meta_and_unknown_fields() {
    let raw = r"tasks:
- id: 6ThC
  status: Done
  description: 'Preflight post'
  meta:
    devto.id: 5048726
    linkedin.url: https://example.com
  custom_future_flag: true
";
    let doc = parse_yaml(raw).unwrap();
    assert_eq!(doc.tasks[0].meta.get("devto.id").unwrap(), 5_048_726);
    assert!(doc.tasks[0].extra.contains_key("custom_future_flag"));

    let out = serialize_yaml(&doc).unwrap();
    assert!(out.contains("5048726"));
    assert!(out.contains("custom_future_flag"));
}

fn single_task_doc(task: lida_core::model::Task) -> lida_core::model::ProjectDoc {
    lida_core::model::ProjectDoc {
        tasks: vec![task],
        actors: Vec::new(),
        comments: Vec::new(),
        project: None,
        extra: std::collections::BTreeMap::new(),
        unrecognised: Vec::new(),
    }
}

#[test]
fn yaml_omits_empty_task_keys() {
    let doc = single_task_doc(lida_core::model::Task::new("aaaa", "Minimal task"));
    let out = serialize_yaml(&doc).unwrap();
    let val: serde_yaml_ng::Value = serde_yaml_ng::from_str(&out).unwrap();
    let task_map = val["tasks"][0].as_mapping().unwrap();
    assert_eq!(task_map.len(), 3);
    for key in ["id", "status", "description"] {
        assert!(task_map.contains_key(key), "task mapping should contain {key}");
    }
    for key in [
        "due",
        "created",
        "position",
        "blocked_reason",
        "priority",
        "actors",
        "comments",
        "tags",
    ] {
        assert!(!task_map.contains_key(key), "task mapping should omit {key}");
    }
}

#[test]
fn yaml_keeps_null_meta_values() {
    let mut task = lida_core::model::Task::new("aaaa", "Task with null meta");
    task.meta.insert("keep_null".to_string(), serde_json::Value::Null);
    let doc = single_task_doc(task);
    let out = serialize_yaml(&doc).unwrap();
    assert!(out.contains("keep_null: null"));
}

#[test]
fn dots_fixtures_round_trip() {
    for fixture in [
        include_str!("fixtures/dots_root_TODO.yaml"),
        include_str!("fixtures/dots_blog_TODO.yaml"),
    ] {
        let doc = parse_yaml(fixture).unwrap();
        let serialized = serialize_yaml(&doc).unwrap();
        let reparsed = parse_yaml(&serialized).unwrap();
        assert_eq!(doc, reparsed);
    }
}
