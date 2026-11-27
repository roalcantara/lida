# lida next: open tasks by urgency

- Date: 2026-11-27
- Status: approved design, awaiting the implementation plan
- Part of: `8NUT` "Design the CLI evolution" (v0.2)
- Precedence: this document wins over every earlier draft of `next`. For the `next` command only, it overrides
  DESIGN.md's shared colour table and CONTEXT.md's "shown as `#GWY9`". When it is built, DESIGN.md gets a `next`
  section and the snapshots become the byte-exact reference, as for every other command.

## Environment

Required: Ghostty only, 24-bit true colour, dark theme. The owner's setup is TokyoNight Storm over background
`#1D212F` at 90% opacity, with Cascadia Code NF at 15pt (a Nerd Font) and `adjust-cell-height = 10`.

Not goals, so nothing is designed for them: light themes, tmux, other terminals, dumb terminals, 16 or 256-colour
fallbacks, colour-blind support, row limits or `--all`, rounded shapes.

## Intent

`~/Work/kb/docs/jobx/TODO.yaml` holds every pending task across the owner's projects: 230 tasks in the task schema
(`kb/docs/superpowers/specs/2026-11-25-task-schema-design.md`), of which 185 are open, across 8 top-level projects.
`lida list` shows every task in file order, Done ones included. `lida next` answers "what next": at a glance, the
urgent tasks, then the next ones, then the overdue ones, then everything else, in a visual language learned once:
`@` is a project, `+` is a tag, and each colour has one meaning in its column.

## Goals

- `lida next` (alias `nx`) lists the open tasks, most urgent first, on a dark Ghostty in true colour.
- Filter words narrow the list: `@project`, `+tag`, `key:value` and plain words.
- `-l` / `--long` adds the URG, TAGS and WHY columns.
- `--raw` prints plain text with every field, for copying; a pipe or a file gets `--raw` automatically.
- `--json` prints the same tasks, in the same order, in the shape `lida list --json` uses.

## Non-goals

- Computing urgency: lida reads the stored `meta.urgency`.
- Making `project`, `depends` or `wait` first-class model fields: they stay in `extra`, read-only.
- Excluding a tag (`-tag`): a leading `-` reads as an option.
- Projects in `TODO.md`: its reader keeps no task-level extra keys, so `@project` and `key:value` never match a Markdown
  task.
- A colour setting in `.lida/config` (a follow-up).

## Anti-goals

- Changing `lida list`, `show` or any other command, their output or their snapshots.
- Writing to the file: `next` is read-only.
- Changing the `--json` task shape.

## Approaches

|               | Approach                                                      | Trade-off                                                               |
| ------------- | ------------------------------------------------------------- | ----------------------------------------------------------------------- |
| **A, picked** | Query rules in `lida-core`; table, raw and JSON in the binary | Filtering and sorting reusable by the TUI and the library API; testable |
| B             | Everything in `commands/next.rs`                              | Fastest, but the TUI would copy the rules; one file holding many ideas  |
| C             | A general report engine with configurable columns             | Machinery for reports that do not exist yet                             |

Command shape (settled earlier): a new command rather than changing `list` (DESIGN.md contract) or adding long flags to
it (too long for the query run most often). `@project` rather than Taskwarrior's `project:x` (shorter); `+tag` as in
Taskwarrior (`#tag` is a comment in zsh, and ids were printed as `#aaaa`).

## Behaviour

```text
lida next [-l|--long] [--raw] [--json] [WORD]...
lida nx   …
```

- **Open:** status is not Done, and `wait` is absent or not after today (local date).
- **Waiting:** status Blocked, or a `depends` id whose task is not Done. Waiting tasks stay open and keep their place.
- **Order:** `meta.urgency` descending (negative scores allowed; tasks without one last), then priority H, M, L, none,
  then id ascending. Overdue is a colour, not a group: on the owner's data every overdue task already ranks urgent.
- **Filter words** (any mix; different kinds must all match):

  | Word                | Matches                                                                                     | Several of the same kind |
  | ------------------- | ------------------------------------------------------------------------------------------- | ------------------------ |
  | `jobx` (no prefix)  | the description contains it, ignoring case                                                  | all must appear          |
  | `@jobx`             | project `jobx` or a dotted subproject (`jobx.hunt`; not `jobxa`)                            | any                      |
  | `+series`           | the task has the tag                                                                        | all                      |
  | `purposes:branding` | `meta.purposes` contains it (a list) or `meta.<key>` equals it (a scalar, e.g. `lead:need`) | any, per key             |

  Counts on the owner's data: `lida nx jobx` 2, `@dots.agentic` 4, `@dots @jobx` 99, `+series` 1, `+series +foo` 0,
  `term @dots @jobx` 2.
- **No match**, for any reason (an unknown key, a lone `@` or `+`, a typo): `No tasks found.` A typo can only narrow
  the list, never widen it. This reverses the earlier draft's "any other word is a usage error (exit 2)".
- Words starting with `-` are options. Text containing `:` cannot be searched (it reads as `key:value`).
- Outside a lida folder: `error: no TODO file found in directory`, exit 1 (as every command).
- Read-only: `next` never writes.

## The look (Ghostty, a terminal)

### `lida nx`

Generated from the mock's renderer (real data, today shifted to Thursday 2026-12-03 so overdue tasks exist; colour
and the chip are described below the frame):

```text
ID     P  DUE    PROJECT         DESCRIPTION
gLIS   H  6d     @jobx.hunt      Log outcomes for the 18 applied opportunities
nTe4   H  1d     @jobx.learn     Study the Forward Deployed Engineer path and write a fit note
DV8O   H  Fri    @jobx.hunt      Build the seed list: 30 people who know your work
G2ri   H  Fri    @jobx.bridge    Apply to 3 platform-task-work roles
GmuC   H  6d     @jobx.cadence   Weekly recalibration, every Friday 45 min
HGGP   H  6d     @jobx.hunt      Follow up the 5 in-progress opportunities and decide GM Financial
ijrV   H  2d     @jobx.bridge    Open the bridge queue: 3 messages to network and prior employers
wuu5   H  6d     @jobx.cadence   Define the weekly scoreboard
6hTP   H  1d     @jobx.hunt      Brainstorm strategy rev 5: why 46 boards gave silence
mxtA   L  -      @lida           README demo (asciinema or GIF) of the Markdown and YAML round-trip
nYv5   L  -      @dots.blog      Series: account-wide numbers and decisions  +series
vINB   L  3d     @dots.blog      Reddit runway: comment and answer only, no own links  +community
6jNz   L  -      @summon         Store listing experiments on Play
ufyn   L  -      @summon         Launch posts: the blog, LinkedIn, r/AndroidGaming and r/IndieDev
```

In the terminal, `H`/`M`/`L` are Nerd Font letter discs, `nYv5` (in progress) sits on a green chip, `6hTP` (waiting on
`gLIS` and `nTe4`) has a muted description, and `6d`, `1d`, `2d`, `3d` are overdue (red).

### `lida nx -l`

```text
ID      URG  P  DUE    PROJECT         TAGS               WHY                      DESCRIPTION
gLIS   23.0  H  6d     @jobx.hunt                                                  Log outcomes for the 18 applied opp…
6hTP   18.0  H  1d     @jobx.hunt                                                  Brainstorm strategy rev 5: why 46 b…
mxtA    5.8  L  -      @lida                              showcase need branding   README demo (asciinema or GIF) of t…
nYv5    5.8  L  -      @dots.blog      +series            branding                 Series: account-wide numbers and de…
6jNz   -3.2  L  -      @summon                            showcase income          Store listing experiments on Play
```

(URG is 5 cells wide so `-13.2` fits; the frame above was drawn at 4, before that call.)

### Rulers and lanes (at width W; the owner's real width is not yet known, 120 shown)

| Ruler       | `lida nx` | `lida nx -l`  | What sits on it                                                                     |
| ----------- | --------- | ------------- | ----------------------------------------------------------------------------------- |
| ID lane     | 0–6       | 0–6           | 6 cells: a space, the 4-character id, a space. The chip fills this lane, never more |
| P           | 8         | after URG     | 1 cell, a Nerd Font disc                                                            |
| URG         | –         | right edge    | numbers end on their ruler; one decimal; `-` when absent                            |
| DUE         | 11        | +3            | 5 cells, left-aligned                                                               |
| PROJECT     | 18        | +7            | 14 cells; longer names cut with `…`                                                 |
| TAGS        | –         | after PROJECT | 17 cells; cut with `…`                                                              |
| WHY         | –         | after TAGS    | 23 cells; cut with `…`                                                              |
| DESCRIPTION | 34        | to W          | from its ruler to the edge W, cut with `…` only at W                                |

Alleys between columns are 2 spaces. Nothing crosses a ruler. The header is a dark band (`#161b22`) with muted bold
labels. No stripes, no glyph columns, ids without `#`.

**Narrow windows.** `lida nx` fits at 80 columns (the description gets 46 cells). `lida nx -l` drops columns, in this
order, until the description has at least 30 cells: WHY, then TAGS (which then follow the description, as in `lida nx`),
then URG.

**Tags.** In `lida nx`, a task's `+tags` follow its description, bold, after two spaces; the description is cut first so
the tags stay whole. In `-l` they are a column instead.

**Search matches** are underlined in the description, mapped back to the original characters (so `résumé` underlines
whole letters).

### Colour (one meaning per colour, per column)

| Element                   | Style                                                                                                                                                                                                                                                                                   |
| ------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| id                        | blue `#7aa2f7`, bold                                                                                                                                                                                                                                                                    |
| task in progress          | the id on a green `#9ece6a` chip with dark text `#1d202f`, inside the ID lane: green behind the 4 characters plus a ⅛-cell margin each side, drawn with `▕` (U+2595) in the cell before and `▏` (U+258F) in the cell after, both green                                                  |
| priority                  | Nerd Font filled letter disc in gold `#e0af68`: `md-alpha-h-circle` U+F0C01, `-m-` U+F0C10, `-l-` U+F0C0D (one cell wide in Cascadia Code NF); blank when the task has no priority                                                                                                      |
| DUE overdue               | days late, no "ago", red `#f7768e` bold: `6d`                                                                                                                                                                                                                                           |
| DUE today / within a week | `today` for today (a time earlier today still counts as today, not overdue), the weekday for 1–6 days ahead, bright bold `#c0caf5`: `Fri`                                                                                                                                               |
| DUE later                 | days for 7–59 days ahead (`36d`); from 60 days a date (`Feb 20`, with the year when it differs: `Feb 20 2028`); plain `#a9b1d6`                                                                                                                                                         |
| DUE none                  | `-`, muted                                                                                                                                                                                                                                                                              |
| project                   | `@top` bold in the project's colour, `.sub` in the same colour at 70% (mixed over the background)                                                                                                                                                                                       |
| project colours           | 8 colours, each pair ΔE ≥ 16 on `#1D212F`, contrast ≥ 3:1: `#12aedb`, `#d78b3d`, `#36ba6d`, `#d6da4d`, `#aa86f5`, `#d958c0`, `#4be9fc`, `#fdb6f0`. A project's colour comes from a stable hash of its top-level name, so it is the same in every repository; two projects may share one |
| tags                      | aqua `#03FFD2`, bold when they follow the description                                                                                                                                                                                                                                   |
| URG (`-l`)                | plain `#c0caf5`; muted for negative scores                                                                                                                                                                                                                                              |
| WHY (`-l`)                | purposes as words, the lead first in bold `#c0caf5`, the others muted; blank when none                                                                                                                                                                                                  |
| description               | plain `#c0caf5`; muted `#737aa2` for waiting tasks (no italics)                                                                                                                                                                                                                         |
| muted                     | `#737aa2` (TokyoNight's comment colour, 3.8:1)                                                                                                                                                                                                                                          |

Ghostty paints explicit cell backgrounds (the chip, the header band) opaque over the 90% window background; that is
accepted. Every text colour passes 3:1 on `#1D212F`; the palette was checked with the dataviz skill's validator.

## `--raw` (and every pipe or file)

Plain text with every field, nothing cut, no header, no colour, no Nerd Font glyphs: the id, the priority letter, the
exact due date and time, the project, then the description with a state word where one applies and the tags after it.
Columns are separated by tabs (shown as `→` below). `-l --raw` adds the urgency, the tags column and the purposes, still uncut. Generated:

```text
gLIS → H → 2026-11-27 → @jobx.hunt → Log outcomes for the 18 applied opportunities
nTe4 → H → 2026-12-02 → @jobx.learn → Study the Forward Deployed Engineer path and write a fit note
nYv5 → L →  → @dots.blog → (doing) Series: account-wide numbers and decisions +series
6hTP → H → 2026-12-02 → @jobx.hunt → (waiting) Brainstorm strategy rev 5: why 46 boards gave silence
```

State words: `(doing)` for in progress, `(waiting)` for a task with an open `depends`, `(blocked)` for status Blocked.

## Settled for the builder (from the builder check)

- **Waiting** counts a `depends` task that is not Done even when its own `wait` hides it from the list.
- **Projects:** `@project` and PROJECT read only the task's own `project` (no fallback to the document's project name).
  When no shown task has a project, the PROJECT column is left out.
- **Mode:** a terminal gets the table; a pipe or a file gets `--raw`. `NO_COLOR` on a terminal also means `--raw`,
  because the chip, the waiting mute and the overdue red would otherwise carry no signal. `CLICOLOR_FORCE` on a pipe
  gets the coloured table at `COLUMNS` wide, or 80 when unset (for snapshots).
- **`-l --raw`** columns, tab-separated: id, urgency (empty when absent), priority letter, due, project, tags
  (`+a +b`), purposes (lead first, space-separated), description (with its state word). Tags are not repeated after the
  description.
- **Bad data** never hides the list: a non-numeric urgency sorts as no urgency, an unparsable `due` or `wait` is treated
  as absent, and a `depends` entry that is not a known id is ignored.

## Text safety

Descriptions, tags and projects come from a hand-edited file. Control characters (C0, C1, ESC) are escaped and newlines
become spaces before printing, so one task is always one line and nothing in a task can move the cursor or change the
terminal.

## Stress story (every frame must handle these)

1. `lida nx`: 185 rows, mixed projects.
2. `lida nx -l`.
3. `lida nx @jobx`, `@dots.agentic`, `@dots @jobx`.
4. `lida nx +series purposes:branding` and `lida nx lead:need`.
5. `lida nx @nope`: `No tasks found.`
6. `lida nx jobx` (search, matches underlined) and `lida nx term +series +foo @dots @jobx` (every kind, empty).
7. A 220-character description.
8. A task with 4 tags and 3 purposes.
9. The longest project name.
10. `résumé`, an emoji, CJK text.
11. Negative urgency, no due date, no priority.
12. An 80-column window.
13. `lida nx | cat`, `--raw`, `--json` (empty: `[]`).
14. Outside a lida folder.

## Build notes (approach A)

- `lida-core`: a query module (filter words parsed into a query; open, waiting, order) that takes today's date as an
  input. A date library is needed (jiff or time; must pass `cargo deny`); due values may carry a time
  (`2026-11-24 09:00`).
- `crates/lida`: `commands/next.rs` (table, raw, JSON), a styling helper for exact true-colour values (`output.rs` has
  named colours only), window width (terminal_size), and Unicode width and grapheme-safe cutting (unicode-width,
  unicode-segmentation). Keep these out of `lida-core` (the core-purity hk step).
- `cli.rs`: the `next` command with alias `nx`, `-l/--long`, `--raw`, `--json`, and a variadic positional for the filter
  words (usage-derive supports both); zsh completion for `@` (projects in the file) and `+` (tags in use).
- Tests: snapshot the table with a fixed today and a fixed width (for example `COLUMNS` with `CLICOLOR_FORCE`); key
  truncation and the automatic `--raw` on the terminal check, not on the colour override.
- Commit type `feat` (a minor release before 1.0). DESIGN.md (`next` section) and CONTEXT.md (ids without `#` in `next`)
  change in the same commit as the code.

## Left to the builder

The module names inside the approach, the exact hash for project colours, the escaping format for control characters,
and the zsh completion details.

## Considered and set aside

In-progress marks: ▶/⧗ glyphs, white or brighter-blue id, orange `#ff6507`, pill, gradient, double underline,
current-line band, half-block chip in a gutter, ¼ and ½ margins. Priority: amber ramp, `#FD6D5E` discs (too close to
overdue red), heat cell, H-only fill, arrows, bars, `!!!`, edge bar, words, description weight, distinct hues, absence,
outline ring. Purposes: letter slots, dots. Layout: zebra stripes, tier sections, bottom-up order, rank numbers,
board-like sections, "after #id" prefixes, a single violet for every project, row limits. Filters: exit 2 for unknown
words and keys.
