//! Golden output of every `lida` subcommand: stdout, stderr and exit code, byte for byte.
//!
//! These snapshots pin today's behaviour so refactors can prove they changed nothing. Output is piped, so it carries no
//! colour; `colour_forced` pins the ANSI codes.

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
fn lida_with_env(dir: &Path, args: &[&str], envs: &[(&str, &str)]) -> Run {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_lida"));
    cmd.args(args)
        .current_dir(dir)
        .env_remove("NO_COLOR")
        .env_remove("CLICOLOR_FORCE");
    for (k, v) in envs {
        cmd.env(k, v);
    }
    let output = cmd.output().expect("lida binary runs");
    Run {
        code: output.status.code().unwrap_or(-1),
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    }
}

fn lida(dir: &Path, args: &[&str]) -> Run {
    lida_with_env(dir, args, &[])
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
    // A release bump must not fail the snapshots: redact exact "lida <version>" lines and
    // 'version "<version>"' tokens without altering other numeric occurrences.
    let version = env!("CARGO_PKG_VERSION");
    let version_token = format!("version \"{version}\"");
    text = text.replace(&version_token, "version \"[VERSION]\"");
    let lida_line = format!("lida {version}");
    let mut lines = Vec::new();
    for line in text.split('\n') {
        if line == lida_line {
            lines.push("lida [VERSION]".to_string());
        } else {
            lines.push(line.to_string());
        }
    }
    text = lines.join("\n");
    let ids: Vec<String> = text
        .match_indices("Added task #")
        .filter_map(|(at, marker)| text.get(at + marker.len()..at + marker.len() + 4))
        .map(str::to_owned)
        .collect();
    for id in ids {
        text = text.replace(&id, "[ID]");
    }
    let comment_ids: Vec<String> = text
        .lines()
        .filter_map(|line| {
            let rest = line.strip_prefix("    #")?;
            let id = rest.split_whitespace().next()?;
            if id.len() == 4 { Some(id.to_string()) } else { None }
        })
        .collect();
    for id in comment_ids {
        text = text.replace(&format!("    #{id} "), "    #[ID] ");
    }
    text
}

fn append_run(out: &mut String, args: &[&str], run: &Run) {
    let _ = write!(
        out,
        "$ lida {}\ncode: {}\n--- stdout\n{}--- stderr\n{}\n",
        args.join(" "),
        run.code,
        run.stdout,
        run.stderr
    );
}

fn transcript_with_env(dir: &Path, commands: &[&[&str]], envs: &[(&str, &str)]) -> String {
    let mut out = String::new();
    for args in commands {
        let run = lida_with_env(dir, args, envs);
        append_run(&mut out, args, &run);
    }
    redact(&out, dir)
}

/// Runs each command in order in `dir` and renders them as one transcript.
fn transcript(dir: &Path, commands: &[&[&str]]) -> String {
    transcript_with_env(dir, commands, &[])
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
    snap_fixture("stop", &[&["start", "aaaa"], &["stop", "aaaa"], &["list"]]);
    snap_fixture(
        "stop_keeps_reason",
        &[
            &["status", "aaaa", "--set", "blocked", "--reason", "Waiting"],
            &["stop", "aaaa"],
            &["show", "aaaa"],
        ],
    );
    snap_fixture("stop_todo_is_noop_success", &[&["stop", "bbbb"]]);
    snap_fixture("stop_unknown", &[&["stop", "zzzz"]]);
}

#[test]
fn show_variants() {
    snap_fixture("show", &[&["show", "bbbb"]]);
    snap_fixture("show_json", &[&["show", "bbbb", "--json"]]);
}

#[test]
fn show_everything() {
    snap_fixture(
        "show_everything",
        &[
            &["update", "aaaa", "--due", "2026-12-01", "--position", "top"],
            &["status", "aaaa", "--set", "blocked", "--reason", "Waiting on review"],
            &["meta", "aaaa", "set", "owner", "alice"],
            &["add", "--task-id", "aaaa", "-c", "First comment"],
            &["show", "aaaa"],
        ],
    );
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
fn file_flag() {
    let d = fixture_dir();
    fs::create_dir(d.path().join("other")).expect("create other");
    fs::write(
        d.path().join("other/TODO.md"),
        FIXTURE.replace("First task", "Other task"),
    )
    .expect("write other fixture");

    insta::assert_snapshot!(
        "file_flag",
        transcript(
            d.path(),
            &[
                &["-f", "other/TODO.md", "list"],
                &["list", "-f", "other"],
                &["--file", "other/TODO.md", "done", "aaaa"],
                &["-f", "nope", "list"],
            ]
        )
    );

    let other = fs::read_to_string(d.path().join("other/TODO.md")).expect("read other");
    assert!(
        other.contains("- [x] aaaa"),
        "-f run must mark other's task done: {other}"
    );
    let own = fs::read_to_string(d.path().join("TODO.md")).expect("read own");
    assert!(
        own.contains("- [ ] aaaa"),
        "-f runs must not touch the current directory: {own}"
    );

    let env_run = lida_with_env(d.path(), &["list"], &[("LIDA_FILE", "other")]);
    assert_eq!(env_run.code, 0, "{}", env_run.stderr);
    assert!(env_run.stdout.contains("Other task"), "LIDA_FILE must be honoured");

    let flag_wins = lida_with_env(d.path(), &["-f", ".", "list"], &[("LIDA_FILE", "other")]);
    assert_eq!(flag_wins.code, 0, "{}", flag_wins.stderr);
    assert!(flag_wins.stdout.contains("First task"));
    assert!(!flag_wins.stdout.contains("Other task"), "-f must win over LIDA_FILE");

    let e = empty_dir();
    fs::create_dir(e.path().join("new")).expect("create new");
    let init = lida(e.path(), &["-f", "new/TODO.yaml", "init"]);
    assert_eq!(init.code, 0, "{}", init.stderr);
    assert!(e.path().join("new/TODO.yaml").exists());
    assert!(!e.path().join("TODO.yaml").exists());

    assert_ne!(lida(e.path(), &["init", "-f"]).code, 0, "-f now wants a value");
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
fn colour_forced() {
    let dir = fixture_dir();
    insta::assert_snapshot!(
        "colour_forced",
        transcript_with_env(dir.path(), &[&["list"]], &[("CLICOLOR_FORCE", "1")])
    );
}

#[test]
fn list_aligned() {
    let dir = fixture_dir();
    insta::assert_snapshot!("list_aligned", transcript(dir.path(), &[&["list"]]));
}

#[test]
fn redaction_leaves_other_numbers() {
    let dir = empty_dir();
    let v = env!("CARGO_PKG_VERSION");
    let input = format!("lida {v}\nversion \"{v}\"\nfoo 1{v}");
    let expected = format!("lida [VERSION]\nversion \"[VERSION]\"\nfoo 1{v}");
    assert_eq!(redact(&input, dir.path()), expected);
}

#[test]
fn completion_is_lidas() {
    let dir = empty_dir();
    let run = lida(dir.path(), &["completion", "zsh"]);
    assert_eq!(run.code, 0);
    assert!(run.stdout.starts_with("#compdef lida"));
    assert!(!run.stdout.contains("todo"));
}

#[test]
fn completion_rejects_unknown_shell() {
    let dir = empty_dir();
    let run = lida(dir.path(), &["completion", "tcsh"]);
    assert_eq!(run.code, 2);
}

#[test]
fn completion_offers_values() {
    let dir = fixture_dir();
    // usage-rs completion callback argv: lida __complete_word__ --shell bash --line <line>
    // add -- offers --priority and --tags
    let run = lida(
        dir.path(),
        &["__complete_word__", "--shell", "bash", "--line", "lida add --"],
    );
    assert_eq!(run.code, 0);
    assert!(run.stdout.contains("--priority"));
    assert!(run.stdout.contains("--tags"));

    // done offers aaaa and bbbb
    let run = lida(
        dir.path(),
        &["__complete_word__", "--shell", "bash", "--line", "lida done "],
    );
    assert_eq!(run.code, 0);
    let items: Vec<&str> = run.stdout.lines().collect();
    assert!(items.contains(&"aaaa"));
    assert!(items.contains(&"bbbb"));

    // list -t offers docs and setup
    let run = lida(
        dir.path(),
        &["__complete_word__", "--shell", "bash", "--line", "lida list -t "],
    );
    assert_eq!(run.code, 0);
    let items: Vec<&str> = run.stdout.lines().collect();
    assert_eq!(items, ["docs", "setup"]);

    // add X -p offers exactly low medium high none
    let run = lida(
        dir.path(),
        &["__complete_word__", "--shell", "bash", "--line", "lida add X -p "],
    );
    assert_eq!(run.code, 0);
    let mut items: Vec<&str> = run.stdout.lines().collect();
    items.sort_unstable();
    assert_eq!(items, ["high", "low", "medium", "none"]);

    // status aaaa --set offers exactly todo in-progress done blocked
    let run = lida(
        dir.path(),
        &[
            "__complete_word__",
            "--shell",
            "bash",
            "--line",
            "lida status aaaa --set ",
        ],
    );
    assert_eq!(run.code, 0);
    let mut items: Vec<&str> = run.stdout.lines().collect();
    items.sort_unstable();
    assert_eq!(items, ["blocked", "done", "in-progress", "todo"]);

    // nx @ and + offer project prefixes and tags from TODO.yaml
    let next_dir = empty_dir();
    let next_fixture = include_str!("../../lida-core/tests/fixtures/next.yaml");
    fs::write(next_dir.path().join("TODO.yaml"), next_fixture).expect("write next fixture");

    let run = lida(
        next_dir.path(),
        &["__complete_word__", "--shell", "bash", "--line", "lida nx @"],
    );
    assert_eq!(run.code, 0);
    let items: Vec<&str> = run.stdout.lines().collect();
    for expected in ["@jobx", "@jobx.hunt", "@dots", "@dots.blog", "@dots.agentic"] {
        assert!(items.contains(&expected), "missing {expected} in {items:?}");
    }

    let run = lida(
        next_dir.path(),
        &["__complete_word__", "--shell", "bash", "--line", "lida nx +"],
    );
    assert_eq!(run.code, 0);
    let items: Vec<&str> = run.stdout.lines().collect();
    assert!(items.contains(&"+series"), "missing +series in {items:?}");
    for expected in ["+OVERDUE", "+TODAY", "+TOMORROW", "+WEEK"] {
        assert!(items.contains(&expected), "missing {expected} in {items:?}");
    }

    completion_follows_the_file_target(dir.path());
}

/// Completion loads the store from the `-f` target, or from `LIDA_FILE`. Fish renders the description beside each
/// value, which is what tells the two directories apart.
#[expect(clippy::expect_used, reason = "test setup helper: a failed setup is a failed test")]
fn completion_follows_the_file_target(dir: &Path) {
    fs::create_dir(dir.join("other")).expect("create other");
    fs::write(dir.join("other/TODO.md"), FIXTURE.replace("First task", "Other task")).expect("write other fixture");

    let run = lida(
        dir,
        &["__complete_word__", "--shell", "fish", "--line", "lida -f other done "],
    );
    assert_eq!(run.code, 0);
    assert!(
        run.stdout.contains("Other task"),
        "-f target must feed completion: {}",
        run.stdout
    );

    let run = lida_with_env(
        dir,
        &["__complete_word__", "--shell", "fish", "--line", "lida done "],
        &[("LIDA_FILE", "other")],
    );
    assert_eq!(run.code, 0);
    assert!(
        run.stdout.contains("Other task"),
        "LIDA_FILE must feed completion: {}",
        run.stdout
    );

    let run = lida(
        dir,
        &["__complete_word__", "--shell", "fish", "--line", "lida -f nope done "],
    );
    assert_eq!(run.code, 0);
    assert!(
        !run.stdout.contains("aaaa") && !run.stdout.contains("Other task"),
        "a missing -f target offers no task candidates: {:?}",
        run.stdout
    );
}

#[test]
fn usage_spec_is_complete_and_committed() {
    let dir = empty_dir();
    let run = lida(dir.path(), &["usage"]);
    assert_eq!(run.code, 0);
    assert!(run.stdout.contains("cmd add"));
    assert!(run.stdout.contains("flag \"-p --priority\""));
    let spec_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../lida.usage.kdl");
    let committed = fs::read_to_string(&spec_path).expect("read lida.usage.kdl");
    assert_eq!(run.stdout, committed);
}

#[test]
fn cwi_md_then_add_keeps_every_task() {
    let dir = empty_dir();
    assert_eq!(lida(dir.path(), &["init", "--both", "--project", "test"]).code, 0);
    assert_eq!(lida(dir.path(), &["add", "First"]).code, 0);
    assert_eq!(lida(dir.path(), &["cwi", "md"]).code, 0);
    assert!(!dir.path().join("TODO.yaml").exists());
    assert_eq!(lida(dir.path(), &["add", "Second"]).code, 0);
    assert_eq!(lida(dir.path(), &["add", "Third"]).code, 0);
    let run = lida(dir.path(), &["list", "--tasks"]);
    assert_eq!(run.code, 0);
    assert!(run.stdout.contains("First"));
    assert!(run.stdout.contains("Second"));
    assert!(run.stdout.contains("Third"));
}

#[test]
fn init_force_replaces_the_other_format() {
    let dir = empty_dir();
    assert_eq!(lida(dir.path(), &["init", "--yaml", "--project", "test"]).code, 0);
    assert_eq!(lida(dir.path(), &["add", "OldTask"]).code, 0);
    assert!(dir.path().join("TODO.yaml").exists());
    assert_eq!(
        lida(dir.path(), &["init", "--md", "--force", "--project", "test"]).code,
        0
    );
    assert!(!dir.path().join("TODO.yaml").exists());
    let run = lida(dir.path(), &["list"]);
    assert_eq!(run.code, 0);
    assert!(!run.stdout.contains("OldTask"));
}

#[test]
fn duplicate_ids_refuse_to_load_cli() {
    let dir = empty_dir();
    let content = "# Tasks\n\n- [ ] aaaa **-** One\n- [ ] aaaa **-** Two\n";
    fs::write(dir.path().join("TODO.md"), content).expect("write");
    let run = lida(dir.path(), &["list"]);
    assert_eq!(run.code, 1);
    assert!(run.stderr.contains("duplicate task ID #aaaa in TODO.md"));
    assert_eq!(fs::read_to_string(dir.path().join("TODO.md")).unwrap(), content);
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

#[test]
fn json_still_has_nulls() {
    let dir = fixture_dir();
    let run = lida(dir.path(), &["show", "aaaa", "--json"]);
    assert_eq!(run.code, 0);
    assert!(run.stdout.contains("\"due\": null"));
}

#[test]
fn input_validation() {
    let dir = fixture_dir();
    let initial_bytes = fs::read(dir.path().join("TODO.md")).expect("read fixture");
    let commands: &[&[&str]] = &[
        &["add", "Line one\n- [ ] evil **-** Injected"],
        &["add", "--task-id", "aaaa", "-c", "a\nb"],
        &["status", "aaaa", "--set", "blocked", "--reason", "a\nb"],
        &["update", "aaaa", "--position", "a\nb"],
        &["update", "aaaa", "--due", "a\nb"],
        &["update", "aaaa", "-t", "a\nb"],
        &["meta", "aaaa", "set", "k\nx", "v"],
        &["meta", "aaaa", "set", "k", "a\nb"],
        &["meta", "aaaa", "set", "a:b", "v"],
    ];
    let mut out = String::new();
    for cmd in commands {
        let run = lida(dir.path(), cmd);
        assert_eq!(run.code, 1, "{cmd:?} must exit 1");
        assert_eq!(
            fs::read(dir.path().join("TODO.md")).expect("read"),
            initial_bytes,
            "{cmd:?} must leave TODO.md unchanged"
        );
        append_run(&mut out, cmd, &run);
    }

    let empty = empty_dir();
    let empty_commands: &[&[&str]] = &[
        &["init", "--md", "--project", "a\nb"],
        &["init", "--md", "--project", "a --> b"],
        &["init", "--yaml", "--md"],
    ];
    for cmd in empty_commands {
        let run = lida(empty.path(), cmd);
        assert_eq!(run.code, 1, "{cmd:?} must exit 1");
        assert!(!empty.path().join("TODO.md").exists());
        assert!(!empty.path().join("TODO.yaml").exists());
        append_run(&mut out, cmd, &run);
    }
    insta::assert_snapshot!("input_validation", redact(&out, dir.path()));
}

/// Keys with ':' are refused only when set; existing ones (TODO.yaml allows them) stay readable and removable.
#[test]
fn meta_colon_keys_stay_readable() {
    let dir = empty_dir();
    let yaml = "tasks:\n- id: aaaa\n  status: todo\n  description: T\n  meta:\n    ci:run: 42\n";
    fs::write(dir.path().join("TODO.yaml"), yaml).expect("write fixture");
    let get = lida(dir.path(), &["meta", "aaaa", "get", "ci:run"]);
    assert_eq!((get.code, get.stdout.as_str()), (0, "42\n"), "{}", get.stderr);
    let unset = lida(dir.path(), &["meta", "aaaa", "unset", "ci:run"]);
    assert_eq!(unset.code, 0, "{}", unset.stderr);
    let set = lida(dir.path(), &["meta", "aaaa", "set", "a:b", "1"]);
    assert_eq!(set.stderr, "error: meta keys cannot contain ':'\n");
}
