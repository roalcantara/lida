# Changelog

Every release of lida, written by semantic-release from the commit messages (docs/adr/0001-releases.md).

## [0.1.0](https://github.com/roalcantara/lida/compare/v0.0.1...v0.1.0) (2026-11-24)

### ⚠ BREAKING CHANGES

* **release:** a minor version until 1.0. Merging this releases v0.1.0,
the tag the dots Brewfile pins.

Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>

### Features

* **release:** Start the v0.1 release line ([acf1a02](https://github.com/roalcantara/lida/commit/acf1a0229538005f8b5ddaf7c9d1a37bdd4866cc))

## [0.0.1](https://github.com/roalcantara/lida/compare/v0.0.0...v0.0.1) (2026-11-24)

### Features

* **cli:** Add completion subcommand with raw zsh template support ([fe60904](https://github.com/roalcantara/lida/commit/fe609041a6d87d6f93ea438d0805030a84df16ca))
* **cli:** Add update subcommand, task-id comment support, and status aliases ([271b3f7](https://github.com/roalcantara/lida/commit/271b3f7db6258a7098dfeeeeff5a0327e51fe4a6))
* **cli:** Command-line interface powered by usage-rs ([e775d25](https://github.com/roalcantara/lida/commit/e775d251ba2533d357887842abd19f584f792f1c))
* **core:** Domain models and 4-char collision-free ID generator ([387b278](https://github.com/roalcantara/lida/commit/387b2788aa187fc9938cd396fd9e11d846afc2ee))
* **core:** Harden dual-write synchronization and preserve YAML extra fields ([6effa8e](https://github.com/roalcantara/lida/commit/6effa8e0ad006f22a4e1eea7c82b9a4487764542))
* **core:** Markdown GFM parser and serializer ([e2b3337](https://github.com/roalcantara/lida/commit/e2b33373561d94c03312abdcccc989226e0323cb))
* **core:** Storage engine with atomic dual-write synchronization ([fb21758](https://github.com/roalcantara/lida/commit/fb21758a61b118345db73d9afaedf7f21f0e3713))
* **core:** YAML format deserializer and serializer ([baf2935](https://github.com/roalcantara/lida/commit/baf29358c23bf88f50b05bf8833f5fee08185efe))

### Bug Fixes

* **assets:** Fix typos and format design proposal palettes ([d86c6a3](https://github.com/roalcantara/lida/commit/d86c6a38426e3ddeca6a538efc9c16468f92c106))
* **ci:** Fetch the locked crates before the core-purity check ([4d86b86](https://github.com/roalcantara/lida/commit/4d86b86e37f76e62ebedee73e99637e71cc2e44c))
* **ci:** Give Dependabot a cooldown so the audit passes ([5604439](https://github.com/roalcantara/lida/commit/560443926265a3e4ad2fb15c88eda93aaaf4c756))
