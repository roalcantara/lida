<!-- todo-project: lida -->

# Tasks

- [x] 4eof **-** Port hk hooks and tool configs
  - **Priority**: medium
  - **Tags**: setup

- [x] zuT2 **-** Pin CLI output with snapshot tests
  - **Priority**: medium
  - **Tags**: setup

- [x] 4mRA **-** Write DESIGN.md output contract
  - **Priority**: medium
  - **Tags**: setup

- [x] NmVi **-** Adopt toma lint table and refactor CLI output
  - **Priority**: medium
  - **Tags**: setup

- [x] V0PI **-** Write AGENTS, CLAUDE, CONTEXT and PRODUCT docs
  - **Priority**: medium
  - **Tags**: setup

- [x] F545 **-** Move CI onto mise and hk
  - **Priority**: medium
  - **Tags**: setup

- [ ] AmZm **-** Respect NO_COLOR and non-TTY output (no ANSI when piped)
  - **Priority**: high
  - **Tags**: bug, output, stable, v0.1.1
  - **Position**: "crates/lida/src/output.rs"

- [ ] DcPc **-** Reject unknown priorities instead of silently clearing them (add -p urgent, update -p none)
  - **Priority**: medium
  - **Tags**: bug, cli, stable, v0.1.1
  - **Position**: "crates/lida/src/commands.rs"

- [ ] 1557 **-** Show comments, due date and blocked reason in lida show
  - **Priority**: medium
  - **Tags**: bug, output, stable, v0.1.1
  - **Position**: "crates/lida/src/commands/show.rs"

- [ ] 8RVU **-** Make error message capitalisation consistent (No TODO file found vs task not found)
  - **Priority**: low
  - **Tags**: bug, output, stable, v0.1.1

- [ ] DjKn **-** Align coloured status and priority columns in lida list
  - **Priority**: low
  - **Tags**: bug, output, stable, v0.1.1
  - **Position**: "crates/lida/src/commands/list.rs"

- [x] 2hI8 **-** Create the public GitHub repo, add origin, push, open the PR
  - **Priority**: high
  - **Tags**: v0.1.0, repo

- [x] 7Wi3 **-** Configure repo security settings, merge methods and the main ruleset
  - **Priority**: high
  - **Tags**: v0.1.0, repo, security

- [x] gMYx **-** Add semantic-release with the v0.0.x rehearsal rules and the v0.0.0 seed tag
  - **Priority**: high
  - **Tags**: v0.1.0, release

- [x] BWYZ **-** Fold release.yml and audit.yml into ci.yml (audit, plan, build, release jobs)
  - **Priority**: high
  - **Tags**: v0.1.0, ci, release

- [x] vIoq **-** Single-source the version and redact it in snapshots
  - **Priority**: high
  - **Tags**: v0.1.0, release

- [x] HUoE **-** Add SECURITY.md and Dependabot
  - **Priority**: medium
  - **Tags**: v0.1.0, security

- [x] a9kP **-** Resolve the RUSTSEC-2024-0449 ignore in deny.toml
  - **Priority**: medium
  - **Tags**: v0.1.0, security

- [x] eJCQ **-** Set rust-version to the tested toolchain (or test the MSRV)
  - **Priority**: low
  - **Tags**: v0.1.0, build

- [x] PjLo **-** Write the releases ADR and the README Install section
  - **Priority**: medium
  - **Tags**: v0.1.0, docs

- [x] dSjs **-** Verify a v0.0.x release: tag, CHANGELOG.md, version bump and the three assets agree
  - **Priority**: high
  - **Tags**: v0.1.0, release

- [x] Mqa7 **-** Rehearse installs from a v0.0.x tag: cargo install --git --tag and brew bundle with a temporary pin
  - **Priority**: high
  - **Tags**: v0.1.0, release

- [x] apDG **-** Cut v0.1.0: switch to the v0.1.0 release rules with a feat commit
  - **Priority**: high
  - **Tags**: v0.1.0, release

- [ ] SXJ8 **-** Pin the dots Brewfile to v0.1.0 and run brew bundle (owner)
  - **Priority**: high
  - **Tags**: v0.1.0, dots

- [x] OvZw **-** Audit every command against README and DESIGN.md
  - **Priority**: high
  - **Tags**: stable, audit

- [ ] 5BI3 **-** README advertises --meta key=val, which does not exist
  - **Priority**: medium
  - **Tags**: stable, docs, v0.1.1

- [ ] Yiq8 **-** lida usage prints a stub instead of the real usage spec
  - **Priority**: medium
  - **Tags**: stable, cli, v0.1.1

- [ ] 8NUT **-** Design the CLI evolution
  - **Priority**: medium
  - **Tags**: v0.2, design

- [ ] Q8Vq **-** Design the lida-core library API
  - **Priority**: medium
  - **Tags**: v0.2, design

- [ ] GDS5 **-** Design a TUI
  - **Priority**: low
  - **Tags**: v0.2, design

- [ ] tgxh **-** Freeze and document the --json schema, exit codes and file formats
  - **Priority**: high
  - **Tags**: v1.0, contract

- [ ] xctA **-** Remove the pre-1.0 breaking-to-minor release rule
  - **Priority**: high
  - **Tags**: v1.0, release

- [ ] AkHy **-** Publish lida-core and lida to crates.io
  - **Priority**: medium
  - **Tags**: v1.0, distribution

- [ ] EnjS **-** Ship prebuilt binaries through a Homebrew tap
  - **Priority**: low
  - **Tags**: v1.0, distribution

- [ ] MRTo **-** Replace serde_yaml_ng (unmaintained unsafe-libyaml, RUSTSEC-2024-0449) with a safe YAML backend, keeping round-trips lossless
  - **Priority**: high
  - **Tags**: v1.0, security, deps
  - **Position**: "crates/lida-core/src/format/yaml.rs"

- [ ] vBbs **-** Generate shell completion from the CLI declaration (replaces todo-cli's zsh script)
  - **Priority**: high
  - **Tags**: v0.1.1, stable, cli

- [ ] pbSp **-** TODO.yaml writer drops null and empty keys (no position: null flood)
  - **Priority**: high
  - **Tags**: v0.1.1, stable, format

- [ ] 7X7b **-** Switching format (cwi) or init --force deletes the file lida stops writing; load follows the config
  - **Priority**: high
  - **Tags**: v0.1.1, stable, format, data-loss

- [ ] 9gc8 **-** Reject multi-line text (descriptions, comments, reasons) instead of corrupting TODO.md
  - **Priority**: high
  - **Tags**: v0.1.1, stable, format, data-loss

- [ ] ijwH **-** Keep comment-task links in TODO.md
  - **Priority**: high
  - **Tags**: v0.1.1, stable, format, data-loss

- [ ] K3CU **-** Refuse to save a TODO.md with content lida does not recognise
  - **Priority**: high
  - **Tags**: v0.1.1, stable, format, data-loss

- [ ] yBp7 **-** Reject meta keys containing a colon
  - **Priority**: medium
  - **Tags**: v0.1.1, stable, cli, data-loss

- [ ] 1k6Q **-** Keep project names with spaces in TODO.md
  - **Priority**: medium
  - **Tags**: v0.1.1, stable, format, data-loss

- [ ] 13FF **-** Refuse to load a file with duplicate task IDs
  - **Priority**: medium
  - **Tags**: v0.1.1, stable, format, data-loss

- [ ] atU7 **-** list -s in-progress (and its aliases) returns nothing
  - **Priority**: high
  - **Tags**: v0.1.1, stable, cli

- [ ] 6OvK **-** Read and write the same config file (.lida/config vs .todo/config)
  - **Priority**: medium
  - **Tags**: v0.1.1, stable, format

- [ ] kBY0 **-** init rejects conflicting format flags (--yaml --md)
  - **Priority**: low
  - **Tags**: v0.1.1, stable, cli

- [ ] PxoT **-** Accept in_progress consistently in status --set and list -s
  - **Priority**: low
  - **Tags**: v0.1.1, stable, cli

- [ ] foNb **-** Fix docs drift: --json claim, exit 2 on argument errors, help text, Rust floor
  - **Priority**: medium
  - **Tags**: v0.1.1, stable, docs

- [ ] ctfm **-** Set an honest rust-version and test it in CI
  - **Priority**: medium
  - **Tags**: v0.1.1, build, ci

- [ ] 9w3r **-** Run the audit job on pull requests
  - **Priority**: medium
  - **Tags**: v0.1.1, ci

- [ ] kx1F **-** Redact the version in snapshots by pattern, not substring
  - **Priority**: low
  - **Tags**: v0.1.1, test

- [ ] YWKq **-** Stamp created on add (todo-cli parity)
  - **Priority**: medium
  - **Tags**: v0.2, cli

- [ ] zmch **-** Accept -t/--tags and --tag on add, update and list
  - **Priority**: medium
  - **Tags**: v0.2, cli

- [ ] lTfk **-** update --due/--position with an empty value clears it; reject empty descriptions
  - **Priority**: low
  - **Tags**: v0.2, cli

- [ ] z5Js **-** Tags containing commas survive the Markdown round-trip
  - **Priority**: low
  - **Tags**: v0.2, format

- [ ] eYxQ **-** Keep YAML comments and formatting on round-trip
  - **Priority**: low
  - **Tags**: v0.2, format

- [ ] OJnq **-** tags counts a duplicate tag once; meta unset reports an absent key
  - **Priority**: low
  - **Tags**: v0.2, cli

# Actors

# Comments

