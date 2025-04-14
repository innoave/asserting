# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Common Changelog](https://common-changelog.org/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.3.0 - 2025-04-14

### Added

* Color highlighted differences between expected and actual
  values [(PR #10)](https://github.com/innoave/asserting/pull/10)

### Changed

* Move 'within margin of...' in failure message for is_close_to to separate
  line [(PR #8)](https://github.com/innoave/asserting/pull/8)
* Set must_use attributes only to methods returning
  Self [(PR #7)](https://github.com/innoave/asserting/pull/7)
* Remove (nearly) useless const for fn [(PR #6)](https://github.com/innoave/asserting/pull/6)

## 0.2.0 - 2025-03-23

### Added

* Assertion `has_error_message` asserting the string representation of an error type in a `Result`.

### Fixed

* Wrong assertions listed for the `Result` type in README
* Some broken links in the README

## 0.1.0 - 2025-03-22

_First release_
