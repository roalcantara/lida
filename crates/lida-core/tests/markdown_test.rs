use lida_core::format::md::{parse_markdown, serialize_markdown};
use lida_core::model::{Priority, Status};

const SAMPLE_MD: &str = r"<!-- todo-project: toma -->

# Tasks

- [ ] r001 **-** Memory and ask history store
  - **Priority**: medium
  - **Created**: 2026-11-17 20:35
  - **Tags**: roadmap

- [x] Y5NO **-** Bring docs in line
  - **Priority**: high
  - **Tags**: docs, spec
  - **Meta**:
    - devto.id: 5048726

# Actors

# Comments
";

#[test]
fn test_parse_markdown_basic() {
    let doc = parse_markdown(SAMPLE_MD).unwrap();
    assert_eq!(doc.project, Some("toma".to_string()));
    assert_eq!(doc.tasks.len(), 2);

    let t1 = &doc.tasks[0];
    assert_eq!(t1.id, "r001");
    assert_eq!(t1.description, "Memory and ask history store");
    assert_eq!(t1.status, Status::Todo);
    assert_eq!(t1.priority, Some(Priority::Medium));
    assert_eq!(t1.tags, vec!["roadmap".to_string()]);

    let t2 = &doc.tasks[1];
    assert_eq!(t2.id, "Y5NO");
    assert_eq!(t2.description, "Bring docs in line");
    assert_eq!(t2.status, Status::Done);
    assert_eq!(t2.priority, Some(Priority::High));
    assert_eq!(t2.meta.get("devto.id").unwrap(), 5_048_726);
}

#[test]
fn test_serialize_markdown_roundtrip() {
    let doc = parse_markdown(SAMPLE_MD).unwrap();
    let serialized = serialize_markdown(&doc).unwrap();
    let reparsed = parse_markdown(&serialized).unwrap();
    assert_eq!(doc, reparsed);
}
