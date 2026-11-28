# Changelog

Every release of lida, written by semantic-release from the commit messages (docs/adr/0001-releases.md).

## [0.4.0](https://github.com/roalcantara/lida/compare/v0.3.0...v0.4.0) (2026-11-28)

### ⚠ BREAKING CHANGES

* **cli:** init --force no longer has the -f short form; -f now selects the TODO file.

### Features

* **cli:** Add stop to send a task back to todo ([6248a3b](https://github.com/roalcantara/lida/commit/6248a3b530e0593a0b2e970d13a0197eeb2eb020))
* **cli:** Annotate a task on add, start, stop and done ([96a282f](https://github.com/roalcantara/lida/commit/96a282ff85338217e471c913b5bb22d9d94b1d9a))
* **cli:** Complete IDs and tags from the --file target ([9aff65b](https://github.com/roalcantara/lida/commit/9aff65bb4da3581b3e94e944ae5cc32a202b6ac9))
* **cli:** Target another TODO file with -f/--file or LIDA_FILE ([6978517](https://github.com/roalcantara/lida/commit/69785172b61ec6c1c0101bb937ee9ba0989de938))

### Bug Fixes

* **cli:** Complete from an attached -f value ([60be51b](https://github.com/roalcantara/lida/commit/60be51b790401f42ecb529c702e3079db5013a0b))
* **cli:** Spare completion and usage from a stale LIDA_FILE ([4287e6f](https://github.com/roalcantara/lida/commit/4287e6fc727a8de28c33ca2524330fe915b84870))

## [0.3.0](https://github.com/roalcantara/lida/compare/v0.2.0...v0.3.0) (2026-11-27)

### Features

* **cli:** Draw next's priority as bold warm letters ([d72c4f9](https://github.com/roalcantara/lida/commit/d72c4f99129bd3dfff80f93b389b9f50bb411c97))
* **cli:** Filter next by due with +OVERDUE, +TODAY, +TOMORROW, +WEEK ([0315378](https://github.com/roalcantara/lida/commit/03153781f191e90c763fc8baeca54f9cefe26292))
* **cli:** List in-progress tasks first in next ([49b4a26](https://github.com/roalcantara/lida/commit/49b4a26690a4cd970031fe7cac1c42df5a44b725))

## [0.2.0](https://github.com/roalcantara/lida/compare/v0.1.1...v0.2.0) (2026-11-27)

### Features

* **cli:** Add lida next to list open tasks by urgency ([d9d6c31](https://github.com/roalcantara/lida/commit/d9d6c314ff2bef35a37a6dbd719eaf9152c3ab7a))
* **cli:** Add the next palette and span writer ([0bae57f](https://github.com/roalcantara/lida/commit/0bae57f6675c856c1595ca74941263c1efa180bd))
* **cli:** Add width-aware text fitting and sanitising ([afdbc3a](https://github.com/roalcantara/lida/commit/afdbc3a55516289d69bbac1d888660c2ca60d7c8))
* **cli:** Complete next filter words ([e75a20b](https://github.com/roalcantara/lida/commit/e75a20b290d745eced5b16cca71784502b7f0b35))
* **cli:** Lay out next columns for any window width ([1a18b68](https://github.com/roalcantara/lida/commit/1a18b681ed12b454d993558a9493e2f0a3904aee))
* **core:** Classify due dates against a given day ([2ec3917](https://github.com/roalcantara/lida/commit/2ec39178f28dbc87e1b59bc43d04e082d6ea1280))
* **core:** Filter, order and mark waiting tasks for next ([5eec7a3](https://github.com/roalcantara/lida/commit/5eec7a320e282b90cd8b399e966e8c27ae7f331f))

### Bug Fixes

* **cli:** Fit far due dates in the next DUE lane ([3f778ac](https://github.com/roalcantara/lida/commit/3f778ac4fee7937c4c9fbebf738c3b0919b65d28))
* **cli:** Keep the day on next's far due dates ([6a016c1](https://github.com/roalcantara/lida/commit/6a016c1da1b63578e7c4fffc944131af80693abc))

## [0.1.1](https://github.com/roalcantara/lida/compare/v0.1.0...v0.1.1) (2026-11-24)

### Bug Fixes

* **cli:** Colour only a terminal and align the list columns ([2f77281](https://github.com/roalcantara/lida/commit/2f77281ff9319eb55361946b301679c12a2bcc67))
* **cli:** Generate completion and the usage spec from the CLI ([f0c0c7d](https://github.com/roalcantara/lida/commit/f0c0c7de6d7b922bb1dd07651aad99edb13ffe79))
* **cli:** Keep meta keys with ':' readable and removable ([6c54686](https://github.com/roalcantara/lida/commit/6c54686f5646a413d214c79779c42153b13f3a92))
* **cli:** Parse statuses and priorities strictly and in one place ([a34c7bb](https://github.com/roalcantara/lida/commit/a34c7bb2c91cb33ec3154a60094ff499d9de4e80))
* **cli:** Reject input that would corrupt TODO.md ([4e75914](https://github.com/roalcantara/lida/commit/4e75914d829a1c8252bdd41a843d8779d6d21926))
* **cli:** Show every field a task stores ([21245b4](https://github.com/roalcantara/lida/commit/21245b4cfe7fd3becea03eeb26e930bbc5f1f9d6))
* **core:** Keep everything TODO.md holds, or refuse to save ([a3d9fd5](https://github.com/roalcantara/lida/commit/a3d9fd50da1a9561a8edaa4a40a812cfb390294d))
* **core:** Keep one source of truth when the format changes ([e760915](https://github.com/roalcantara/lida/commit/e76091535989ff1896bb75ab2069d699fec510d5))
* **core:** Leave empty keys out of TODO.yaml ([02bc2bf](https://github.com/roalcantara/lida/commit/02bc2bfdc1794d0d54267a5b2388bc5e6d07e5e4))
* **core:** Refuse only the format switches that would lose data ([8056016](https://github.com/roalcantara/lida/commit/8056016a86a2b2f79103ecb7e56f063eef3e8577))
* Start every error message lower-case ([b46a1d7](https://github.com/roalcantara/lida/commit/b46a1d727f0fe4d26b65b34d5bd13c8ebf25712d))

## [0.1.0](https://github.com/roalcantara/lida/compare/v0.0.1...v0.1.0) (2026-11-24)

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
