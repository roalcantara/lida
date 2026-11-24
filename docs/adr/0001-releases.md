# 0001. Released by semantic-release from GitHub Actions

- Status: accepted
- Date: 2026-11-24

## Context

lida's first users are this repository, which manages its own `TODO.md` with lida, and the owner's dots repository,
whose Brewfile builds lida from source at a pinned tag (`cargo "lida", source: "…?tag=v0.1.0"`). Versions, the changelog
and release binaries had been made by hand, if at all. Commit messages already follow Conventional Commits
(`.quick-commitlint.json`), which is all a release tool needs. kbx runs the same flow (its ADR 0007).

## Decision

semantic-release (`.releaserc.yml`) derives each release from the commits since the last tag. It writes
`CHANGELOG.md`, sets the version in `Cargo.toml` (the workspace package and the `lida-core` pin), `Cargo.lock` and
`lida.usage.kdl` through `mise run release:version`, commits them as `chore(release): X.Y.Z [skip ci]`, tags `vX.Y.Z`
and creates the GitHub Release with three archives (macOS arm64 and x86-64, Linux x86-64) and their SHA-256 files. The
CLI reads its version from `Cargo.toml`, so nothing else carries it.

One workflow, `.github/workflows/ci.yml`, runs it, calling only mise tasks:

- every pull request and push to `main`: `gate` (Linux and macOS) and `install` (`cargo install` from the checkout, as
  dots does);
- a push to `main`: `audit` (coverage, full-history gitleaks, cargo-deny, zizmor), then `plan` (a semantic-release dry
  run); only when it finds a version do `build` and `release` run.

Releases push with the workflow's `GITHUB_TOKEN`. Pushes made with it start no other workflow, so the audit cannot
react to a release tag: it runs inside `ci.yml` and gates the release instead.

Versions:

| Version     | Rule                                                                                              |
| ----------- | ------------------------------------------------------------------------------------------------- |
| v0.0.x      | Rehearsal. Every release type (`feat`, `fix`, `perf`, breaking) is a patch. Seeded by `v0.0.0`.   |
| v0.1.0      | Cut deliberately: one PR switches the rules and carries a `feat` commit.                          |
| v0.1 → v0.x | `feat` → minor, `fix`/`perf` → patch, breaking → minor (pre-1.0).                                 |
| v1.0.0      | The breaking → minor rule is removed; the `--json` shape, exit codes and file formats are frozen. |

`ref`, `docs`, `chore`, `test`, `ci`, `build` and `style` commits release nothing. The owner merges every pull
request, so every release is a merge the owner chose.

## Consequences

- A commit's type decides whether it ships; `fix(ci)` is the way to make a pipeline fix produce a rehearsal release.
- `main` blocks deletion and force pushes but requires no status check: the release commit is pushed straight to
  `main` by `GITHUB_TOKEN`. CI must be green on the pull request before the owner merges.
- `mise run release -- --dry-run --no-ci` previews the next release locally (`GITHUB_TOKEN=$(gh auth token)`).
- `CHANGELOG.md` is generated; formatters and typos skip it.
