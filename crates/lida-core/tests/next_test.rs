//! Integration tests for `lida_core::next` and `lida_core::query`.

use lida_core::date::Date;
use lida_core::model::ProjectDoc;
use lida_core::next::{Waiting, next};
use lida_core::query::Query;
use std::collections::HashMap;

const FIXTURE: &str = include_str!("fixtures/next.yaml");

#[expect(clippy::expect_used, reason = "test fixture helper: invalid YAML fails the test")]
fn doc() -> ProjectDoc {
    serde_yaml_ng::from_str(FIXTURE).expect("valid next fixture YAML")
}

fn today() -> Date {
    Date::constant(2026, 12, 3)
}

fn ids_on(words: &[&str], day: Date) -> Vec<String> {
    let document = doc();
    let query = Query::parse(words);
    next(&document, &query, day)
        .into_iter()
        .map(|entry| entry.task.id.clone())
        .collect()
}

fn ids_for(words: &[&str]) -> Vec<String> {
    ids_on(words, today())
}

fn waiting_by_id() -> HashMap<String, Waiting> {
    let document = doc();
    let query = Query::parse::<&str>(&[]);
    next(&document, &query, today())
        .into_iter()
        .map(|entry| (entry.task.id.clone(), entry.waiting))
        .collect()
}

#[test]
fn order_and_open() {
    assert_eq!(
        ids_for(&[]),
        [
            "bbbb", "aaaa", "cccc", "iiii", "dddd", "jjjj", "kkkk", "mmmm", "eeee", "llll", "ffff"
        ]
    );
}

#[test]
fn in_progress_sort_by_urgency_among_themselves() {
    let mut document = doc();
    for task in &mut document.tasks {
        if task.id == "dddd" {
            task.status = lida_core::model::Status::InProgress;
        }
    }
    let ids: Vec<_> = next(&document, &Query::parse::<&str>(&[]), today())
        .iter()
        .map(|e| e.task.id.as_str())
        .collect();
    assert_eq!(&ids[..3], ["dddd", "bbbb", "aaaa"]);
}

#[test]
fn waiting_states() {
    let w = waiting_by_id();
    assert_eq!(
        (w["cccc"], w["iiii"], w["mmmm"], w["llll"], w["aaaa"]),
        (
            Waiting::Depends,
            Waiting::Blocked,
            Waiting::Depends,
            Waiting::No,
            Waiting::No,
        )
    );
}

#[test]
fn filters() {
    assert_eq!(ids_for(&["@jobx"]), ["aaaa", "cccc", "dddd"]);
    assert_eq!(ids_for(&["@dots"]), ["bbbb", "jjjj", "mmmm"]);
    assert_eq!(ids_for(&["@dots.agentic"]), ["jjjj"]);
    assert_eq!(ids_for(&["@dots", "@jobx"]).len(), 6);
    assert_eq!(ids_for(&["@job"]), Vec::<String>::new());
    assert_eq!(ids_for(&["+series"]), ["bbbb"]);
    assert_eq!(ids_for(&["+series", "+foo"]), Vec::<String>::new());
    assert_eq!(ids_for(&["purposes:branding"]), ["bbbb"]);
    assert_eq!(ids_for(&["lead:showcase"]), ["eeee"]);
    assert_eq!(ids_for(&["RÉSUMÉ"]), ["ffff"]);
    assert_eq!(ids_for(&["line", "two"]), ["llll"]);
    assert_eq!(ids_for(&["@", "+", "nope:x"]), Vec::<String>::new());
}

#[test]
fn search_spans_map_to_original_bytes() {
    let q = Query::parse(&["RÉSUMÉ"]);
    let text = "my résumé, résumé";
    assert_eq!(q.search_spans(text), [3..11, 13..21]);
}

#[test]
fn due_tags_by_day() {
    assert_eq!(ids_for(&["+OVERDUE"]), ["aaaa", "cccc"]);
    assert_eq!(ids_for(&["+TODAY"]), ["jjjj"]); // due "2026-12-03 09:00" counts as that day
    assert_eq!(ids_for(&["+TOMORROW"]), Vec::<String>::new());
    assert_eq!(ids_on(&["+TOMORROW"], Date::constant(2026, 12, 1)), ["cccc"]);
    assert_eq!(ids_for(&["+WEEK"]), ["dddd", "jjjj"]); // dddd is today + 6: inside
    assert_eq!(ids_on(&["+WEEK"], Date::constant(2026, 12, 2)), ["cccc", "jjjj"]); // dddd at + 7: outside
    // llll ("tomorrow", unparsable) and tasks without due never match
    for tag in ["+OVERDUE", "+TODAY", "+TOMORROW", "+WEEK"] {
        assert!(!ids_for(&[tag]).iter().any(|id| id == "llll" || id == "bbbb"), "{tag}");
    }
}

#[test]
fn due_tags_or_together() {
    assert_eq!(ids_for(&["+TODAY", "+OVERDUE"]), ["aaaa", "cccc", "jjjj"]);
    assert_eq!(ids_for(&["+OVERDUE", "+TODAY"]), ["aaaa", "cccc", "jjjj"]);
    assert_eq!(ids_for(&["+TODAY", "+TODAY"]), ["jjjj"]);
}

#[test]
fn due_tags_and_with_other_clauses() {
    assert_eq!(ids_for(&["+OVERDUE", "@jobx"]), ["aaaa", "cccc"]);
    assert_eq!(ids_for(&["+TODAY", "@jobx"]), Vec::<String>::new());
    assert_eq!(ids_for(&["+WEEK", "@dots"]), ["jjjj"]);
    assert_eq!(ids_for(&["+OVERDUE", "+series"]), Vec::<String>::new());
    assert_eq!(ids_for(&["+WEEK", "follow"]), ["dddd"]);
}

#[test]
fn due_tags_are_case_sensitive() {
    assert_eq!(ids_for(&["+today"]), Vec::<String>::new());
    assert_eq!(ids_for(&["+Today"]), Vec::<String>::new());
    let q = Query::parse(&["+TODAY", "+today"]);
    assert_eq!(q.due_tags, [lida_core::query::DueTag::Today]);
    assert_eq!(q.tags, ["today"]);
}
