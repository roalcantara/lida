//! Integration tests for `lida next` (alias `nx`), covering every stress-story moment (1–14).

use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;
use unicode_width::UnicodeWidthStr;

const FIXTURE: &str = include_str!("../../lida-core/tests/fixtures/next.yaml");
const TODAY: &str = "2026-12-03";

struct Run {
    code: i32,
    stdout: String,
    stderr: String,
}

#[expect(clippy::expect_used, reason = "test setup helper: a failed setup is a failed test")]
fn run_lida(dir: &Path, args: &[&str], envs: &[(&str, &str)]) -> Run {
    let out = Command::new(env!("CARGO_BIN_EXE_lida"))
        .args(args)
        .current_dir(dir)
        .env("LIDA_TODAY", TODAY)
        .env_remove("NO_COLOR")
        .env_remove("CLICOLOR_FORCE")
        .env_remove("COLUMNS")
        .envs(envs.iter().copied())
        .output()
        .expect("lida binary runs");
    Run {
        code: out.status.code().unwrap_or(-1),
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
    }
}

fn run_table(dir: &Path, args: &[&str], cols: &str) -> Run {
    run_lida(dir, args, &[("CLICOLOR_FORCE", "1"), ("COLUMNS", cols)])
}

#[expect(clippy::expect_used, reason = "test setup helper: a failed setup is a failed test")]
fn fixture_dir() -> TempDir {
    let dir = tempfile::tempdir().expect("temp dir");
    fs::write(dir.path().join("TODO.yaml"), FIXTURE).expect("write fixture");
    dir
}

fn strip_sgr(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\u{1b}' && chars.peek() == Some(&'[') {
            chars.next();
            for c in chars.by_ref() {
                if c == 'm' {
                    break;
                }
            }
        } else {
            out.push(ch);
        }
    }
    out
}

fn raw_ids(stdout: &str) -> Vec<&str> {
    stdout.lines().filter_map(|line| line.split('\t').next()).collect()
}

#[test]
fn s01_default_table() {
    let dir = fixture_dir();
    let run = run_table(dir.path(), &["nx"], "120");
    assert_eq!(run.code, 0, "stderr: {}", run.stderr);
    assert_eq!(run.stderr, "");
    insta::assert_snapshot!("next_default_table", run.stdout);
}

#[test]
fn s02_long_table() {
    let dir = fixture_dir();
    let run = run_table(dir.path(), &["nx", "-l"], "120");
    assert_eq!(run.code, 0, "stderr: {}", run.stderr);
    insta::assert_snapshot!("next_long_table", run.stdout);
}

#[test]
fn s03_project_filters() {
    let dir = fixture_dir();
    let jobx = run_lida(dir.path(), &["nx", "@jobx", "--raw"], &[]);
    assert_eq!(raw_ids(&jobx.stdout), ["aaaa", "cccc", "dddd"]);

    let agentic = run_lida(dir.path(), &["nx", "@dots.agentic", "--raw"], &[]);
    assert_eq!(raw_ids(&agentic.stdout), ["jjjj"]);

    let both = run_lida(dir.path(), &["nx", "@dots", "@jobx", "--raw"], &[]);
    assert_eq!(raw_ids(&both.stdout).len(), 6);
}

#[test]
fn s04_tag_and_meta() {
    let dir = fixture_dir();
    let r1 = run_lida(dir.path(), &["nx", "+series", "purposes:branding", "--raw"], &[]);
    assert_eq!(raw_ids(&r1.stdout), ["bbbb"]);

    let r2 = run_lida(dir.path(), &["nx", "lead:showcase", "--raw"], &[]);
    assert_eq!(raw_ids(&r2.stdout), ["eeee"]);
}

#[test]
fn s05_empty() {
    let dir = fixture_dir();
    let run = run_lida(dir.path(), &["nx", "@nope"], &[]);
    assert_eq!(run.code, 0);
    assert_eq!(run.stdout, "No tasks found.\n");
}

#[test]
fn s06_search() {
    let dir = fixture_dir();
    let raw = run_lida(dir.path(), &["nx", "résumé", "--raw"], &[]);
    assert_eq!(raw_ids(&raw.stdout), ["ffff"]);

    let table = run_table(dir.path(), &["nx", "résumé"], "120");
    assert_eq!(table.code, 0);
    assert!(
        table.stdout.contains("\u{1b}[4;") || table.stdout.contains(";4;"),
        "expected underline SGR 4 in: {}",
        table.stdout
    );
    insta::assert_snapshot!("next_search_underline", table.stdout);

    let empty = run_lida(dir.path(), &["nx", "term", "+series", "+foo", "@dots", "@jobx"], &[]);
    assert_eq!(empty.code, 0);
    assert_eq!(empty.stdout, "No tasks found.\n");
}

#[test]
fn s07_long_description_cut() {
    let dir = fixture_dir();
    let run = run_table(dir.path(), &["nx"], "120");
    assert_eq!(run.code, 0);
    let ffff_line = run.stdout.lines().map(strip_sgr).find(|line| line.contains("ffff"));
    assert!(ffff_line.is_some(), "missing ffff row");
    if let Some(line) = ffff_line {
        assert_eq!(UnicodeWidthStr::width(line.as_str()), 120);
        assert!(
            line.trim_end().ends_with("+alpha +beta +gamma +delta"),
            "unexpected ffff line: {line}"
        );
    }
}

#[test]
fn s08_many_tags_long() {
    let dir = fixture_dir();
    let run = run_table(dir.path(), &["nx", "-l"], "120");
    assert_eq!(run.code, 0);
    let ffff_line = run.stdout.lines().map(strip_sgr).find(|line| line.contains("ffff"));
    assert!(ffff_line.is_some());
    if let Some(line) = ffff_line {
        assert!(
            line.contains("+alpha +beta +ga…"),
            "expected truncated tags cell in: {line}"
        );
    }
}

#[test]
fn s09_long_project_cut() {
    let dir = fixture_dir();
    let run = run_table(dir.path(), &["nx"], "120");
    assert_eq!(run.code, 0);
    let kkkk_line = run.stdout.lines().map(strip_sgr).find(|line| line.contains("kkkk"));
    assert!(kkkk_line.is_some());
    if let Some(line) = kkkk_line {
        assert!(
            line.contains("@a-very-long-…"),
            "expected 14-cell truncated project in: {line}"
        );
    }
}

#[test]
fn s10_wide_text_alignment() {
    let dir = fixture_dir();
    let run = run_table(dir.path(), &["nx"], "120");
    assert_eq!(run.code, 0);
    for line in run.stdout.lines() {
        let plain = strip_sgr(line);
        assert_eq!(UnicodeWidthStr::width(plain.as_str()), 120, "misaligned line: {plain}");
    }
}

#[test]
fn s11_odd_values() {
    let dir = fixture_dir();
    let run = run_table(dir.path(), &["nx", "-l"], "120");
    assert_eq!(run.code, 0);
    let lines: Vec<String> = run.stdout.lines().map(strip_sgr).collect();
    let eeee = lines.iter().find(|l| l.contains("eeee"));
    let ffff = lines.iter().find(|l| l.contains("ffff"));
    assert!(eeee.is_some_and(|l| l.contains("-3.2")));
    assert!(ffff.is_some_and(|l| l.contains("    -     -    ")));
}

#[test]
fn s12_narrow_80() {
    let dir = fixture_dir();
    let run = run_table(dir.path(), &["nx", "-l"], "80");
    assert_eq!(run.code, 0);
    let mut lines = run.stdout.lines().map(strip_sgr);
    let header = lines.next().unwrap_or_default();
    assert!(!header.contains("WHY"), "header should drop WHY: {header}");
    assert!(!header.contains("TAGS"), "header should drop TAGS: {header}");
    assert!(UnicodeWidthStr::width(header.as_str()) <= 80);
    for line in lines {
        assert!(
            UnicodeWidthStr::width(line.as_str()) <= 80,
            "line exceeded 80 cells: {line}"
        );
    }
}

#[test]
fn s13_pipe_raw_json() {
    let dir = fixture_dir();
    let piped = run_lida(dir.path(), &["nx"], &[]);
    let raw = run_lida(dir.path(), &["nx", "--raw"], &[]);
    assert_eq!(piped.code, 0);
    assert_eq!(piped.stdout, raw.stdout);

    let bbbb = raw.stdout.lines().find(|l| l.starts_with("bbbb\t"));
    assert_eq!(
        bbbb,
        Some("bbbb\tL\t\t@dots.blog\t(doing) Series: account-wide numbers +series")
    );

    let json = run_lida(dir.path(), &["nx", "--json"], &[]);
    assert_eq!(json.code, 0);
    let parsed: Result<Vec<serde_json::Value>, _> = serde_json::from_str(&json.stdout);
    assert!(parsed.as_ref().is_ok_and(|v| v.len() == 11));

    let empty_json = run_lida(dir.path(), &["nx", "--json", "@nope"], &[]);
    assert_eq!(empty_json.code, 0);
    assert_eq!(empty_json.stdout.trim(), "[]");
}

#[test]
fn s14_outside_folder() {
    let dir = tempfile::tempdir();
    assert!(dir.is_ok());
    if let Ok(dir) = dir {
        let run = run_lida(dir.path(), &["nx"], &[]);
        assert_eq!(run.code, 1);
        assert_eq!(run.stderr.trim(), "error: no TODO file found in directory");
    }
}

#[test]
fn sanitised_row() {
    let dir = fixture_dir();
    let raw = run_lida(dir.path(), &["nx", "--raw"], &[]);
    let raw_llll = raw.stdout.lines().find(|l| l.starts_with("llll\t"));
    assert!(raw_llll.is_some());
    if let Some(line) = raw_llll {
        assert!(!line.contains('\u{1b}'));
        assert!(!line.contains('\n'));
    }

    let table = run_table(dir.path(), &["nx"], "120");
    let table_llll = table.stdout.lines().map(strip_sgr).find(|l| l.contains("llll"));
    assert!(table_llll.is_some());
    if let Some(line) = table_llll {
        assert!(!line.contains('\u{1b}'));
        assert!(!line.contains('\n'));
    }
}

#[test]
fn tags_longer_than_room() {
    let dir = fixture_dir();
    let run = run_table(dir.path(), &["nx"], "60");
    assert_eq!(run.code, 0);
    for line in run.stdout.lines() {
        let plain = strip_sgr(line);
        assert!(
            UnicodeWidthStr::width(plain.as_str()) <= 60,
            "line exceeded 60 cells: {plain}"
        );
    }
}

#[test]
fn tiny_window() {
    let dir = fixture_dir();
    let run = run_lida(dir.path(), &["nx", "-l"], &[("CLICOLOR_FORCE", "1"), ("COLUMNS", "20")]);
    assert_eq!(run.code, 0, "stderr: {}", run.stderr);
}

#[test]
fn in_progress_first_in_raw_and_json() {
    let dir = fixture_dir();
    let raw = run_lida(dir.path(), &["nx", "--raw"], &[]);
    assert_eq!(raw_ids(&raw.stdout).first(), Some(&"bbbb"));
    let json = run_lida(dir.path(), &["nx", "--json"], &[]);
    let parsed: Vec<serde_json::Value> = serde_json::from_str(&json.stdout).unwrap_or_default();
    assert_eq!(parsed.first().and_then(|t| t["id"].as_str()), Some("bbbb"));
}

#[test]
fn due_tags_from_the_cli() {
    let dir = fixture_dir();
    let run = run_lida(dir.path(), &["nx", "+TODAY", "+OVERDUE", "--raw"], &[]);
    assert_eq!(run.code, 0, "stderr: {}", run.stderr);
    assert_eq!(raw_ids(&run.stdout), ["aaaa", "cccc", "jjjj"]);
    let tomorrow = run_lida(
        dir.path(),
        &["nx", "+TOMORROW", "--raw"],
        &[("LIDA_TODAY", "2026-12-01")],
    );
    assert_eq!(raw_ids(&tomorrow.stdout), ["cccc"]);
    let none = run_lida(dir.path(), &["nx", "+TOMORROW"], &[]);
    assert_eq!((none.code, none.stdout.as_str()), (0, "No tasks found.\n"));
}
