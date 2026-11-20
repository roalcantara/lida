use lida_core::id::generate_id;
use lida_core::model::{Priority, Status, Task};
use std::collections::HashSet;

#[test]
fn test_generate_id_is_four_chars_alphanumeric() {
    let existing = HashSet::new();
    let id = generate_id(&existing);
    assert_eq!(id.len(), 4);
    assert!(id.chars().all(|c| c.is_ascii_alphanumeric()));
}

#[test]
fn test_generate_id_avoids_collisions() {
    let mut existing = HashSet::new();
    for _ in 0..100 {
        let id = generate_id(&existing);
        assert!(!existing.contains(&id));
        existing.insert(id);
    }
}

#[test]
fn test_status_deserialize_english_and_french() {
    let todo: Status = serde_json::from_str("\"Todo\"").unwrap();
    assert_eq!(todo, Status::Todo);

    let in_prog: Status = serde_json::from_str("\"InProgress\"").unwrap();
    assert_eq!(in_prog, Status::InProgress);

    let en_cours: Status = serde_json::from_str("\"en-cours\"").unwrap();
    assert_eq!(en_cours, Status::InProgress);

    let done: Status = serde_json::from_str("\"Done\"").unwrap();
    assert_eq!(done, Status::Done);

    let blocked: Status = serde_json::from_str("\"Blocked\"").unwrap();
    assert_eq!(blocked, Status::Blocked);

    let bloqued: Status = serde_json::from_str("\"bloqued\"").unwrap();
    assert_eq!(bloqued, Status::Blocked);
}

#[test]
fn test_priority_deserialize() {
    let high: Priority = serde_json::from_str("\"High\"").unwrap();
    assert_eq!(high, Priority::High);
    let medium: Priority = serde_json::from_str("\"medium\"").unwrap();
    assert_eq!(medium, Priority::Medium);
    let low: Priority = serde_json::from_str("\"low\"").unwrap();
    assert_eq!(low, Priority::Low);
}

#[test]
fn test_task_serialization_roundtrip() {
    let task = Task::new("zOaU", "Implement lida engine");
    let json = serde_json::to_string(&task).unwrap();
    let parsed: Task = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.id, "zOaU");
    assert_eq!(parsed.description, "Implement lida engine");
    assert_eq!(parsed.status, Status::Todo);
}
