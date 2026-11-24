# lida

## Project Name and Description

> **Zero ceremony. Dual-format. Git-native task management for developers and AI coding agents.**

Dual-format project task engine maintaining bidirectional, lossless synchronicity between human-readable Markdown
(`TODO.md`) and structured YAML (`TODO.yaml`).

## Technology Stack

- Rust 1.99+ (edition 2024), Cargo workspace (`lida-core` + `lida`)
- usage-rs for CLI parsing, owo-colors for terminal output
- serde, serde_json and serde_yaml_ng for the task model and its two file formats
- mise for tool versions and tasks; hk for git hooks and quality gates

## Project Architecture

| Crate / folder     | Role                                                                                     |
| ------------------ | ---------------------------------------------------------------------------------------- |
| `crates/lida-core` | Pure domain: task model, IDs, Markdown and YAML formats, the store and its atomic writes |
| `crates/lida`      | CLI: argument parsing, commands, completion, terminal output                             |
| `assets/design`    | Brand identity and logo proposals                                                        |
| `docs/superpowers` | Specs and implementation plans                                                           |
| `tools/graph`      | Code graphs for agents: CRG, Gortex, Graphify ([README](tools/graph/README.md))          |

`lida-core` never depends on `usage-rs`, `usage`, `owo-colors` or `lida`; the `core-purity` hk step enforces it. See
[CONTEXT.md](CONTEXT.md) for the vocabulary and [DESIGN.md](DESIGN.md) for the CLI output contract.

## Getting Started

**Install a release.** From a Brewfile (Homebrew Bundle), or with cargo directly:

```ruby
cargo "lida", source: "https://github.com/roalcantara/lida?tag=vX.Y.Z"
```

```bash
cargo install --locked --git https://github.com/roalcantara/lida --tag vX.Y.Z lida
```

Releases, their binaries and the changelog are on the [releases page](https://github.com/roalcantara/lida/releases)
([CHANGELOG.md](CHANGELOG.md)). From a checkout: `cargo install --locked --path crates/lida`.

**Create a task list.**

```bash
lida init --md --project my-project
lida add "Ship the first release" -t setup -p high
lida list
```

## Project Structure

```text
crates/
  lida-core/   pure domain logic (no CLI or terminal dependencies)
  lida/        binary, commands, completion, terminal output
assets/design/ brand identity and logo proposals
docs/          specs and implementation plans
tools/graph/   code graphs for agents: CRG, Gortex, Graphify (see its README)
```

## Key Features

- **Dual Native:** Read and write both `TODO.md` and `TODO.yaml` with atomic synchronization.
- **Git-Centric Reliability:** Collision-resilient random 4-character base62 IDs (`zOaU`).
- **First-Class Metadata:** Native typed key-value maps on tasks (`lida meta <id> set <key> <value>`).
- **Agent Ready:** Instant `--json` output on `list` and `show`.
- **Sub-millisecond Speed:** Built in Rust using `usage-rs`.

## Development Workflow

| Command                                 | Does                                                           |
| --------------------------------------- | -------------------------------------------------------------- |
| `mise run setup`                        | Install tools, fetch crates, and install git hooks             |
| `mise run gate`                         | Run all quality gates in order (`--fast` for inner loop)       |
| `hk check --all`                        | Run all baseline repository checks                             |
| `mise run graph setup`                  | Initialize the code graphs for agents (CRG, Gortex, Graphify)  |
| `mise doctor project`                   | Verify code graph health, MCP wiring and secrets               |
| `mise run release -- --dry-run --no-ci` | Preview the next release (CI releases on every push to `main`) |

## Coding Standards

- Follow [AGENTS.md](AGENTS.md) and [CONTEXT.md](CONTEXT.md).
- `unsafe_code = "forbid"`, clippy `pedantic` with `-D warnings`, no crate- or module-wide `allow`.
- One concept per file, `module.rs` + `module/` (no `mod.rs`).

## Testing

```bash
cargo nextest run --workspace --locked
cargo test --doc --workspace --locked
```

## Contributing

Every change must pass `mise run gate` and follow Conventional Commits with a sentence-case subject (≤ 72 characters).
Pending roadmap items live in [TODO.md](TODO.md).

## License

[MIT](LICENSE).
