use lida_core::format::yaml::{parse_yaml, serialize_yaml};
use lida_core::model::{Priority, Status};

const SAMPLE_YAML: &str = r#"tasks:
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
"#;

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
    let raw = r#"tasks:
- id: 6ThC
  status: Done
  description: 'Preflight post'
  meta:
    devto.id: 5048726
    linkedin.url: https://example.com
  custom_future_flag: true
"#;
    let doc = parse_yaml(raw).unwrap();
    assert_eq!(doc.tasks[0].meta.get("devto.id").unwrap(), 5048726);
    assert!(doc.tasks[0].extra.contains_key("custom_future_flag"));

    let out = serialize_yaml(&doc).unwrap();
    assert!(out.contains("5048726"));
    assert!(out.contains("custom_future_flag"));
}
