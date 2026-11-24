# Quality tooling and agent docs for lida

## Intent

lida was bootstrapped from toma (`~/Work/toma`) but only part of toma's quality setup came across. Bring lida to
parity: the same hk hooks, lint configs, agent and product docs, and a roadmap in `TODO.md` that lida itself manages
(dogfooding). The outcome is a repo where `mise run setup` installs working hooks, `mise run gate` is the single
definition of "green" used locally and in CI, and a fresh agent session can find conventions (AGENTS.md), vocabulary
(CONTEXT.md), product intent (PRODUCT.md), output contract (DESIGN.md) and the next task (TODO.md).

## Goals

- `hk.pkl` ported from toma, adapted to lida's crates, paths and tools; `hk validate` passes and `hk check --all` is
  green on the current tree.
- Every tool hk calls is pinned in `mise.toml`, with a committed `mise.lock` for `linux-x64` and `macos-arm64`.
- Config files that still name toma (`_typos.toml`, `dprint.json`) are corrected for lida.
- The missing configs exist: `.gitleaks.toml`, `.jscpd.json`, `.ls-lint.yml`, `.mdschema.yml`,
  `.quick-commitlint.json`, `cchk.toml`, `rustfmt.toml`, `taplo.toml`, `tombi.toml`.
- AGENTS.md, CLAUDE.md (`@AGENTS.md` + `@CONTEXT.md`), CONTEXT.md, PRODUCT.md and DESIGN.md exist and describe lida.
- `TODO.md` exists, created and edited only through `lida` (Markdown-only mode, `lida init --md`).
- CI runs through mise and hk (`mise run gate`), with every action pinned to a commit SHA.

## Non-goals

- No `TODO.yaml` in the repo and no hook that verifies lida's Markdown/YAML sync (decided: TODO.md only).
- No new product features or behaviour changes to lida's code. The lint-driven CLI refactor (below) changes structure
  only, never output.
- No choice of final brand icon among the ten proposals in `assets/design/`.
- No ADRs backfilled for past decisions (`docs/adr/` may be created later, as toma has).

## Anti-goals

- Loosening any existing threshold to get green: `unsafe_code = "forbid"`, clippy `pedantic` with `-D warnings`, no
  crate- or module-wide `allow` (scoped `#[expect(…, reason = "…")]` only).
- Docs that summarise the architecture or restate the code; AGENTS.md carries rules and commands, not a tour.
- Two definitions of the gate that can drift (CI and local must run the same `mise run gate`).
- Formatters or `newlines`/`trailing-whitespace` fixers rewriting `TODO.md`: lida owns its bytes, as `todo` does in
  toma.

## Constraints

- Keep toma's pinned versions where the tool is shared (hk 2.6.0 with Pkl package 2.0.1, dprint 0.63.0, typos 1.51.1,
  etc.), so the two repos stay in step.
- Rust comes from `rust-toolchain.toml` (1.99.0), read by mise via `idiomatic_version_file_enable_tools`.
- Repo layout today: top-level folders `.github`, `assets`, `crates`, plus `docs` once this spec lands; root files
  include `lida.usage.kdl`.
- `lida-core` deps today: `fastrand`, `serde`, `serde_json`, `serde_yaml_ng`, `thiserror`. The CLI crate `lida` adds
  `owo-colors` and `usage-rs`.

## Approaches

**CI (decided: A).**

- A. Move CI onto mise + hk: `jdx/mise-action` with `mise install --locked`, then `mise run gate`, actions SHA-pinned.
  One gate, local and CI. Chosen.
- B. Keep the hand-written cargo steps, just pin them. Smaller diff, but a second gate that drifts.
- C. Leave workflows out of scope and drop `actionlint`/`pinact`/`zizmor` from hk. Leaves known-failing workflows.

**Dogfooded task list (decided: B).**

- A. Commit `TODO.md` + `TODO.yaml` and verify the pair with a hook. Strongest self-test, but a lida bug blocks commits.
- B. Commit `TODO.md` only, edited through `lida`. Same as toma's model, simplest. Chosen.
- C. Both files, no hook. Dogfood without enforcement.

**Lint table (decided: match toma).**

- A. Match toma's `[workspace.lints]`: drop the workspace-wide `too_many_lines` and `struct_excessive_bools` allows and
  add `print_stdout`/`print_stderr = "warn"`. Forces a CLI refactor, which also makes output testable. Chosen.
- B. Keep lida's looser table. No code work, but lida stays below toma's bar.

**Slow and release checks (decided: separate workflow).**

- A. Run hk's `candidate` + `release` profiles in every gate. One gate, but every run needs full history and a GitHub
  token and gets slower.
- B. Keep `mise run gate` fast (toma's shape); a second workflow runs the same hk profiles on push to `main` and on
  release tags. Chosen.

## Decided design

### Lints and CLI output

Replace lida's `[workspace.lints.clippy]` with toma's (keeps `module_name_repetitions = "allow"`; removes
`too_many_lines` and `struct_excessive_bools`; adds `print_stdout` and `print_stderr`). Fix what this flags, not
silence it:

- All CLI output goes through one writer abstraction (e.g. functions taking `&mut impl Write` for stdout and stderr),
  so `println!`/`eprintln!` disappear from `crates/lida/src`. At most one scoped `#[expect(clippy::print_stderr,
  reason = "…")]` at the outermost error report in `main`, if needed.
- The single large `match` in `crates/lida/src/main.rs` is split into one function per subcommand (one concept per
  file where that reads well, e.g. `commands/<name>.rs`).
- Argument structs with many bools get enums where the bools are mutually exclusive (e.g. `init` mode
  `--yaml|--md|--both` becomes one `Mode`).
- Behaviour and output bytes stay identical: existing integration tests must pass unchanged, and DESIGN.md (below)
  documents the output first so the refactor can be checked against it.

### hk.pkl

Port toma's file structure verbatim (baseline / push / candidate / release / pre-commit / commit-msg mappings, hooks
`pre-commit`, `commit-msg`, `pre-push`, `fix`, `check`) with these changes:

- `generated` becomes `List("crates/*/tests/fixtures/**/*", "**/snapshots/**/*", ".mise/**/*")` (only the globs that
  match something are kept; the builder checks).
- `core-purity` targets `crates/lida-core/Cargo.toml` and `-p lida-core`, and rejects
  `^(usage-rs|usage|owo-colors|lida)$` (CLI parsing, terminal colour, the binary). Keep its two Pkl tests, adapted (the
  forbidden fixture crate becomes `owo-colors`).
- `newlines` and `trailing-whitespace` also exclude `TODO.md` (lida owns its bytes); so does dprint (already excluded).
- `root-layout` allows `.agents|.claude|.github|assets|crates|docs`.
- `docs` (mdschema) validates `README.md` against `.mdschema.yml`, which means the README is restructured to toma's
  section list (see README below).
- `tombi` excludes the same fixture paths as `generated`.
- Everything else (gitleaks, pkl, mise, dprint, shellcheck, actionlint, pinact, typos, ls-lint, cchk branch, rustfmt,
  taplo, cargo-check, clippy, machete, jscpd, test, build, rustdoc, coverage, cargo-deny, zizmor, conventional-commit)
  is carried over unchanged.

### mise.toml

Add the toma tools lida lacks: `jactionlint`, `aqua:gitleaks/gitleaks`, `aqua:jackchuka/mdschema`, `ls-lint`,
`npm:jscpd`, `pinact`, `pipx:commit-check`, `uv`, `python`, `pkl`, `shellcheck`, `shfmt`, `taplo`, `tombi`, `zizmor`,
`npm:quick-commitlint`, `node`, at toma's versions. Do not add `cargo:https://github.com/RayanBO/todo/`; lida is the
task tool. Extend `gate` to toma's version (adds `hk check --all` and `cargo doc -D warnings`) and add toma's
`outdated` and `bump` tasks. No wrapper task for the task list: agents run `cargo run -q -p lida -- <args>` or an
installed `lida`.

Run `mise lock` and commit `mise.lock`.

### Config files

- Copied from toma as is: `.gitleaks.toml`, `.quick-commitlint.json`, `cchk.toml`, `rustfmt.toml`, `taplo.toml`,
  `tombi.toml`, `.mdschema.yml`.
- `.jscpd.json`: same, `path` stays `["crates"]`.
- `.ls-lint.yml`: same rules; ignore list swaps toma's fixture/snapshot/stub paths for lida's (if any exist).
  `.dir` regex already allows `assets` (kebab), and the design asset folders are `NN-kebab-name`, which matches
  `[a-z0-9-]+`.
- `_typos.toml`: drop toma paths and words (`toma`, `ratatui`, `smol`, `cours`); add whatever lida's own words need
  (e.g. `lida`), found by running typos.
- `dprint.json`: replace `crates/toma/...` excludes with lida's equivalents; keep `TODO.md` excluded.

### Docs

Written to toma's shape, sourced from README.md, `assets/design/BRANDING.md` and the code. No architecture summaries.

- **AGENTS.md**: project overview (two lines), repository map (`crates/lida-core`, `crates/lida`, `assets/design`,
  `docs/`, the four docs and `TODO.md`), core engineering rules (core purity enforced by `core-purity`; thresholds
  unchanged; no global `allow`; writes are atomic and lossless; preserve unknown YAML fields), naming conventions,
  setup (`mise run setup`), validation (`mise run gate`), testing expectations (test first, unit tests beside code,
  integration tests in `crates/*/tests/`), commits (same wording as toma), and a **Tasks** section: manage `TODO.md`
  only with `lida`, never edit it by hand.
- **CLAUDE.md**: `@AGENTS.md` and `@CONTEXT.md`, as in toma.
- **CONTEXT.md**: canonical terms taken from `lida-core` (task, ID, status and its aliases, priority, tags, meta,
  store, Markdown-only / YAML-only / dual-write modes, authoritative file, atomic write, extra fields).
- **PRODUCT.md**: why, users (developers in Git repos; coding agents using `--json`), principles, anti-goals; drawn from
  BRANDING.md sections A.1–A.4 and the README.
- **DESIGN.md**: the CLI output contract: per command, the exact human output and error copy, colour use, the `--json`
  shape, and exit codes, taken from the current implementation (document what is, not what should be). Links to
  `assets/design/BRANDING.md` for visual identity.

### README.md

Restructure to the `.mdschema.yml` headings (Project Name and Description, Technology Stack, Project Architecture,
Getting Started, Project Structure, Key Features, Development Workflow, Coding Standards, Testing, Contributing,
License), keeping the current text and content.

### TODO.md

`lida init --md`, then add one task per remaining step of the implementation plan (tag `setup`) and nothing else: no
product roadmap exists yet, and the author adds it later. Close tasks with lida as the plan progresses, so the plan's
last commit leaves an accurate list.

### CI

- `ci.yml`: one job matrix (`ubuntu-latest`, `macos-latest`): checkout, `jdx/mise-action` (installs locked tools,
  caches), `mise run gate`. All actions pinned by SHA with version comments (`pinact`), `permissions: contents: read`,
  `persist-credentials: false` on checkout (zizmor).
- `audit.yml` (new): on push to `main` and on `v*` tags, checkout with `fetch-depth: 0`, `jdx/mise-action`, then
  `HK_PROFILE=candidate,release hk check --all` (rustdoc, coverage, cargo-deny, full-history gitleaks, zizmor). Passes
  `GITHUB_TOKEN` for pinact/zizmor; `permissions: contents: read`. Same hk steps as local, so nothing is duplicated.
  Locally the same run is `mise run audit` (a new task wrapping that command).
- `release.yml` (builder's call, reviewer default): stays on rustup, not mise. Pin actions by SHA, read the channel from
  `rust-toolchain.toml` instead of `@stable`, add cross targets (`x86_64-apple-darwin` etc.) with `rustup target add` in
  the workflow so `rust-toolchain.toml` stays identical to toma's, and satisfy zizmor (least-privilege permissions per
  job).

## Left to the builder

- Order of the work and commit split (the configs that make hk green on the current tree must land before the hooks are
  installed).
- Fixing whatever newly enabled checks flag in existing code and docs (typos, clippy, machete, jscpd, rustdoc,
  dprint), without loosening thresholds.
- Exact gitleaks/typos allowlist entries lida needs.
- Whether coverage gets a minimum threshold (default: report only, as in toma).
