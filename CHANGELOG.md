# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.3](https://github.com/so1ve/gpui-markup/compare/v0.5.2...v0.5.3) - 2026-08-24

### Other

- *(deps)* lock file maintenance ([#53](https://github.com/so1ve/gpui-markup/pull/53))
- *(deps)* lock file maintenance ([#52](https://github.com/so1ve/gpui-markup/pull/52))
- *(deps)* lock file maintenance ([#51](https://github.com/so1ve/gpui-markup/pull/51))
- *(deps)* update rust crate trybuild to v1.0.120 ([#50](https://github.com/so1ve/gpui-markup/pull/50))
- *(deps)* lock file maintenance ([#49](https://github.com/so1ve/gpui-markup/pull/49))
- *(deps)* update rust crate trybuild to v1.0.119 ([#48](https://github.com/so1ve/gpui-markup/pull/48))
- *(deps)* lock file maintenance ([#47](https://github.com/so1ve/gpui-markup/pull/47))
- *(deps)* update all non-major dependencies to v1.0.107 ([#45](https://github.com/so1ve/gpui-markup/pull/45))
- *(deps)* update rust crate syn to v2.0.119 ([#44](https://github.com/so1ve/gpui-markup/pull/44))
- *(deps)* lock file maintenance ([#43](https://github.com/so1ve/gpui-markup/pull/43))
- *(deps)* update rust crate trybuild to v1.0.118 ([#42](https://github.com/so1ve/gpui-markup/pull/42))
- *(deps)* lock file maintenance ([#41](https://github.com/so1ve/gpui-markup/pull/41))
- *(deps)* lock file maintenance ([#40](https://github.com/so1ve/gpui-markup/pull/40))
- *(deps)* update all non-major dependencies to v1.0.46 ([#39](https://github.com/so1ve/gpui-markup/pull/39))
- *(deps)* lock file maintenance ([#38](https://github.com/so1ve/gpui-markup/pull/38))
- *(deps)* update all non-major dependencies to v2.0.118 ([#37](https://github.com/so1ve/gpui-markup/pull/37))
- *(deps)* lock file maintenance ([#36](https://github.com/so1ve/gpui-markup/pull/36))
- *(deps)* update all non-major dependencies to v1.48.0 ([#35](https://github.com/so1ve/gpui-markup/pull/35))
- *(deps)* lock file maintenance ([#34](https://github.com/so1ve/gpui-markup/pull/34))
- *(deps)* lock file maintenance ([#33](https://github.com/so1ve/gpui-markup/pull/33))
- *(deps)* lock file maintenance ([#32](https://github.com/so1ve/gpui-markup/pull/32))
- *(deps)* lock file maintenance ([#30](https://github.com/so1ve/gpui-markup/pull/30))
- *(deps)* update rust crate insta to v1.47.1 ([#29](https://github.com/so1ve/gpui-markup/pull/29))
- *(deps)* update rust crate insta to v1.47.0 ([#28](https://github.com/so1ve/gpui-markup/pull/28))
- *(deps)* lock file maintenance ([#27](https://github.com/so1ve/gpui-markup/pull/27))
- *(deps)* lock file maintenance ([#26](https://github.com/so1ve/gpui-markup/pull/26))
- *(deps)* lock file maintenance ([#14](https://github.com/so1ve/gpui-markup/pull/14))
- *(deps)* update rust crate quote to v1.0.45 ([#25](https://github.com/so1ve/gpui-markup/pull/25))
- *(deps)* update rust crate syn to v2.0.117 ([#24](https://github.com/so1ve/gpui-markup/pull/24))
- *(deps)* update rust crate syn to v2.0.116 ([#23](https://github.com/so1ve/gpui-markup/pull/23))
- *(deps)* update all non-major dependencies ([#22](https://github.com/so1ve/gpui-markup/pull/22))
- *(deps)* update rust crate insta to v1.46.3 ([#20](https://github.com/so1ve/gpui-markup/pull/20))
- *(deps)* update autofix-ci/action digest to 7a166d7 ([#19](https://github.com/so1ve/gpui-markup/pull/19))
- *(deps)* update rust crate insta to v1.46.2 ([#18](https://github.com/so1ve/gpui-markup/pull/18))
- *(deps)* update rust crate trybuild to v1.0.115 ([#17](https://github.com/so1ve/gpui-markup/pull/17))
- *(deps)* update rust crate quote to v1.0.44 ([#16](https://github.com/so1ve/gpui-markup/pull/16))
- *(deps)* update rust crate proc-macro2 to v1.0.106 ([#15](https://github.com/so1ve/gpui-markup/pull/15))
- *(deps)* update rust crate insta to v1.46.1 ([#13](https://github.com/so1ve/gpui-markup/pull/13))
- *(deps)* lock file maintenance ([#12](https://github.com/so1ve/gpui-markup/pull/12))
- rewrite parser logic for better readability

## [0.5.2](https://github.com/so1ve/gpui-markup/compare/v0.5.1...v0.5.2) - 2026-01-11

### Fixed

- should treat Uppercase identifiers as components only

### Other

- update doc comments to match latest output

## [0.5.1](https://github.com/so1ve/gpui-markup/compare/v0.5.0...v0.5.1) - 2026-01-11

### Added

- show errors when `@` is not followed by `[]`

### Fixed

- use prototype methods from traits for more accurate error spans and runtime behavior

### Other

- simplify internal implementation
- fix example code
- revise installation instructions in README

## [0.5.0](https://github.com/so1ve/gpui-markup/compare/v0.4.0...v0.5.0) - 2026-01-11

### Added

- notice why `{}` is required at top level in error message
- [**breaking**] refined attributes syntax

### Other

- sync latest usage
- update comments

## [0.4.0](https://github.com/so1ve/gpui-markup/compare/v0.3.0...v0.4.0) - 2026-01-10

### Added

- support component expressions
- [**breaking**] move `[]` into `{}`
- [**breaking**] refined markup syntax

### Fixed

- remove duplicated braces in error message

### Other

- update snapshot
- use nightly rust
- update code formatting
- update README
- apply automatic fixes
- remove `__assert_parent_element` guard
- add nested `ui!` calls
- apply automatic fixes
- add comments for `parse_method_chain`
- extract `parse_element_body` and early returns
- bump version

## [0.3.0](https://github.com/so1ve/gpui-markup/compare/v0.2.0...v0.3.0) - 2026-01-10

### Added

- [**breaking**] disallow empty braces, use standard rust comments instead

## [0.2.0](https://github.com/so1ve/gpui-markup/compare/v0.1.2...v0.2.0) - 2026-01-10

### Added

- support method calls in children
- [**breaking**] use `.child` by default, add new syntax `{..children}` to express `Vec<Element>`

### Fixed

- should capture whole `TokenStream` for method call instead of implementing our own naive parser
- should not wrap braces in braces

### Other

- add more invalid syntax cases
- document nested macros usage

## [0.1.2](https://github.com/so1ve/gpui-markup/compare/v0.1.1...v0.1.2) - 2026-01-10

### Added

- comments

### Other

- add docs for `deferred`

## [0.1.1](https://github.com/so1ve/gpui-markup/compare/v0.1.0...v0.1.1) - 2026-01-10

### Added

- support `deferred`
- allow components to have children
- support more native elements
- init

### Fixed

- remove `canvas` and `img` from native element list
- don't use `generate_base_with_spans` in `deferred` codegen
- add `ParentElement` trait bound check
- should generate navigation to both start tag and end tag

### Other

- update snapshot
- apply automatic fixes
- use prettyplease to prettify snapshot
- add tests for components with children
- update snapshot
- add vscode config
- fix clippy
- use macos for clippy
- okay if pinning zbus does not work...
- pin zbus to fix build
- apply automatic fixes
- wtf tauri
- use nightly rust for linting jobs
- configure ci
- add repository link
- Merge pull request #1 from so1ve/renovate/configure
- add README and LICENSE
- remove `Attribute::KeyMultiValue`
