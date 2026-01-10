# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Common Changelog](https://common-changelog.org/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.12.0 - 2026-01-10

_Asserting some elements in a collection or an iterator_

### Added

* verify that at least one element of an iterator or collection satisfies assertions
  [(PR #73)](https://github.com/innoave/asserting/pull/73)
* assert some elements of an iterator against a predicate
  [(PR #71)](https://github.com/innoave/asserting/pull/71)

### Changed

* **Breaking:** rename method `each_item()` to `each_element()`
  [(PR #72)](https://github.com/innoave/asserting/pull/72)

## 0.11.0 - 2025-12-27

_Pick elements of collections or iterators for assertion_

### Added

* add new filter assertions for collections and iterators to pick specific elements for assertion
  [(PR #70)](https://github.com/innoave/asserting/pull/70)

## 0.10.0 - 2025-11-09

### Added

* add a new assertion that assures that the type and value are as expected
  [(PR #66)](https://github.com/innoave/asserting/pull/66)

### Changed

* do not print "assertion failed" with every assertion failure
  [(PR #65)](https://github.com/innoave/asserting/pull/65)

### Documented

* add an example of a helper function for asserting fields of a custom struct reusing existing
  assertions
  [(PR #67)](https://github.com/innoave/asserting/pull/67)

## 0.9.0 - 2025-06-28

_"Does not contain" assertions for strings, collections and iterators and expectation combinators
(not, all, any)_

### Added

* assert types formatted for debug and display
  [(PR #56)](https://github.com/innoave/asserting/pull/56)
* provide "does not..." assertions for strings
  [(PR #53)](https://github.com/innoave/asserting/pull/53)
* `does_not_contain` assertion for collections and iterators
  [(PR #57)](https://github.com/innoave/asserting/pull/57)
* `does_not_contain_any_of` assertion for collection and iterators
  [(PR #58)](https://github.com/innoave/asserting/pull/58)
* **Breaking:** provide `Not` expectation combinator and revise failure messages
  [(PR #52)](https://github.com/innoave/asserting/pull/52)
* provide expectation combinators `All` and `Any`
  [(PR #54)](https://github.com/innoave/asserting/pull/54)
* **Breaking:** provide constructor functions for all expectations (including the combinators)
  [(PR #55)](https://github.com/innoave/asserting/pull/55)

## 0.8.0 - 2025-06-08

_Asserting big and accurate numbers_

### Added

* Number assertions for `num_bigint::BigInt` and `num_bigint::BigUint`
  [(PR #46)](https://github.com/innoave/asserting/pull/46)
* Number assertions for `bigdecimal::BigDecimal` and `bigdecimal::BigDecimalRef`
  [(PR #48)](https://github.com/innoave/asserting/pull/48)
* Number assertions for `rust_decimal::Decimal`
  [(PR #47)](https://github.com/innoave/asserting/pull/47)
* Decimal number specific assertions `has_scale_of`, `has_precision_of` and `is_integer`
  [(PR #49)](https://github.com/innoave/asserting/pull/49)
* Assertions for properties and classification of characters (`char`)
  [(PR #45)](https://github.com/innoave/asserting/pull/45)

### Fixed

* Assertions for `Result` are not available for borrowed `&Result`
  [(PR #43)](https://github.com/innoave/asserting/pull/43)
* Assertions for `Option` are not available for borrowed `&Option`
  [(PR #42)](https://github.com/innoave/asserting/pull/42)

### Development

* Use a fake environment for testing functionality that depends on environment variables
  [(PR #50)](https://github.com/innoave/asserting/pull/50)

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
