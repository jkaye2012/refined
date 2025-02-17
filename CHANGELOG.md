# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

- Adds the `Regex` predicate to `string`
## [0.0.3] - 2025-02-11

- Updates documentation and readme to improve discoverability of provided refinements
- Adds documentation of "smart equality" under implication

## [0.0.2] - 2025-02-02

There are no functional changes in this release. The version was bumped to update crates.io metadata
and documentation for a proper release.

## [0.0.1] - 2025-02-02

### Added

As the initial release, the "core" functionality of the library is all included:

- The `Predicate` trait
- The `Refinement` struct
- The `serde` feature
- The `implication` feature (and the associated `Implies` trait)
- An initial suite of shared refinements:
  - `Boundable` for both signed and unsigned types
  - `String`
  - `Char`
  - `Boolean`
