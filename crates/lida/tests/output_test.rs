//! Golden output of every `lida` subcommand: stdout, stderr and exit code, byte for byte.
//!
//! These snapshots pin today's behaviour (ANSI colour codes included) so refactors can prove they changed nothing.

use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

/// Two open tasks with fixed IDs, in lida's own Markdown format.
const FIXTURE: &str = "<!-- todo-project: fixture -->

# Tasks

- [ ] aaaa **-** First task
  - **Priority**: medium
  - **Tags**: setup

- [ ] bbbb **-** Second task
  - **Priority**: high
  - **Tags**: setup, docs

# Actors

# Comments
";

/// One finished `lida` invocation.
struct Run {
    code: i32,
    stdout: String,
    stderr: String,
}

#[expect(clippy::expect_used, reason = "test setup helper: a failed setup is a failed test")]
fn lida(dir: &Path, args: &[&str]) -> Run {
    let output = Command::new(env!("CARGO_BIN_EXE_lida"))
        .args(args)
        .current_dir(dir)
        .env_remove("NO_COLOR")
        .output()
        .expect("lida binary runs");
    Run {
        code: output.status.code().unwrap_or(-1),
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    }
}

#[expect(clippy::expect_used, reason = "test setup helper: a failed setup is a failed test")]
fn empty_dir() -> TempDir {
    tempfile::tempdir().expect("temp dir")
}

#[expect(clippy::expect_used, reason = "test setup helper: a failed setup is a failed test")]
fn fixture_dir() -> TempDir {
    let dir = empty_dir();
    fs::write(dir.path().join("TODO.md"), FIXTURE).expect("write fixture");
    dir
}

/// Replaces the temp dir (both spellings macOS uses) and every newly generated task ID with stable placeholders.
fn redact(text: &str, dir: &Path) -> String {
    let mut text = text.to_owned();
    if let Ok(canonical) = dir.canonicalize() {
        text = text.replace(&canonical.display().to_string(), "[DIR]");
    }
    text = text.replace(&dir.display().to_string(), "[DIR]");
    // A release bump must not fail the snapshots.
    text = text.replace(env!("CARGO_PKG_VERSION"), "[VERSION]");
    let ids: Vec<String> = text
        .match_indices("Added task #")
        .filter_map(|(at, marker)| text.get(at + marker.len()..at + marker.len() + 4))
        .map(str::to_owned)
        .collect();
    for id in ids {
        text = text.replace(&id, "[ID]");
    }
    text
}

/// Runs each command in order in `dir` and renders them as one transcript.
fn transcript(dir: &Path, commands: &[&[&str]]) -> String {
    let mut out = String::new();
    for args in commands {
        let run = lida(dir, args);
        let _ = write!(
            out,
            "$ lida {}\ncode: {}\n--- stdout\n{}--- stderr\n{}\n",
            args.join(" "),
            run.code,
            run.stdout,
            run.stderr
        );
    }
    redact(&out, dir)
}

fn snap_empty(name: &str, commands: &[&[&str]]) {
    let dir = empty_dir();
    insta::assert_snapshot!(name, transcript(dir.path(), commands));
}

fn snap_fixture(name: &str, commands: &[&[&str]]) {
    let dir = fixture_dir();
    insta::assert_snapshot!(name, transcript(dir.path(), commands));
}

#[test]
fn init_modes() {
    snap_empty("init_md", &[&["init", "--md", "--project", "demo"]]);
    snap_empty("init_yaml", &[&["init", "--yaml", "--project", "demo"]]);
    snap_empty("init_both", &[&["init", "--both", "--project", "demo"]]);
}

#[test]
fn init_existing_without_force() {
    snap_fixture(
        "init_existing_without_force",
        &[&["init", "--md"], &["init", "--md", "--force"]],
    );
}

#[test]
fn add_variants() {
    snap_fixture("add_positional", &[&["add", "Third task"]]);
    snap_fixture("add_legacy_task_flag", &[&["add", "--task", "Third task"]]);
    snap_fixture(
        "add_with_meta_and_tags",
        &[
            &[
                "add",
                "Third task",
                "-p",
                "low",
                "-t",
                "a,b",
                "-d",
                "2026-12-01",
                "--position",
                "src/x.rs:1",
            ],
            &["list"],
        ],
    );
    snap_fixture(
        "add_comment_to_task_id",
        &[&["add", "--task-id", "aaaa", "-c", "A note"], &["show", "aaaa"]],
    );
}

#[test]
fn update_task() {
    snap_fixture(
        "update",
        &[
            &[
                "update",
                "aaaa",
                "-d",
                "Renamed",
                "-p",
                "high",
                "--tag",
                "docs",
                "--due",
                "2026-12-01",
            ],
            &["show", "aaaa"],
        ],
    );
}

#[test]
fn list_variants() {
    snap_fixture("list_default", &[&["list"]]);
    snap_fixture("list_tasks_flag", &[&["list", "--tasks"]]);
    snap_fixture(
        "list_filters",
        &[
            &["list", "-s", "todo"],
            &["list", "-t", "docs"],
            &["list", "-p", "high"],
        ],
    );
    snap_fixture("list_json", &[&["list", "--json"]]);
    snap_fixture("no_args_lists", &[&[]]);
}

#[test]
fn status_changes() {
    snap_fixture("done", &[&["done", "aaaa"], &["list"]]);
    snap_fixture("start", &[&["start", "aaaa"], &["list"]]);
    snap_fixture(
        "status_set_with_reason",
        &[
            &["status", "aaaa", "--set", "blocked", "--reason", "Waiting"],
            &["show", "aaaa"],
        ],
    );
    snap_fixture(
        "status_alias",
        &[&["status", "aaaa", "--set", "en-cours"], &["show", "aaaa"]],
    );
}

#[test]
fn show_variants() {
    snap_fixture("show", &[&["show", "bbbb"]]);
    snap_fixture("show_json", &[&["show", "bbbb", "--json"]]);
}

#[test]
fn meta_tags_delete() {
    snap_fixture(
        "meta_set_get_unset",
        &[
            &["meta", "aaaa", "set", "owner", "ro"],
            &["meta", "aaaa", "get", "owner"],
            &["meta", "aaaa", "unset", "owner"],
        ],
    );
    snap_fixture("tags", &[&["tags"]]);
    snap_fixture("delete", &[&["delete", "bbbb"], &["list"]]);
}

#[test]
fn cwi_formats() {
    snap_fixture("cwi", &[&["cwi"], &["cwi", "both"], &["cwi", "yaml"], &["cwi", "md"]]);
}

#[test]
fn list_filter_in_progress() {
    snap_fixture(
        "list_filter_in_progress",
        &[
            &["start", "aaaa"],
            &["list", "-s", "in-progress"],
            &["list", "-s", "in_progress"],
            &["list", "-s", "en-cours"],
        ],
    );
}

#[test]
fn strict_values() {
    snap_fixture(
        "strict_values",
        &[
            &["add", "X", "-p", "urgent"],
            &["update", "aaaa", "-p", "none"],
            &["show", "aaaa"],
            &["list", "-s", "nope"],
            &["list", "-p", "none"],
            &["list", "-p", "urgent"],
            &["status", "aaaa", "--set", "in_progress"],
        ],
    );
}

#[test]
fn generated_specs() {
    snap_empty("completion_zsh", &[&["completion", "zsh"]]);
    snap_empty("usage", &[&["usage"]]);
}

#[test]
fn optional_branches() {
    snap_fixture(
        "list_empty",
        &[&["list", "-s", "done"], &["list", "-s", "done", "--json"]],
    );
    snap_fixture(
        "list_blocked_and_bare_rows",
        &[
            &["update", "bbbb", "-p", "none", "-t", ""],
            &["status", "aaaa", "--set", "blocked"],
            &["list"],
            &["list", "--tasks"],
            &["list", "--tasks", "--json"],
            &["show", "bbbb"],
        ],
    );
    snap_fixture(
        "show_position_and_meta",
        &[
            &["update", "aaaa", "--position", "src/x.rs:1"],
            &["meta", "aaaa", "set", "count", "3"],
            &["meta", "aaaa", "set", "flag", "true"],
            &["meta", "aaaa", "get", "count"],
            &["show", "aaaa"],
            &["show", "aaaa", "--json"],
            &["meta", "aaaa", "rm", "flag"],
        ],
    );
    snap_empty("completion_other_shell", &[&["completion", "bash"]]);
    snap_empty("tags_empty", &[&["init", "--md"], &["tags"]]);
    snap_fixture(
        "unknown_id_other_commands",
        &[
            &["update", "zzzz", "-d", "x"],
            &["start", "zzzz"],
            &["status", "zzzz", "--set", "done"],
            &["meta", "zzzz", "get", "k"],
        ],
    );
}

#[test]
fn errors() {
    snap_fixture(
        "unknown_id",
        &[&["done", "zzzz"], &["show", "zzzz"], &["delete", "zzzz"]],
    );
    snap_empty("no_todo_file", &[&["list"]]);
    snap_fixture("bad_status", &[&["status", "aaaa", "--set", "nope"]]);
    snap_fixture("bad_priority", &[&["add", "Third task", "-p", "urgent"]]);
    snap_fixture(
        "meta_errors",
        &[
            &["meta", "aaaa", "get", "missing"],
            &["meta", "aaaa", "frob", "key"],
            &["meta", "aaaa", "set", "key"],
        ],
    );
    snap_fixture("cwi_bad_format", &[&["cwi", "toml"]]);
    snap_fixture(
        "add_without_text",
        &[
            &["add"],
            &["add", "--task-id", "aaaa"],
            &["add", "--task-id", "zzzz", "-c", "x"],
        ],
    );
}

#[test]
fn version_matches_manifest() {
    let dir = empty_dir();
    let run = lida(dir.path(), &["--version"]);
    assert_eq!(run.code, 0, "--version exit code; stderr: {}", run.stderr);
    assert!(
        run.stdout.contains(env!("CARGO_PKG_VERSION")),
        "--version prints the Cargo.toml version, got {:?}",
        run.stdout
    );
}

#[test]
fn json_outputs_are_pure() {
    let dir = fixture_dir();
    for args in [&["list", "--json"][..], &["show", "aaaa", "--json"][..]] {
        let run = lida(dir.path(), args);
        assert_eq!(run.code, 0, "{args:?} exit code");
        assert!(
            serde_json::from_str::<serde_json::Value>(&run.stdout).is_ok(),
            "{args:?} stdout is JSON"
        );
        assert!(run.stderr.is_empty(), "{args:?} stderr is empty");
    }
}
