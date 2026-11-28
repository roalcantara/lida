# Target File Flag and `--annotate` Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans (or
> superpowers:subagent-driven-development) to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax
> for tracking. Each task ends with an **Acceptance** list: the task is done only when every item holds, and the
> handoff claims cite the test or command that shows it.

**Goal:** A global `-f, --file` / `LIDA_FILE` target for every command, a `stop` command, and `-a, --annotate` on
`add`, `start`, `done` and `stop` that appends a jobx-shaped annotation in the same save.

**Architecture:** The binary resolves the target to a directory once, in `commands::run`, and passes it where `cwd`
goes today; `lida-core` is untouched. Annotations are written into `task.extra["annotations"]` (a YAML sequence of
`{entry, description}` mappings), so the task model does not change.

**Tech Stack:** Rust, usage-rs 7.4 (`global`, `env =`), serde_yaml_ng `Value`, jiff dates, insta snapshots.

**Spec:** `docs/superpowers/specs/2026-11-28-file-flag-and-annotate-design.md`

**Base:** branch `feat/file-flag-and-annotate`, at the commit that adds this plan (`base=$(git log -1 --format=%h -- docs/superpowers/plans/2026-11-28-file-flag-and-annotate.md)`). No owner checkpoints: every decision below is settled.

## Decisions (settled; do not reopen)

| #  | Decision                                                                                                                                                                                                                                | Rejected alternative and why                                                |
| -- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------- |
| D1 | The target is a **directory**; a `TODO.yaml`/`TODO.md` path resolves to its parent. lida then behaves exactly as if run there (its `.lida/config`, dual-write).                                                                         | Exact single file: breaks dual-write sync; needs a core store change.       |
| D2 | One root field `file: Option<String>`, `#[usage(short = 'f', long, global, env = "LIDA_FILE")]`; usage-rs makes the flag win over the variable.                                                                                         | Hand-rolled env lookup: duplicates usage-rs.                                |
| D3 | `init --force` loses `-f`. This is the plan's only breaking change (Task 1's commit only).                                                                                                                                              | `-F`: rejected by the owner.                                                |
| D4 | Resolution lives in the binary (`crates/lida/src/target.rs`). `lida-core` gets no new code, dependency or model field.                                                                                                                  | Core helper: core takes paths already; nothing to add there.                |
| D5 | Annotations are stored in `task.extra["annotations"]` as `{entry: "YYYY-MM-DD", description: "<text>"}`, key order `entry` then `description`, appended last; existing entries and their unknown keys are never rewritten or reordered. | Typed model field or lida comments: pre-empts the schema work in task 8NUT. |
| D6 | `entry` is today's **local date** from the shared `today()` (`LIDA_TODAY` override, as `next` uses), not a timestamp.                                                                                                                   | ISO timestamps: left to 8NUT.                                               |
| D7 | Markdown-only mode refuses `-a` before any write. YAML-only and dual-write accept it (`TODO.md` is regenerated without annotations, as with every unknown field today).                                                                 | Markdown syntax for annotations: out of scope.                              |
| D8 | `stop` sets `todo` from any status, never refuses, never clears `blocked_reason`.                                                                                                                                                       | Refuse unless in-progress: rejected by the owner for now.                   |
| D9 | Output stays one line: the usual success line, plus `, with annotation` when `-a` was given.                                                                                                                                            | A second "Added annotation" line: rejected by the owner.                    |

## Out of scope (do not touch)

`status --set` (no `-a`), `show`/`list`/`next` output, the comment model and `add --task-id`'s comment behaviour,
`lida-core`, `TODO.md` (this repo's task file), a standalone `annotate` command.

## Verified facts (checked against the real code on 2026-11-28)

- usage-rs 7.4 parses the root `global` + `env` field as D2 says: `lida -f a nx`, `lida nx -f b`, `lida --file=e tags`,
  `lida done X -f g -a hi`, bare `lida -f h`, `LIDA_FILE=c lida nx` and `LIDA_FILE=c lida -f d tags` (flag wins) all
  reach `cli.file` with the expected value.
- The `completion` subcommand's own `--force` has no short form, so a global `-f` collides only with `init`.
- serde_yaml_ng writes the string `2026-11-28` unquoted (`entry: 2026-11-28`), as jobx's existing entries.
- `Store::save` in dual-write mode serialises unknown task fields to YAML and skips them in Markdown without error.
  Markdown-only mode has no such guard on save, so D7's explicit refusal is required: without it the annotation would
  be silently lost.

## Global Constraints

- No `println!`; commands write through `out`; errors are returned (`main` prints `error: <message>`, exit 1).
- No crate- or module-wide `allow`; scoped `#[expect(…, reason = "…")]` only. Clippy pedantic, `-D warnings`.
- Every output change updates its snapshot (`INSTA_UPDATE=always cargo nextest run -p lida`, review each `.snap`) and
  DESIGN.md in the same commit. Whenever the CLI declaration changes, regenerate `lida.usage.kdl` with
  `cargo run -q -p lida -- usage > lida.usage.kdl` (`usage_spec_is_complete_and_committed` enforces it).
- A snapshot that is not new in the task must stay unchanged, except the help and usage output listing the new flags.
- Commits: Conventional Commits, sentence-case subject ≤ 72 chars, `git add <paths>` then `git commit`. Run
  `hk fix --all` and then `mise run gate` before each commit. Only Task 1's commit carries a breaking marker.
- Copy, verbatim:
  - suffix `, with annotation`
  - `✓ Task #<ID> marked as todo`
  - `no such directory: <path>`
  - `--file must be a directory, TODO.yaml or TODO.md: <path>`
  - `annotation must be a single line`
  - `TODO.md cannot hold annotations (task #<ID>): stay on yaml or both`
  - `annotations of task #<ID> is not a list`
  - `--annotate cannot be used with --task-id`
- `-a` error order, first match wins; every error leaves every file byte-identical:
  1. `annotation must be a single line`
  2. `--annotate cannot be used with --task-id` (`add` only)
  3. `task #<ID> not found`
  4. the Markdown refusal
  5. `annotations of task #<ID> is not a list`

## Review Focus

1. `annotations` present but not a sequence (hand-edited YAML): refuse, never overwrite. Task 4.
2. `-f` after the subcommand (`lida nx -f <path>`) behaves like before it. Task 1.
3. `-f new/TODO.yaml` for `init` where only `new/` exists: creates `new/TODO.yaml`. Task 1.
4. `add --task-id X -a …`: refuse rather than drop the text. Task 4.
5. Dual-write: `-a` succeeds, `TODO.yaml` keeps the annotation across a later command. Task 4.

## Shared test fixture (introduced in Task 1, reused in Task 4)

In `crates/lida/tests/output_test.rs`, add `fn yaml_dir(extra: &str) -> TempDir`. It writes `TODO.yaml` (and nothing
else, so the store infers YAML-only) as below, with `<extra>` replaced by the `extra` argument, inserted verbatim:

```yaml
tasks:
- id: aaaa
  status: Todo
  description: First task
<extra>
actors: []
comments: []
project: fixture
```

Pass `extra` already indented two spaces as task fields, e.g. `"  annotations: nope"`, or `""` for none. Task 1 needs
only `fixture_dir()`. `yaml_dir` belongs to Task 4; it is listed here so both tasks agree on one helper.

---

### Task 1: Global `-f, --file` and `LIDA_FILE`

**Files:**

- Create: `crates/lida/src/target.rs`
- Modify:
  - `crates/lida/src/main.rs`: `mod target;`
  - `crates/lida/src/cli.rs:7-17`: the root field from D2, doc comment
    `TODO file or its directory (default: current directory)`
  - `cli.rs:115-117`: `InitArgs::force` becomes `#[usage(long)]`
  - `crates/lida/src/commands.rs:40-41`
  - `DESIGN.md`, `lida.usage.kdl`
- Test: `target.rs` unit module; `crates/lida/tests/output_test.rs`, new test `file_flag`

**Interfaces:**

- Produces: `pub fn resolve(file: Option<&str>, cwd: &Path) -> Result<PathBuf, String>` in `target.rs`.

- [ ] **Step 1: Write the failing unit tests** in `target.rs` (`tempfile` is already a dev-dependency). Use a temp dir
      `d` holding an empty directory `sub/` and a file `notes.txt`:
  - `none_is_cwd`: `resolve(None, d)` is `Ok(d)`.
  - `directory_is_used`:
    - `resolve(Some("sub"), d)` is `Ok(d/sub)`; a relative path is joined to `cwd`.
    - `resolve(Some(<absolute d/sub>), <other dir>)` is `Ok(d/sub)`.
  - `todo_file_resolves_to_parent`: `Some("sub/TODO.yaml")` and `Some("sub/TODO.md")` give `Ok(d/sub)`, whether or not
    the file exists.
  - `other_file_name_errors`: `Some("notes.txt")` gives `Err("--file must be a directory, TODO.yaml or TODO.md:
    <d>/notes.txt")`.
  - `missing_path_errors`:
    - `Some("nope")` gives `Err("no such directory: <d>/nope")`.
    - `Some("nope/TODO.yaml")` gives the same error, naming `<d>/nope`.

- [ ] **Step 2: Run** `cargo nextest run -p lida target::` — expected FAIL (`resolve` not defined).

- [ ] **Step 3: Implement `resolve`.**
  1. Join a relative path to `cwd`.
  2. An existing directory is returned as is.
  3. Otherwise, if the file name is `TODO.yaml` or `TODO.md`, return the parent when it is an existing directory, else
     the missing-directory error naming that parent.
  4. Otherwise an existing path gets the wrong-name error, and a missing one the missing-directory error.

- [ ] **Step 4: Run** `cargo nextest run -p lida target::` — expected PASS.

- [ ] **Step 5: Write the failing integration test** `file_flag`. Start from `let d = fixture_dir();` and write
      `d/other/TODO.md` with `FIXTURE.replace("First task", "Other task")`. Then:
  - `insta::assert_snapshot!("file_flag", transcript(d.path(), &[&["-f", "other/TODO.md", "list"], &["list", "-f",
    "other"], &["--file", "other/TODO.md", "done", "aaaa"], &["-f", "nope", "list"]]))`.
  - Assert `d/other/TODO.md` now has `aaaa` done (`- [x] aaaa`) and `d/TODO.md` still has `- [ ] aaaa`.
  - `lida_with_env(d, &["list"], &[("LIDA_FILE", "other")])` stdout contains `Other task`.
  - `lida_with_env(d, &["-f", ".", "list"], &[("LIDA_FILE", "other")])` stdout contains `First task` and not `Other
    task` (flag wins).
  - In `e = empty_dir()` with `e/new/` created: `lida(e, &["-f", "new/TODO.yaml", "init"])` exits 0, `e/new/TODO.yaml`
    exists and `e/TODO.yaml` does not.
  - `lida(e, &["init", "-f"])` exits non-zero (`-f` now wants a value; `init --force` remains the only way to
    force).

- [ ] **Step 6: Run** `cargo nextest run -p lida file_flag` — expected FAIL (unknown flag).

- [ ] **Step 7: Implement.**
  - Add the root field and drop `short = 'f'` from `force`.
  - In `commands::run`, read `cli.file` before `cli.command` is moved, then
    `let cwd = crate::target::resolve(cli.file.as_deref(), &env::current_dir()?)?;`.
  - Regenerate `lida.usage.kdl`.
  - DESIGN.md:
    - New `## Global options` section before `## Commands`, covering `-f, --file <path>`, `LIDA_FILE`, the
      flag-wins rule and the resolution rules (D1).
    - `init`: `--force` has no short form.
    - `## Errors shared by all commands`: both new error lines.

- [ ] **Step 8: Run** `INSTA_UPDATE=always cargo nextest run -p lida`; review `file_flag.snap` and every changed
      snapshot; then `hk fix --all && mise run gate` — expected PASS.

- [ ] **Step 9: Commit**

```bash
git add crates/lida/src/target.rs crates/lida/src/main.rs crates/lida/src/cli.rs crates/lida/src/commands.rs \
  crates/lida/tests/output_test.rs crates/lida/tests/snapshots DESIGN.md lida.usage.kdl
git commit -m "feat(cli)!: Target another TODO file with -f/--file or LIDA_FILE" \
  -m "BREAKING CHANGE: init --force no longer has the -f short form; -f now selects the TODO file."
```

**Acceptance:**

- [ ] All five `target::` unit tests and `file_flag` pass.
- [ ] The `file_flag` snapshot shows:
  - `Other task` for both `list` runs
  - `✓ Task #aaaa marked as done` for the `--file` run
  - exit 1 and `error: no such directory: [DIR]/nope` for the last run
- [ ] `d/TODO.md` is untouched by the `-f` runs; `LIDA_FILE` is honoured and loses to `-f`.
- [ ] `init` creates the file in the `-f` target; `init -f` alone is a parse error.
- [ ] No snapshot other than `file_flag` and the help/usage output changed. `git diff --stat crates/lida-core` is
      empty.
- [ ] `mise run gate` passes. The commit subject has `!`, and its only body line starting with "BREAKING CHANGE" is
      the one above.

### Task 2: Completion follows the target

**Files:**

- Modify:
  - `crates/lida/src/cli.rs:19-22`: `load_current_store`, and its callers `complete_task_id`, `complete_tags`,
    `complete_next_word`
  - `crates/lida/src/target.rs`
- Test: `target.rs` unit module; extend `completion_offers_values` in `output_test.rs`

**Interfaces:**

- Consumes: `target::resolve` (Task 1).
- Produces:
  - `pub fn file_in_words(words: &[String]) -> Option<&str>` in `target.rs`: the value after the last `-f` or `--file`
    word, or after `--file=`. `None` when the flag is absent or is the last word.
  - `fn load_current_store(ctx: &CompleteCtx<'_>) -> Option<Store>` in `cli.rs`:
    1. Take `file_in_words(ctx.words)`, else `std::env::var("LIDA_FILE").ok()`, else none.
    2. `target::resolve(that, &current_dir)`, then `Store::load`.
    3. Any error gives `None` (no candidates, no output on stderr).

- [ ] **Step 1: Write the failing tests.**
  - Unit tests for `file_in_words`:
    - `["lida","-f","x","done",""]` gives `Some("x")`.
    - `["lida","done","--file=y",""]` gives `Some("y")`.
    - `["lida","-f","x","--file","z",""]` gives `Some("z")`.
    - `["lida","done","-f"]` gives `None`.
    - `["lida","done"]` gives `None`.
  - Integration, appended to `completion_offers_values`, with `d/other/TODO.md` written as in Task 1:
    - Line `"lida -f other done "`: stdout contains `Other task`.
    - Line `"lida done "` with env `LIDA_FILE=other`: stdout contains `Other task`.
    - Line `"lida -f nope done "`: exit 0, empty stdout.

- [ ] **Step 2: Run** `cargo nextest run -p lida completion_offers_values target::` — expected FAIL.

- [ ] **Step 3: Implement** `file_in_words` and the new `load_current_store(ctx)`. Each completer passes its `ctx`
      (rename the `_ctx` parameter to `ctx`).

- [ ] **Step 4: Run** `cargo nextest run -p lida`, then `hk fix --all && mise run gate` — expected PASS.

- [ ] **Step 5: Commit** `feat(cli): Complete IDs and tags from the --file target`. Stage `cli.rs`, `target.rs` and
      `output_test.rs`.

**Acceptance:**

- [ ] The five `file_in_words` cases and the three completion cases pass.
- [ ] Completion without `-f` or `LIDA_FILE` still offers `aaaa` and `bbbb` from the current directory (existing
      asserts unchanged).
- [ ] No snapshot changed; `mise run gate` passes.

### Task 3: `lida stop <ID>`

**Files:**

- Modify:
  - `crates/lida/src/cli.rs`: `StopArgs { id }`, a copy of `StartArgs` with the same completer; a `Stop` variant
    with doc comment `Mark a task as todo`, placed right after `Start`
  - `crates/lida/src/commands.rs`: dispatch
  - `crates/lida/src/commands/status.rs`: `stop`
  - `DESIGN.md`: the heading becomes `` ### `done`, `start`, `stop`, `status` ``
  - `lida.usage.kdl`
- Test: `status_changes` in `output_test.rs`

**Interfaces:**

- Produces: `pub fn stop(cwd: &Path, id: &str, out: &mut impl Write) -> CmdResult` in `status.rs`, built on the
  existing private `mark(cwd, id, Status::Todo)`.

- [ ] **Step 1: Write the failing snapshots** in `status_changes`:
  - `snap_fixture("stop", &[&["start", "aaaa"], &["stop", "aaaa"], &["list"]])`
  - `snap_fixture("stop_keeps_reason", &[&["status", "aaaa", "--set", "blocked", "--reason", "Waiting"], &["stop",
    "aaaa"], &["show", "aaaa"]])`
  - `snap_fixture("stop_todo_is_noop_success", &[&["stop", "bbbb"]])`
  - `snap_fixture("stop_unknown", &[&["stop", "zzzz"]])`

- [ ] **Step 2: Run** `cargo nextest run -p lida status_changes` — expected FAIL (new snapshots pending, `stop`
      unknown).

- [ ] **Step 3: Implement** `stop` and its dispatch. Regenerate `lida.usage.kdl`. In DESIGN.md add
      `$ lida stop aaaa` / `✓ Task #aaaa marked as todo` to the example block, and the rule: "`stop` sets `todo` from
      any status; like `done` and `start`, it never clears the reason."

- [ ] **Step 4: Run** `INSTA_UPDATE=always cargo nextest run -p lida`; review the four new snapshots; then
      `hk fix --all && mise run gate`.

- [ ] **Step 5: Commit** `feat(cli): Add stop to send a task back to todo`. Stage the files above and the four new
      `.snap` files.

**Acceptance:**

- [ ] `stop`: `✓ Task #aaaa marked as todo`, exit 0, and `list` shows `aaaa` open.
- [ ] `stop_keeps_reason`: `show` still prints the reason `Waiting` and status `Todo`.
- [ ] `stop_todo_is_noop_success`: exit 0, same success line.
- [ ] `stop_unknown`: exit 1, `error: task #zzzz not found`.
- [ ] Every pre-existing snapshot is unchanged except help/usage listing `stop`. `mise run gate` passes.

### Task 4: `-a, --annotate` on `add`, `start`, `done` and `stop`

**Files:**

- Create: `crates/lida/src/commands/annotation.rs`
- Modify:
  - `crates/lida/src/cli.rs`: `annotate: Option<String>` with `#[usage(short = 'a', long)]` and doc comment
    `Annotation to append to the task`, on `AddArgs`, `DoneArgs`, `StartArgs` and `StopArgs`
  - `crates/lida/src/commands.rs`: `mod annotation;`; `fn today() -> Date` extracted verbatim from the `Next` arm
    (`LIDA_TODAY`, else `jiff::Zoned::now().date()`), which `next` then calls
  - `crates/lida/src/commands/status.rs`, `crates/lida/src/commands/add.rs`
  - `DESIGN.md`, `CONTEXT.md`, `lida.usage.kdl`
- Test: `annotation.rs` unit module; new test `annotate` in `output_test.rs`, with the shared `yaml_dir` helper

**Interfaces:**

- Consumes: `stop` (Task 3); `single_line`, `not_found` (`commands.rs`).
- Produces, in `annotation.rs`:
  - `pub(super) fn check(store: &Store, id: &str, text: &str) -> Result<(), String>`. Runs the global error order's
    steps 3–5: not found, the Markdown refusal when `store.format() == StorageFormat::MdOnly`, not a list. Step 1
    (`single_line("annotation", text)`) runs before the store is loaded.
  - `pub(super) fn append(task: &mut Task, text: &str, today: Date) -> Result<(), String>`. Implements D5. It creates
    the sequence when absent and returns the not-a-list error, with `extra` untouched, when `annotations` exists but
    is not a sequence.
- Changes:
  - `status::{done, start, stop}` gain `annotate: Option<&str>, today: Date`, and the private `mark` takes them too.
    `mark` runs `check` before mutating and `append` before `save`, then prints the line plus D9's suffix.
  - `add::Args` gains `annotate: Option<String>`, and `add::run` gains `today: Date`.
  - `add` checks `--task-id` together with `-a` (error 2), then the Markdown refusal, before generating an ID. For a
    new task, `append` runs on it before `store.add_task`.

- [ ] **Step 1: Write the failing unit tests** in `annotation.rs`, with `today = date(2026, 11, 28)`:
  - `appends_to_absent`: `append` on a fresh `Task::new("aaaa","x")` makes `extra["annotations"]` exactly
    `[{entry: "2026-11-28", description: "note"}]`.
  - `keeps_existing_and_appends_last`: with an existing `[{entry: "2026-11-25", description: "old", foo: "bar"}]`, the
    first element is equal to it before and after, and the second is the new one.
  - `refuses_non_list`: with `annotations: "nope"`, it returns `Err("annotations of task #aaaa is not a list")` and
    `extra` is unchanged.

- [ ] **Step 2: Run** `cargo nextest run -p lida annotation::` — expected FAIL.

- [ ] **Step 3: Implement `annotation.rs`.**

- [ ] **Step 4: Run** `cargo nextest run -p lida annotation::` — expected PASS.

- [ ] **Step 5: Write the failing integration test** `annotate`. Run every command with `LIDA_TODAY=2026-11-28`
      through `lida_with_env` or `transcript_with_env`, and add `yaml_dir` as specified above.
  - **Lifecycle:** in `y = yaml_dir("")`, snapshot `annotate_yaml` of `start aaaa -a "picked up"`,
    `stop aaaa -a paused`, `done aaaa -a shipped` and `done aaaa`. Then assert `TODO.yaml` contains this block once,
    verbatim:

    ```yaml
    annotations:
    - entry: 2026-11-28
      description: picked up
    - entry: 2026-11-28
      description: paused
    - entry: 2026-11-28
      description: shipped
    ```

  - **Existing entries kept:** `yaml_dir("  annotations:\n  - entry: 2026-11-25\n    description: old\n    foo:
    bar")`, then `done aaaa -a new`. The file contains the old entry with `foo: bar` unchanged, followed by the new
    one.
  - **Not a list:** `yaml_dir("  annotations: nope")`, then `done aaaa -a x`. Exit 1,
    `error: annotations of task #aaaa is not a list`, file byte-identical.
  - **Markdown-only:** `fixture_dir()`, then `done aaaa -a x`. Exit 1, `error: TODO.md cannot hold annotations (task
    #aaaa): stay on yaml or both`, `TODO.md` byte-identical. `done aaaa` without `-a` still exits 0.
  - **Multi-line:** `done aaaa -a "a\nb"` in `yaml_dir("")` exits 1 with `error: annotation must be a single line`.
  - **Unknown ID:** `done zzzz -a x` in `fixture_dir()` exits 1 with `error: task #zzzz not found` (not-found comes
    before the Markdown refusal).
  - **Dual-write:** in `b = empty_dir()`, run `init --both`, then `add First -a created`. Stdout matches
    `✓ Added task #<ID> "First", with annotation`. Then `done <ID> -a x`. `TODO.yaml` contains
    `description: created` and `description: x`, and `list` exits 0 afterwards.
  - **Comment mode:** `add --task-id aaaa -c hi -a x` in `fixture_dir()` exits 1 with
    `error: --annotate cannot be used with --task-id`, and `TODO.md` is byte-identical.

- [ ] **Step 6: Run** `cargo nextest run -p lida annotate` — expected FAIL (unknown flag `-a`).

- [ ] **Step 7: Implement** the CLI fields, `today()`, the `status.rs` and `add.rs` changes, and the dispatch.
  - Regenerate `lida.usage.kdl`.
  - DESIGN.md:
    - `add`: example `$ lida add "Third task" -a "from review"` printing
      `✓ Added task #Xy7Q "Third task", with annotation`.
    - `done`/`start`/`stop`/`status`: example `$ lida done aaaa -a shipped` printing
      `✓ Task #aaaa marked as done, with annotation`.
    - A paragraph covering the stored shape, the `entry` date, the Markdown refusal and the error order.
    - The new error lines under `## Errors shared by all commands`.
  - CONTEXT.md, after **comment**: ``- **annotation**: A dated note in a task's \``annotations\` list (\`entry\`,
    \`description\`), TaskWarrior's shape; appended by \`-a\`/\`--annotate\`; YAML only.`

- [ ] **Step 8: Run** `INSTA_UPDATE=always cargo nextest run -p lida`; review the snapshots. Then
      `hk fix --all && mise run gate`.

- [ ] **Step 9: Commit** `feat(cli): Annotate a task while adding, starting, stopping or finishing it`. Stage
      `annotation.rs`, `cli.rs`, `commands.rs`, `status.rs`, `add.rs`, `output_test.rs`, the new snapshots,
      `DESIGN.md`, `CONTEXT.md` and `lida.usage.kdl`.

**Acceptance:**

- [ ] The three unit tests and every `annotate` case pass.
- [ ] `annotate_yaml` shows four lines: three ending `, with annotation` and a final `✓ Task #aaaa marked as done`
      without it.
- [ ] The YAML block above appears verbatim, dates unquoted.
- [ ] Every refused case exits 1 with its exact message, and the file is byte-identical (compare bytes read before and
      after).
- [ ] The pre-existing snapshots `done`, `start`, `stop`, `add*` and `next*` are unchanged; only help/usage output
      gains `-a`.
- [ ] `git diff --stat crates/lida-core` is empty for the whole branch.
- [ ] `mise run gate` passes.

## Branch acceptance (checked by the reviewer after Task 4)

- [ ] Four commits, one per task, in order; only the first is breaking.
- [ ] `git diff "$base" --stat -- crates/lida-core TODO.md` is empty.
- [ ] From `~/Work/lida`, against a copy of `~/Work/kb/docs/jobx`:
  - `lida -f <copy>/TODO.yaml nx` lists jobx tasks.
  - `lida -f <copy>/TODO.yaml done GmuC -a "test"` appends one entry after GmuC's two existing annotations, and the
    rest of the file is unchanged (`diff` shows only the status line and the three new lines).
- [ ] DESIGN.md documents every new flag, command, line and error; CONTEXT.md defines **annotation**.
