# `lida next`: in-progress first, due filters, readable priority

Size: a small change (three independent tweaks to `next`). Short design.

## Intent

`lida nx` is the daily "what now?" view. Three gaps showed up in real use (Ghostty, 24-bit colour, dark):

1. Work already started is scattered through the list by urgency; it should lead.
2. There is no way to ask "what is due today?" (nor overdue, tomorrow, this week).
3. The priority disc (Nerd Font circled letter) renders too small to read; `H` and `M` are hard to tell apart.

## Goals

- In-progress tasks sort above every other open task, in table, `--raw`, and `--json` output alike.
- Due filters as Taskwarrior-style virtual tags: `+OVERDUE`, `+TODAY`, `+TOMORROW`, `+WEEK`.
- A priority marker clearly more readable than today's disc, in the same 1-cell `P` column.

## Non-goals

- No general date expressions (`due.before:eom`, `due:2026-12-01`); no OR syntax for ordinary filter words.
- No other virtual tags (`+ACTIVE`, `+BLOCKED`) in this change.
- No change to urgency itself or to the `DUE` column's rendering.

## Anti-goals

- A filter that silently hides overdue work when the user asked for "today and overdue".
- Breaking an existing `meta.due` lookup (which is why this does not take over `due:`).

## Approaches

Ordering (1): one sensible way, a status key placed ahead of urgency in the core sort, so every output mode
agrees. A large urgency bonus (Taskwarrior's `active` coefficient) was rejected: it would not guarantee "on top".

Due filter (2), the operator picked **C**:

- A. One word meaning due today or earlier: simple, but cannot ask for "exactly today" or "this week".
- B. Reserved `due:` key (`due:today`): familiar from Taskwarrior, but breaks filtering on a `meta.due` field.
- **C. Virtual tags `+OVERDUE` `+TODAY` `+TOMORROW` `+WEEK`** (picked): most complete, no clash with `meta`, and
  completes like ordinary tags.

Priority (3), the operator handed over the how ("more readable than today"); the call is below.

## Design

**Ordering.** `compare_entries` in `crates/lida-core/src/next.rs` gains a first key: `InProgress` before anything
else; then the existing urgency, priority, ID order. `Blocked` and `Todo` keep their current relative order.

**Virtual due tags** (`crates/lida-core/src/query.rs`). `+WORD` where `WORD` is one of the four reserved uppercase
names becomes a due clause instead of a tag clause. Against `today`:

| tag         | matches a task whose `due` is |
| ----------- | ----------------------------- |
| `+OVERDUE`  | before today                  |
| `+TODAY`    | today                         |
| `+TOMORROW` | today + 1                     |
| `+WEEK`     | today through today + 6       |

Tasks without `due` never match. _Call:_ due clauses combine with **OR** among themselves (so
`+TODAY +OVERDUE` means "due today or overdue", the morning list) and with **AND** against every other clause, the same
rule `@project` already uses. Matching is case-sensitive: `+today` stays an ordinary tag. `+WEEK` uses the same
seven-day window as the `DUE` column's weekday lane, not a calendar week. Completion (`complete_next_word`) offers the
four names.

**Priority marker** (_call_). Replace the discs with a full-size bold letter `H`/`M`/`L` in the existing 1-cell `P`
column, coloured by level so the three can be told apart at a glance: `H` `#ff9e64`, `M` `#e0af68`, `L` `#a9b1d6`;
blank when absent. Plain letters render at full cell height in any font, with no Nerd Font dependency.

```text
ID     P  DUE     PROJECT         DESCRIPTION
▕Ab12▏ M  today   @jobx.hunt      Follow up the 5 in-progress opportunities
gLIS   H  today   @jobx.hunt      Log outcomes for the 18 applied opportunities
wUol   L  7d      @lion           Contact Lion and agree on scope and next steps
```

**Contract.** DESIGN.md (`next` section: ordering sentence, filter words, priority style row) and every affected insta
snapshot change in the same commits.

## Left to the builder

Test fixtures and names; where the due-tag enum lives; whether `--raw`'s priority letter is unchanged (it already
prints letters).
