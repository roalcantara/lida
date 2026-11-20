# lida

> **Zero ceremony. Dual-format. Git-native task management for developers and AI coding agents.**

Dual-format project task engine maintaining bidirectional, lossless synchronicity between human-readable Markdown (`TODO.md`) and structured YAML (`TODO.yaml`).

## Features
- **Dual Native:** Read and write both `TODO.md` and `TODO.yaml` with atomic synchronization.
- **Git-Centric Reliability:** Collision-resilient random 4-character base62 IDs (`zOaU`).
- **First-Class Metadata:** Native typed key-value maps on tasks (`--meta key=val`).
- **Agent Ready:** Instant `--json` output across all commands.
- **Sub-millisecond Speed:** Built in Rust using `usage-rs`.

## License
MIT
