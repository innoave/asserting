# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Common Changelog](https://common-changelog.org/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.7.0 - 2025-06-02

_Assertions for `Error` types_

### Added

* Provide assertions for types that implement `std::error::Error`
  [(PR #38)](https://github.com/innoave/asserting/pull/38)
* Add examples to the rustdoc of every assertion method
  [(PR #39)](https://github.com/innoave/asserting/pull/39)

### Fixed

* Missing #[track_caller] annotations on some assertion methods
  [(PR #40)](https://github.com/innoave/asserting/pull/40)
* Delete the trait `RangeLike` as it is unused
  [(PR #41)](https://github.com/innoave/asserting/pull/41)

## 0.6.0 - 2025-05-10

_Assertions for map-like types (`HashMap`, `BTreeMap`) and assert items of collections/iterators_

### Added

* Provide assertions for map-like types
  [(PR #31)](https://github.com/innoave/asserting/pull/31)
* Assert each item in a collection or iterator
  [(PR #35)](https://github.com/innoave/asserting/pull/35)
* Support dynamically composed expression in `Spec`
  [(PR #33)](https://github.com/innoave/asserting/pull/33)
* Support dynamically composed description in `Spec`
  [(PR #34)](https://github.com/innoave/asserting/pull/34)

### Changed

* Compile time error when assertions for closures are chained
  [(PR #36)](https://github.com/innoave/asserting/pull/36)
* Remove implementation of `Default` for `DoesPanic`
  [(PR #37)](https://github.com/innoave/asserting/pull/37)

### Fixed

* `assert_that_code` does not highlight diffs in failure
  [(PR #30)](https://github.com/innoave/asserting/pull/30)

## 0.5.0 - 2025-04-26

_Soft assertions_

### Added

* Support for soft assertions
  [(PR #28)](https://github.com/innoave/asserting/pull/28)
* Respect `NO_COLOR` environment variable `colored` feature
  [(PR #23)](https://github.com/innoave/asserting/pull/23)
* Provide `matches` regex assertion for strings
  [(PR #24)](https://github.com/innoave/asserting/pull/24)
* Accept any type of range in range assertions
  [(PR #21)](https://github.com/innoave/asserting/pull/21)
* Doc: mark api elements gated behind a feature flag in rust doc
  [(PR #22)](https://github.com/innoave/asserting/pull/22)

### Changed

* **Breaking:** Rename crate feature `float_cmp` to `float-cmp`
  [(PR #26)](https://github.com/innoave/asserting/pull/26)<br/>
  The functionality of the crate feature remains the same.<br/>
  Replace any occurrence of the crate feature `float_cmp` with `float-cmp`.
* Read diff format configuration only once per test run
  [(PR #27)](https://github.com/innoave/asserting/pull/27)

### Fixed

* `assert_that_code` does not highlight diffs in failure messages
  [(PR #30)](https://github.com/innoave/asserting/pull/30)

## 0.4.0 - 2025-04-20

_More assertions provided_

### Added

* Provide assertions for the length of `CString` and `CStr`
  [(PR #13)](https://github.com/innoave/asserting/pull/13)
* Provide assertions for the number of characters in strings
  [(PR #14)](https://github.com/innoave/asserting/pull/14)
* Provide more assertions for the length of strings and collections
  [(PR #15)](https://github.com/innoave/asserting/pull/15)
* Provide additional assertions for the order of values
  [(PR #16)](https://github.com/innoave/asserting/pull/16)
* Provide additional assertions for integers and floats
  [(PR #17)](https://github.com/innoave/asserting/pull/17)

### Changed

* **Breaking:** Rename crate feature `float` to `float_cmp`
  [(PR #18)](https://github.com/innoave/asserting/pull/18)<br/>
  The functionality of the crate feature remains the same.<br/>
  Replace any occurrence of the crate feature `float` with `float_cmp`.

## 0.3.0 - 2025-04-14

_Color-highlighted diffs in assertion failures_

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
