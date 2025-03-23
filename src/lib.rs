//! Fluent assertions for tests in Rust that are convenient to write and easy
//! to extend.
//!
//! Fluent assertions have some significant advantages - in general and
//! particularly as provided by this crate:
//!
//! * express the intent of an assertion
//! * an assertion reads more like natural english
//! * concise and expressive assertions for more complex types like collections
//! * distinct and more informative error messages for specific assertions
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
//! ## Asserting custom types
//!
//! We can extract a property of a custom type and assert its value:
//!
//! ```
//! # use asserting::prelude::*;
//! struct MyStruct {
//!     important_property: String,
//!     other_property: f64,
//! }
//!
//! let some_thing = MyStruct {
//!     important_property: "imperdiet aliqua zzril eiusmod".into(),
//!     other_property: 99.9,
//! };
//!
//! assert_that!(some_thing).extracting(|s| s.important_property)
//!     .is_equal_to("imperdiet aliqua zzril eiusmod");
//!
//! ```
//!
//! Or we can map a custom type that does not implement a required trait to some
//! supported type, e.g. a tuple in this example:
//!
//! ```
//! # use asserting::prelude::*;
//! struct Point {
//!     x: i64,
//!     y: i64,
//! }
//!
//! let target = Point { x: 12, y: -64 };
//!
//! assert_that!(target).mapping(|s| (s.x, s.y)).is_equal_to((12, -64));
//! ```
//!
//! ## Predicate as custom assertion
//!
//! We can use any predicate function for a custom assertion:
//!
//! ```
//! # use asserting::prelude::*;
//! fn is_odd(value: &i32) -> bool {
//!     value & 1 == 1
//! }
//!
//! assert_that!(37).satisfies_with_message("expected my number to be odd", is_odd);
//! ```
//!
//! ## Assert that some code panics or does not panic
//!
//! ```
//! # #[cfg(not(feature = "std"))]
//! # fn main() {}
//! # #[cfg(feature = "std")]
//! # fn main() {
//! use asserting::prelude::*;
//!
//! fn divide(a: i32, b: i32) -> i32 {
//!     a / b
//! }
//!
//! assert_that_code!(|| { divide(7, 0); }).panics();
//!
//! assert_that_code!(|| { divide(7, 0); })
//!     .panics_with_message("attempt to divide by zero");
//!
//! assert_that_code!(|| { divide(7, 3); }).does_not_panic();
//! # }
//! ```
//!
//! # The `assert_that` and `verify_that` functions and macros
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
//! [`display_failures()`] method, like so:
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
//! # Custom assertions
//!
//! `asserting` provides 4 ways to do custom assertions:
//!
//! 1. Predicate functions as custom assertions used with the [`Spec::satisfies()`] method
//! 2. Property base assertions for any type that implements a property trait
//! 3. Custom expectations used with the [`Spec::expecting()`] method
//! 4. Custom assertions methods
//!
//! How to use predicate functions as custom assertions is described on the
//! [`Spec::satisfies()`] method and in the [Examples](#predicate-as-custom-assertion)
//! chapter above. The other 3 ways are described in the following subchapters.
//!
//! ## Property based assertions
//!
//! Some assertions provided by `asserting` are so-called property based
//! assertions. They are implemented for all types that implement a related
//! property trait.
//!
//! For example the `has_length()` assertion is implemented for all types that
//! implement the [`LengthProperty`].
//!
//! If we want to provide the `has_length()` assertion for a custom type we
//! simply need to implement the [`LengthProperty`] trait for this type.
//!
//! Let's assume we have a custom struct `PathWay` and we implement the
//! [`LengthProperty`] for `PathWay`:
//!
//! ```
//! use asserting::properties::LengthProperty;
//!
//! #[derive(Debug)]
//! struct PathWay {
//!     len: usize
//! }
//!
//! impl LengthProperty for PathWay {
//!     fn length_property(&self) -> usize {
//!         self.len
//!     }
//! }
//! ```
//!
//! Then we can assert the length of a `PathWay` using the `has_length()`
//! assertion:
//!
//! ```
//! # use asserting::properties::LengthProperty;
//! #
//! # #[derive(Debug)]
//! # struct PathWay {
//! #    len: usize
//! # }
//! #
//! # impl LengthProperty for PathWay {
//! #     fn length_property(&self) -> usize {
//! #         self.len
//! #     }
//! # }
//! use asserting::prelude::*;
//!
//! let some_path = PathWay { len: 27 };
//!
//! assert_that!(some_path).has_length(27);
//! ```
//!
//! Browse the [`properties`] module to see which property traits are available.
//!
//! ## Writing custom expectations
//!
//! A custom expectation is any type that implements the [`Expectation`] trait.
//! For example, lets assume we have a custom type `Either` and want to write
//! an expectation that verifies that a value of type `Either` is a left value.
//!
//! ```no_run
//! use asserting::spec::{Expectation, Expression, Unknown};
//! use std::fmt::Debug;
//!
//! #[derive(Debug)]
//! enum Either<L, R> {
//!     Left(L),
//!     Right(R),
//! }
//!
//! struct IsLeft;
//!
//! impl<L, R> Expectation<Either<L, R>> for IsLeft
//! where
//!     L: Debug,
//!     R: Debug,
//! {
//!     fn test(&mut self, subject: &Either<L, R>) -> bool {
//!         match subject {
//!             Either::Left(_) => true,
//!             _ => false,
//!         }
//!     }
//!
//!     fn message(&self, expression: Expression<'_>, actual: &Either<L, R>) -> String {
//!         format!(
//!             "expected {expression} is {:?}\n   but was: {actual:?}\n  expected: {:?}",
//!             Either::Left::<_, Unknown>(Unknown),
//!             Either::Left::<_, Unknown>(Unknown),
//!         )
//!      }
//! }
//! ```
//!
//! We can now use the expectation `IsLeft` with the [`Spec::expecting()`]
//! method:
//!
//! ```
//! # use asserting::spec::{Expectation, Expression, Unknown};
//! # use std::fmt::Debug;
//! #
//! # #[derive(Debug)]
//! # enum Either<L, R> {
//! #     Left(L),
//! #     Right(R),
//! # }
//! #
//! # struct IsLeft;
//! #
//! # impl<L, R> Expectation<Either<L, R>> for IsLeft
//! # where
//! #     L: Debug,
//! #     R: Debug,
//! # {
//! #     fn test(&mut self, subject: &Either<L, R>) -> bool {
//! #         match subject {
//! #             Either::Left(_) => true,
//! #             _ => false,
//! #         }
//! #     }
//! #
//! #     fn message(&self, expression: Expression<'_>, actual: &Either<L, R>) -> String {
//! #         format!(
//! #             "expected {expression} is {:?}\n   but was: {actual:?}\n  expected: {:?}",
//! #             Either::Left::<_, Unknown>(Unknown),
//! #             Either::Left::<_, Unknown>(Unknown),
//! #         )
//! #      }
//! # }
//! use asserting::prelude::*;
//!
//! let subject: Either<String, i64> = Either::Left("left value".to_string());
//!
//! assert_that!(subject).expecting(IsLeft);
//! ```
//!
//! ## Providing a custom assertion method
//!
//! In the previous chapter we implement a custom expectation which can be used
//! with the [`Spec::expecting()`] method. But this way is not very expressive.
//!
//! Additionally, we can implement a custom assertion method via an extension
//! trait.
//!
//! ```
//! # use asserting::spec::{Expectation, Expression, Unknown};
//! #
//! # #[derive(Debug)]
//! # enum Either<L, R> {
//! #     Left(L),
//! #     Right(R),
//! # }
//! #
//! # struct IsLeft;
//! #
//! # impl<L, R> Expectation<Either<L, R>> for IsLeft
//! # where
//! #     L: Debug,
//! #     R: Debug,
//! # {
//! #     fn test(&mut self, subject: &Either<L, R>) -> bool {
//! #         match subject {
//! #             Either::Left(_) => true,
//! #             _ => false,
//! #         }
//! #     }
//! #
//! #     fn message(&self, expression: Expression<'_>, actual: &Either<L, R>) -> String {
//! #         format!(
//! #             "expected {expression} is {:?}\n   but was: {actual:?}\n  expected: {:?}",
//! #             Either::Left::<_, Unknown>(Unknown),
//! #             Either::Left::<_, Unknown>(Unknown),
//! #         )
//! #      }
//! # }
//! use asserting::spec::{FailingStrategy, Spec};
//! use std::fmt::Debug;
//!
//! pub trait AssertEither {
//!     fn is_left(self) -> Self;
//! }
//!
//! impl<L, R, Q> AssertEither for Spec<'_, Either<L, R>, Q>
//! where
//!     L: Debug,
//!     R: Debug,
//!     Q: FailingStrategy,
//! {
//!     fn is_left(self) -> Self {
//!         self.expecting(IsLeft)
//!     }
//! }
//! ```
//!
//! Now we can use the assertion method `is_left()` for asserting whether a
//! subject of type `Either` is a left value.
//!
//! ```
//! # use asserting::spec::{Expectation, Expression, Unknown};
//! # use std::fmt::Debug;
//! #
//! # #[derive(Debug)]
//! # enum Either<L, R> {
//! #     Left(L),
//! #     Right(R),
//! # }
//! #
//! # struct IsLeft;
//! #
//! # impl<L, R> Expectation<Either<L, R>> for IsLeft
//! # where
//! #     L: Debug,
//! #     R: Debug,
//! # {
//! #     fn test(&mut self, subject: &Either<L, R>) -> bool {
//! #         match subject {
//! #             Either::Left(_) => true,
//! #             _ => false,
//! #         }
//! #     }
//! #
//! #     fn message(&self, expression: Expression<'_>, actual: &Either<L, R>) -> String {
//! #         format!(
//! #             "expected {expression} is {:?}\n   but was: {actual:?}\n  expected: {:?}",
//! #             Either::Left::<_, Unknown>(Unknown),
//! #             Either::Left::<_, Unknown>(Unknown),
//! #         )
//! #      }
//! # }
//! # use asserting::spec::{FailingStrategy, Spec};
//! #
//! # pub trait AssertEither {
//! #     fn is_left(self) -> Self;
//! # }
//! #
//! # impl<L, R, Q> AssertEither for Spec<'_, Either<L, R>, Q>
//! # where
//! #     L: Debug,
//! #     R: Debug,
//! #     Q: FailingStrategy,
//! # {
//! #     fn is_left(self) -> Self {
//! #         self.expecting(IsLeft)
//! #     }
//! # }
//! use asserting::prelude::*;
//!
//! let subject: Either<String, i64> = Either::Left("left value".to_string());
//!
//! assert_that!(subject).is_left();
//! ```
//!
//! [`AssertFailure`]: spec::AssertFailure
//! [`Expectation`]: spec::Expectation
//! [`LengthProperty`]: properties::LengthProperty
//! [`Spec`]: spec::Spec
//! [`Spec::expecting()`]: spec::Spec::expecting
//! [`Spec::satisfies()`]: spec::Spec::satisfies
//! [`assert_that`]: spec::assert_that
//! [`assert_that_code`]: spec::assert_that_code
//! [`verify_that`]: spec::verify_that
//! [`verify_that_code`]: spec::verify_that_code
//! [`display_failures()`]: spec::Spec::display_failures
//! [`failures()`]: spec::Spec::failures
//! [`named()`]: spec::Spec::named
//! [`located_at()`]: spec::Spec::located_at

#![doc(html_root_url = "https://docs.rs/asserting/0.2.0")]
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

// workaround for false positive 'unused extern crate' warnings until
// Rust issue [#95513](https://github.com/rust-lang/rust/issues/95513) is fixed
#[cfg(test)]
mod dummy_extern_uses {
    use version_sync as _;
}
