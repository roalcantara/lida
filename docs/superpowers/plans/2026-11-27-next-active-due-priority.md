# `lida next`: in-progress first, due tags, readable priority — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or
> superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** In `lida next`, list in-progress tasks first, add the due tags `+OVERDUE` `+TODAY` `+TOMORROW` `+WEEK`, and
draw priority as full-size bold letters in one warm colour family.

**Architecture:** Ordering and due-tag matching are pure rules in `lida-core` (`next.rs`, `query.rs`), so table,
`--raw` and `--json` agree. The priority letter is a rendering change in the binary only (`commands/next/palette.rs`,
`row.rs`). Completion and the CLI help line live in `crates/lida/src/cli.rs`.

**Tech Stack:** Rust 2024 workspace, `jiff` civil dates (already in `lida-core`), `usage-rs` completion, `insta`
snapshots, `cargo nextest`.

**Spec:** `docs/superpowers/specs/2026-11-27-next-active-due-priority-design.md`. It is the only source of
architectural decisions; where this plan and the spec differ, the spec wins and the plan is the bug. Read it first.

## Global Constraints

- `lida-core` must never depend on `usage-rs`, `usage`, `owo-colors` or `lida` (hk `core-purity` step).
- Lints unchanged: `unsafe_code = "forbid"`, clippy `pedantic`, `print_stdout`/`print_stderr`, `-D warnings`, jscpd
  threshold 0. No crate- or module-wide `allow`; only scoped `#[expect(…, reason = "…")]`.
- `next` stays read-only; no `Store::save`.
- Only `next` output changes. Every snapshot outside `crates/lida/tests/snapshots/next_test__*.snap` stays byte for
  byte.
- Every output change updates its snapshot (`INSTA_UPDATE=always cargo nextest run -p lida`, then read each `.snap`
  diff) **and** `DESIGN.md` in the same commit.
- Exact colours: H `#ff9e64`, M `#e0af68`, L `#cec0ac`, all bold; blank cell when no priority.
- Due tags are exactly `OVERDUE`, `TODAY`, `TOMORROW`, `WEEK`, uppercase, case-sensitive. Several due tags OR together;
  due tags AND with every other clause. A task without a parseable `due` never matches a due tag.
- `+WEEK` = due in today through today + 6 (same window as `classify`'s `Soon` lane plus today), not a calendar week.
- Commits: Conventional Commits, sentence-case subject ≤ 72 chars, stage named paths with `git add <paths>`, then
  `git commit`. Run `hk fix --all` before the gate. Never start a body line with "BREAKING CHANGE".
- `TODO.md` is written only by `lida`; never edit it by hand.

## Review Focus

1. A `due` stored with a time (`"2026-12-03 09:00"`) counts as that day for every due tag (Task 2 test
   `due_tags_by_day`, `jjjj`).
2. An unparsable `due` (`"tomorrow"`, task `llll`) matches no due tag and does not panic (Task 2 `due_tags_by_day`).
3. Lower- or mixed-case `+today` / `+Today` stays an ordinary tag and matches nothing in the fixture (Task 2
   `due_tags_are_case_sensitive`).
4. A repeated due tag (`+TODAY +TODAY`) behaves like one (Task 2 `due_tags_or_together`).
5. A due tag beside a non-matching ordinary tag returns nothing: the OR must not leak into tag AND (Task 2
   `due_tags_and_with_other_clauses`).

## Fixture facts (do not change `crates/lida-core/tests/fixtures/next.yaml`)

Open tasks with a `due`, relative to the tests' `TODAY` 2026-12-03 (Thursday), urgency in brackets:
`aaaa` 2026-11-27 (23), `cccc` 2026-12-02 (18), `jjjj` "2026-12-03 09:00" (2.8), `dddd` 2026-12-09 (11.9, exactly
today + 6), `iiii` 2027-01-08 (12, Blocked), `kkkk` 2028-01-15, `llll` "tomorrow" (unparsable). `bbbb` is the only
`InProgress` task (urgency 5.8, no due). Tests needing another day pass a different `Date` (core) or override
`LIDA_TODAY` via `run_lida`'s `envs` (integration; later `.envs` override the default).

---

### Task 1: In-progress tasks sort first

**Files:**

- Modify: `crates/lida-core/src/next.rs` (`compare_entries`, doc comment of `next`)
- Test: `crates/lida-core/tests/next_test.rs`, `crates/lida/tests/next_test.rs`
- Snapshots: `crates/lida/tests/snapshots/next_test__next_default_table.snap`, `…next_long_table.snap`
- Docs: `DESIGN.md` (`### next (also nx)` first paragraph)

**Interfaces:**

- Consumes: nothing new.
- Produces: `next()` keeps its signature; order is now status `InProgress` first, then urgency desc (missing last),
  priority, ID.

- [ ] **Step 1: Write the failing tests**

In `crates/lida-core/tests/next_test.rs`, change `order_and_open`'s expected list to
`["bbbb", "aaaa", "cccc", "iiii", "dddd", "jjjj", "kkkk", "mmmm", "eeee", "llll", "ffff"]`, and add:

```rust
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
```

In `crates/lida/tests/next_test.rs` add:

```rust
#[test]
fn in_progress_first_in_raw_and_json() {
    let dir = fixture_dir();
    let raw = run_lida(dir.path(), &["nx", "--raw"], &[]);
    assert_eq!(raw_ids(&raw.stdout).first(), Some(&"bbbb"));
    let json = run_lida(dir.path(), &["nx", "--json"], &[]);
    let parsed: Vec<serde_json::Value> = serde_json::from_str(&json.stdout).unwrap_or_default();
    assert_eq!(parsed.first().and_then(|t| t["id"].as_str()), Some("bbbb"));
}
```

(If clippy rejects anything in a test, use a scoped `#[expect(…, reason = …)]` like the existing helpers.)

- [ ] **Step 2: Run to verify they fail**

Run: `cargo nextest run -p lida-core order_and_open in_progress_sort && cargo nextest run -p lida in_progress_first`
Expected: FAIL — `bbbb` is fifth, not first.

- [ ] **Step 3: Implement**

In `compare_entries`, put a first key: `InProgress` before any other status (`Todo` and `Blocked` stay equal to each
other), then the existing chain. Update `next`'s doc comment to "ordered in-progress first, then by urgency, priority,
then ID".

- [ ] **Step 4: Run to verify they pass, then refresh snapshots**

Run: `cargo nextest run -p lida-core` → PASS.
Run: `INSTA_UPDATE=always cargo nextest run -p lida` then `git diff crates/lida/tests/snapshots`.
Expected diff: only the two `next_test__next_*_table.snap` files change, and only by the `bbbb` row moving to the first
data row. Any other change is a bug.

- [ ] **Step 5: Update DESIGN.md**

Replace "Lists open tasks (status not `Done`, and `wait` absent or not after today), ordered by `meta.urgency`
descending" with "Lists open tasks (status not `Done`, and `wait` absent or not after today), `InProgress` first, then
by `meta.urgency` descending". Leave the rest of the sentence.

- [ ] **Step 6: Gate and commit**

Run: `hk fix --all && mise run gate --fast` → all pass.

```bash
git add crates/lida-core/src/next.rs crates/lida-core/tests/next_test.rs crates/lida/tests/next_test.rs \
  crates/lida/tests/snapshots/next_test__next_default_table.snap \
  crates/lida/tests/snapshots/next_test__next_long_table.snap DESIGN.md
git commit -m "feat(cli): List in-progress tasks first in next"
```

**Acceptance criteria:**

- [ ] With the fixture, `lida nx --raw`, `lida nx` (table) and `lida nx --json` all list `bbbb` first.
- [ ] Two in-progress tasks order between themselves by urgency, and both precede a `Todo` with higher urgency.
- [ ] Order below the in-progress block is unchanged from before (the snapshot diff shows only `bbbb` moving).
- [ ] `DESIGN.md` states the new order.

---

### Task 2: Due tags `+OVERDUE` `+TODAY` `+TOMORROW` `+WEEK`

**Files:**

- Modify: `crates/lida-core/src/query.rs` (new `DueTag`, `Query::due_tags`, `parse`, `matches`)
- Modify: `crates/lida-core/src/next.rs` (pass `today` to `matches`)
- Modify: `crates/lida/src/cli.rs` (`complete_next_word`, `NextArgs::words` doc line)
- Modify: `lida.usage.kdl` (regenerated)
- Test: `crates/lida-core/tests/next_test.rs`, `crates/lida/tests/next_test.rs`, `crates/lida/tests/output_test.rs`
- Docs: `DESIGN.md` (`### next (also nx)` filter-words sentence)

**Interfaces:**

- Consumes: `lida_core::date::{Date, parse_day}`.
- Produces (in `lida_core::query`):
  - `pub enum DueTag { Overdue, Today, Tomorrow, Week }` deriving `Debug, Clone, Copy, PartialEq, Eq`.
  - `impl DueTag { pub const ALL: [DueTag; 4]; pub fn parse(name: &str) -> Option<DueTag>; pub fn name(self) -> &'static str; pub fn matches(self, due: Date, today: Date) -> bool }`
    — `parse` accepts exactly `"OVERDUE"`, `"TODAY"`, `"TOMORROW"`, `"WEEK"` (no `+`); `name` returns the same strings.
  - `Query` gains `pub due_tags: Vec<DueTag>`.
  - `Query::matches(&self, task: &Task, today: Date) -> bool` (new `today` parameter; the only caller is
    `lida_core::next::next`).

- [ ] **Step 1: Write the failing core tests** (`crates/lida-core/tests/next_test.rs`)

Add a helper `ids_on(words: &[&str], day: Date) -> Vec<String>` beside `ids_for` (same body, `day` for `today()`),
then:

```rust
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
```

Also add a unit test in `query.rs`'s new `#[cfg(test)] mod tests` asserting `DueTag::parse(t.name()) == Some(t)` for
every `DueTag::ALL`, and `DueTag::parse("today") == None`.

- [ ] **Step 2: Run to verify they fail**

Run: `cargo nextest run -p lida-core`
Expected: FAIL to compile (`DueTag`, `due_tags` undefined).

- [ ] **Step 3: Implement in `query.rs` and `next.rs`**

In `Query::parse`, a `+word` whose rest is `DueTag::parse(rest) == Some(tag)` pushes to `due_tags` (skip if already
present) instead of `tags`. `matches` ANDs a new `matches_due(task, today)`: true when `due_tags` is empty; otherwise
the task's `due` must parse with `parse_day` and match **any** due tag. `DueTag::matches` compares
`(due - today).get_days()` as `classify` does: Overdue `< 0`, Today `== 0`, Tomorrow `== 1`, Week `0..=6`. In
`next.rs`, call `query.matches(task, today)`.

- [ ] **Step 4: Run core tests**

Run: `cargo nextest run -p lida-core` → PASS.

- [ ] **Step 5: Write the failing CLI tests**

In `crates/lida/tests/next_test.rs`:

```rust
#[test]
fn due_tags_from_the_cli() {
    let dir = fixture_dir();
    let run = run_lida(dir.path(), &["nx", "+TODAY", "+OVERDUE", "--raw"], &[]);
    assert_eq!(run.code, 0, "stderr: {}", run.stderr);
    assert_eq!(raw_ids(&run.stdout), ["aaaa", "cccc", "jjjj"]);
    let tomorrow = run_lida(dir.path(), &["nx", "+TOMORROW", "--raw"], &[("LIDA_TODAY", "2026-12-01")]);
    assert_eq!(raw_ids(&tomorrow.stdout), ["cccc"]);
    let none = run_lida(dir.path(), &["nx", "+TOMORROW"], &[]);
    assert_eq!((none.code, none.stdout.as_str()), (0, "No tasks found.\n"));
}
```

In `crates/lida/tests/output_test.rs`, in `completion_offers_values`, after the `lida nx +` assertion, assert the items also contain
`"+OVERDUE"`, `"+TODAY"`, `"+TOMORROW"`, `"+WEEK"`.

- [ ] **Step 6: Run to verify the completion assertion fails**

Run: `cargo nextest run -p lida due_tags_from_the_cli completion_offers_values`
Expected: `due_tags_from_the_cli` PASS (core already works), completion assertion FAIL (missing `+OVERDUE`).

- [ ] **Step 7: Implement CLI pieces**

In `complete_next_word`, insert `format!("+{}", tag.name())` for every `DueTag::ALL`. Change the `words` doc comment
to exactly `Filter words: @project, +tag, +TODAY/+OVERDUE/+TOMORROW/+WEEK, key:value, or search text`. Regenerate the
usage spec: `cargo run -q -p lida -- usage > lida.usage.kdl`.

- [ ] **Step 8: Update DESIGN.md**

In the `next` filter-words sentence, after the `+tag (all must match)` clause, insert: "`+OVERDUE`, `+TODAY`,
`+TOMORROW`, `+WEEK` (due before today, today, tomorrow, or today through today + 6; uppercase only; any of several,
and only tasks with a parseable `due`)".

- [ ] **Step 9: Gate and commit**

Run: `hk fix --all && mise run gate --fast` → all pass. `git diff --stat crates/lida/tests/snapshots` → empty.

```bash
git add crates/lida-core/src/query.rs crates/lida-core/src/next.rs crates/lida-core/tests/next_test.rs \
  crates/lida/src/cli.rs crates/lida/tests/next_test.rs crates/lida/tests/output_test.rs lida.usage.kdl DESIGN.md
git commit -m "feat(cli): Filter next by due with +OVERDUE, +TODAY, +TOMORROW, +WEEK"
```

**Acceptance criteria:**

- [ ] Every assertion in Steps 1 and 5 holds, including the boundaries (`+WEEK` includes today + 6, excludes
      today + 7; due with a time counts by its day).
- [ ] `+TODAY +OVERDUE` lists the union, ordered by the normal `next` order; adding `@project`, `+tag` or a search word
      narrows it.
- [ ] `+today`/`+Today` are ordinary tags, never due tags.
- [ ] `lida nx +<TAB>` offers the four due tags; `lida.usage.kdl` matches `lida usage` (test
      `usage_spec_is_complete_and_committed` passes).
- [ ] No snapshot changes in this task; `DESIGN.md` documents the tags.

---

### Task 3: Priority as full-size bold letters

**Files:**

- Modify: `crates/lida/src/commands/next/palette.rs` (constants + test)
- Modify: `crates/lida/src/commands/next/row.rs` (`push_prio`, imports, unit test `chip_and_disc_spans`)
- Snapshots: `crates/lida/tests/snapshots/next_test__next_*.snap` (whichever change)
- Docs: `DESIGN.md` (`next` style table, `priority` row)

**Interfaces:**

- Consumes: existing `priority_letter(Option<Priority>) -> &'static str` in `row.rs` (reuse it; do not duplicate).
- Produces: `palette::{PRIORITY_HIGH, PRIORITY_MEDIUM, PRIORITY_LOW}: Rgb`; `PRIORITY`, `DISC_H`, `DISC_M`, `DISC_L`
  are removed.

- [ ] **Step 1: Write the failing tests**

In `palette.rs`'s `palette_matches_spec_hex_values`: drop the `PRIORITY` and `DISC_*` asserts, add
`PRIORITY_HIGH == (0xff, 0x9e, 0x64)`, `PRIORITY_MEDIUM == (0xe0, 0xaf, 0x68)`, `PRIORITY_LOW == (0xce, 0xc0, 0xac)`.
In `row.rs`, rename `chip_and_disc_spans` to `chip_and_priority_spans` and replace its disc assertion with: for each of
`(High, "H", PRIORITY_HIGH)`, `(Medium, "M", PRIORITY_MEDIUM)`, `(Low, "L", PRIORITY_LOW)` a row contains a span with
that `text`, `style.fg == Some(colour)` and `style.bold`; and a task with `priority = None` renders a single `" "`
span in the `P` lane (no letter, no colour). Add an assertion that no span contains any char in
`'\u{F0000}'..='\u{FFFFF}'`.

- [ ] **Step 2: Run to verify they fail**

Run: `cargo nextest run -p lida palette chip_and_priority`
Expected: FAIL to compile (`PRIORITY_HIGH` undefined).

- [ ] **Step 3: Implement**

Replace the three discs and `PRIORITY` with the three new constants. `push_prio` pushes `priority_letter(priority)`
styled `fg` = level colour, `bold: true`; blank `" "` when `None` (unchanged). `--raw` output is untouched.

- [ ] **Step 4: Run tests, refresh snapshots**

Run: `cargo nextest run -p lida` → unit tests PASS.
Run: `INSTA_UPDATE=always cargo nextest run -p lida`, then `git diff crates/lida/tests/snapshots`.
Expected diff: in `next_test__*.snap` only the `P` cell changes — each disc glyph becomes `H`/`M`/`L` with SGR
`38;2;255;158;100` / `38;2;224;175;104` / `38;2;206;192;172` plus bold. No column moves, no other file changes.

- [ ] **Step 5: Update DESIGN.md**

Replace the `priority` row's style cell with: "bold letter `H`/`M`/`L` in `#ff9e64` / `#e0af68` / `#cec0ac`; blank
when absent". Re-run `hk fix --all` so dprint realigns the table.

- [ ] **Step 6: Gate and commit**

Run: `hk fix --all && mise run gate` (full gate, last task) → all pass.

```bash
git add crates/lida/src/commands/next/palette.rs crates/lida/src/commands/next/row.rs \
  crates/lida/tests/snapshots DESIGN.md
git commit -m "feat(cli): Draw next's priority as bold warm letters"
```

**Acceptance criteria:**

- [ ] `lida nx` in a true-colour terminal shows `H`/`M`/`L` in `#ff9e64`/`#e0af68`/`#cec0ac`, bold, in the 1-cell `P`
      column; no Nerd Font private-use glyph is emitted anywhere in `next` output.
- [ ] Column widths and every other cell are byte-identical to before (snapshot diff limited to the `P` cell).
- [ ] `--raw` and `--json` output unchanged by this task.
- [ ] `DESIGN.md`'s priority row matches the code.

---

## Done when (whole branch)

- [ ] Three commits above, on `feat/next-active-and-due`, after `d891a48` and `1f1d018`.
- [ ] `mise run gate` passes from a clean tree.
- [ ] `git diff main --stat -- crates/lida/tests/snapshots` lists only `next_test__*.snap` files.
- [ ] Manual check in Ghostty from the repo root: `cargo run -q -p lida -- nx +TODAY +OVERDUE` shows in-progress rows
      first and readable letters.
