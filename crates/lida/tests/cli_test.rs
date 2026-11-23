use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_cli_lifecycle() {
    let dir = tempdir().unwrap();
    let bin = env!("CARGO_BIN_EXE_lida");

    // 1. init
    let output = Command::new(bin)
        .arg("init")
        .arg("--both")
        .current_dir(dir.path())
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "init failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(dir.path().join("TODO.yaml").exists());
    assert!(dir.path().join("TODO.md").exists());

    // 2. add with comment and tags
    let output = Command::new(bin)
        .arg("add")
        .arg("Test Task")
        .arg("-c")
        .arg("First note")
        .arg("-t")
        .arg("cli,test")
        .arg("-p")
        .arg("high")
        .current_dir(dir.path())
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "add failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Added task"));

    // Extract ID (e.g. Added task #abcd)
    let id_line = stdout.lines().find(|l| l.contains("Added task")).unwrap();
    let id = id_line.split('#').nth(1).unwrap().split_whitespace().next().unwrap();

    // 3. list --json
    let output = Command::new(bin)
        .arg("list")
        .arg("--json")
        .current_dir(dir.path())
        .output()
        .unwrap();
    assert!(output.status.success());
    let list_json = String::from_utf8_lossy(&output.stdout);
    assert!(list_json.contains(id));
    assert!(list_json.contains("Test Task"));

    // Verify initial comment was stored in TODO.yaml
    let yaml_str = std::fs::read_to_string(dir.path().join("TODO.yaml")).unwrap();
    assert!(yaml_str.contains("First note"));

    // 4. done
    let output = Command::new(bin)
        .arg("done")
        .arg(id)
        .current_dir(dir.path())
        .output()
        .unwrap();
    assert!(output.status.success());

    // 5. meta set and get
    let output = Command::new(bin)
        .arg("meta")
        .arg(id)
        .arg("set")
        .arg("devto.id")
        .arg("999")
        .current_dir(dir.path())
        .output()
        .unwrap();
    assert!(output.status.success());

    let output = Command::new(bin)
        .arg("meta")
        .arg(id)
        .arg("get")
        .arg("devto.id")
        .current_dir(dir.path())
        .output()
        .unwrap();
    assert!(output.status.success());
    let meta_val = String::from_utf8_lossy(&output.stdout);
    assert_eq!(meta_val.trim(), "999");

    // 6. legacy comment attachment: add --comment <text> --task-id <id>
    let output = Command::new(bin)
        .arg("add")
        .arg("--comment")
        .arg("Second note via --task-id")
        .arg("--task-id")
        .arg(id)
        .current_dir(dir.path())
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "add --comment --task-id failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );
    let yaml_str_after = std::fs::read_to_string(dir.path().join("TODO.yaml")).unwrap();
    assert!(yaml_str_after.contains("Second note via --task-id"));

    // 7. update command
    let output = Command::new(bin)
        .arg("update")
        .arg(id)
        .arg("--tag")
        .arg("post,guide")
        .arg("--position")
        .arg("docs/blog/posts/01-guide")
        .current_dir(dir.path())
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "update failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    // 8. French status alias: status <id> --set en-cours
    let output = Command::new(bin)
        .arg("status")
        .arg(id)
        .arg("--set")
        .arg("en-cours")
        .current_dir(dir.path())
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "status en-cours failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    // 9. delete with orphaned comment cleanup
    let output = Command::new(bin)
        .arg("delete")
        .arg(id)
        .current_dir(dir.path())
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "delete failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );
    let yaml_final = std::fs::read_to_string(dir.path().join("TODO.yaml")).unwrap();
    assert!(!yaml_final.contains(id));
    assert!(!yaml_final.contains("Second note via --task-id"));
}
