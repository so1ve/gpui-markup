# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
