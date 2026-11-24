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

Colour is applied with `owo-colors`. It is enabled on a terminal stream, unless overridden by standard environment
variables: `false` when `NO_COLOR` is set and non-empty; else `true` when `CLICOLOR_FORCE` is set, non-empty and not
`0`; else `is_terminal`.

| Element                                                                        | Style                               |
| ------------------------------------------------------------------------------ | ----------------------------------- |
| `✓` success mark                                                               | bright green                        |
| `error` label                                                                  | bright red, bold                    |
| task ID after a status change, `update`, `delete`, `show`'s title, `list` rows | bold                                |
| metadata key in `meta set` / `meta unset`                                      | bold                                |
| table headers                                                                  | bold                                |
| table rule, `-` placeholders, tags                                             | dimmed                              |
| status `Todo`                                                                  | dimmed                              |
| status `InProgress`                                                            | bright yellow                       |
| status `Done`                                                                  | bright green                        |
| status `Blocked`                                                               | bright red                          |
| priority `High` / `Medium` / `Low`                                             | bright red / bright yellow / dimmed |
| `No tasks found.`                                                              | dimmed                              |

IDs in `add` messages, errors and `show`'s metadata lines are not bold.

## Commands

Examples run against two tasks, `aaaa` (medium, `setup`) and `bbbb` (high, `setup, docs`). Colour codes are left out.

### `init`

```text
$ lida init --md --project demo
✓ Initialized project tasks in /path/to/repo
```

`--md` writes `TODO.md`, `--yaml` (the default) writes `TODO.yaml`, `--both` writes both. If either file exists:
`error: TODO file already exists. Use --force to overwrite.` (exit 1). With `--force`, the file of the format no longer written is deleted. Passing more than one format flag errors: `error: choose one of --yaml, --md or --both`. Project names cannot contain `-->` (`error: project name cannot contain '-->'`).

### `add`

```text
$ lida add "Third task" -p low -t a,b
✓ Added task #Xy7Q "Third task"
$ lida add --task-id aaaa -c "A note"
✓ Added comment to task #aaaa
```

The ID is four random base62 characters. Priority takes `low`, `medium`, `high`, or `none` (clears priority). Unknown values error. Errors:

```text
error: invalid priority 'urgent'. Valid values: low, medium, high, none
error: task description is required (e.g. `lida add "Task description"`)
error: comment text is required when specifying --task-id
error: task #zzzz not found
```

### `update`

```text
$ lida update aaaa -d Renamed -p high --tag docs
✓ Updated task #aaaa
```

`-t`/`--tags` replace the task's tags; `--tag` is a long-only alias, used only when `--tags` is absent.

### `list` (also bare `lida`)

```text
$ lida list
ID     STATUS       PRIORITY   TITLE                          TAGS
──────────────────────────────────────────────────────────────────────
#aaaa  Todo         Medium     First task                     setup
#bbbb  Todo         High       Second task                    setup, docs
```

Status and priority columns are padded before colouring so they align cleanly. Filters: `-s` status (aliases below; errors on unknown values),
`-t` tag, `-p` priority (`none` matches tasks without priority; errors on unknown values). Empty result: `No tasks found.`.

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

Status values and aliases: `todo`; `in-progress`, `inprogress`, `progress`, `en-cours`, `encours` (and `_` forms such as `in_progress`); `done`, `completed`;
`blocked`, `bloqued`. `--set blocked` stores `--reason` (or clears it when absent). Another status clears the reason
only when `--reason` is passed; `done` and `start` never clear it. Error:
`invalid status 'nope'. Valid values: todo, in-progress, done, blocked`.

### `show`

```text
$ lida show bbbb
Task #bbbb: Second task
  Status: Todo
  Priority: High
  Tags: setup, docs
```

Optional lines appear in order when present: `Due: …`, `Blocked reason: …`, `Position: …`, a `Metadata:` block
(`    key: value`), and a `Comments:` block (`    #<id> <text>`). `--json` prints one task:

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

`set` parses the value as JSON and falls back to a string; `get` prints strings bare and other values as JSON; `rm` is
an alias of `unset`. Errors:

```text
error: metadata key 'missing' not set on task #aaaa
error: value required for `meta set <key> <value>`
error: meta keys cannot contain ':'
error: unknown meta action 'frob'. Use set, get, or unset
```

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

Values `yaml`, `md`, `both`; printed as `YamlOnly`, `MdOnly`, `Both`. Switching deletes the file the new format no longer writes (`md` deletes `TODO.yaml`, `yaml` deletes `TODO.md`, `both` deletes nothing). Switching to `md` or `both` refuses if the document contains data Markdown cannot hold (actor types, actor pictures, task actors, comment actors, unknown fields): `error: TODO.md cannot hold <what> (<target>): stay on yaml or both`. Switching to `yaml` refuses if `TODO.md` holds unrecognised lines. Error: `format must be 'yaml', 'md', or 'both'`.

### `completion`, `usage`

`completion <shell>` generates the shell completion script for the named shell (`bash`, `elvish`, `fish`, `nu`, `nushell`, `powershell`, `pwsh`, `zsh`). Supports `--install` to write the script where the shell looks for it, and `--force` to replace an existing file. Unsupported shells exit 2.

`usage` prints the full portable `.usage.kdl` CLI specification generated from the CLI declaration.

## Errors shared by all commands

- `no TODO file found in directory` when neither `TODO.md` nor `TODO.yaml` exists.
- `duplicate task ID #<id> in <file>` when a file contains repeated task IDs.
- `TODO.md line <line> is not something lida writes: <text>` when `TODO.md` contains unrecognised content.
- `TODO.md cannot hold <what> (<target>): stay on yaml or both` when format conversion would lose fields.
- `task #<id> not found` for an unknown ID.
- `<field> must be a single line` when multi-line text is passed for a single-line field.

## Files

| Mode       | Files                   | Authoritative |
| ---------- | ----------------------- | ------------- |
| YAML only  | `TODO.yaml`             | `TODO.yaml`   |
| Markdown   | `TODO.md`               | `TODO.md`     |
| Dual-write | `TODO.yaml` + `TODO.md` | `TODO.yaml`   |

Switching formats (`cwi` or `init --force`) deletes the file no longer written. When both files exist, loading follows the configured format (`TODO.md` for `MdOnly`, else `TODO.yaml`). In `TODO.md`, tasks with comments record them with a trailing `  - **Comments**: <id>, ...` line. Lines that lida does not recognise prevent saving to protect unrecognised content from being silently overwritten. Writes are atomic: both files are written to temporary siblings, then renamed into place.
