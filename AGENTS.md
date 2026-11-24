# AGENTS.md

## Project overview

`lida`: a Rust CLI that keeps a project's tasks in Git as `TODO.md`, `TODO.yaml`, or both kept in lossless sync, with
`--json` output for coding agents.

## Repository map

- `crates/lida-core/`: pure domain logic (task model, Markdown and YAML formats, store with atomic dual-write, ID
  generator). No CLI, terminal-colour, or binary dependency.
- `crates/lida/`: binary crate. `cli.rs` declares the command line (usage-rs), `commands/<name>.rs` runs one
  subcommand each, `output.rs` is the only place output streams live.
- `crates/lida/tests/snapshots/`: golden output of every command (insta).
- `assets/design/`: brand identity and icon proposals.
- `docs/superpowers/`: design specs and implementation plans.
- `tools/graph/`: code graphs for agents (code-review-graph MCP server, Gortex, Graphify); `mise run graph`, see its
  README. Graph outputs are local state and never committed.
- `PRODUCT.md`: why `lida` exists, users, principles, and anti-goals.
- `DESIGN.md`: the CLI output contract (copy, colours, streams, exit codes, JSON shape). Read before changing output.
- `CONTEXT.md`: domain vocabulary.
- `TODO.md`: pending work, managed with `lida` itself (see Tasks).

## Core engineering rules

- `lida-core` must never depend on `usage-rs`, `usage`, `owo-colors`, or `lida`, directly or transitively. The
  `core-purity` hk step enforces it; do not bypass it.
- Keep quality gates and lint thresholds unchanged (`unsafe_code = "forbid"`, clippy `pedantic`, `print_stdout` and
  `print_stderr`, `-D warnings`, jscpd threshold 0). No crate- or module-wide `allow`; use scoped
  `#[expect(…, reason = "…")]` only.
- Commands write through the `out` writer they are given, never `println!`; errors are returned, and `main` prints them
  once as `error: <message>` with exit code 1.
- Writes are atomic (temporary sibling, then rename) and lossless: unknown YAML fields (`extra`) survive every
  round-trip.
- A change to output must update its snapshot (`INSTA_UPDATE=always cargo nextest run -p lida`, then review each
  `.snap` diff) and DESIGN.md in the same commit. A refactor must leave every snapshot unchanged.

## Naming conventions

- Crate directories kebab-case; Rust files and modules snake_case; one concept per file.
- Prefer `module.rs` with `module/` children; no `mod.rs`.
- Use the domain vocabulary in [CONTEXT.md](CONTEXT.md).

## Setup

```bash
mise run setup
```

## Validation

```bash
mise run gate
```

It runs, in order, stopping at the first failure: `cargo nextest run`, `cargo test --doc`, `cargo clippy -D warnings`,
`cargo build --release`, `hk check --all`, `cargo deny check`, and `cargo doc -D warnings`. `mise run gate --fast` runs
only the first three. `mise run audit` adds the slow and release checks (coverage, full-history gitleaks, zizmor).

## Testing expectations

- Write the failing test first. Unit tests sit beside the code in `#[cfg(test)]` modules; integration tests live in
  `crates/*/tests/`.
- CLI output is verified with insta snapshots in `crates/lida/tests/output_test.rs`; fixtures use fixed task IDs.

## Commits

Conventional Commits with a sentence-case subject, at most 72 characters.

The type decides the release ([docs/adr/0001-releases.md](docs/adr/0001-releases.md)): `feat`, `fix` and `perf` ship on
the next merge to `main`; `ref`, `docs`, `chore`, `test`, `ci`, `build` and `style` do not. The changelog parser reads any
body line that starts with "BREAKING CHANGE" (any case) as a breaking-change note: never start a body line with
those words unless the commit really is one. Before 1.0, a breaking
change bumps the minor version.

Stage named paths with `git add <paths>`, then run `git commit`; never `git commit <paths>` and never `git commit -a`.
Format with `hk fix --all` before running the gate; `hk check --all` and `mise run gate` only check.

## Tasks

`TODO.md` is written only by `lida`; never edit it by hand (no formatter touches it either). From the repository root:

```bash
cargo run -q -p lida -- list                       # open work
cargo run -q -p lida -- add "Subject" -t tag -p medium
cargo run -q -p lida -- start <id>
cargo run -q -p lida -- done <id>                  # stage TODO.md with the commit that finishes it
```

Do not run `lida cwi` or `lida init` here: they would change the repository to another format or write `.lida/config`.
