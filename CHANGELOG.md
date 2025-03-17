# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-03-17

- Removes `Clone` constraints for all structs and traits; thanks to
  [yjhmelody](https://github.com/yjhmelody)
- Adds `no_std` support; thanks to [yjhmelody](https://github.com/yjhmelody)
- Adds an `optimize` function to the `Predicate` and `StatefulPredicate` traits to prevent possible
  downstream soundness problems from being unintentionally introduced; thanks to
  [Scott Taylor](https://github.com/scott2000) and [Nuutti Kotivuori](https://github.com/nakedible)
  for their input!

## [0.0.4] - 2025-03-02

- Adds the `Regex` predicate (and stateful implementation) to `string`
- Adds the `RefinementOps` trait, which is used internally to provide refinement generically
- Adds the `StatefulPredicate` and`StatefulRefinementOps` traits to allow for stateful refinement
- Adds the `arithmetic` feature, allowing for simple arithmetic operations on `Refinement`
- Refactors `NamedRefinement` into a generic `Named` wrapper
- Deprecates `Refinement::extract` in favor of `Refinement::take` (via `RefinementOps`)
- Adds the `optimized` feature to allow enabling unsafe optimizations
- Adds `CONTRIBUTING.md`

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
