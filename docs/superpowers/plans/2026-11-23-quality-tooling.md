# Quality Tooling and Agent Docs Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or
> superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Bring lida to toma's quality bar: hk hooks, pinned tools, lint configs, toma's lint table, agent and product
docs, a lida-managed `TODO.md`, and CI that runs the same gate as local.

**Architecture:** Port toma's files (`~/Work/toma`) and adapt names, paths and crate boundaries. Pin the CLI's current
output with snapshot tests first, then tighten lints and refactor the CLI against those snapshots. Docs are written from
the code and `assets/design/BRANDING.md`.

**Tech Stack:** Rust 1.99 workspace (`lida-core`, `lida`), usage-rs, mise, hk 2.6.0 (Pkl package 2.0.1), dprint,
insta (new dev-dependency, as in toma), GitHub Actions with `jdx/mise-action`.

**Spec:** [docs/superpowers/specs/2026-11-23-quality-tooling-design.md](../specs/2026-11-23-quality-tooling-design.md)

## Global Constraints

- Work on branch `chore/quality-tooling`. Conventional Commits, sentence-case subject, at most 72 characters, types
  from `.quick-commitlint.json` (`feat fix chore docs style ref perf test build ci`).
- Stage named paths (`git add <paths>`); never `git commit -a` or `git commit <paths>`.
- Tool versions are toma's `mise.toml` versions, copied exactly. hk `2.6.0`, Pkl package `hk@2.0.1`.
- `rust-toolchain.toml` stays byte-identical to toma's.
- Never loosen: `unsafe_code = "forbid"`, clippy `pedantic`, `-D warnings`. No crate- or module-wide `allow`; only
  scoped `#[expect(…, reason = "…")]`.
- `lida-core` must never depend on `usage-rs`, `usage`, `owo-colors` or `lida`.
- `TODO.md` is written only by `lida` (`cargo run -q -p lida -- <args>`); no formatter or fixer touches it.
- CLI output bytes, stderr and exit codes must not change in this plan.
- Docs describe what is; no architecture tours.

## Review Focus

1. **Formatter vs. lida on `TODO.md`:** after any `lida` write, `hk fix --all` must leave `TODO.md` untouched (Task 2,
   Step 6).
2. **CLI error paths after the refactor:** unknown ID, missing TODO file, bad status and bad priority keep their exact
   stderr text and non-zero exit code (Task 3 snapshots cover them).
3. **`--json` purity:** `--json` output is valid JSON on stdout with nothing on stderr, before and after the refactor
   (Task 3, `json_outputs_are_pure`).
4. **Merge and revert commits:** `commit-msg` lets `Merge …`, `Revert …`, `fixup! …` through without commitlint
   (Task 2, Step 7).
5. **Branch names:** `cchk --branch` accepts `chore/quality-tooling` and the agent prefixes (`claude/…`, `codex/…`)
   (Task 2, Step 7).

## File map

| File | Task | Responsibility |
| --- | --- | --- |
| `TODO.md` | 1, every task | lida-managed task list for this plan |
| `mise.toml`, `mise.lock` | 2, 7 | pinned tools; `setup`, `gate`, `audit`, `outdated`, `bump` tasks |
| `hk.pkl` | 2 | hooks and steps, `core-purity` with Pkl tests |
| `.gitleaks.toml`, `.jscpd.json`, `.ls-lint.yml`, `.mdschema.yml`, `.quick-commitlint.json`, `cchk.toml`, `rustfmt.toml`, `taplo.toml`, `tombi.toml` | 2 | tool configs |
| `_typos.toml`, `dprint.json` | 2 | de-toma'd configs |
| `README.md` | 2 | restructured to `.mdschema.yml` |
| `crates/lida/tests/output_test.rs`, `crates/lida/tests/snapshots/` | 3 | golden CLI output |
| `DESIGN.md` | 4 | CLI output contract |
| `Cargo.toml`, `crates/lida/src/**`, `crates/lida-core/src/format.rs` | 5 | lint table, CLI refactor |
| `AGENTS.md`, `CLAUDE.md`, `CONTEXT.md`, `PRODUCT.md` | 6 | agent and product docs |
| `.github/workflows/{ci,audit,release}.yml` | 7 | CI on mise and hk |

---

### Task 1: Seed TODO.md with lida

**Files:**

- Create: `TODO.md`

**Interfaces:**

- Produces: one open task per Task 2–7, tag `setup`; record each ID in your notes. Each later task ends with
  `cargo run -q -p lida -- done <id>` and stages `TODO.md` in its commit.

- [ ] **Step 1: Initialise**

Run: `cargo run -q -p lida -- init --md --project lida`
Expected: `TODO.md` exists and `TODO.yaml` does not.

- [ ] **Step 2: Add the tasks**

One `cargo run -q -p lida -- add "<subject>" -t setup -p medium` per Task 2–7, with these subjects:
`Port hk hooks and tool configs`, `Pin CLI output with snapshot tests`, `Write DESIGN.md output contract`,
`Adopt toma lint table and refactor CLI output`, `Write AGENTS, CLAUDE, CONTEXT and PRODUCT docs`,
`Move CI onto mise and hk`.

- [ ] **Step 3: Verify**

Run: `cargo run -q -p lida -- list --json`
Expected: six open tasks tagged `setup`.

- [ ] **Step 4: Commit**

```bash
git add TODO.md
git commit -m "chore: Track quality tooling setup in TODO.md"
```

---

### Task 2: Port hk hooks and tool configs

**Files:**

- Create: `hk.pkl`, `.gitleaks.toml`, `.jscpd.json`, `.ls-lint.yml`, `.mdschema.yml`, `.quick-commitlint.json`,
  `cchk.toml`, `rustfmt.toml`, `taplo.toml`, `tombi.toml`, `mise.lock`
- Modify: `mise.toml`, `_typos.toml`, `dprint.json`, `README.md`

**Interfaces:**

- Produces: `mise run setup` installs hooks; `hk check --all` (default profile) green; hk step names as in toma.

- [ ] **Step 1: Copy the unchanged configs**

Copy from `~/Work/toma` byte for byte: `.gitleaks.toml`, `.quick-commitlint.json`, `cchk.toml`, `rustfmt.toml`,
`taplo.toml`, `tombi.toml`, `.mdschema.yml`, `.jscpd.json`.

- [ ] **Step 2: Adapt the toma-specific configs**

- `.ls-lint.yml`: toma's rules; in `ignore`, drop `crates/toma/...` entries and add `crates/lida/tests/snapshots`.
- `_typos.toml`: `extend-exclude` = `[".mise/", "crates/*/tests/snapshots/", "target/", "*.lock"]`; remove the
  `cours`, `smol`, `toma`, `ratatui` words and identifiers; add only words typos actually flags (Step 5).
- `dprint.json`: replace the two `crates/toma/...` excludes with `crates/*/tests/snapshots/**`; keep `TODO.md`.

- [ ] **Step 3: Extend mise.toml and lock it**

Add to `[tools]`, at toma's exact versions and with toma's comments: `jactionlint`, `aqua:gitleaks/gitleaks`,
`aqua:jackchuka/mdschema`, `ls-lint`, `npm:jscpd`, `pinact`, `pipx:commit-check`, `uv`, `python`, `pkl`, `shellcheck`,
`shfmt`, `taplo`, `tombi`, `zizmor`, `npm:quick-commitlint` (with `allow_low_downloads = true`), `node`. Do not add the
`todo` CLI. Replace `gate` with toma's (adds `hk check --all` and `RUSTDOCFLAGS='-D warnings' cargo doc`); add toma's
`outdated` and `bump` tasks.

Run: `mise install && mise lock`
Expected: `mise.lock` created with `linux-x64` and `macos-arm64` entries.

- [ ] **Step 4: Write hk.pkl**

Start from toma's `hk.pkl`, keeping every mapping and hook. Changes:

- `generated = List("crates/*/tests/snapshots/**/*", ".mise/**/*")`.
- `core-purity`: glob `crates/lida-core/Cargo.toml`, `-p lida-core`, reject regex
  `^(usage-rs|usage|owo-colors([-_].*)?|lida)$`, message
  `lida-core must not depend on CLI, terminal, or binary crates, but reaches:` and hint `-> move that code into lida`.
  Port both Pkl tests, renamed `accepts a pure core` / `rejects a direct owo-colors dependency`, with fixture crate
  `owo-colors` in place of `crossterm` and `expect.stdout` matching the new message.
- `newlines` excludes `binary_excludes + generated + List("TODO.md")`; `trailing-whitespace` excludes
  `binary_excludes + generated + List("TODO.md")`.
- `tombi` excludes `generated`.
- `root-layout` allow-list `\.agents|\.claude|\.github|assets|crates|docs`.

Run: `hk validate && hk test`
Expected: both pass; `core-purity` shows 2 passing tests.

- [ ] **Step 5: Restructure README.md and make the tree green**

Rewrite `README.md` under the `.mdschema.yml` headings, keeping its current text (features go under Key Features;
license under License). Then:

Run: `hk fix --all && hk check --all`
Expected: PASS. Fix findings at the source (typos, dprint, taplo, shellcheck on `crates/lida/assets/*.zsh` if
selected). `actionlint`, `pinact` and `zizmor` findings on `.github/workflows/` are left for Task 7: if they fail here,
note them and confirm they are the only failures.

- [ ] **Step 6: Verify the formatter leaves TODO.md alone**

Run: `cargo run -q -p lida -- add "probe" -t probe && hk fix --all && git diff --stat -- TODO.md`
Expected: only lida's own change. Then remove the probe task with `cargo run -q -p lida -- delete <id>` and confirm
`git diff -- TODO.md` is empty.

- [ ] **Step 7: Install hooks and exercise commit-msg and branch checks**

Run: `mise run setup`
Then, for each message `Merge branch 'x'`, `Revert "feat: X"`, `fixup! feat: X`, `feat: Sentence case subject`:
`printf '%s\n' "<msg>" > tmp/msg && hk run commit-msg tmp/msg` (check `hk run commit-msg --help` for the
argument form). Expected: all pass; `feat: lower case` fails.
Run: `mise exec -- cchk --branch` on this branch. Expected: pass.

- [ ] **Step 8: Close the task and commit**

```bash
cargo run -q -p lida -- done <task-2-id>
git add hk.pkl mise.toml mise.lock .gitleaks.toml .jscpd.json .ls-lint.yml .mdschema.yml .quick-commitlint.json \
  cchk.toml rustfmt.toml taplo.toml tombi.toml _typos.toml dprint.json README.md TODO.md
git commit -m "build: Port hk hooks and quality tool configs from toma"
```

If `hk fix --all` reformatted Rust or TOML files, commit those separately first as `style: Apply rustfmt and taplo`.

---

### Task 3: Pin CLI output with snapshot tests

**Files:**

- Modify: `Cargo.toml` (workspace dev-dependency `insta` at toma's version), `crates/lida/Cargo.toml`
- Create: `crates/lida/tests/output_test.rs`, `crates/lida/tests/snapshots/*.snap`

**Interfaces:**

- Produces: helper `fn run(dir: &Path, args: &[&str]) -> Output` returning `{ code: i32, stdout: String, stderr:
  String }`, run with `NO_COLOR` unset and stdout piped (the real pipe case), plus one snapshot per case below. Tasks 4
  and 5 depend on these snapshots being unchanged.

IDs are random: seed fixtures by writing a fixed `TODO.md` into the temp dir (IDs `aaaa`, `bbbb`) rather than by
`add`, and redact timestamps with insta filters.

- [ ] **Step 1: Write the snapshot tests**

One `#[test]` per case, each `insta::assert_snapshot!` of `format!("code: {}\n--- stdout\n{}\n--- stderr\n{}", …)`:

- success: `init_md`, `init_yaml`, `init_both`, `init_existing_without_force` (error), `add_positional`,
  `add_with_meta_and_tags`, `add_comment_to_task_id`, `update`, `list_default`, `list_json`, `list_tasks_flag`,
  `done`, `start`, `status_set_with_reason`, `status_alias`, `show`, `show_json`, `meta_set_and_get`, `tags`,
  `delete`, `cwi` (each `--format` value), `completion_zsh`, `usage`, `no_args_lists`.
- errors: `unknown_id` (for `done`, `show`, `delete`), `no_todo_file`, `bad_status`, `bad_priority`.
- `json_outputs_are_pure`: for every `--json` command, `serde_json::from_str::<serde_json::Value>(&stdout).is_ok()` and
  `stderr.is_empty()`.

Take the exact flags from `crates/lida/src/cli.rs`; skip any case the CLI does not support and say so in the commit
body.

- [ ] **Step 2: Record and review**

Run: `cargo insta test -p lida --accept` (or `INSTA_UPDATE=always cargo nextest run -p lida`)
Expected: snapshots written; read each one and confirm it shows today's behaviour (nothing to fix here: these pin
what is).

- [ ] **Step 3: Verify they are stable**

Run: `cargo nextest run -p lida` twice.
Expected: PASS both times, no new `.snap.new` files.

- [ ] **Step 4: Close and commit**

```bash
cargo run -q -p lida -- done <task-3-id>
git add Cargo.toml Cargo.lock crates/lida/Cargo.toml crates/lida/tests/output_test.rs crates/lida/tests/snapshots TODO.md
git commit -m "test: Pin CLI output with insta snapshots"
```

---

### Task 4: Write DESIGN.md output contract

**Files:**

- Create: `DESIGN.md`

**Interfaces:**

- Consumes: Task 3 snapshots (the source of truth for every quoted byte).

- [ ] **Step 1: Write DESIGN.md**

Sections: a two-line intro (what this file pins, and that `crates/lida/tests/snapshots/` wins if they differ); **Voice**
(from BRANDING.md A.4, two to four lines); **Colour** (which owo-colors styles each element uses, and that output is
uncoloured when not a TTY, if that is what the code does); **Commands** (per subcommand: one example invocation, its
exact human output in a `text` block, its `--json` shape where it has one, and exit code); **Errors** (exact copy and
exit code per error); **Files** (the three modes and which file is authoritative). Link `assets/design/BRANDING.md` for
visual identity.

- [ ] **Step 2: Verify**

Run: `hk check --all`
Expected: PASS (dprint, typos).
Spot-check three command examples against their snapshots by diff.

- [ ] **Step 3: Close and commit**

```bash
cargo run -q -p lida -- done <task-4-id>
git add DESIGN.md TODO.md
git commit -m "docs: Add DESIGN.md with the CLI output contract"
```

---

### Task 5: Adopt toma lint table and refactor CLI output

**Files:**

- Modify: `Cargo.toml` (`[workspace.lints.clippy]`), `crates/lida/src/main.rs`, `crates/lida/src/cli.rs`
- Create: `crates/lida/src/output.rs`, `crates/lida/src/commands.rs`, `crates/lida/src/commands/<subcommand>.rs` (one
  per subcommand)
- Move: `crates/lida-core/src/format/mod.rs` → `crates/lida-core/src/format.rs` (no `mod.rs`)

**Interfaces:**

- Produces:
  - `pub struct Output<O: Write, E: Write> { pub out: O, pub err: E }` in `output.rs`, with the colour decision made
    once there.
  - `pub fn run(cli: LidaCli, io: &mut Output<impl Write, impl Write>) -> Result<(), Box<dyn std::error::Error>>` in
    `commands.rs`, dispatching to `commands::<name>::run(…, io)`.
  - `main` builds `Output` over locked stdout/stderr, calls `commands::run`, and reports a top-level error with at most
    one `#[expect(clippy::print_stderr, reason = "last-resort error report")]`.

- [ ] **Step 1: Switch the lint table**

Set `[workspace.lints.clippy]` to toma's table exactly: remove `too_many_lines` and `struct_excessive_bools`, add
`print_stdout = "warn"` and `print_stderr = "warn"`. Update the `clippy.toml` comment that mentions `too_many_lines`.

Run: `cargo clippy --workspace --all-targets --locked -- -D warnings`
Expected: FAIL, listing the print and size findings to fix.

- [ ] **Step 2: Move format/mod.rs**

`git mv crates/lida-core/src/format/mod.rs crates/lida-core/src/format.rs`.
Run: `cargo nextest run -p lida-core`. Expected: PASS.

- [ ] **Step 3: Add `Output` and route every print through it**

Replace each `println!`/`print!`/`eprintln!` in `main.rs` with `writeln!(io.out, …)` / `writeln!(io.err, …)`,
keeping the exact text.

- [ ] **Step 4: Split the dispatcher**

Move each `Subcommand::…` arm of `run` into `commands/<name>.rs` as `pub fn run(<that variant's fields>, io) -> Result`.
Where an arm's bools are mutually exclusive (`init --yaml|--md|--both`), convert them to an enum at the top of that
command's `run`; keep the CLI flags as they are.

- [ ] **Step 5: Verify behaviour is unchanged**

Run: `cargo nextest run --workspace --locked`
Expected: PASS with zero snapshot changes (`git status` shows no `.snap` diffs and no `.snap.new`).
Run: `cargo clippy --workspace --all-targets --locked -- -D warnings`
Expected: PASS, and `rg -n 'allow\(' crates` shows no new allows.

- [ ] **Step 6: Gate, close and commit**

Run: `mise run gate`. Expected: PASS.

```bash
cargo run -q -p lida -- done <task-5-id>
git add Cargo.toml clippy.toml crates/lida/src crates/lida-core/src TODO.md
git commit -m "ref(cli): Adopt toma lint table and route output through a writer"
```

---

### Task 6: Write AGENTS, CLAUDE, CONTEXT and PRODUCT docs

**Files:**

- Create: `AGENTS.md`, `CLAUDE.md`, `CONTEXT.md`, `PRODUCT.md`

**Interfaces:**

- Consumes: Task 5's module layout (`commands/`, `output.rs`) for the repository map; Task 4's `DESIGN.md`.

- [ ] **Step 1: Write the four files**

Follow toma's files section by section, and the spec's Docs section for content:

- `AGENTS.md`: Project overview, Repository map, Core engineering rules, Naming conventions, Setup, Validation (the
  gate's exact order from `mise.toml`), Testing expectations (snapshots in `crates/lida/tests/snapshots/`, reviewed with
  `cargo insta review`), Commits (toma's wording), Tasks (`TODO.md` only via `lida`; how to add, start, close).
- `CLAUDE.md`: exactly `@AGENTS.md` and `@CONTEXT.md`, one per line.
- `CONTEXT.md`: terms as defined in `crates/lida-core/src/model.rs` and `store.rs`: task, ID (4-char base62), status
  and each alias the CLI accepts, priority, tags, meta, comment, store, Markdown-only / YAML-only / dual-write mode,
  authoritative file, atomic write, extra fields.
- `PRODUCT.md`: Why, Users, Principles, Anti-goals, from BRANDING.md A.1–A.4 and the README.

- [ ] **Step 2: Verify**

Run: `hk check --all`
Expected: PASS. Every command quoted in AGENTS.md runs (run each once).

- [ ] **Step 3: Close and commit**

```bash
cargo run -q -p lida -- done <task-6-id>
git add AGENTS.md CLAUDE.md CONTEXT.md PRODUCT.md TODO.md
git commit -m "docs: Add agent, vocabulary and product docs"
```

---

### Task 7: Move CI onto mise and hk

**Files:**

- Modify: `.github/workflows/ci.yml`, `.github/workflows/release.yml`, `mise.toml` (new `audit` task)
- Create: `.github/workflows/audit.yml`

**Interfaces:**

- Consumes: `mise run gate` (Task 2), hk profiles `candidate` and `release` from `hk.pkl`.

- [ ] **Step 1: Add the audit task**

`[tasks.audit]`, description `Run the slow and release hk profiles (rustdoc, coverage, cargo-deny, gitleaks history,
zizmor)`, running `hk check --all` with profiles `candidate` and `release` (use the flag or `HK_PROFILE` form that
`hk check --help` documents for 2.6.0).

Run: `mise run audit`. Expected: only workflow findings fail (fixed in Steps 2–4).

- [ ] **Step 2: Rewrite ci.yml**

Triggers unchanged. `permissions: contents: read`. Matrix `ubuntu-latest`, `macos-latest`; steps: checkout with
`persist-credentials: false`, `jdx/mise-action` (install from `mise.lock`, cache on), `mise run gate`. Pass
`GITHUB_TOKEN` as env for pinact.

- [ ] **Step 3: Add audit.yml**

On push to `main` and tags `v*`; one `ubuntu-latest` job; checkout with `fetch-depth: 0` and
`persist-credentials: false`; `jdx/mise-action`; `mise run audit` with `GITHUB_TOKEN` env; `permissions: contents:
read`.

- [ ] **Step 4: Harden release.yml**

Keep its matrix and upload steps. Replace `dtolnay/rust-toolchain@stable` with rustup reading `rust-toolchain.toml`
(e.g. `rustup show active-toolchain || rustup toolchain install`), then `rustup target add ${{ matrix.target }}`.
Least-privilege `permissions` per job (`contents: write` only on the upload job). Move `${{ matrix.* }}` used in
`run:` into `env:` (zizmor template-injection).

- [ ] **Step 5: Pin every action**

Run: `mise exec -- pinact run --verify-comment .github/workflows/*.yml`
Expected: every `uses:` is a SHA with a version comment.

- [ ] **Step 6: Verify**

Run: `hk check --all && mise run audit`
Expected: PASS (actionlint, pinact, zizmor included).

- [ ] **Step 7: Close and commit**

```bash
cargo run -q -p lida -- done <task-7-id>
cargo run -q -p lida -- list --json   # expected: no open `setup` tasks
git add .github/workflows mise.toml TODO.md
git commit -m "ci: Run the mise gate in CI and audit on main and tags"
```

After pushing, confirm the first CI and audit runs are green; a failing run is reported, not hidden.
