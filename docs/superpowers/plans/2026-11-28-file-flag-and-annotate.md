# Target File Flag and `--annotate` Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or
> superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** A global `-f, --file` / `LIDA_FILE` target for every command, a `stop` command, and `-a, --annotate` on
`add`, `start`, `done` and `stop` that appends a jobx-shaped annotation in the same save.

**Architecture:** The binary resolves the target to a directory once in `commands::run` and passes it where `cwd` goes
today; `lida-core` is untouched. Annotations are written into `task.extra["annotations"]` (a YAML sequence of
`{entry, description}` mappings), so the task model does not change.

**Tech Stack:** Rust, usage-rs 7.4 (`global`, `env =`), serde_yaml_ng `Value`, jiff dates, insta snapshots.

**Spec:** `docs/superpowers/specs/2026-11-28-file-flag-and-annotate-design.md`

## Global Constraints

- `lida-core` gains no dependency and no model change; all new code lives in `crates/lida`.
- No `println!`; commands write through `out`; errors are returned (`main` prints `error: <message>`, exit 1).
- No crate- or module-wide `allow`; scoped `#[expect(…, reason = "…")]` only. Clippy pedantic, `-D warnings`.
- Every output change updates its snapshot (`INSTA_UPDATE=always cargo nextest run -p lida`, review each `.snap`) and
  DESIGN.md in the same commit; `lida.usage.kdl` is regenerated (`cargo run -q -p lida -- usage > lida.usage.kdl`)
  whenever the CLI declaration changes (`usage_spec_is_complete_and_committed` enforces it).
- Commits: Conventional Commits, sentence-case subject ≤ 72 chars, `git add <paths>` then `git commit`. Run
  `hk fix --all` before `mise run gate --fast`. Only Task 1's commit carries the breaking marker.
- Copy, verbatim: success suffix `, with annotation`; stop line `✓ Task #<ID> marked as todo`; Markdown refusal
  `TODO.md cannot hold annotations (task #<ID>): stay on yaml or both`.

## Review Focus

1. `annotations` present but not a sequence (hand-edited YAML): `-a` must refuse with
   `annotations of task #<ID> is not a list`, never overwrite it. Test in Task 4.
2. `-f` given after the subcommand (`lida nx -f <path>`) must work like before it. Test in Task 1.
3. `-f path/TODO.yaml` for `init` where the file does not exist yet: resolves to the existing parent and creates it
   there. Test in Task 1.
4. `add --task-id X -a "…"` (comment mode): refuse with `--annotate cannot be used with --task-id` rather than
   silently dropping the text. Test in Task 4.
5. Dual-write (`both`) mode: `-a` succeeds, `TODO.yaml` holds the annotation, `TODO.md` is regenerated without it and
   the next load still has it. Test in Task 4.

---

### Task 1: Global `-f, --file` and `LIDA_FILE`

**Files:**

- Create: `crates/lida/src/target.rs`
- Modify: `crates/lida/src/main.rs` (add `mod target;`), `crates/lida/src/cli.rs:7-17` (root field), `cli.rs:115-117`
  (drop `short = 'f'` from `InitArgs::force`), `crates/lida/src/commands.rs:40-41` (resolve target)
- Modify: `DESIGN.md` (new "Global options" section before `## Commands`; `init` paragraph says `--force` has no
  short form), `lida.usage.kdl`
- Test: `crates/lida/src/target.rs` (unit), `crates/lida/tests/output_test.rs` (new `file_flag` test)

**Interfaces:**

- Produces: `LidaCli::file: Option<String>` declared `#[usage(short = 'f', long, global, env = "LIDA_FILE")]`, doc
  comment `TODO file or its directory (default: current directory)`.
- Produces: `pub fn resolve(file: Option<&str>, cwd: &Path) -> Result<PathBuf, String>` in `target.rs`.

- [ ] **Step 1: Write the failing unit tests** in `target.rs`'s `#[cfg(test)]` module, using a `tempfile` dir `d`
      holding an empty `sub/` directory and a file `notes.txt`:
  - `none_is_cwd`: `resolve(None, d)` == `d`.
  - `directory_is_used`: `resolve(Some("sub"), d)` == `d/sub` (relative joined to `cwd`); an absolute path is used as
    is.
  - `todo_file_resolves_to_parent`: `resolve(Some("sub/TODO.yaml"), d)` and `Some("sub/TODO.md")` == `d/sub`, whether
    or not the file exists.
  - `other_file_name_errors`: `Some("notes.txt")` → `Err("--file must be a directory, TODO.yaml or TODO.md:
    <path>")`, where `<path>` is the joined path.
  - `missing_path_errors`: `Some("nope")` and `Some("nope/TODO.yaml")` → `Err("no such directory: <path>")` (the
    directory that was looked for).

- [ ] **Step 2: Run** `cargo nextest run -p lida target::` — expected FAIL (`resolve` not defined).

- [ ] **Step 3: Implement `resolve`.** Join a relative path to `cwd`. If it is an existing directory, return it. Else if
      its file name is `TODO.yaml` or `TODO.md`, return its parent when that is an existing directory, else the
      missing-directory error. Else if it exists, the wrong-name error; else the missing-directory error.

- [ ] **Step 4: Run** `cargo nextest run -p lida target::` — expected PASS.

- [ ] **Step 5: Write the failing integration test** `file_flag` in `output_test.rs`: in `fixture_dir()` `d`, create
      `d/other/TODO.md` with `FIXTURE` with `aaaa`'s description changed to `Other task`; then, running in `d`:
  - `snap_fixture("file_flag", …)` style transcript of `["-f", "other/TODO.md", "list"]`, `["list", "-f", "other"]`
    and `["--file", "other/TODO.md", "done", "aaaa"]`; the snapshot shows `Other task` and the done line.
  - `lida_with_env(d, &["list"], &[("LIDA_FILE", "other")])` stdout contains `Other task`;
    `lida_with_env(d, &["-f", ".", "list"], &[("LIDA_FILE", "other")])` stdout contains `First task` (flag wins).
  - In an `empty_dir()` `e` with `e/new/`: `["-f", "new/TODO.yaml", "init"]` exits 0 and creates `e/new/TODO.yaml`.
  - `["-f", "nope", "list"]` exits 1 with stderr `error: no such directory: [DIR]/nope` (after `redact`).

- [ ] **Step 6: Run** `cargo nextest run -p lida file_flag` — expected FAIL (unknown flag `-f`).

- [ ] **Step 7: Implement.** Add the root field; remove `short = 'f'` from `InitArgs::force`; in `commands::run`
      replace `let cwd = env::current_dir()?;` with `let cwd = crate::target::resolve(cli.file.as_deref(),
      &env::current_dir()?)?;` (read `cli.file` before `cli.command` is moved). Regenerate `lida.usage.kdl`. Write the
      DESIGN.md "Global options" section: the flag, `LIDA_FILE`, flag-wins, the three resolution rules and both error
      lines.

- [ ] **Step 8: Run** `INSTA_UPDATE=always cargo nextest run -p lida`, review the new `file_flag` snapshot and any
      changed help/usage snapshot (only the new flag and `init --force`'s missing `-f` may differ), then
      `hk fix --all && mise run gate --fast` — expected PASS.

- [ ] **Step 9: Commit**

```bash
git add crates/lida/src/target.rs crates/lida/src/main.rs crates/lida/src/cli.rs crates/lida/src/commands.rs \
  crates/lida/tests/output_test.rs crates/lida/tests/snapshots DESIGN.md lida.usage.kdl
git commit -m "feat(cli)!: Target another TODO file with -f/--file or LIDA_FILE" \
  -m "BREAKING CHANGE: init --force no longer has the -f short form; -f now selects the TODO file."
```

### Task 2: Completion follows the target

**Files:**

- Modify: `crates/lida/src/cli.rs:19-22` (`load_current_store`) and its three callers (`complete_task_id`,
  `complete_tags`, `complete_next_word`)
- Modify: `crates/lida/src/target.rs` (new helper + unit tests)
- Test: `crates/lida/tests/output_test.rs` (extend `completion_offers_values`)

**Interfaces:**

- Consumes: `target::resolve` (Task 1).
- Produces: `pub fn file_in_words(words: &[String]) -> Option<&str>` in `target.rs`: the value after the last `-f` or
  `--file`, or after `--file=`; `None` when absent or when the flag is the last word.
- Produces: `fn load_current_store(ctx: &CompleteCtx<'_>) -> Option<Store>`: `file_in_words(ctx.words)`, else
  `std::env::var("LIDA_FILE")`, else none; then `resolve(…, &current_dir)` and `Store::load`.

- [ ] **Step 1: Write the failing tests.** Unit: `file_in_words` on `["lida","-f","x","done",""]` → `Some("x")`;
      `["lida","done","--file=y",""]` → `Some("y")`; `["lida","done","-f"]` → `None`; `["lida","done"]` → `None`.
      Integration, appended to `completion_offers_values` with the `other/` fixture from Task 1: line
      `lida -f other done ` offers `aaaa` described `Other task` (assert stdout contains `Other task`), and
      `lida done ` with env `LIDA_FILE=other` does too.

- [ ] **Step 2: Run** `cargo nextest run -p lida completion_offers_values target::` — expected FAIL.

- [ ] **Step 3: Implement** `file_in_words` and the new `load_current_store(ctx)`; pass `ctx` from each completer
      (rename their `_ctx` parameter to `ctx`).

- [ ] **Step 4: Run** `cargo nextest run -p lida` then `hk fix --all && mise run gate --fast` — expected PASS, no
      snapshot changes.

- [ ] **Step 5: Commit** `feat(cli): Complete IDs and tags from the --file target` (stage `cli.rs`, `target.rs`,
      `output_test.rs`).

### Task 3: `lida stop <ID>`

**Files:**

- Modify: `crates/lida/src/cli.rs` (`StopArgs { id }` like `StartArgs`; variant `/// Mark a task as todo` `Stop`
  after `Start`), `crates/lida/src/commands.rs` (dispatch), `crates/lida/src/commands/status.rs` (`stop`)
- Modify: `DESIGN.md` (`### done, start, stop, status`: example and "`stop` sets `todo` from any status and never
  clears the blocked reason"), `lida.usage.kdl`
- Test: `crates/lida/tests/output_test.rs` (`status_changes`)

**Interfaces:**

- Produces: `pub fn stop(cwd: &Path, id: &str, out: &mut impl Write) -> CmdResult` in `status.rs`.

- [ ] **Step 1: Write the failing snapshots** in `status_changes`: `snap_fixture("stop", &[&["start", "aaaa"],
      &["stop", "aaaa"], &["list"]])` and `snap_fixture("stop_keeps_reason", &[&["status", "aaaa", "--set",
      "blocked", "--reason", "Waiting"], &["stop", "aaaa"], &["show", "aaaa"]])`.

- [ ] **Step 2: Run** `INSTA_UPDATE=always cargo nextest run -p lida status_changes` — expected: new snapshots show
      `error: unknown subcommand` (or usage-rs's equivalent); discard them.

- [ ] **Step 3: Implement** `stop` with the existing `mark(cwd, id, Status::Todo)` and the line
      `✓ Task #<ID> marked as todo`; dispatch `Subcommand::Stop`. Regenerate `lida.usage.kdl`; update DESIGN.md.

- [ ] **Step 4: Run** `INSTA_UPDATE=always cargo nextest run -p lida`; review: `stop` shows `marked as todo` and `aaaa`
      open again; `stop_keeps_reason` still shows the reason `Waiting`. Then `hk fix --all && mise run gate --fast`.

- [ ] **Step 5: Commit** `feat(cli): Add stop to send a task back to todo` (stage the files above and the two new
      `.snap` files).

### Task 4: `-a, --annotate` on `add`, `start`, `done` and `stop`

**Files:**

- Create: `crates/lida/src/commands/annotation.rs`
- Modify: `crates/lida/src/cli.rs` (`annotate: Option<String>` with `#[usage(short = 'a', long)]`, doc
  `Annotation to append to the task`, on `AddArgs`, `DoneArgs`, `StartArgs`, `StopArgs`),
  `crates/lida/src/commands.rs` (`mod annotation;`; a shared `fn today() -> Date` extracted from the `Next` arm —
  `LIDA_TODAY` else `jiff::Zoned::now().date()` — used by both), `crates/lida/src/commands/status.rs`,
  `crates/lida/src/commands/add.rs`
- Modify: `DESIGN.md` (`add` and `done, start, stop, status`: `-a` example, suffix, Markdown refusal, `entry` format),
  `lida.usage.kdl`
- Test: `annotation.rs` (unit), `crates/lida/tests/output_test.rs` (new `annotate` test)

**Interfaces:**

- Consumes: `stop` (Task 3); `single_line` (`commands.rs`).
- Produces, in `annotation.rs`:
  - `pub(super) fn check(store: &Store, id: &str, text: &str) -> Result<(), String>`: `single_line("annotation",
    text)`, then the Markdown refusal when `store.format() == StorageFormat::MdOnly`.
  - `pub(super) fn append(task: &mut Task, text: &str, today: Date) -> Result<(), String>`: pushes a mapping
    `entry: <today as YYYY-MM-DD string>`, `description: <text>` (in that key order) onto
    `task.extra["annotations"]`, creating the sequence when absent; `annotations of task #<ID> is not a list` when it
    exists and is not a sequence.
- Changes: `status::{done, start, stop}` gain `annotate: Option<&str>, today: Date`; the private `mark` takes them
  too and calls `check` before mutating and `append` before `save`. `add::Args` gains `annotate: Option<String>`;
  `add::run` gains `today: Date`.

- [ ] **Step 1: Write the failing unit tests** in `annotation.rs`: `append` on a task with no `extra` yields
      `annotations: [{entry: "2026-11-28", description: "x"}]`; on a task with one existing annotation (any extra
      key, e.g. `foo: 1`) the old entry is unchanged (including `foo`) and the new one is last; on `annotations: "s"`
      returns the not-a-list error and leaves `extra` untouched. `check` with text `"a\nb"` → `annotation must be a
      single line`.

- [ ] **Step 2: Run** `cargo nextest run -p lida annotation::` — expected FAIL.

- [ ] **Step 3: Implement `annotation.rs`.**

- [ ] **Step 4: Run** `cargo nextest run -p lida annotation::` — expected PASS.

- [ ] **Step 5: Write the failing integration test** `annotate`, with env `LIDA_TODAY=2026-11-28`:
  - YAML store (`init --yaml` then `add "First" -a "created"`, using the redacted `[ID]`, or write a `TODO.yaml`
    fixture with task `aaaa`): snapshot `annotate_yaml` of `start aaaa -a "picked up"`, `stop aaaa -a "paused"`,
    `done aaaa -a "shipped"`; each line ends `, with annotation`. Then assert `TODO.yaml` contains, in order,
    `entry: 2026-11-28` (unquoted) and `description: picked up`, `description: paused`, `description: shipped`.
  - A fixture task with an existing `annotations: [{entry: 2026-11-25, description: old, foo: bar}]`: after `done -a`
    the file still contains `foo: bar` and `description: old` before the new entry.
  - `annotations: nope` fixture: `done aaaa -a x` exits 1 with `error: annotations of task #aaaa is not a list`; the
    file is byte-identical afterwards.
  - Markdown fixture (`fixture_dir()`): `done aaaa -a x` exits 1 with `error: TODO.md cannot hold annotations (task
    #aaaa): stay on yaml or both`; `TODO.md` byte-identical; `done aaaa` without `-a` still succeeds.
  - `init --both` store: `done <id> -a x` exits 0; `TODO.yaml` contains `description: x`; `show <id>` exits 0.
  - `add --task-id aaaa -c hi -a x` exits 1 with `error: --annotate cannot be used with --task-id`.

- [ ] **Step 6: Run** `cargo nextest run -p lida annotate` — expected FAIL (unknown flag `-a`).

- [ ] **Step 7: Implement** the CLI fields, `today()`, the `status.rs` and `add.rs` changes, and dispatch. Success
      lines append `, with annotation` only when `-a` was given. Regenerate `lida.usage.kdl`; update DESIGN.md.

- [ ] **Step 8: Run** `INSTA_UPDATE=always cargo nextest run -p lida`; review: only the new `annotate_*` snapshots and
      help/usage snapshots showing `-a` change; `done`, `start`, `stop` snapshots without `-a` are unchanged. Then
      `hk fix --all && mise run gate`.

- [ ] **Step 9: Commit** `feat(cli): Annotate a task while adding, starting, stopping or finishing it` (stage
      `annotation.rs`, `cli.rs`, `commands.rs`, `status.rs`, `add.rs`, `output_test.rs`, new snapshots, `DESIGN.md`,
      `lida.usage.kdl`).
