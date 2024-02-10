# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate

## [0.2.0] - 2024-02-10

### Added

- add documentation
- add `Span` trait
- implement `Span` for `proc_macro2::Span`

### Changed

- `debug_span` function now accepts `Span` trait instead of `proc_macro2::Span`
- `proc-macro2` is now an optional dependency

## [1.0.0] - 2024-02-03

### Added

- `debug_span` function

<!-- next-url -->
[Unreleased]: https://github.com/smmoosavi/debug-span/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/smmoosavi/debug-span/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/smmoosavi/debug-span/tree/v0.1.0
