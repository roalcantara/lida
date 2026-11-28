# Target file flag and `--annotate` design

Date: 2026-11-28. Size: small change.

## Intent

Change a task's status and leave a note in one command (`lida done GmuC -a "…"`), and run lida against a TODO file in
another directory (`lida -f ~/Work/kb/docs/jobx/TODO.yaml nx` from `~/Work/lida`). The note uses the shape the jobx
file already has, which seeds the coming schema work (8NUT) without pre-empting it.

## Goals

1. A global `-f, --file <path>` flag and a `LIDA_FILE` environment variable select the TODO file for every command.
2. `-a, --annotate "Text"` on `add`, `start`, `done` and a new `stop` appends an annotation in the same save.
3. `lida stop <ID>` pauses a task: status back to `todo`.

## Non-goals

- Schema changes: no new model fields, no `start`/`end`/`modified` timestamps, no event history, no status renames.
  All of it goes to 8NUT.
- A standalone `lida annotate`, ID-first commands (`lida <ID> annotate`), showing annotations in `show`.
- Markdown support for annotations.

## Anti-goals

- Rewriting or reordering existing annotations or any other unknown field.
- A half-applied command: a refused `-a` writes nothing.

## Constraints

- `lida-core` stays free of CLI dependencies; the task model does not change (annotations live in `extra`).
- Lossless round-trip of unknown fields; atomic writes.
- Output changes update snapshots and DESIGN.md in the same commit.

## Approaches

- **Annotations through `extra` (picked).** jobx tasks already carry `annotations: [{entry, description}]`, which lida
  keeps as an unknown field. Appending to it needs no model change and matches the target schema (TaskWarrior names).
- Core model change: a typed `annotations` field. Rejected: settles the schema before 8NUT.
- lida comments (`comments:` with IDs). Rejected: not the target schema; jobx has none.

## Design

### `-f, --file <path>` and `LIDA_FILE`

- Declared once on the root command as `global`, so every subcommand accepts it before or after its name; `env =
  "LIDA_FILE"` supplies it when the flag is absent; the flag wins.
- Resolution: a directory is used as is; a file whose name is `TODO.yaml` or `TODO.md` resolves to its parent
  directory; any other file name is an error. A path that does not exist is an error naming the path. `~` is left to
  the shell.
- The resolved directory replaces the current directory everywhere `commands::run` passes `cwd`, including `init`
  (creates the file there) and `cwi`. lida then behaves as if run in that directory (its `.lida/config`, dual-write).
- Task-ID and tag completion load the store from the same target (flag from the completion context when available,
  else `LIDA_FILE`, else the current directory).
- `init --force` loses its `-f` short form. Breaking change: the next release is 0.4.0 (`feat!` or a `BREAKING
  CHANGE` footer on that commit only).

### `-a, --annotate "Text"`

- On `add`, `start`, `done` and `stop`. Single-line text (same check as comments today).
- Appends `{entry: <today, local, YYYY-MM-DD>, description: <text>}` to the task's `annotations` sequence in `extra`,
  creating it when absent. Existing entries are untouched; the new one goes last.
- Status change and annotation are one save.
- Markdown-only mode: refused before any write with `error: TODO.md cannot hold annotations (task #<ID>): stay on yaml
  or both`. Without `-a`, every command behaves as today in every mode.
- Output: the command's usual single line, with `, with annotation` appended, e.g.
  `✓ Task #GmuC marked as done, with annotation`.

### `lida stop <ID>`

- Sets status to `todo` from any status; never refuses. Does not clear `blocked_reason` (as `start`/`done`).
- Output: `✓ Task #<ID> marked as todo` (plus `, with annotation`).

## Left to the builder

- How `entry`'s date is obtained (reuse the date helper `next` uses for "today").
- Exact usage-rs mechanics for the global flag and for reading it during completion.
- Help text wording, within DESIGN.md's copy rules.

## Hand-off to 8NUT

A comment on 8NUT records the deferred schema questions: TaskWarrior names (`entry`, `start`, `end`, `modified`),
`in-progress` versus "active", status-change history, ID-first commands, freezing Markdown, and a one-off migration
script rather than a `migrate` command.
