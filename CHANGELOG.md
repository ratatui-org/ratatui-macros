# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0](https://github.com/ratatui-org/ratatui-macros/compare/v0.2.4...v0.3.0) - 2024-05-09

### Added
- Use release-plz ([#38](https://github.com/ratatui-org/ratatui-macros/pull/38))
- Add text! macro ([#36](https://github.com/ratatui-org/ratatui-macros/pull/36))
- Add fill constraint ([#34](https://github.com/ratatui-org/ratatui-macros/pull/34))
- [**breaking**] Remove color `palette!` macro ([#32](https://github.com/ratatui-org/ratatui-macros/pull/32))
- Replace raw! and styled! with span! macro ([#30](https://github.com/ratatui-org/ratatui-macros/pull/30))
- Add `line!` attribute macro ([#29](https://github.com/ratatui-org/ratatui-macros/pull/29))
- *(text)* add raw! and styled! macros ([#4](https://github.com/ratatui-org/ratatui-macros/pull/4))

### Fixed
- Update repo url in Cargo.toml ([#39](https://github.com/ratatui-org/ratatui-macros/pull/39))

### Other
- Use `.areas(area)` instead of `.split(area).to_vec().try_into().unwrap()` ([#37](https://github.com/ratatui-org/ratatui-macros/pull/37))
- Update README.md with short description of span and line macros ([#33](https://github.com/ratatui-org/ratatui-macros/pull/33))
- format using cargo +nightly fmt ([#31](https://github.com/ratatui-org/ratatui-macros/pull/31))
- *(deps)* bump ratatui from 0.27.0-alpha.3 to 0.27.0-alpha.5 ([#27](https://github.com/ratatui-org/ratatui-macros/pull/27))
- *(deps)* bump trybuild from 1.0.91 to 1.0.93 ([#28](https://github.com/ratatui-org/ratatui-macros/pull/28))
- *(deps)* bump ratatui from 0.27.0-alpha.2 to 0.27.0-alpha.3 ([#24](https://github.com/ratatui-org/ratatui-macros/pull/24))
- *(deps)* bump trybuild from 1.0.90 to 1.0.91 ([#23](https://github.com/ratatui-org/ratatui-macros/pull/23))
- *(deps)* bump trybuild from 1.0.88 to 1.0.90 ([#20](https://github.com/ratatui-org/ratatui-macros/pull/20))
- *(deps)* bump ratatui from 0.27.0-alpha.0 to 0.27.0-alpha.2 ([#22](https://github.com/ratatui-org/ratatui-macros/pull/22))
- *(deps)* bump mio from 0.8.10 to 0.8.11 ([#18](https://github.com/ratatui-org/ratatui-macros/pull/18))
- *(deps)* bump ratatui from 0.26.0-alpha.1 to 0.27.0-alpha.0 ([#19](https://github.com/ratatui-org/ratatui-macros/pull/19))
- add cargo husky pre-commit hook ([#8](https://github.com/ratatui-org/ratatui-macros/pull/8))
- Create dependabot.yml ([#7](https://github.com/ratatui-org/ratatui-macros/pull/7))
- use rust cache to cache deps ([#6](https://github.com/ratatui-org/ratatui-macros/pull/6))
- readme tweaks ([#5](https://github.com/ratatui-org/ratatui-macros/pull/5))
- Update README.md
- Update README.md
- Update README.md
- Update README.md
- Add link to ratatui