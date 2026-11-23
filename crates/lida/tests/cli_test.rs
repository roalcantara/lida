use std::path::Path;
use std::process::Command;
use tempfile::tempdir;

/// Runs `lida` in `dir`, asserts it succeeded, and returns its stdout.
#[expect(clippy::expect_used, reason = "test setup helper: a failed setup is a failed test")]
fn run_ok(dir: &Path, args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_lida"))
        .args(args)
        .current_dir(dir)
        .output()
        .expect("lida binary runs");
    assert!(
        output.status.success(),
        "lida {args:?} failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[expect(clippy::expect_used, reason = "test setup helper: a failed setup is a failed test")]
fn read_yaml(dir: &Path) -> String {
    std::fs::read_to_string(dir.join("TODO.yaml")).expect("TODO.yaml is readable")
}

#[test]
fn test_cli_lifecycle() {
    let dir = tempdir().unwrap();
    let dir = dir.path();

    // 1. init
    run_ok(dir, &["init", "--both"]);
    assert!(dir.join("TODO.yaml").exists());
    assert!(dir.join("TODO.md").exists());

    // 2. add with comment and tags; extract the ID from "Added task #abcd"
    let stdout = run_ok(
        dir,
        &["add", "Test Task", "-c", "First note", "-t", "cli,test", "-p", "high"],
    );
    let id_line = stdout.lines().find(|l| l.contains("Added task")).unwrap();
    let id = id_line.split('#').nth(1).unwrap().split_whitespace().next().unwrap();

    // 3. list --json, and the initial comment is stored in TODO.yaml
    let list_json = run_ok(dir, &["list", "--json"]);
    assert!(list_json.contains(id));
    assert!(list_json.contains("Test Task"));
    assert!(read_yaml(dir).contains("First note"));

    // 4. done
    run_ok(dir, &["done", id]);

    // 5. meta set and get
    run_ok(dir, &["meta", id, "set", "devto.id", "999"]);
    assert_eq!(run_ok(dir, &["meta", id, "get", "devto.id"]).trim(), "999");

    // 6. legacy comment attachment: add --comment <text> --task-id <id>
    run_ok(dir, &["add", "--comment", "Second note via --task-id", "--task-id", id]);
    assert!(read_yaml(dir).contains("Second note via --task-id"));

    // 7. update, 8. French status alias
    run_ok(
        dir,
        &[
            "update",
            id,
            "--tag",
            "post,guide",
            "--position",
            "docs/blog/posts/01-guide",
        ],
    );
    run_ok(dir, &["status", id, "--set", "en-cours"]);

    // 9. delete with orphaned comment cleanup
    run_ok(dir, &["delete", id]);
    let yaml_final = read_yaml(dir);
    assert!(!yaml_final.contains(id));
    assert!(!yaml_final.contains("Second note via --task-id"));
}
