# `lida next` Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or
> superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add `lida next` (alias `nx`): the open tasks, most urgent first, filtered by `@project`, `+tag`,
`key:value` and plain words, drawn as a true-colour table in Ghostty, as plain `--raw` text in pipes, or as `--json`.

**Architecture:** Approach A of the spec. The rules (filter words, open, waiting, order, due-date classes) live in
`lida-core` as pure functions that take today's date as an input. The binary owns everything terminal: mode selection,
width, Unicode-aware cutting, sanitising, the palette and the ANSI writer. The table is built as styled spans first and
turned into ANSI last, so tests assert styles, not escape codes.

**Tech Stack:** Rust 2024 workspace; `usage-rs` 7.4.1 (CLI), `owo-colors` 4 (existing colour switch), new: `jiff`
(dates, in `lida-core`), `terminal_size`, `unicode-width`, `unicode-segmentation` (binary only); `insta` snapshots.

**Spec:** `docs/superpowers/specs/2026-11-27-next-command-design.md`. It is the only source of architectural decisions;
where this plan and the spec differ, the spec wins and the plan is the bug. Read it before Task 1.

## Global Constraints

- `lida-core` must never depend on `usage-rs`, `usage`, `owo-colors`, `terminal_size`, `unicode-width`,
  `unicode-segmentation` or `lida` (the `core-purity` hk step). `jiff` is the only new core dependency.
- Lints unchanged: `unsafe_code = "forbid"`, clippy `pedantic`, `print_stdout`/`print_stderr` denied, `-D warnings`, jscpd
  threshold 0. No crate- or module-wide `allow`; only scoped `#[expect(…, reason = "…")]`.
- Commands write through the `out` writer they are given; errors are returned and printed once by `main`.
- `next` is read-only: no task in this plan may call `Store::save`.
- `lida list`, `show` and every other command keep their output byte for byte: no existing `.snap` may change.
- New dependencies must pass `cargo deny check` (licences in `deny.toml`); add them to `[workspace.dependencies]`.
- Colours are exact 24-bit values copied from the spec (table in Task 4); no 16/256-colour fallback, no colour-blind
  variants, no light theme.
- One concept per file, `module.rs` with `module/` children, no `mod.rs`; vocabulary from CONTEXT.md.
- Commits: Conventional Commits, sentence-case subject ≤ 72 characters; stage named paths with `git add <paths>`, then
  `git commit`; run `hk fix --all` before checking. Never `git commit -a` or `git commit <paths>`.
- Test seams (decided here, not in the spec): `LIDA_TODAY=YYYY-MM-DD` overrides today's local date; `COLUMNS` sets
  the table width when stdout is not a terminal and `CLICOLOR_FORCE` forces the table.

## Acceptance criteria (the branch is done when all hold)

1. `mise run gate` passes (nextest, doc tests, clippy, release build, `hk check --all`, `cargo deny`, docs).
2. `git diff main --stat -- crates/lida/tests/snapshots` shows only new `output_test__next_*.snap` files; no existing
   snapshot changed.
3. `cargo tree -p lida-core -e normal` lists none of the binary-only crates named in Global Constraints.
4. Every stress-story moment of the spec (1–14) has a named test in `crates/lida/tests/next_test.rs` (Task 7 maps them).
5. On the owner's real file, with `LIDA_TODAY=2026-12-03`:
   with `L='cargo run -q --manifest-path /Users/roalcantara/Work/lida/Cargo.toml -p lida --'` and
   `cd ~/Work/kb/docs/jobx`: `$L nx --raw | wc -l` prints `185`, `$L nx @dots @jobx --raw | wc -l` prints `99`,
   `$L nx jobx --raw | wc -l` prints `2`, `$L nx +series +foo` prints `No tasks found.` and exits 0, and
   `$L nx --json | jq length` prints `185`. The file is read-only for this check.
6. `lida.usage.kdl` regenerated and committed (`usage_spec_is_complete_and_committed` passes) and lists `cmd next` with
   alias `nx`.
7. DESIGN.md has a `### next (also nx)` section and CONTEXT.md notes that `next` shows ids without `#`, in the same
   commit as the command's first snapshots.

## Review Focus

1. Wide characters (CJK, emoji, `é` composed or decomposed) must keep every column after them aligned and must never be
   cut through a grapheme. Task 3 owns the tests; Task 6 asserts one full row.
2. Hand-edited text with control characters (ESC, CR, LF, C1) must not reach the terminal raw and must stay one line,
   in the table and in `--raw`. Task 3 owns `sanitise`; Task 6 asserts a row.
3. Tags longer than the space left: the description shrinks to zero first, then the tags themselves are cut with `…`;
   no line may exceed the width. Task 6.
4. A tiny window (`COLUMNS=20`) must not panic or underflow: columns keep their widths, the description gets at least
   one cell, and the line may exceed the width. Task 5 (`layout`) and Task 6.
5. Bad data never hides the list (spec "Settled for the builder"): `urgency: "high"`, `due: tomorrow`, `wait: soon`,
   `depends: aaaa` (not a list), `depends: [zzzz]` (unknown id) all load and show. Task 2.

---

### Task 1: Dates and due classes in `lida-core`

**Files:**

- Modify: `Cargo.toml` (add `jiff = "0.2"` to `[workspace.dependencies]`), `crates/lida-core/Cargo.toml`
  (`jiff.workspace = true`), `crates/lida-core/src/lib.rs` (`pub mod date;`)
- Create: `crates/lida-core/src/date.rs` (with `#[cfg(test)] mod tests`)

**Interfaces:**

- Produces:
  - `pub use jiff::civil::Date;` re-exported from `lida_core::date`.
  - `pub fn parse_day(value: &str) -> Option<Date>`: accepts `YYYY-MM-DD` and `YYYY-MM-DD HH:MM` (the time is
    ignored), trims whitespace, `None` otherwise.
  - `pub enum Due { None, Overdue { days: i64 }, Today, Soon { weekday: jiff::civil::Weekday }, Later { days: i64 }, Far { date: Date, same_year: bool } }`
    with `pub fn classify(due: Option<&str>, today: Date) -> Due`.

- [ ] **Step 1: Write the failing tests** in `date.rs` (`today = 2026-12-03`, a Thursday):

```rust
#[test] fn parses_day_and_day_with_time() {
    assert_eq!(parse_day("2026-11-24"), Some(date(2026, 11, 24)));
    assert_eq!(parse_day("2026-11-24 09:00"), Some(date(2026, 11, 24)));
    assert_eq!(parse_day("tomorrow"), None);
}
#[test] fn classifies_every_due_class() {
    let t = date(2026, 12, 3);
    assert_eq!(classify(None, t), Due::None);
    assert_eq!(classify(Some("garbage"), t), Due::None);
    assert_eq!(classify(Some("2026-11-27"), t), Due::Overdue { days: 6 });
    assert_eq!(classify(Some("2026-12-03 09:00"), t), Due::Today);
    assert_eq!(classify(Some("2026-12-04"), t), Due::Soon { weekday: Weekday::Friday });
    assert_eq!(classify(Some("2026-12-09"), t), Due::Soon { weekday: Weekday::Wednesday });
    assert_eq!(classify(Some("2026-12-10"), t), Due::Later { days: 7 });
    assert_eq!(classify(Some("2027-01-31"), t), Due::Later { days: 59 });
    assert_eq!(classify(Some("2027-02-01"), t), Due::Far { date: date(2027, 2, 1), same_year: false });
    assert_eq!(classify(Some("2026-12-03"), date(2026, 1, 1)), Due::Far { date: date(2026, 12, 3), same_year: true });
}
```

- [ ] **Step 2: Run** `cargo nextest run -p lida-core date` → FAIL (`date` module missing).
- [ ] **Step 3: Implement** `parse_day` and `classify` in `date.rs`: days = `due - today` in whole days; `< 0` Overdue,
      `0` Today, `1..=6` Soon, `7..=59` Later, `>= 60` Far (`same_year` compares `due.year()` with `today.year()`).
- [ ] **Step 4: Run** `cargo nextest run -p lida-core date` → PASS; `cargo tree -p lida-core -e normal | grep -c jiff`
      ≥ 1.
- [ ] **Step 5: Commit** `git add Cargo.toml Cargo.lock crates/lida-core/Cargo.toml crates/lida-core/src/lib.rs crates/lida-core/src/date.rs`
      then `git commit -m "feat(core): Classify due dates against a given day"`.

### Task 2: The next query in `lida-core`

**Files:**

- Create: `crates/lida-core/src/query.rs` (filter words), `crates/lida-core/src/next.rs` (open, waiting, order)
- Modify: `crates/lida-core/src/lib.rs` (`pub mod next; pub mod query;`)
- Test: `crates/lida-core/tests/next_test.rs`, fixture `crates/lida-core/tests/fixtures/next.yaml` (shared with Task 6;
  write it here, content below)

**Interfaces:**

- Consumes: `lida_core::date::{Date, parse_day}`; `Task`, `ProjectDoc`, `Status`, `Priority` from `model`.
- Produces (`query.rs`):
  - `pub struct Query { pub words: Vec<String>, pub projects: Vec<String>, pub tags: Vec<String>, pub meta: BTreeMap<String, Vec<String>> }`
  - `impl Query { pub fn parse<S: AsRef<str>>(words: &[S]) -> Self; pub fn matches(&self, task: &Task) -> bool; pub fn search_spans(&self, text: &str) -> Vec<std::ops::Range<usize>> }`
  - `pub fn task_project(task: &Task) -> Option<&str>` (reads `extra["project"]` as a string).
- Produces (`next.rs`):
  - `pub enum Waiting { No, Blocked, Depends }`
  - `pub struct Entry<'a> { pub task: &'a Task, pub urgency: Option<f64>, pub waiting: Waiting }`
  - `pub fn next<'a>(doc: &'a ProjectDoc, query: &Query, today: Date) -> Vec<Entry<'a>>`

Rules, verbatim from the spec: open = status not Done and `wait` absent or not after today (an unparsable `wait` counts
as absent); waiting = Blocked (`Waiting::Blocked`) else any `depends` id whose task exists and is not Done, even if that
task is hidden by its own `wait` (`Waiting::Depends`); `depends` not a list, or naming an unknown id, is ignored; order
= urgency descending (non-numeric or missing urgency last), then priority High, Medium, Low, none, then id ascending.
Filter words: `@x` → `projects`, `+x` → `tags`, `k:v` (first `:` splits; empty key or value still parsed) → `meta`, any
other word → `words`. A lone `@` or `+` is kept as an empty project/tag that matches nothing. Matching: every word is a
case-insensitive substring of the description; project equals `x` or starts with `x.` (any of several); every tag
present; per meta key any value matches (a list contains it as a string, a scalar's string form equals it). `search_spans`
returns byte ranges into the original `text` for every word, case-insensitively, so `RÉSUMÉ` finds `résumé`.

Fixture `next.yaml` (ids fixed; today in tests is `2026-12-03`):

| id     | status     | priority | meta.urgency | due              | project                        | tags                      | other                                                                                            |
| ------ | ---------- | -------- | ------------ | ---------------- | ------------------------------ | ------------------------- | ------------------------------------------------------------------------------------------------ |
| `aaaa` | Todo       | High     | 23           | 2026-11-27       | jobx.hunt                      | –                         | –                                                                                                |
| `bbbb` | InProgress | Low      | 5.8          | –                | dots.blog                      | series                    | description `Series: account-wide numbers`; `meta.purposes: [branding]`, `meta.lead: branding`   |
| `cccc` | Todo       | High     | 18           | 2026-12-02       | jobx.hunt                      | –                         | `depends: [aaaa]`                                                                                |
| `dddd` | Todo       | Medium   | 11.9         | 2026-12-09       | jobx.hunt                      | –                         | –                                                                                                |
| `eeee` | Todo       | Low      | -3.2         | –                | summon                         | –                         | `meta.purposes: [showcase, income]`, `meta.lead: showcase`                                       |
| `ffff` | Todo       | –        | –            | –                | toma                           | alpha, beta, gamma, delta | description: 220 characters containing `résumé`, `🚀` and `日本語`, and neither `line` nor `two` |
| `gggg` | Done       | High     | 30           | –                | jobx                           | –                         | –                                                                                                |
| `hhhh` | Todo       | High     | 25           | –                | jobx                           | –                         | `wait: 2027-02-20`                                                                               |
| `iiii` | Blocked    | Medium   | 12           | 2027-01-08       | lida                           | –                         | –                                                                                                |
| `jjjj` | Todo       | Low      | 2.8          | 2026-12-03 09:00 | dots.agentic                   | –                         | –                                                                                                |
| `kkkk` | Todo       | Low      | 1.8          | 2028-01-15       | a-very-long-project.subproject | –                         | –                                                                                                |
| `llll` | Todo       | Low      | "high"       | tomorrow         | lion                           | –                         | description `Line one\nline two \u001b[31mred`; `wait: soon`; `depends: zzzz`                    |
| `mmmm` | Todo       | Low      | 1.8          | –                | dots.blog                      | –                         | `depends: [hhhh]`                                                                                |

- [ ] **Step 1: Write the failing tests** in `tests/next_test.rs` (load the fixture with `serde_yaml_ng::from_str::<ProjectDoc>`):

```rust
#[test] fn order_and_open() {
    let ids: Vec<_> = next(&doc(), &Query::parse::<&str>(&[]), today()).iter().map(|e| e.task.id.as_str()).collect();
    assert_eq!(ids, ["aaaa", "cccc", "iiii", "dddd", "bbbb", "jjjj", "kkkk", "mmmm", "eeee", "llll", "ffff"]);
}
#[test] fn waiting_states() {   // cccc Depends, iiii Blocked, mmmm Depends (hhhh hidden by wait), llll No
    let w = waiting_by_id();
    assert_eq!((w["cccc"], w["iiii"], w["mmmm"], w["llll"], w["aaaa"]), (Waiting::Depends, Waiting::Blocked, Waiting::Depends, Waiting::No, Waiting::No));
}
#[test] fn filters() {
    assert_eq!(ids_for(&["@jobx"]), ["aaaa", "cccc", "dddd"]);
    assert_eq!(ids_for(&["@dots"]), ["bbbb", "jjjj", "mmmm"]);
    assert_eq!(ids_for(&["@dots.agentic"]), ["jjjj"]);
    assert_eq!(ids_for(&["@dots", "@jobx"]).len(), 6);
    assert_eq!(ids_for(&["@job"]), Vec::<&str>::new());
    assert_eq!(ids_for(&["+series"]), ["bbbb"]);
    assert_eq!(ids_for(&["+series", "+foo"]), Vec::<&str>::new());
    assert_eq!(ids_for(&["purposes:branding"]), ["bbbb"]);
    assert_eq!(ids_for(&["lead:showcase"]), ["eeee"]);
    assert_eq!(ids_for(&["RÉSUMÉ"]), ["ffff"]);
    assert_eq!(ids_for(&["line", "two"]), ["llll"]);
    assert_eq!(ids_for(&["@", "+", "nope:x"]), Vec::<&str>::new());
}
#[test] fn search_spans_map_to_original_bytes() {
    let q = Query::parse(&["RÉSUMÉ"]);
    let text = "my résumé, résumé";
    assert_eq!(q.search_spans(text), [3..11, 13..21]);
}
```

- [ ] **Step 2: Run** `cargo nextest run -p lida-core --test next_test` → FAIL (modules missing).
- [ ] **Step 3: Implement** `query.rs` and `next.rs` with the signatures above. Use `f64::total_cmp` for urgency; read
      `meta["urgency"]` with `serde_json::Value::as_f64` (non-numbers → `None`); read `extra` values with
      `serde_yaml_ng::Value::as_str` / `as_sequence`.
- [ ] **Step 4: Run** `cargo nextest run -p lida-core` → PASS, and `cargo clippy -p lida-core -- -D warnings` clean.
- [ ] **Step 5: Commit** the two modules, `lib.rs`, the test and the fixture:
      `feat(core): Filter, order and mark waiting tasks for next`.

### Task 3: Terminal text helpers in the binary

**Files:**

- Modify: `Cargo.toml` (workspace deps `terminal_size = "0.4"`, `unicode-width = "0.2"`, `unicode-segmentation = "1"`),
  `crates/lida/Cargo.toml` (the three, `.workspace = true`), `crates/lida/src/main.rs` (`mod text;`)
- Create: `crates/lida/src/text.rs` (with tests)

**Interfaces:**

- Produces:
  - `pub fn sanitise(s: &str) -> String`: CR, LF and TAB become one space each; every other C0, DEL and C1 control
    (including ESC) becomes a visible escape of the builder's choice that contains no control character.
  - `pub fn width(s: &str) -> usize`: terminal cells (unicode-width).
  - `pub fn fit(s: &str, cells: usize) -> String`: if `width(s) <= cells`, `s` padded with spaces to exactly `cells`;
    otherwise the longest grapheme prefix that fits in `cells - 1`, then `…`, then spaces to exactly `cells`.
    `cells == 0` returns an empty string.
  - `pub fn cut(s: &str, cells: usize) -> String`: like `fit` without the padding.

- [ ] **Step 1: Write the failing tests**:

```rust
#[test] fn sanitise_keeps_one_line_and_no_controls() {
    let s = sanitise("a\nb\r\tc\u{1b}[31md\u{9b}e");
    assert!(!s.contains('\n') && !s.contains('\r') && !s.chars().any(char::is_control));
    assert!(s.starts_with("a b  c"));
}
#[test] fn width_counts_cells() { assert_eq!(width("日本語"), 6); assert_eq!(width("🚀"), 2); assert_eq!(width("re\u{301}sume\u{301}"), 6); }
#[test] fn fit_pads_and_cuts_on_graphemes() {
    assert_eq!(fit("abc", 5), "abc  ");
    assert_eq!(fit("abcdef", 4), "abc…");
    assert_eq!(fit("日本語", 4), "日… ");          // never half a wide char
    assert_eq!(width(&fit("re\u{301}sume\u{301}x", 6)), 6);
    assert_eq!(fit("abc", 0), "");
}
```

- [ ] **Step 2: Run** `cargo nextest run -p lida text` → FAIL.
- [ ] **Step 3: Implement** with `unicode_segmentation::UnicodeSegmentation::graphemes(s, true)` and
      `unicode_width::UnicodeWidthStr::width` per grapheme.
- [ ] **Step 4: Run** → PASS; `cargo deny check` → ok.
- [ ] **Step 5: Commit** `feat(cli): Add width-aware text fitting and sanitising`.

### Task 4: Palette, spans and the ANSI writer

**Files:**

- Create: `crates/lida/src/commands/next.rs` (module root, empty `run` added in Task 6), `crates/lida/src/commands/next/palette.rs`,
  `crates/lida/src/commands/next/span.rs`
- Modify: `crates/lida/src/commands.rs` (`mod next;`), `crates/lida/src/output.rs` only if a helper is shared

**Interfaces:**

- Produces (`palette.rs`), values verbatim from the spec:

  | const       | value     | const        | value     |
  | ----------- | --------- | ------------ | --------- |
  | `ID`        | `#7aa2f7` | `CHIP`       | `#9ece6a` |
  | `CHIP_TEXT` | `#1d202f` | `PRIORITY`   | `#e0af68` |
  | `OVERDUE`   | `#f7768e` | `SOON`       | `#c0caf5` |
  | `LATER`     | `#a9b1d6` | `MUTED`      | `#737aa2` |
  | `TAGS`      | `#03ffd2` | `TEXT`       | `#c0caf5` |
  | `HEADER_BG` | `#161b22` | `BACKGROUND` | `#1d212f` |

  `pub const PROJECTS: [Rgb; 8] = [#12aedb, #d78b3d, #36ba6d, #d6da4d, #aa86f5, #d958c0, #4be9fc, #fdb6f0];`
  `pub fn project_colour(top: &str) -> Rgb` (a stable, documented, non-randomised hash of the top-level name modulo 8,
  e.g. FNV-1a 32), `pub fn dimmed(c: Rgb) -> Rgb` (70% of `c` mixed over `BACKGROUND`),
  `pub const DISC_H: char = '\u{F0C01}'; DISC_M = '\u{F0C10}'; DISC_L = '\u{F0C0D}'`,
  `pub const CHIP_LEFT: char = '\u{2595}'; CHIP_RIGHT: char = '\u{258F}'`.
- Produces (`span.rs`): `#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)] pub struct Style { pub fg: Option<Rgb>, pub bg: Option<Rgb>, pub bold: bool, pub underline: bool }`,
  `pub struct Span { pub text: String, pub style: Style }`, `pub type Rgb = (u8, u8, u8)`,
  `pub fn ansi(line: &[Span]) -> String` (24-bit SGR `38;2;r;g;b` / `48;2;r;g;b`, `1`, `4`, reset `0` after each styled
  span), `pub fn plain(line: &[Span]) -> String`.

- [ ] **Step 1: Write the failing tests**: every palette const equals the spec's hex (one assert each);
      `project_colour("lida") == project_colour("lida")` and its value is in `PROJECTS`;
      `dimmed((0x12,0xae,0xdb))` equals the 70% mix computed by hand in the test;
      `ansi(&[Span{text:"6d".into(), style: Style{fg: Some(OVERDUE), bold: true, ..Default::default()}}]) == "\u{1b}[1;38;2;247;118;142m6d\u{1b}[0m"`; `plain` of the same is `"6d"`.
- [ ] **Step 2: Run** `cargo nextest run -p lida next::` → FAIL.
- [ ] **Step 3: Implement** the consts, the hash, `dimmed`, `ansi`, `plain`.
- [ ] **Step 4: Run** → PASS.
- [ ] **Step 5: Commit** `feat(cli): Add the next palette and span writer`.

### Task 5: Layout (rulers, lanes, narrow windows)

**Files:**

- Create: `crates/lida/src/commands/next/layout.rs`

**Interfaces:**

- Produces: `pub enum Col { Id, Urg, Prio, Due, Project, Tags, Why, Desc }`,
  `pub struct Layout { pub cols: Vec<(Col, usize)>, pub desc: usize, pub tags_after_desc: bool }`,
  `pub fn layout(width: usize, long: bool, has_project: bool) -> Layout`.
  Widths from the spec: Id 6 (lane), Urg 5, Prio 1, Due 5, Project 14, Tags 17, Why 23; alleys of 2 between columns
  (the Id lane's own spaces count as part of the lane; one alley after it). `desc = width - used`, at least 1.
  `-l` drops Why, then Tags (then `tags_after_desc = true`), then Urg, while `desc < 30`. Project is dropped when
  `!has_project`. Default view has `tags_after_desc = true`.

- [ ] **Step 1: Write the failing tests**:

```rust
#[test] fn default_at_120() { let l = layout(120, false, true); assert_eq!(cols(&l), [Col::Id, Col::Prio, Col::Due, Col::Project]); assert_eq!(l.desc, 86); assert!(l.tags_after_desc); }
#[test] fn default_at_80()  { assert_eq!(layout(80, false, true).desc, 46); }
#[test] fn long_at_120()    { let l = layout(120, true, true); assert_eq!(cols(&l), [Col::Id, Col::Urg, Col::Prio, Col::Due, Col::Project, Col::Tags, Col::Why]); assert!(!l.tags_after_desc); assert_eq!(l.desc, 35); }
#[test] fn long_at_80_drops_why_then_tags() { let l = layout(80, true, true); assert_eq!(cols(&l), [Col::Id, Col::Urg, Col::Prio, Col::Due, Col::Project]); assert!(l.tags_after_desc); assert!(l.desc >= 30); }
#[test] fn no_project_column_when_no_task_has_one() { assert!(!cols(&layout(120, false, false)).contains(&Col::Project)); }
#[test] fn tiny_window_never_underflows() { assert_eq!(layout(20, true, true).desc, 1); }
```

Accounting: each column's width plus its 2-cell alley; default 6+2, 1+2, 5+2, 14+2 = 34 used; `-l` adds 5+2, 17+2,
23+2 = 85 used.

- [ ] **Step 2–4:** run → FAIL, implement, run → PASS.
- [ ] **Step 5: Commit** `feat(cli): Lay out next columns for any window width`.

### Task 6: Rows, table, raw, JSON and the command

**Files:**

- Create: `crates/lida/src/commands/next/row.rs` (one task → `Vec<Span>` for the table, `String` for raw),
  `crates/lida/src/commands/next/mode.rs` (mode selection)
- Modify: `crates/lida/src/commands/next.rs` (`pub fn run`), `crates/lida/src/cli.rs` (`Next(NextArgs)` with
  `#[usage(alias = "nx")]`, `NextArgs { #[usage(short = 'l', long)] long: bool, #[usage(long)] raw: bool, #[usage(long)] json: bool, words: Vec<String> }`),
  `crates/lida/src/commands.rs` (dispatch), `lida.usage.kdl` (regenerate with `cargo run -q -p lida -- usage > lida.usage.kdl`),
  `DESIGN.md` (`### next (also nx)`: copy the spec's frames, colour table and `--raw` rules; state it overrides the
  shared colour table for `next`), `CONTEXT.md` (ID entry: "`next` shows it without `#`")
- Test: `crates/lida/tests/next_test.rs` (integration, runs the binary on a temp dir holding `next.yaml` from Task 2 as
  `TODO.yaml`, `LIDA_TODAY=2026-12-03`), snapshots under `crates/lida/tests/snapshots/`

**Interfaces:**

- Consumes: Tasks 1–5.
- Produces: `pub enum Mode { Table { width: usize }, Raw, Json }` and
  `pub fn mode(json: bool, raw: bool, stdout_terminal: bool, no_color: bool, force_colour: bool, columns: Option<usize>, terminal_width: Option<usize>) -> Mode`:
  `json` → Json; `raw` → Raw; `no_color` → Raw; terminal → Table at `terminal_width` (else `columns`, else 80);
  not a terminal: `force_colour` → Table at `columns` or 80, else Raw.
  `pub fn run(cwd: &Path, words: &[String], long: bool, mode: Mode, today: Date, out: &mut impl Write) -> CmdResult`.

Cell rules (spec "The look", "Colour", "`--raw`"): header band `HEADER_BG` with muted bold labels (`ID`, `URG`, `P`,
`DUE`, `PROJECT`, `TAGS`, `WHY`, `DESCRIPTION`); id `ID` bold in its lane (`" " + id + " "`); in progress →
`CHIP_LEFT` fg `CHIP`, id bold `CHIP_TEXT` on `CHIP`, `CHIP_RIGHT` fg `CHIP`; priority disc fg `PRIORITY`, blank when
none; URG one decimal, `-` when absent, `TEXT`, `MUTED` when negative; DUE: Overdue `{n}d` `OVERDUE` bold, Today
`today` and Soon weekday (`Mon`…`Sun`) `SOON` bold, Later `{n}d` `LATER`, Far `Feb 20` or `Feb 20 2028` `LATER`, None
`-` `MUTED`; project `@top` bold in `project_colour(top)`, `.rest` in `dimmed(that)`; tags `+a +b` `TAGS` (bold when
after the description, two spaces before them); WHY lead bold `TEXT`, others `MUTED`, blank when none; description
`TEXT`, `MUTED` when waiting, search spans underlined; every text field through `sanitise` first; the description is
cut before the tags; header and rows go through `fit`. Raw: no header, tab-separated `id`, `H|M|L|`, the due exactly as
stored, `@project`, `(doing) `/`(waiting) `/`(blocked) ` + description + ` +tags`; `-l --raw`: `id`, urgency (empty when
absent), letter, due, `@project`, `+a +b`, purposes lead first space-separated, state word + description. Empty
result: `No tasks found.` (table and raw), `[]` for JSON. JSON = `serde_json::to_string_pretty` of the tasks in order,
the same shape as `lida list --json`.

- [ ] **Step 1: Write the failing integration tests** (names are the acceptance map for stress moments):
      `s01_default_table` (`CLICOLOR_FORCE=1 COLUMNS=120`, snapshot), `s02_long_table` (`-l`, snapshot),
      `s03_project_filters` (`@jobx`, `@dots.agentic`, `@dots @jobx` with `--raw`: assert id lists 3, 1, 6),
      `s04_tag_and_meta` (`+series purposes:branding` → `bbbb`; `lead:showcase` → `eeee`), `s05_empty` (`@nope` →
      stdout `No tasks found.\n`, exit 0), `s06_search` (`résumé` raw → `ffff`; table snapshot shows the underline SGR `4`;
      `term +series +foo @dots @jobx` → `No tasks found.`), `s07_long_description_cut` (table at 120: the `ffff` line has
      width exactly 120 and ends with `+alpha +beta +gamma +delta` padding-trimmed), `s08_many_tags_long` (`-l`: TAGS cell
      ends with `…`), `s09_long_project_cut` (`kkkk` project cell is 14 cells ending in `…`), `s10_wide_text_alignment`
      (every table line at `COLUMNS=120` has `text::width == 120` after stripping SGR), `s11_odd_values` (`eeee` URG `-3.2`,
      `ffff` P blank and URG `-` and DUE `-`), `s12_narrow_80` (`-l` at 80: no `WHY`/`TAGS` header, every line width ≤ 80),
      `s13_pipe_raw_json` (no env: output equals `--raw`; `--json` parses and has 11 entries; `--json @nope` is `[]`; the
      raw `bbbb` line is exactly `bbbb\tL\t\t@dots.blog\t(doing) Series: account-wide numbers +series`), `s14_outside_folder` (stderr
      `error: no TODO file found in directory`, exit 1); plus `sanitised_row` (`llll` table and raw lines contain no ESC and
      no LF), `tags_longer_than_room` (`COLUMNS=60`: no line wider than 60), `tiny_window` (`COLUMNS=20`, exit 0),
      `no_color_on_terminal_is_raw` (unit test of `mode`), `chip_and_disc_spans` (unit test in `row.rs`: `bbbb` spans have
      `CHIP` background on the id and `DISC_L` with fg `PRIORITY`), `waiting_is_muted` (unit: `cccc` description span fg
      `MUTED`), `list_untouched` (run the existing `output_test` suite: unchanged).
- [ ] **Step 2: Run** `cargo nextest run -p lida --test next_test` → FAIL (`next` unknown).
- [ ] **Step 3: Implement** `mode.rs`, `row.rs`, `run`, the CLI variant (alias `nx`), the dispatch (read `LIDA_TODAY`,
      else `jiff::Zoned::now().date()`; `terminal_size::terminal_size()` for the width; `NO_COLOR`/`CLICOLOR_FORCE` read the
      same way `main.rs` reads them), regenerate `lida.usage.kdl`, write the DESIGN.md and CONTEXT.md text.
- [ ] **Step 4: Run** `INSTA_UPDATE=always cargo nextest run -p lida`, review every new `.snap` against the spec's
      frames (columns on their rulers, colours per the table), then `cargo nextest run` → PASS and
      `git status --short crates/lida/tests/snapshots` lists only `next_*` files.
- [ ] **Step 5: Commit** all of the above, snapshots included: `feat(cli): Add lida next to list open tasks by urgency`.

### Task 7: Completion, real-data check and gate

**Files:**

- Modify: `crates/lida/src/cli.rs` (`complete = complete_next_word` on `words`: `@<project>` for every distinct project
  and its parent prefixes in the file, `+<tag>` for every tag in use), `crates/lida/tests/output_test.rs`
  (`completion_offers_values` gains `lida nx @` → contains `@jobx`-style entries from a fixture), `lida.usage.kdl`

- [ ] **Step 1: Write the failing completion test** (fixture from Task 2: `@jobx`, `@jobx.hunt`, `@dots`, `@dots.blog`,
      `@dots.agentic` and `+series` are offered).
- [ ] **Step 2–4:** run → FAIL, implement, run → PASS.
- [ ] **Step 5:** run Acceptance criterion 5 on the owner's file (read-only; never run `init`, `cwi` or any writing
      command there) and paste the five outputs into the commit body.
- [ ] **Step 6:** `hk fix --all`, then `mise run gate` → PASS.
- [ ] **Step 7: Commit** `feat(cli): Complete next filter words`.
