# Design

The CLI's output contract: what each command prints, in which colour, on which stream, and with which exit code. It
documents what lida does today. The snapshots in [`crates/lida/tests/snapshots/`](crates/lida/tests/snapshots/) are the
byte-exact reference and win wherever this file differs. Visual identity (icon, palette) is in
[`assets/design/BRANDING.md`](assets/design/BRANDING.md).

## Voice

A precise, quiet UNIX tool. One line per result, no exclamation marks, no chatter. Success lines start with a green `✓`;
errors start with `error:` and say what to do when there is a fix (`Use --force to overwrite.`).

## Streams and exit codes

- Results go to stdout. Errors go to stderr as one line, `error: <message>`, and exit `1`.
- Success exits `0`. Nothing else is printed on success; `--json` output is pure JSON with an empty stderr.

## Colour

Colour is applied with `owo-colors` and is always on.

| Element                             | Style                               |
| ----------------------------------- | ----------------------------------- |
| `✓` success mark                    | bright green                        |
| `error` label                       | bright red, bold                    |
| task IDs in messages, metadata keys | bold                                |
| table headers                       | bold                                |
| table rule, `-` placeholders, tags  | dimmed                              |
| status `Todo`                       | dimmed                              |
| status `InProgress`                 | bright yellow                       |
| status `Done`                       | bright green                        |
| status `Blocked`                    | bright red                          |
| priority `High` / `Medium` / `Low`  | bright red / bright yellow / dimmed |
| `No tasks found.`                   | dimmed                              |

Known gap: colour is emitted even when stdout is piped or `NO_COLOR` is set.

## Commands

Examples run against two tasks, `aaaa` (medium, `setup`) and `bbbb` (high, `setup, docs`). Colour codes are left out.

### `init`

```text
$ lida init --md --project demo
✓ Initialized project tasks in /path/to/repo
```

`--md` writes `TODO.md`, `--yaml` (the default) writes `TODO.yaml`, `--both` writes both. If either file exists:
`error: TODO file already exists. Use --force to overwrite.` (exit 1).

### `add`

```text
$ lida add "Third task" -p low -t a,b
✓ Added task #Xy7Q "Third task"
$ lida add --task-id aaaa -c "A note"
✓ Added comment to task #aaaa
```

The ID is four random base62 characters. An unknown priority is ignored silently (the task gets none). Errors:
``task description is required (e.g. \``lida add "Task description"\`)``, ``comment text is required when specifying
--task-id``, ``task #zzzz not found`.

### `update`

```text
$ lida update aaaa -d Renamed -p high --tag docs
✓ Updated task #aaaa
```

`-t`/`--tag` replace the task's tags.

### `list` (also bare `lida`)

```text
$ lida list
ID     STATUS       PRIORITY   TITLE                          TAGS
──────────────────────────────────────────────────────────────────────
#aaaa  Todo Medium First task                     setup
#bbbb  Todo High Second task                    setup, docs
```

Columns are padded before colour is applied, so coloured cells are not aligned. Filters: `-s` status (aliases below),
`-t` tag, `-p` priority. Empty result: `No tasks found.`.

`--tasks` prints one plain line per task, for shell completion:

```text
[Todo] [Medium] aaaa — First task
```

`--json` prints a JSON array of task objects (shape under `show`).

### `done`, `start`, `status`

```text
$ lida done aaaa
✓ Task #aaaa marked as done
$ lida start aaaa
✓ Task #aaaa marked as in-progress
$ lida status aaaa --set blocked --reason Waiting
✓ Task #aaaa status set to Blocked
```

Status values and aliases: `todo`; `in-progress`, `inprogress`, `progress`, `en-cours`, `encours`; `done`, `completed`;
`blocked`, `bloqued`. `--reason` is kept only for `blocked`. Error:
`invalid status 'nope'. Valid values: todo, in-progress, done, blocked`.

### `show`

```text
$ lida show bbbb
Task #bbbb: Second task
  Status: Todo
  Priority: High
  Tags: setup, docs
```

Optional lines: `Position: …` and a `Metadata:` block (`    key: value`). Comments, due date and blocked reason are
not shown. `--json` prints one task:

```json
{
  "id": "bbbb",
  "status": "Todo",
  "description": "Second task",
  "due": null,
  "actors": [],
  "comments": [],
  "blocked_reason": null,
  "tags": ["setup", "docs"],
  "priority": "High",
  "created": null,
  "position": null
}
```

`meta` appears only when set; unknown fields read from `TODO.yaml` are kept and printed alongside.

### `meta`

```text
$ lida meta aaaa set owner ro
✓ Set metadata owner for task #aaaa
$ lida meta aaaa get owner
ro
$ lida meta aaaa unset owner
✓ Removed metadata owner from task #aaaa
```

``set`` parses the value as JSON and falls back to a string; ``get`` prints strings bare and other values as JSON; ``rm`` is
an alias of ``unset``. Errors: ``metadata key 'missing' not set on task #aaaa``, ``value required for \``meta set <key>
<value>\```, ``unknown meta action 'frob'. Use set, get, or unset`.

### `tags`

```text
$ lida tags
  docs               1
  setup              2
```

Sorted by name; name padded to 15, count right-aligned to 4.

### `delete`

```text
$ lida delete bbbb
✓ Deleted task #bbbb
```

Also deletes the task's comments.

### `cwi`

```text
$ lida cwi
Current working format: MdOnly
$ lida cwi both
✓ Switched working format to Both
```

Values `yaml`, `md`, `both`; printed as `YamlOnly`, `MdOnly`, `Both`. Error: `format must be 'yaml', 'md', or 'both'`.

### `completion`, `usage`

`completion zsh` prints the bundled zsh script; any other shell prints `# Completion for <shell> not implemented yet`
(exit 0). `usage` prints a five-line `lida.usage.kdl` header.

## Errors shared by all commands

- `No TODO file found in directory` when neither `TODO.md` nor `TODO.yaml` exists.
- `task #<id> not found` for an unknown ID.

## Files

| Mode       | Files                   | Authoritative |
| ---------- | ----------------------- | ------------- |
| YAML only  | `TODO.yaml`             | `TODO.yaml`   |
| Markdown   | `TODO.md`               | `TODO.md`     |
| Dual-write | `TODO.yaml` + `TODO.md` | `TODO.yaml`   |

Writes are atomic: both files are written to temporary siblings, then renamed into place.
