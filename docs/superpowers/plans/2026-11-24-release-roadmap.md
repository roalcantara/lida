# Release Pipeline to v0.1.0 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or
> superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Publish lida on GitHub with semantic-release, rehearse it on v0.0.x releases, then cut v0.1.0 for the dots
Brewfile.

**Architecture:** Port kbx's release flow (`~/Work/kbx`: `.releaserc.yml`, `mise run release`, `release:version`,
`.github/actions/setup`, one `ci.yml`). Rehearsal rules map every release type to patch until a deliberate cut PR.
Tasks 1–6 happen on the work branch with no network side effects; Tasks 7–9 publish, with the owner merging.

**Tech Stack:** semantic-release 25 via `npx` (Node 26.11.1 from `mise.toml`), GitHub Actions, `gh`, mise, hk.

**Spec:** [docs/superpowers/specs/2026-11-24-release-roadmap-design.md](../specs/2026-11-24-release-roadmap-design.md)

## Global Constraints

- Branch `chore/quality-tooling`; Conventional Commits, sentence-case subject ≤ 72 chars; stage named paths only.
- Mirror kbx's files and pinned versions: semantic-release 25.0.9, `@semantic-release/changelog` 7.0.0,
  `@semantic-release/exec` 7.1.0, `@semantic-release/git` 11.0.1, `@semantic-release/github` 12.0.10,
  `conventional-changelog-conventionalcommits` 9.3.1.
- Rehearsal rules until the cut: `feat`, `fix`, `perf`, breaking → **patch**. After the cut: `feat` → minor,
  `fix`/`perf` → patch, breaking → **minor**.
- Release commit message: `chore(release): ${nextRelease.version} [skip ci]`.
- Releases push with `GITHUB_TOKEN`; nothing may rely on a workflow triggered by the release's commit or tag.
- Every action pinned by SHA (pinact); `permissions: {}` at workflow level; `contents: write` only on `plan` and
  `release`; `persist-credentials: false`; graph tools stay in `MISE_DISABLE_TOOLS`.
- Gates unchanged: `mise run gate`, `mise run audit`, all hk steps; no lint loosening.
- Owner merges every PR and edits dots. The agent never merges, never moves or deletes a pushed tag, never pushes to
  `main` directly.
- Each task ends by closing its `TODO.md` task with lida (`cargo run -q -p lida -- done <id>`, ID from
  `list -t v0.1.0 --json` by description) and staging `TODO.md` in its commit.

## Review Focus

1. **Snapshots after a bump:** once `release:version` sets 0.0.1, `output_test` must still pass (Task 1 redacts the
   version and tests it with a bumped version).
2. **`--locked` after a bump:** the build job runs `release:version` then `cargo build --locked`; the lockfile must
   match the bumped manifest (Task 2 Step 3 builds with `--locked` after a bump).
3. **First release number:** with `v0.0.0` on the root commit and rehearsal rules, the dry run must report **0.0.1**,
   never 0.1.0 or 1.0.0 (Task 7 Step 7).
4. **CI cold start:** first runs have no mise cache; `rustup toolchain install` must precede mise (Task 3 composite
   action) and job timeouts must fit a cold install (Task 7 Step 6 records real durations).
5. **Merge commits:** `Merge pull request #N …` subjects must not break commitlint or semantic-release (Task 7 sets
   merge commits allowed; Task 8 confirms the changelog lists the branch's `feat`/`fix` entries, not the merge).

---

### Task 1: Single-source the version

**Files:**

- Modify: `crates/lida/src/cli.rs:8`, `crates/lida/src/commands/generate.rs:18`, `crates/lida/tests/output_test.rs`
- Snapshots: `crates/lida/tests/snapshots/output_test__usage.snap` (version line becomes `[VERSION]`)

**Interfaces:**

- Produces: the binary's `--version` and `lida usage` report `env!("CARGO_PKG_VERSION")`; Task 2's `release:version`
  only edits `Cargo.toml`, `lida.usage.kdl` (and `cli.rs` if Step 3's fallback applies).

- [ ] **Step 1: Failing test** in `output_test.rs`:
      `version_matches_manifest`: `lida(dir, &["--version"])` exits 0 and its stdout contains `env!("CARGO_PKG_VERSION")`.
      Extend `redact` to replace `env!("CARGO_PKG_VERSION")` with `[VERSION]` (after the path redaction).
- [ ] **Step 2:** Run `INSTA_UPDATE=no cargo nextest run -p lida`. Expected: `usage` snapshot fails (its `0.1.0` is now
      `[VERSION]`); `version_matches_manifest` passes or fails depending on usage-rs (note which).
- [ ] **Step 3:** `generate.rs` writes `env!("CARGO_PKG_VERSION")`. In `cli.rs`, try `version = env!("CARGO_PKG_VERSION")`
      in the `#[usage(...)]` attribute. If usage-rs rejects a non-literal, keep the literal and record
      `Deviation: usage-rs needs a literal version; release:version rewrites cli.rs` for Task 2.
- [ ] **Step 4:** Re-record only the `usage` snapshot (`INSTA_UPDATE=always cargo nextest run -p lida -E 'test(generated_specs)'`),
      open it, confirm the only change is `version "[VERSION]"`. Then `INSTA_UPDATE=no cargo nextest run --workspace`: PASS.
- [ ] **Step 5:** Bump check: set `[workspace.package] version` to `9.9.9` by hand, run
      `INSTA_UPDATE=no cargo nextest run -p lida`: PASS (snapshots stable, `--version` says 9.9.9). Revert to `0.1.0`.
- [ ] **Step 6:** `mise run gate --fast`; close the TODO task "Single-source the version and redact it in snapshots";
      commit `ref(cli): Read the version from Cargo.toml everywhere`.

### Task 2: semantic-release configuration and tasks

**Files:**

- Create: `.releaserc.yml`
- Modify: `mise.toml` (tasks `release`, `release:version`), `dprint.json`/`_typos.toml` only if they flag `CHANGELOG.md`
  later (it is generated: add it to dprint `excludes` and hk `newlines`/`trailing-whitespace` excludes now)
- Modify: `hk.pkl` (`CHANGELOG.md` next to `TODO.md` in those excludes)

**Interfaces:**

- Consumes: Task 1 (what `release:version` must edit).
- Produces: `mise run release [-- <semantic-release args>]`; `mise run release:version <version>`; `.releaserc.yml`
  writing `version=<x>` to `$GITHUB_OUTPUT` in `verifyReleaseCmd`. Used by Task 3's `plan`, `build`, `release` jobs.

- [ ] **Step 1:** `.releaserc.yml` from kbx's, changed: `repositoryUrl: https://github.com/roalcantara/lida.git`;
      commit-analyzer `releaseRules` = rehearsal rules (`{type: feat, release: patch}`, `{type: fix, release: patch}`,
      `{type: perf, release: patch}`, `{breaking: true, release: patch}`) with a comment `# Rehearsal (v0.0.x): every
      release is a patch until the v0.1.0 cut (docs/adr/0001-releases.md).`; changelog title per spec §2; git assets
      `[CHANGELOG.md, Cargo.toml, Cargo.lock, lida.usage.kdl]` (+ `crates/lida/src/cli.rs` if Task 1 deviated); github
      assets `dist/*.tar.gz` and `dist/*.sha256` (labels: target triple); no `releaseBodyTemplate` app text.
- [ ] **Step 2:** `mise.toml`: copy kbx's `[tasks.release]` verbatim (same `-p` pins) and `[tasks."release:version"]`,
      adding after the `Cargo.toml` edit: `perl -pi -e 's/^version "[^"]*"/version "$ENV{usage_version}"/' lida.usage.kdl`
      with a `grep -q` guard like kbx's (and the `cli.rs` literal if Task 1 deviated).
- [ ] **Step 3:** Verify in a throwaway worktree (`git worktree add $SCRATCH/rel HEAD`): `mise run release:version 0.0.7`,
      then `git diff --stat` shows exactly `Cargo.toml`, `Cargo.lock`, `lida.usage.kdl` (+`cli.rs`), and
      `cargo build --locked -q -p lida && ./target/debug/lida --version` prints 0.0.7. Remove the worktree.
- [ ] **Step 4:** `mise exec -- node --version` ≥ 22.14 (semantic-release 25's floor). `npx` resolution check:
      `mise run release -- --help | head -3` prints semantic-release's usage (downloads the pinned packages).
- [ ] **Step 5:** `hk check --all` PASS; close "Add semantic-release with the v0.0.x rehearsal rules…" only in Task 7
      (the seed tag lands there); commit `build(release): Add semantic-release with rehearsal rules`.

### Task 3: One CI workflow

**Files:**

- Create: `.github/actions/setup/action.yml` (kbx's, verbatim; pin SHAs with pinact)
- Modify: `.github/workflows/ci.yml`
- Delete: `.github/workflows/release.yml`, `.github/workflows/audit.yml`
- Modify: `mise.toml` only if a `ci:*` task is needed (prefer existing `gate`, `audit`, `release`, `release:version`)

**Interfaces:**

- Consumes: Task 2 tasks and `$GITHUB_OUTPUT` `version`.
- Produces: jobs `gate` (matrix ubuntu/macos), `install`, `audit`, `plan` (output `version`), `build` (matrix of the three
  targets from today's `release.yml`), `release`. Task 7 reads their names for the PR checks.

- [ ] **Step 1:** Write `ci.yml` per spec §4, using kbx's `ci.yml` as the template for `plan`/`release` (fetch-depth 0,
      `GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}`), today's `release.yml` build matrix and packaging (env-passed values)
      for `build`, with `mise run release:version "$VERSION"` before `cargo build --locked`, artifacts named
      `lida-${{ matrix.target }}`, downloaded by `release` into `dist/` with `merge-multiple: true`. Triggers:
      `pull_request`, `push: branches [main]`, `workflow_dispatch`. Concurrency group `ci-${{ github.ref }}`,
      `cancel-in-progress: ${{ github.ref != 'refs/heads/main' }}`. `audit`/`plan` run only on push to `main`;
      `plan` needs `[gate, install, audit]`; `build` needs `plan` and `if: needs.plan.outputs.version != ''`;
      `release` needs `[plan, build]`. Every job uses `./.github/actions/setup`. Keep `MISE_LOCKED` and
      `MISE_DISABLE_TOOLS`. Timeouts: gate 30, install 20, audit 45, plan 10, build 45, release 15.
- [ ] **Step 2:** `install` job: `cargo install --locked --path crates/lida --root "$RUNNER_TEMP/lida"` then
      `"$RUNNER_TEMP/lida/bin/lida" --version | grep -F "$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.name=="lida").version')"`.
      If `jq` is not on the runner, use `grep '^version' Cargo.toml`.
- [ ] **Step 3:** `pinact run --verify-comment .github/workflows/*.yml .github/actions/setup/action.yml`;
      `jactionlint .github/workflows/*.yml`; `zizmor .github` → all clean (fix findings; no suppressions without a comment).
- [ ] **Step 4:** `mise run gate` PASS; close "Fold release.yml and audit.yml into ci.yml…"; commit
      `ci: Fold release and audit into one workflow with release jobs`.

### Task 4: Security basics and toolchain floor

**Files:**

- Create: `SECURITY.md`, `.github/dependabot.yml`
- Modify: `deny.toml`, `Cargo.toml` (`rust-version`)

- [ ] **Step 1:** `SECURITY.md`: supported = latest release; report via GitHub private vulnerability reporting
      (`https://github.com/roalcantara/lida/security/advisories/new`); no email address.
- [ ] **Step 2:** `dependabot.yml`: ecosystems `cargo` (`/`) and `github-actions` (`/`), weekly Monday, one group per
      ecosystem, `commit-message.prefix` `build(deps)` / `ci(deps)`. Run `jactionlint`-equivalent check: `tombi`/`dprint`
      via `hk check --all`.
- [ ] **Step 3:** Remove the `RUSTSEC-2024-0449` ignore, run `cargo deny check advisories`. If clean: done. If it
      fires: restore the ignore with a comment `# <crate> via <path>: <why it does not apply>` taken from the advisory text.
- [ ] **Step 4:** `rust-version = "1.99"`; `cargo check --workspace --locked` PASS.
- [ ] **Step 5:** `mise run gate` PASS; close the three TODO tasks (SECURITY/Dependabot, RUSTSEC, rust-version);
      commit `build: Add security policy, Dependabot and an honest toolchain floor`.

### Task 5: Release docs

**Files:**

- Create: `docs/adr/0001-releases.md`
- Modify: `README.md` (Getting Started gains install-from-release; Development Workflow gains `mise run release -- --dry-run --no-ci`),
  `AGENTS.md` (Commits: which types release; rehearsal note), `.ls-lint.yml`/`hk.pkl` root-layout only if `docs/adr`
  trips them (it should not)

- [ ] **Step 1:** ADR from kbx's 0007 structure (Status accepted, Date 2026-11-24, Context, Decision, Consequences),
      content from spec §2, §4 and the versioning table; cite `GITHUB_TOKEN` and why audit gates release in `ci.yml`.
- [ ] **Step 2:** README install snippet: the Brewfile line with `tag=vX.Y.Z` and
      `cargo install --locked --git https://github.com/roalcantara/lida --tag vX.Y.Z lida`. Headings stay as `.mdschema.yml` requires.
- [ ] **Step 3:** `hk check --all` PASS (mdschema, dprint, typos); close "Write the releases ADR…"; commit
      `docs: Add the releases ADR and install instructions`.

### Task 6: Local rehearsal (no network side effects)

- [ ] **Step 1:** `git tag rehearsal-0.0.0-local HEAD`; `cargo install --locked --git "file://$PWD" --tag rehearsal-0.0.0-local --root "$SCRATCH/inst" lida`;
      `"$SCRATCH/inst/bin/lida" --version` prints the manifest version. Delete the tag (`git tag -d`); it was never pushed.
- [ ] **Step 2:** Check `brew bundle`'s `cargo` entry: `brew bundle --help` and Homebrew Bundle's source for the `cargo`
      DSL options. Record whether `source:` with a `?tag=` URL is supported. If not, write the supported form in the ADR
      and in a note for the owner (do not edit dots).
- [ ] **Step 3:** Report results in the handoff note; no commit unless Step 2 changed the ADR
      (`docs: Note the Brewfile form brew bundle supports`).

### Task 7: Publish the repository (owner checkpoint first)

- [ ] **Step 1: Checkpoint.** Ask the owner to confirm creating the **public** repo now. Wait.
- [ ] **Step 2:** `mise run audit` PASS (full-history gitleaks).
- [ ] **Step 3:** `gh repo create roalcantara/lida --public --source . --remote origin --description "<README tagline>"`;
      `git push origin main chore/quality-tooling`.
- [ ] **Step 4:** Settings: `gh api -X PATCH repos/roalcantara/lida` with `allow_merge_commit=true`,
      `allow_rebase_merge=true`, `allow_squash_merge=false`, `delete_branch_on_merge=true`,
      `security_and_analysis` secret scanning + push protection enabled; `gh api -X PUT …/vulnerability-alerts`,
      `…/automated-security-fixes`, `…/private-vulnerability-reporting`; ruleset on `~DEFAULT_BRANCH` with rules
      `deletion` and `non_fast_forward` only. Verify each with a `gh api` GET.
- [ ] **Step 5:** Seed: `git tag v0.0.0 "$(git rev-list --max-parents=0 main)"`; `git push origin v0.0.0`.
- [ ] **Step 6:** `gh pr create --base main --head chore/quality-tooling` (body: summary + the roadmap link + "Merging
      releases v0.0.1"). Watch CI (`gh pr checks --watch`); fix failures with `fix(ci):`/`ci:` commits until green; record
      each job's duration.
- [ ] **Step 7:** Local plan: `GITHUB_TOKEN=$(gh auth token) mise run release -- --dry-run --no-ci --branches chore/quality-tooling`
      (or on a local `main` fast-forwarded to the branch in a scratch clone) reports **next release version is 0.0.1**.
- [ ] **Step 8:** Close "Create the public GitHub repo…", "Configure repo security settings…", "Add semantic-release…";
      commit `TODO.md` as `chore: Close the repository setup tasks`; push. Hand the PR to the owner to merge.

### Task 8: Verify v0.0.1 (after the owner merges)

- [ ] **Step 1:** `gh run watch` on `main`; then `gh release view v0.0.1 --json tagName,assets` lists 3 `.tar.gz` and
      3 `.sha256`; `git pull`; `CHANGELOG.md` has a `0.0.1` section listing the history's `feat`/`fix` entries;
      `Cargo.toml` and `lida.usage.kdl` say 0.0.1.
- [ ] **Step 2:** Download one archive, `shasum -a 256 -c` its `.sha256`, run the binary: `--version` = 0.0.1.
- [ ] **Step 3:** `cargo install --locked --git https://github.com/roalcantara/lida --tag v0.0.1 --root "$SCRATCH/v001" lida`
      → `--version` 0.0.1. Scratch Brewfile with the dots line pinned to `v0.0.1` (Task 6's supported form):
      `brew bundle --file "$SCRATCH/Brewfile"` installs it.
- [ ] **Step 4:** Any defect → a `fix(...)` PR (owner merges → v0.0.2) and repeat Steps 1–3. When all pass, close the
      two verification tasks on a branch, open a PR (owner merges; it releases nothing: `chore:`).

### Task 9: Cut v0.1.0

- [ ] **Step 1:** Branch `feat/v0.1.0-cut`: `.releaserc.yml` `releaseRules` → `{type: feat, release: minor}`,
      `{type: fix, release: patch}`, `{type: perf, release: patch}`, `{breaking: true, release: minor}` with comment
      `# Pre-1.0: breaking changes bump the minor version (removed at v1.0.0).`; AGENTS.md rehearsal note removed;
      close "Cut v0.1.0…". Commit `feat(release): Start the v0.1 release line`.
- [ ] **Step 2:** Local dry run as in Task 7 Step 7 → **0.1.0**. Open the PR; owner merges.
- [ ] **Step 3:** Repeat Task 8 Steps 1–3 for v0.1.0. Hand off to the owner: pin dots to `v0.1.0` and `brew bundle`
      (they close "Pin the dots Brewfile…").
