# Release pipeline and roadmap to 1.0

## Intent

lida is about to get its first users: this repository (which manages its own `TODO.md` with lida) and the owner's
public dots repo, whose Brewfile already pins the first real release:

```ruby
cargo "lida", source: "https://github.com/roalcantara/lida?tag=v0.1.0"
```

This design closes out the current work so that tag can exist and keep working: the GitHub repository and remote, an
automated release flow that writes `CHANGELOG.md`, a reviewed CI setup, the security basics a public repo needs, and a
roadmap from the first rehearsal release to 1.0 that lida itself tracks in `TODO.md`.

## Goals

- `roalcantara/lida` exists on GitHub, public, with `origin` set and `main` protected against deletion and force push.
- Every push to `main` that contains a `feat`, `fix` or `perf` commit produces a release automatically: version, tag,
  `CHANGELOG.md`, GitHub Release with binaries. Nothing is released by hand.
- The pipeline is rehearsed for real on **v0.0.x** releases; **v0.1.0** is cut deliberately, only after a 0.0.x release
  has proven the whole chain, including `cargo install --git … --tag` and `brew bundle`.
- The version lives in one place (`Cargo.toml`); every other spot reads it or is written by the release.
- The roadmap is in `TODO.md`, one task per known piece of work, each tagged with its milestone, so
  `lida list -t <milestone>` is that milestone's checklist.

## Non-goals

- crates.io publishing, a Homebrew tap, or new binary targets before 1.0.
- Designing the CLI, library API or TUI evolution (v0.2+): only placeholders are added; each gets its own design.
- Fixing the known output bugs in this work: they are scheduled for the `stable` milestone.
- Rewriting history: commits keep the owner's existing author email (as in the public kbx repo).

## Anti-goals

- A release that lands on v0.1.0 or v1.0.0 by accident (semantic-release maps `feat` to minor and breaking changes to
  major by default).
- A tag without its release assets, or a release whose `CHANGELOG.md` and `Cargo.toml` disagree with its tag.
- Two release mechanisms (a tag-push workflow alongside semantic-release).
- A release that skipped the slow security checks.

## Constraints

- Mirror kbx (`~/Work/kbx`), which already runs this flow: `.releaserc.yml`, `mise run release` (semantic-release via
  `npx` with pinned plugin versions), `release:version`, and one CI workflow (checks → plan → build → release), pushing
  with `GITHUB_TOKEN`. Its ADR is `~/Work/kbx/docs/adr/0007-releases.md`.
- Commit types are fixed by `.quick-commitlint.json`: `feat fix chore docs style ref perf test build ci`.
- All existing gates stay as they are (`mise run gate`, `mise run audit`, hk hooks, actionlint, pinact, zizmor).
- Pushes made with `GITHUB_TOKEN` trigger no other workflow, so nothing may depend on a workflow reacting to the
  release's own commit or tag.
- `main` already contains `feat` commits, so **the first push to `main` with the pipeline in place releases** (v0.0.1).

## Roles

- **The agent** prepares everything on branches, and, once the owner has said go, creates the public repo, applies its
  settings and ruleset, pushes branches and the `v0.0.0` seed tag, and opens pull requests.
- **The owner** merges every pull request (a merge to `main` is a release) and edits the dots Brewfile.

## Approaches

**Release pipeline (decided: A).**

- A. semantic-release as in kbx: one CI workflow on push to `main`; a dry run plans the version, binaries are built,
  then semantic-release writes the changelog, bumps, tags and publishes. Chosen: it is the flow asked for and already
  proven in kbx.
- B. release-plz: Rust-native release PRs with git-cliff changelogs. A second model next to kbx's, built around
  crates.io, which waits until 1.0.
- C. git-cliff plus a manual tag push: a generated changelog, but releasing stays manual.

**First releases (decided: A).**

- A. A v0.0.x patch series on `main`, with every release type mapped to patch until v0.1.0 is cut by a deliberate
  config change. Chosen: the real pipeline is rehearsed in the real repo without touching the Brewfile's pin.
- B. Release candidates (`v0.1.0-rc.N`) from a `next` prerelease branch, then `main`. More precise numbers, but two
  release branches and semantic-release's fiddly merge-back.

**Roadmap representation (decided: A).**

- A. One milestone tag per task (`v0.1.0`, `stable`, `v0.2`, `v1.0`); `lida list -t <tag>` filters. Chosen: tags are
  the field `lida list` filters on today.
- B. A `milestone` meta key: richer, but `lida list` cannot filter on meta yet.

## Versioning policy (decided)

| Version     | Milestone              | Meaning                                                                        |
| ----------- | ---------------------- | ------------------------------------------------------------------------------ |
| v0.0.x      | (rehearsal)            | Real releases of the pipeline itself; any release type is a patch.             |
| v0.1.0      | `v0.1.0`               | dots can install it: pipeline proven, single-sourced version, security basics. |
| v0.1.x      | `stable`               | Patch releases until no `bug` task is open and the feature audit passes.       |
| v0.2 → v0.x | `v0.2` (evolution)     | CLI, library API and TUI work; breaking changes bump the **minor** version.    |
| v1.0.0      | `v1.0` (recommendable) | `--json` shape, exit codes and file formats frozen; wider distribution.        |

"Stable" is a state (the latest v0.1.x with no known bugs), not a version. dots upgrades by editing its pinned tag.

## Decided design

### 1. GitHub repository

- Create `roalcantara/lida` with `gh repo create`, **public** (dots must clone it anonymously; public also makes
  rulesets, secret scanning and private vulnerability reporting free). Description from README's tagline.
- Before creating it: rerun the full-history gitleaks scan (`mise run audit`) and confirm no private material in
  `git log`; publishing history cannot be undone. The author email stays as it is.
- `git remote add origin`, push `main` and the work branch, open the PR. CI on the PR is the first real run of the
  workflows: fix what fails there before the owner merges.
- Repository settings (via `gh api`): secret scanning with push protection, Dependabot alerts and security updates,
  private vulnerability reporting, delete branch on merge. Merge method: merge commits or rebase merges (both keep
  every conventional subject for semantic-release); squash merging disabled, since it would collapse a PR into one
  subject.
- A ruleset on `main`: no deletion, no force push. No required status checks or required reviews: semantic-release
  pushes the release commit straight to `main` with `GITHUB_TOKEN`, and a personal repo cannot reliably exempt that
  push. The owner merging through PRs, after CI is green, is the check.

### 2. Release flow

- `.releaserc.yml`, ported from kbx:
  - `branches: [main]`.
  - commit-analyzer, `conventionalcommits` preset. **Rehearsal rules (v0.0.x):** `feat`, `fix`, `perf` and
    `breaking: true` all release a **patch**. **From the v0.1.0 cut:** `feat` → minor, `fix`/`perf` → patch,
    `breaking: true` → **minor** while below 1.0 (removed in the v1.0.0 work).
  - release-notes-generator: sections Features, Bug Fixes, Performance; `ref docs chore test ci build style` hidden.
  - changelog → `CHANGELOG.md`, title "Every release of lida, written by semantic-release from the commit messages
    (docs/adr/0001-releases.md)."
  - exec: `verifyReleaseCmd` writes `version=` to `$GITHUB_OUTPUT` (the plan job reads it); `prepareCmd` runs
    `mise run release:version ${nextRelease.version}`.
  - git: assets `CHANGELOG.md`, `Cargo.toml`, `Cargo.lock`, `lida.usage.kdl`; message
    `chore(release): ${nextRelease.version} [skip ci]`.
  - github: the three archives and their `.sha256` files as assets; no success/fail comments or labels.
- `mise run release` and `mise run release:version <version>` as in kbx (same pinned `npx -p` versions).
  `release:version` sets `[workspace.package] version` in `Cargo.toml`, the `version` line in `lida.usage.kdl`, then
  refreshes `Cargo.lock` offline.
- **Seed:** tag the root commit of `main` as `v0.0.0` and push the tag with the branches. With the rehearsal rules, the
  first merge releases **v0.0.1**, whose changelog covers the whole history.
- **Rehearsal:** later releases come from `fix` (or `fix(ci)`, `fix(release)`) commits merged to `main`: v0.0.2, …
  Plain `ci:`/`build:` commits do not release.
- **Cut:** one PR switches `.releaserc.yml` to the v0.1.0 rules and carries a `feat:` commit; its merge releases
  v0.1.0.

### 3. One version source

- `crates/lida/src/cli.rs` (`version = "0.1.0"`) and `commands/generate.rs` (`usage`) read `env!("CARGO_PKG_VERSION")`.
- The output snapshots redact the version (`[VERSION]`), so a release bump never fails `output_test`.
- `lida.usage.kdl` keeps a literal version, written by `release:version`.

### 4. CI workflows

- `ci.yml` becomes the only workflow, kbx's shape, on `pull_request`, `push` to `main` and `workflow_dispatch`:
  - `gate` (Ubuntu and macOS): `mise run gate`, as today.
  - `install` (Ubuntu): `cargo install --locked --path crates/lida`, then `lida --version` prints the `Cargo.toml`
    version (the Brewfile's build, minus the git fetch).
  - `audit` (push to `main` only, Ubuntu, full history): `mise run audit`, as `audit.yml` does today, without a cache.
  - `plan` (push to `main` only, after gate, install and audit): `mise run release -- --dry-run`, output `version`.
  - `build` (only when `version` is set): today's three-target matrix, built after
    `mise run release:version "$VERSION"` so the binaries report the release version; archives uploaded as artifacts.
  - `release` (after build): downloads the artifacts, runs `mise run release`.
  - Concurrency: never cancel a run on `main`.
- `release.yml` and `audit.yml` are deleted; their jobs live in `ci.yml`.
- Every action pinned by SHA (pinact), `permissions: {}` at the top, per-job grants, `persist-credentials: false`,
  timeouts; `contents: write` only on `plan` (semantic-release checks push access even on `--dry-run`) and `release`.
  The graph tools stay in `MISE_DISABLE_TOOLS`.
- Node for `npx`: already pinned in `mise.toml` (26.11.1, which satisfies semantic-release 25).

### 5. Security basics

- `SECURITY.md`: supported versions (latest release only), report through GitHub private vulnerability reporting.
- `.github/dependabot.yml`: `cargo` and `github-actions`, weekly, grouped; commit messages prefixed `build(deps)` and
  `ci(deps)` so they pass commitlint and do not release by themselves.
- `deny.toml` ignores `RUSTSEC-2024-0449` with no reason: find which dependency it concerns; remove the ignore if no
  longer needed, else keep it with a comment naming the advisory, the path and why it does not apply.
- `rust-version` is 1.85 but nothing tests it: set it to the toolchain's 1.99 (source installs get a current Rust),
  unless the builder adds an MSRV CI job instead.

### 6. Docs

- `docs/adr/0001-releases.md`: the decision above, kbx's ADR adapted (no macOS app; three targets; the v0.0.x
  rehearsal and v0.x policy; `GITHUB_TOKEN` and why audit gates the release inside `ci.yml`).
- README: an Install section (the Brewfile line and `cargo install --locked --git … --tag vX.Y.Z`), and the release
  commands in Development Workflow. AGENTS.md: commit types decide releases (`feat`/`fix`/`perf` ship; until v0.1.0
  every release is a patch).

### 7. Roadmap in TODO.md

Added with `lida add … -t <milestone>[,<topic>]`; existing bug tasks get the `stable` tag added. Priorities in
brackets.

**`v0.1.0`**

- Create the public GitHub repo, add origin, push, open the PR [high]
- Configure repo security settings, merge methods and the `main` ruleset [high]
- Add semantic-release with the v0.0.x rehearsal rules and the v0.0.0 seed tag [high]
- Fold release.yml and audit.yml into ci.yml (audit, plan, build, release jobs) [high]
- Single-source the version and redact it in snapshots [high]
- Add SECURITY.md and Dependabot [medium]
- Resolve the RUSTSEC-2024-0449 ignore in deny.toml [medium]
- Set rust-version to the tested toolchain (or test the MSRV) [low]
- Write the releases ADR and the README Install section [medium]
- Verify a v0.0.x release: tag, CHANGELOG.md, version bump and the three assets agree [high]
- Rehearse installs from a v0.0.x tag: `cargo install --git --tag` and `brew bundle` with a temporary pin [high]
- Cut v0.1.0: switch to the v0.1.0 release rules with a `feat` commit [high]
- Pin the dots Brewfile to v0.1.0 and run `brew bundle` (owner) [high]

**`stable`** (plus the five open `bug` tasks)

- Audit every command against README and DESIGN.md [high]
- README advertises `--meta key=val`, which does not exist [medium]
- `lida usage` prints a stub instead of the real usage spec [medium]
- `completion` exits 0 for unsupported shells [low]

**`v0.2`** (placeholders; each needs its own design)

- Design the CLI evolution [medium]
- Design the lida-core library API [medium]
- Design a TUI [low]

**`v1.0`**

- Freeze and document the `--json` schema, exit codes and file formats [high]
- Remove the pre-1.0 breaking→minor release rule [high]
- Publish lida-core and lida to crates.io [medium]
- Ship prebuilt binaries through a Homebrew tap [low]

## Order

1. Roadmap tasks into `TODO.md` (so the rest is tracked).
2. On the work branch: the release flow (rehearsal rules), the CI fold, one version source, security files, docs.
3. Rehearse locally: `mise run release -- --dry-run --no-ci` reports v0.0.1; `cargo install --git file://… --tag`
   against a throwaway local tag works; the dots `cargo … source:` form is checked against `brew bundle`'s docs or a
   scratch Brewfile.
4. With the owner's go: create the repo, apply settings, push `main`, the branch and `v0.0.0`, open the PR; fix CI on
   real runners until green.
5. Owner merges → v0.0.1. Verify it (tag, `CHANGELOG.md`, assets, `cargo install --git … --tag v0.0.1`, `brew bundle`
   with a temporary pin). Fixes ship as v0.0.2+.
6. Cut PR → owner merges → v0.1.0 → owner pins dots to v0.1.0.

## Left to the builder

- Exact ruleset and settings API payloads.
- Whether `brew bundle`'s `cargo` entry supports `source:` with `?tag=`; if not, propose the working form for dots.
- Dependabot grouping details and schedule day.
- Whether the `install` job also runs on macOS.
