//! Fluent assertions for tests in Rust that are convenient to write and easy
//! to extend.
//!
//! Fluent assertions have some significant advantages - in general and
//! particularly as provided by this crate:
//!
//! * express the intent of an assertion
//! * an assertion reads more like natural english
//! * concise and expressive assertions for more complex types like collections
//! * distinct and more helpful error messages for specific assertions
//! * easy spotting the difference between the expected and the actual value
//! * chaining of multiple assertions on the same subject
//! * soft assertions
//!
//! An additional benefit of `asserting` is that it highlights differences
//! between the expected value and the actual value for failed assertions.
//! See the documentation of the [`colored`] module for more information on
//! "colored diffs".
//!
//! # Usage
//!
//! To write fluent assertions in tests, import this crate's `prelude`
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
//! Start writing assertion by applying the [`assert_that!`] macro to the subject
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
//! The variable or expression inside the call of the [`assert_that!`] macro is
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
//! expected 6 * 8 - 5 to be equal to 42
//!    but was: 43
//!   expected: 42
//! ```
//!
//! By default, the differences between the expected value and the actual value
//! are highlighted using colors. See the [`colored`] module for more
//! information on "colored diffs".
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
//! let subject = "at invidunt quis placerat".to_string();
//! assert_that!(subject).is_equal_to("at invidunt quis placerat");
//!
//! let subject = "justo clita in stet".to_string();
//! assert_that!(subject).is_same_as("justo clita in stet".to_string());
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
//! ## Asserting some elements of a collection or an iterator
//!
//! Asserting some elements of a collection or an iterator:
//!
//! ```
//! use asserting::prelude::*;
//!
//! let numbers = [2, 4, 6, 8, 10];
//!
//! assert_that!(numbers).each_element(|e|
//!     e.is_greater_than(1)
//!         .is_at_most(10)
//! );
//!
//! assert_that!(numbers).any_element(|e|
//!     e.is_equal_to(4)
//! );
//! ```
//!
//! See [`Spec::each_element()`] for more details.
//!
//! Assert some elements of a collection or an iterator to satisfy a predicate:
//!
//! ```
//! use asserting::prelude::*;
//!
//! let subject = [1, 41, 43, 42, 5];
//! assert_that!(subject).any_satisfies(|e| *e == 42);
//!
//! let subject = [43, 44, 45, 46, 47];
//! assert_that!(subject).all_satisfy(|e| *e > 42);
//!
//! let subject = [42, 43, 44, 45, 46];
//! assert_that!(subject).none_satisfies(|e| *e < 42);
//! ```
//!
//! See [`AssertElements`] for more details.
//!
//! ## Asserting specific elements of a collection or an iterator
//!
//! Filter assertions are handy to assert specific elements of a collection or
//! an iterator.
//!
//! Assert the only element of a collection or an iterator:
//!
//! ```
//! use asserting::prelude::*;
//!
//! let subject = ["single"];
//!
//! assert_that!(subject).single_element().is_equal_to("single");
//! ```
//!
//! Assert the first, the last, or the nth element of a collection or an iterator:
//!
//! ```
//! use asserting::prelude::*;
//!
//! let numbers = [1, 2, 3, 4, 5];
//!
//! assert_that!(numbers).first_element().is_equal_to(1);
//! assert_that!(numbers).last_element().is_equal_to(5);
//! assert_that!(numbers).nth_element(3).is_equal_to(4);
//! ```
//!
//! Filter the elements to be asserted on a condition:
//!
//! ```
//! use asserting::prelude::*;
//!
//! let subject = [1, 2, 3, 4, 5];
//!
//! assert_that!(subject)
//!     .filtered_on(|e| e & 1 == 0)
//!     .contains_exactly_in_any_order([2, 4]);
//!
//! let subject = ["one", "two", "three", "four"];
//!
//! assert_that!(subject)
//!     .filtered_on(|e| e.len() == 5)
//!     .single_element()
//!     .is_equal_to("three");
//! ```
//!
//! Pick the elements of a collection or an iterator at given positions:
//!
//! ```
//! use asserting::prelude::*;
//!
//! let subject = ["one", "two", "three", "four", "five"];
//!
//! assert_that!(subject)
//!     .elements_at([0, 2, 4])
//!     .contains_exactly(["one", "three", "five"]);
//! ```
//!
//! ## Soft assertions
//!
//! ```should_panic
//! use asserting::prelude::*;
//!
//! verify_that!("the answer to all important questions is 42")
//!     .contains("unimportant")
//!     .has_at_most_length(41)
//!     .soft_panic();
//! ```
//!
//! executes both assertions and prints the messages of both failing
//! assertions in the panic message:
//!
//! ```console
//! expected subject to contain "unimportant"
//!    but was: "the answer to all important questions is 42"
//!   expected: "unimportant"
//!
//! expected subject to have at most a length of 41
//!    but was: 43
//!   expected: <= 41
//! ```
//!
//! For more details see [`Spec::soft_panic()`].
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
//! supported type, e.g., a tuple in this example:
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
//! Requires crate feature `panic`.
//!
//! ```
//! # #[cfg(not(feature = "panic"))]
//! # fn main() {}
//! # #[cfg(feature = "panic")]
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
//! To call assertion functions on a subject, it is wrapped into the [`Spec`]
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
//! For convenience, a set of macros with the same names as the functions above
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
//! failing assertion, a failure of type [`AssertFailure`] is added to the
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
//!     r"expected 7 * 5 to be equal to 42
//!    but was: 35
//!   expected: 42
//! "
//! ]);
//! ```
//!
//! # Custom assertions
//!
//! `asserting` provides 5 ways to do custom assertions:
//!
//! 1. Predicate functions as custom assertions used with the [`Spec::satisfies()`] method
//! 2. Property base assertions for any type that implements a property trait
//! 3. Custom expectations used with the [`Spec::expecting()`] method
//! 4. Custom assertions methods
//! 5. Custom assertions without writing an expectation
//!
//! > &#x1F4A1;
//! > Often the easiest way to assert a custom type is to write a helper
//! > function that asserts parts (e.g., fields of a struct) using existing
//! > assertions. See the example [`assertion_function.rs`](examples/assertion_function.rs)
//! > which demonstrates how to use a helper function for asserting a custom
//! > struct.
//!
//! How to use predicate functions as custom assertions is described on the
//! [`Spec::satisfies()`] method and in the [Examples](#predicate-as-custom-assertion)
//! chapter above. The other 4 ways are described in the following subchapters.
//!
//! [`Expectation`]s enable us to write specialized assertions by combining
//! several basic expectations. In case a custom assertion cannot be composed
//! out of the provided expectations but writing a custom [`Expectation`] is too
//! cumbersome, we can write a custom assertion method directly without any
//! custom [`Expectation`]. See the
//! [Writing custom assertions without writing an expectation](#writing-custom-assertions-without-writing-an-expectation)
//! chapter below for an example.
//!
//! ## Property-based assertions
//!
//! Some assertions provided by `asserting` are so-called property-based
//! assertions. They are implemented for all types that implement a related
//! property trait.
//!
//! For example, the `has_length()` assertion is implemented for all types that
//! implement the [`LengthProperty`].
//!
//! If we want to provide the `has_length()` assertion for a custom type, we
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
//! For example, let's assume we have a custom type `Either` and want to write
//! an expectation that verifies that a value of type `Either` is a left value.
//!
//! ```no_run
//! use asserting::spec::{DiffFormat, Expectation, Expression, Unknown};
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
//!     fn message(&self, expression: &Expression<'_>, actual: &Either<L, R>, _inverted: bool, _format: &DiffFormat) -> String {
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
//! # use asserting::spec::{DiffFormat, Expectation, Expression, Unknown};
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
//! #     fn message(&self, expression: &Expression<'_>, actual: &Either<L, R>, _inverted: bool, _format: &DiffFormat) -> String {
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
//! In the previous chapter, we implemented a custom expectation which can be
//! used with the [`Spec::expecting()`] method. But this way is not very
//! expressive.
//!
//! Additionally, we can implement a custom assertion method via an extension
//! trait.
//!
//! ```
//! # use asserting::spec::{DiffFormat, Expectation, Expression, Unknown};
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
//! #     fn message(&self, expression: &Expression<'_>, actual: &Either<L, R>, _inverted: bool, _format: &DiffFormat) -> String {
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
//! # use asserting::spec::{DiffFormat, Expectation, Expression, Unknown};
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
//! #     fn message(&self, expression: &Expression<'_>, actual: &Either<L, R>, _inverted: bool, _format: &DiffFormat) -> String {
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
//! ## Writing custom assertions without writing an expectation
//!
//! In real world projects custom assertions are often very specific, and custom
//! expectations will not be reusable anyway. Writing a custom assertion without
//! having to provide a custom [`Expectation`] is most likely the preferred way.
//!
//! Here is a simple assertion that checks whether a person is over 18 years
//! old:
//!
//! ```
//! use asserting::prelude::*;
//! use asserting::spec::{FailingStrategy, Spec};
//!
//! struct Person {
//!     name: String,
//!     age: u8,
//! }
//!
//! trait AssertOver18 {
//!     fn is_over_18(self) -> Self;
//! }
//!
//! impl<'a, R> AssertOver18 for Spec<'a, Person, R>
//! where
//!     R: FailingStrategy,
//! {
//!     fn is_over_18(mut self) -> Self {
//!         let actual = self.subject().age;
//!         if actual < 18 {
//!             let expression = self.expression();
//!             self.do_fail_with_message(
//!                 "expected {expression} to be over 18\n   but was: {actual}\n  expected: >= 18",
//!             );
//!         }
//!         self
//!     }
//! }
//!
//! let person = Person { name: "Silvia".to_string(), age: 18 };
//!
//! assert_that!(person).is_over_18();
//! ```
//!
//! [`AssertElements`]: assertions::AssertElements
//! [`AssertFailure`]: spec::AssertFailure
//! [`Expectation`]: spec::Expectation
//! [`LengthProperty`]: properties::LengthProperty
//! [`Spec`]: spec::Spec
//! [`Spec::each_element()`]: spec::Spec::each_element
//! [`Spec::expecting()`]: spec::Spec::expecting
//! [`Spec::satisfies()`]: spec::Spec::satisfies
//! [`Spec::soft_panic()`]: spec::Spec::soft_panic
//! [`assert_that`]: spec::assert_that
//! [`assert_that_code`]: spec::assert_that_code
//! [`verify_that`]: spec::verify_that
//! [`verify_that_code`]: spec::verify_that_code
//! [`display_failures()`]: spec::Spec::display_failures
//! [`failures()`]: spec::Spec::failures
//! [`named()`]: spec::Spec::named
//! [`located_at()`]: spec::Spec::located_at

#![doc(html_root_url = "https://docs.rs/asserting/0.13.0")]
#![cfg_attr(not(feature = "std"), no_std)]
// Render feature requirements in docs.rs
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
mod std {
    extern crate alloc;
    pub use alloc::*;
    pub use core::*;

    pub mod borrow {
        extern crate alloc;
        pub use alloc::borrow::*;
        pub use core::borrow::*;
    }

    pub mod fmt {
        extern crate alloc;
        pub use alloc::fmt::*;
        pub use core::fmt::*;
    }

    pub mod slice {
        extern crate alloc;
        pub use alloc::slice::*;
        pub use core::slice::*;
    }

    pub mod str {
        extern crate alloc;
        pub use alloc::str::*;
        pub use core::str::*;
    }

    pub mod sync {
        extern crate alloc;
        pub use alloc::sync::*;
        pub use core::sync::*;
    }

    pub mod ffi {
        extern crate alloc;
        pub use alloc::ffi::*;
        pub use core::ffi::*;
    }
}

#[cfg(feature = "std")]
mod std {
    pub use std::*;
}

pub mod assertions;
pub mod colored;
pub mod expectations;
pub mod prelude;
pub mod properties;
pub mod spec;

#[cfg(feature = "bigdecimal")]
mod bigdecimal;
mod boolean;
mod c_string;
mod char;
mod char_count;
mod collection;
#[cfg(feature = "std")]
mod env;
mod equality;
mod error;
mod expectation_combinators;
mod float;
mod integer;
mod iterator;
mod length;
mod map;
#[cfg(feature = "num-bigint")]
mod num_bigint;
mod number;
mod option;
mod order;
#[cfg(feature = "std")]
mod os_sting;
#[cfg(feature = "panic")]
mod panic;
mod predicate;
mod range;
mod result;
#[cfg(feature = "rust-decimal")]
mod rust_decimal;
mod slice;
mod string;
mod vec;

// test code snippets in the README.md
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
#[allow(dead_code)]
type TestCodeSnippetsInReadme = ();

// workaround for false positive 'unused extern crate' warnings until
// Rust issue [#95513](https://github.com/rust-lang/rust/issues/95513) is fixed
#[cfg(test)]
mod dummy_extern_uses {
    use fakeenv as _;
    use proptest as _;
    use time as _;
    use version_sync as _;
}
