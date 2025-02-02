# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
