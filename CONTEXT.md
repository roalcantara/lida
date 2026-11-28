# Context & Vocabulary

Canonical terms used across code, tests, config, and docs.

- **task**: One unit of work: ID, description, status, and optional priority, tags, due date, position, meta, and
  comment IDs.
- **ID**: Four random base62 characters (`0-9a-zA-Z`, e.g. `GWY9`), unique within the document; shown as `#GWY9` (`next`
  shows it without `#`). Random rather than sequential so tasks added on different branches rarely collide.
- **status**: `todo`, `in-progress`, `done`, or `blocked`. Accepted aliases: `inprogress`, `progress`, `en-cours`,
  `encours` (in-progress); `completed` (done); `bloqued` (blocked).
- **blocked reason**: Free text set by `status --set blocked --reason`; another status clears it only when `--reason`
  is passed, and `done`/`start` never clear it.
- **priority**: `low`, `medium`, or `high`; optional.
- **tags**: Comma-separated labels on a task; `--tag`/`--tags` replace them.
- **position**: Where the work lives in code: a path, `path:line`, or URL.
- **meta**: Typed key-value pairs on a task (`lida meta`); a value is JSON when it parses, a string otherwise.
- **comment**: A note with its own ID, attached to one task; deleting the task deletes its comments.
- **annotation**: A dated note in a task's `annotations` list (`entry`, `description`), TaskWarrior's shape; appended
  by `-a`/`--annotate`; YAML only.
- **actor**: A person or agent with an ID and a pseudonym, listed under `# Actors`.
- **project document**: Everything in one TODO file: project name, tasks, actors, comments.
- **store**: Loads and saves the project document in the working directory.
- **format mode**: Which files the store writes: YAML only (`TODO.yaml`), Markdown only (`TODO.md`), or dual-write
  (both). Inferred from the files present, or read from `.lida/config` (or `.todo/config` when that exists); switched with
  `lida cwi`. Switching format mode deletes the file no longer written so only one source of truth remains.
- **authoritative file**: The file read back in dual-write mode: `TODO.yaml`. `TODO.md` is regenerated from it.
- **atomic write**: Every save writes temporary siblings, then renames them into place, so a crash never leaves a
  half-written file.
- **extra fields**: Unknown keys read from `TODO.yaml`, kept and written back unchanged.
