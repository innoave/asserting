//! Fluent assertions for tests in Rust that are convenient to write and easy
//! to extend.
//!
//! Fluent assertions have some significant advantages - in general and
//! particularly as provided by this crate:
//!
//! * express the intent of an assertion
//! * an assertion reads more like natural english
//! * concise and expressive assertions for more complex types like collections
//! * different and more informative error messages for specific assertions
//! * easy spotting the difference between the expected and the actual value
//! * chaining of multiple assertions on the same subject
//!
//! # Usage
//!
//! To write fluent assertions in tests import this crate's `prelude`
//! module in your test module, like so:
//!
//! ```
//! use asserting::prelude::*;
//! ```
//!
//! Importing the `prelude` module is the intended way to use this crate. The
//! prelude re-exports all types, traits and functions and macros that are
//! needed to write assertions in tests.
//!
//! Start writing assertion by applying the [`assert_that`] macro to the subject
//! to be asserted. Then call an assertion
//! function like `is_equal_to`, like so:
//!
//! ```
//! # use asserting::prelude::*;
//! let some_result = 7 * 6;
//! assert_that!(some_result).is_equal_to(42);
//! ```
//!
//! The subject can be any expression, e.g.:
//!
//! ```
//! # use asserting::prelude::*;
//! assert_that!(6 * 8 - 6).is_equal_to(42);
//! ```
//!
//! The variable or expression inside the call of the [`assert_that`] macro is
//! repeated in the error message when an assertion fails. For example, the
//! assertion:
//!
//! ```no_run
//! # use asserting::prelude::*;
//! assert_that!(6 * 8 - 5).is_equal_to(42);
//! ```
//!
//! will print the error message:
//!
//! ```console
//! assertion failed: expected 6 * 8 - 5 is equal to 42
//!    but was: 43
//!   expected: 42
//! ```
//!
//! # Examples
//!
//! ## Basic assertions
//!
//! ```
//! use asserting::prelude::*;
//!
//! assert_that!(3 + 5).is_equal_to(8);
//! assert_that!(69).is_not_equal_to(42);
//!
//! assert_that!(5).is_greater_than(3);
//! assert_that!(42).is_at_most(99);
//!
//! assert_that!(-0.57).is_in_range(-1.0..=1.0);
//! assert_that!('M').is_in_range('A'..='Z');
//! assert_that!('M').is_not_in_range('a'..='z');
//!
//! let subject = "anim proident eiusmod sint".to_string();
//! assert_that!(subject).contains("eiusmod");
//!
//! let subject = Some("consectetur veniam at nulla".to_string());
//! assert_that!(subject).has_value("consectetur veniam at nulla");
//!
//! let subject: Result<i8, String> = Ok(42);
//! assert_that!(subject).has_value(42);
//!
//! let subject: Option<f64> = None;
//! assert_that!(subject).is_none();
//!
//! let subject: Result<(), String> = Err("labore qui eu illum".to_string());
//! assert_that!(subject).has_error("labore qui eu illum");
//!
//! let subject = vec![1, 3, 5, 7, 9, 11];
//! assert_that!(subject).contains_exactly([1, 3, 5, 7, 9, 11]);
//! ```
//!
//! ## Chaining assertions on the same subject
//!
//! ```
//! use asserting::prelude::*;
//!
//! assert_that!("commodo nobis cum duis")
//!     .starts_with("commodo")
//!     .ends_with(" duis")
//!     .has_length(22);
//!
//! assert_that!(vec![1, 19, 1, 29, 5, 5, 7, 23, 17, 11, 3, 23, 13, 1])
//!     .contains_all_of([1, 11, 13, 17, 19])
//!     .contains_only([1, 3, 5, 7, 9, 11, 13, 17, 19, 23, 29, 31, 37, 43]);
//! ```
//!
//! # Writing assertions
//!
//! Assertions can be written in two ways. The standard way that panics when
//! an assertion fails or the alternative way that collects failures from
//! failed assertions which can be read later.
//!
//! To call assertion functions on a subject it is wrapped into the [`Spec`]
//! struct. This can be done by calling one of the functions:
//!
//! * [`assert_that`] - wraps the subject into a [`Spec`] that panics if an
//!   assertion fails
//! * [`verify_that`] - wraps the subject into a [`Spec`] that collects failures
//!   from assertions, which can be read later.
//! * [`assert_that_code`] - wraps a closure into a [`Spec`] for asserting
//!   whether the code in the closure panics or does not panic. It panics if an
//!   assertion fails.
//! * [`verify_that_code`] - wraps a closure into a [`Spec`] for asserting
//!   whether the code in the closure panics or does not panic. It collects
//!   failures from assertions, which can be read later.
//!
//! The [`Spec`] can hold additional information about the subject, such as the
//! expression we are asserting, the code location of the assert statement and
//! an optional description of what we are going to assert. These attributes are
//! all optional and must be set explicitly by the user.
//!
//! For convenience a set of macros with the same names as the functions above
//! is provided which set the expression and the code location for the user.
//!
//! * [`assert_that!`] - calls the [`assert_that`] function and sets the
//!   expression inside the macro call as the expression in the [`Spec`] as well
//!   as the location of the macro call as the code location.
//! * [`verify_that!`] - calls the [`verify_that`] function and sets the
//!   expression inside the macro call as the expression in the [`Spec`] as well
//!   as the location of the macro call as the code location.
//! * [`assert_that_code!`] - calls the [`assert_that_code`] function and sets
//!   the expression inside the macro call as the expression in the [`Spec`] as
//!   well as the location of the macro call as the code location.
//! * [`verify_that_code!`] - calls the [`verify_that_code`] function and sets
//!   the expression inside the macro call as the expression in the [`Spec`] as
//!   well as the location of the macro call as the code location.
//!
//! For example, calling the macro [`assert_that!`] like so:
//!
//! ```
//! # use asserting::prelude::*;
//! assert_that!(7 * 6).is_equal_to(42);
//! ```
//!
//! is equivalent to calling the function [`assert_that`] and then calling
//! the methods [`named()`] and [`located_at()`] on the returned [`Spec`],
//! like so:
//!
//! ```
//! # use asserting::prelude::*;
//! assert_that(7 * 6)
//!     .named("7 * 6")
//!     .located_at(
//!         Location {
//!             file: file!(),
//!             line: line!(),
//!             column: column!(),
//!         }
//!     ).is_equal_to(42);
//! ```
//!
//! When using the `verfiy_*` variants of the macros or functions for each
//! failing assertion a failure of type [`AssertFailure`] is added to the
//! [`Spec`]. We can read the failures collected by calling the [`failures()`]
//! method, like so:
//!
//! ```
//! # use asserting::prelude::*;
//! let failures = verify_that!(7 * 5).is_equal_to(42).failures();
//!
//! assert_that!(failures).has_length(1);
//! ```
//!
//! or to get a list of formatted failure messages, we can call the
//! [`display_failures`](spec::Spec::display_failures) method, like so:
//!
//! ```
//! # use asserting::prelude::*;
//!
//! let failures = verify_that!(7 * 5).is_equal_to(42).display_failures();
//!
//! assert_that!(failures).contains_exactly([
//!     r"assertion failed: expected 7 * 5 is equal to 42
//!    but was: 35
//!   expected: 42
//! "
//! ]);
//! ```
//!
//! [`AssertFailure`]: spec::AssertFailure
//! [`Spec`]: spec::Spec
//! [`assert_that`]: spec::assert_that
//! [`assert_that_code`]: spec::assert_that_code
//! [`verify_that`]: spec::verify_that
//! [`verify_that_code`]: spec::verify_that_code
//! [`failures()`]: spec::Spec::failures
//! [`named()`]: spec::Spec::named
//! [`located_at()`]: spec::Spec::located_at

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
mod std {
    pub use core::*;
}

#[cfg(feature = "std")]
mod std {
    pub use std::*;
}

pub mod assertions;
pub mod expectations;
pub mod prelude;
pub mod properties;
pub mod spec;

mod boolean;
mod collection;
mod equality;
mod integer;
mod iterator;
mod length;
mod option;
mod order;
mod predicate;
mod range;
mod result;
mod slice;
mod string;
mod vec;

#[cfg(feature = "float")]
mod float;

#[cfg(feature = "panic")]
mod panic;

// test code snippets in the README.md
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
#[allow(dead_code)]
type TestCodeSnippetsInReadme = ();
