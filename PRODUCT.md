# lida

_"Zero ceremony. Dual-format. Tasks in Git."_ A project's task list lives in the repository, readable by people and
writable by agents.

## Why

Task trackers outside the repository drift from the code, need an account and a server, and are awkward for coding
agents. A hand-edited `TODO.md` stays close to the code but has no structure an agent can query safely. `lida` keeps
tasks in Git as plain files: Markdown for people reading a pull request, YAML for tools, or both kept in sync, with one
fast CLI and `--json` output on top.

## Users

- Developers who want their task list versioned, reviewed, and branched with the code, without a server.
- Coding agents that read and update tasks through `--json` and stable IDs, guided by [AGENTS.md](AGENTS.md),
  [CONTEXT.md](CONTEXT.md), and [DESIGN.md](DESIGN.md).

## Principles

1. **Files in Git are the database.** No server, no lock, no account; history and review come from Git.
2. **Lossless both ways.** Every save keeps what it did not write: unknown YAML fields, comments, metadata.
3. **Human and agent on equal footing.** Markdown reads well in a diff; YAML and `--json` are typed for tools.
4. **IDs that survive branches.** Short random IDs, not counters, so parallel branches rarely collide.
5. **Fast and quiet.** A single Rust binary, one line per result, nothing printed that a script would have to skip.

## Anti-goals

- A hosted service, sync backend, or web UI.
- A project-management suite: boards, sprints, estimates, notifications.
- Rewriting a file the user did not ask to change, or losing a field it does not understand.
- Output that needs scraping: anything an agent needs is available as `--json`.
