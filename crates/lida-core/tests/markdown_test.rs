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

#[test]
fn project_name_keeps_spaces() {
    let input = "<!-- todo-project: My Cool Project -->\n\n# Tasks\n\n# Actors\n\n# Comments\n\n";
    let doc = parse_markdown(input).unwrap();
    assert_eq!(doc.project, Some("My Cool Project".to_string()));
    let output = serialize_markdown(&doc).unwrap();
    assert_eq!(input, output);
}

#[test]
fn comment_links_round_trip() {
    let input = "<!-- todo-project: test -->\n\n# Tasks\n\n- [ ] aaaa **-** Task 1\n  - **Comments**: c1, c2\n\n# Actors\n\n# Comments\n\n- c1 **-** First comment\n- c2 **-** Second comment\n";
    let doc = parse_markdown(input).unwrap();
    assert_eq!(doc.tasks[0].comments, vec!["c1".to_string(), "c2".to_string()]);
    assert_eq!(doc.comments[0].task_id, "aaaa");
    assert_eq!(doc.comments[1].task_id, "aaaa");

    let serialized = serialize_markdown(&doc).unwrap();
    let reparsed = parse_markdown(&serialized).unwrap();
    assert_eq!(doc, reparsed);
}

#[test]
fn old_md_without_comment_links_round_trips() {
    let input = "<!-- todo-project: test -->\n\n# Tasks\n\n- [ ] aaaa **-** Task 1\n\n# Actors\n\n# Comments\n\n- c1 **-** First comment\n- c2 **-** Second comment\n";
    let doc = parse_markdown(input).unwrap();
    assert_eq!(doc.tasks[0].comments, Vec::<String>::new());
    assert_eq!(doc.comments[0].task_id, "");
    let serialized = serialize_markdown(&doc).unwrap();
    assert!(!serialized.contains("**Comments**:"));
    let reparsed = parse_markdown(&serialized).unwrap();
    assert_eq!(doc, reparsed);
}

#[test]
fn crlf_markdown_is_recognised() {
    let content = include_str!("fixtures/lida_TODO.md");
    let crlf_content = content.replace('\n', "\r\n");
    let normal_doc = parse_markdown(content).unwrap();
    let crlf_doc = parse_markdown(&crlf_content).unwrap();
    assert_eq!(crlf_doc.unrecognised, Vec::<(usize, String)>::new());
    assert_eq!(normal_doc, crlf_doc);
}

#[test]
fn real_markdown_round_trips() {
    let lida_content = include_str!("fixtures/lida_TODO.md");
    let lida_doc = parse_markdown(lida_content).unwrap();
    assert_eq!(lida_doc.unrecognised, Vec::<(usize, String)>::new());
    let lida_serialized = serialize_markdown(&lida_doc).unwrap();
    assert_eq!(lida_content, lida_serialized);
    let lida_reparsed = parse_markdown(&lida_serialized).unwrap();
    assert_eq!(lida_doc, lida_reparsed);

    let toma_content = include_str!("fixtures/toma_TODO.md");
    let toma_doc = parse_markdown(toma_content).unwrap();
    assert_eq!(toma_doc.unrecognised, Vec::<(usize, String)>::new());
    let toma_serialized = serialize_markdown(&toma_doc).unwrap();
    let toma_reparsed = parse_markdown(&toma_serialized).unwrap();
    assert_eq!(toma_doc, toma_reparsed);
}

#[test]
fn invalid_status_and_priority_lines_are_unrecognised() {
    let input =
        "# Tasks\n\n- [ ] aaaa **-** T\n  - **Status**: wip\n  - **Priority**: urgent\n\n# Actors\n\n# Comments\n";
    let doc = parse_markdown(input).unwrap();
    let lines: Vec<usize> = doc.unrecognised.iter().map(|(n, _)| *n).collect();
    assert_eq!(lines, vec![4, 5]);
}
