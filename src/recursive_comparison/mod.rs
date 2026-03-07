//! Field-by-field recursive comparison of graphs of structs, enums, and tuples.
//!
//! Any type that implements [`serde::Serialize`] can be compared recursively.
//! This is useful for comparing graphs of structs, enums, and tuples.
//! The type to be compared does not need to implement `PartialEq` and `Debug`
//! or any other trait (besides [`serde::Serialize`]).
//!
//! There are several scenarios where recursive comparison is useful:
//!
//! * comparing types that have similar fields, like an entity and a DTO
//!   representation of the same type in the application's domain.
//! * comparing only fields that are relevant for a specific test case and
//!   ignoring others.
//! * comparing types field-by-field but ignoring fields, where the actual value
//!   may vary like for IDs or timestamps.
//! * comparing types that implement `Serialize` but not `PartialEq` or `Debug`
//!
//! Recursive comparison provides detailed failure messages in case of a failing
//! assertion. The failure details contain a list of fields, for which the
//! actual value is not equal to the expected one. This is another reason why
//! recursive comparison might be the preferred way, especially when comparing
//! structs that have many fields and/or contain sub-structs.
//!
//! Recursive comparison is started by calling the `using_recursive_comparison`
//! method.
//!
//! Recursive comparison is not symmetrical since it is limited to the fields
//! of the subject (actual value). It gathers the actual fields of the subject
//! and compares them to the corresponding fields haven the same name in the
//! expected value.
//!
//! Structs, enums, and tuples in the subject and expected value do not
//! have to be of the exact same type. They are compared field-by-field. As
//! long as the actual and expected fields have the same name and value, they
//! are considered equal. Though primitive types like char, integer, float, and
//! bool have to be of the same type. For example, an actual field of an `u8`
//! value is only equal to the expected field if the names and values are equal
//! and the expected value is of type `u8` too.
//!
//! The recursive comparison is limited down to a max depth of 128 levels,
//! which is the default max depth of [`serde::Serialize`].
//!
//! # Examples
//!
//! ## Comparing structs with several fields and containing other structs
//!
//! The following example shows how to compare two structs. The structs are
//! compared field-by-field recursively. The actual and the expected value can
//! be of the same struct type or different struct types. By default, the
//! expected struct must have at least all the fields of the actual struct but
//! can have more fields not present in the actual struct.
//!
//! ```
//! use asserting::prelude::*;
//! use serde::Serialize;
//!
//! #[derive(Serialize)]
//! struct Email {
//!     purpose: String,
//!     address: String,
//! }
//!
//! #[derive(Serialize)]
//! struct Person {
//!     name: String,
//!     email: Vec<Email>,
//!     age: u8,
//! }
//!
//! let person = Person {
//!     name: "Silvia".into(),
//!     email: vec![
//!         Email {
//!             purpose: "main".into(),
//!             address: "silvia@domain.com".into(),
//!         },
//!         Email {
//!             purpose: "private".into(),
//!             address: "silvia@mail.com".into(),
//!         },
//!     ],
//!     age: 25,
//! };
//!
//! assert_that!(person)
//!     .using_recursive_comparison()
//!     .is_equal_to(Person {
//!         name: "Silvia".into(),
//!         email: vec![
//!             Email {
//!                 purpose: "main".into(),
//!                 address: "silvia@domain.com".into(),
//!             },
//!             Email {
//!                 purpose: "private".into(),
//!                 address: "silvia@mail.com".into(),
//!             },
//!         ],
//!         age: 25,
//!     });
//! ```
//!
//! The field-by-field recursive comparison is started by calling the
//! `using_recursive_comparison` method.
//!
//! ## Ignoring some fields
//!
//! We can ignore some fields of the subject, which will be excluded from the
//! field-by-field recursive comparison. To do so, we add the names of the fields
//! that shall be ignored to the configuration of the recursive comparison using
//! either the `ignoring_field` method, which adds one field at a time, or the
//! `ignoring_fields` methods which adds multiple fields at once.
//!
//! ```
//! use asserting::prelude::*;
//! use serde::Serialize;
//!
//! #[derive(Serialize)]
//! struct Address {
//!     street: String,
//!     city: String,
//!     zip: u16,
//! }
//!
//! #[derive(Serialize)]
//! struct Person {
//!     name: String,
//!     age: u8,
//!     address: Address,
//! }
//!
//! let person = Person {
//!     name: "Silvia".into(),
//!     age: 25,
//!     address: Address {
//!         street: "Second Street".into(),
//!         city: "New York".into(),
//!         zip: 12345,
//!     }
//! };
//!
//! assert_that!(&person)
//!     .using_recursive_comparison()
//!     .ignoring_fields(["age", "address.street"])
//!     .is_equal_to(Person {
//!         name: "Silvia".into(),
//!         age: 27,
//!         address: Address {
//!             street: "Main Street".into(),
//!             city: "New York".into(),
//!             zip: 12345,
//!         }
//!     });
//!
//! assert_that!(person)
//!     .using_recursive_comparison()
//!     .ignoring_field("age")
//!     .ignoring_field("address.street")
//!     .is_equal_to(Person {
//!         name: "Silvia".into(),
//!         age: 27,
//!         address: Address {
//!             street: "Main Street".into(),
//!             city: "New York".into(),
//!             zip: 12345,
//!         }
//!     });
//! ```
//!
//! Once a field is ignored, its subfields are ignored as well. In the following
//! example the assertion succeeds because the `address` field is ignored and
//! therefore also the fields `street`, `city`, and `zip` are ignored.
//!
//! ```
//! use asserting::prelude::*;
//! use serde::Serialize;
//! #
//! # #[derive(Serialize)]
//! # struct Address {
//! #     street: String,
//! #     city: String,
//! #     zip: u16,
//! # }
//! #
//! # #[derive(Serialize)]
//! # struct Person {
//! #     name: String,
//! #     age: u8,
//! #     address: Address,
//! # }
//!
//! let person = Person {
//!     name: "Silvia".into(),
//!     age: 25,
//!     address: Address {
//!         street: "Second Street".into(),
//!         city: "Chicago".into(),
//!         zip: 33333,
//!     }
//! };
//!
//! assert_that!(person)
//!     .using_recursive_comparison()
//!     .ignoring_field("address")
//!     .is_equal_to(Person {
//!         name: "Silvia".into(),
//!         age: 25,
//!         address: Address {
//!             street: "Main Street".into(),
//!             city: "New York".into(),
//!             zip: 12345,
//!         }
//!     });
//! ```
//!
//! ## Ignoring not expected fields
//!
//! With field-by-field recursive comparison, it is possible to compare similar
//! structs that share most of their fields but not all, like a domain object,
//! an entity, and a DTO of the same thing.
//!
//! ```
//! use asserting::prelude::*;
//! use serde::Serialize;
//!
//! #[derive(Serialize)]
//! struct PersonEntity {
//!     id: u64,
//!     name: String,
//!     age: u8,
//! }
//!
//! #[derive(Serialize)]
//! struct PersonDto {
//!     name: String,
//!     age: u8,
//! }
//!
//! let person = PersonEntity {
//!     id: 123,
//!     name: "Silvia".into(),
//!     age: 25,
//! };
//!
//! assert_that!(person)
//!     .using_recursive_comparison()
//!     .ignoring_not_expected_fields()
//!     .is_equal_to(PersonDto {
//!         name: "Silvia".into(),
//!         age: 25,
//!     });
//! ```
//!
//! Here we are comparing the subject of type `PersonEntity` with a `PersonDto`.
//! In contrast to the `PersonEntity` the `PersonDto` does not have an `id`
//! field. So we ignore it by using the `ignoring_not_expected_fields`
//! option.
//!
//! [`serde::Serialize`]: Serialize

pub mod path;
pub mod serialize;
pub mod value;

use crate::assertions::{AssertEquality, AssertEquivalence};
use crate::recursive_comparison::path::Path;
use crate::recursive_comparison::serialize::to_recursive_value;
use crate::recursive_comparison::value::Value;
use crate::spec::{
    AssertFailure, CollectFailures, DoFail, FailingStrategy, GetFailures, SoftPanic, Spec,
};
use crate::std::fmt::{self, Display};
use serde_core::Serialize;

/// Data of an actual assertion in field-by-field recursive comparison mode.
///
/// It wraps a [`Spec`] and holds additional options for the field-by-field
/// recursive comparison, such as which fields to compare and which to ignore.
///
/// See the [module documentation](crate::recursive_comparison) for details
/// about field-by-field recursive comparison.
pub struct RecursiveComparison<'a, S, R> {
    spec: Spec<'a, S, R>,
    compared_fields: Vec<Path<'a>>,
    ignored_fields: Vec<Path<'a>>,
    ignore_not_expected_fields: bool,
}

impl<S, R> GetFailures for RecursiveComparison<'_, S, R> {
    fn has_failures(&self) -> bool {
        self.spec.has_failures()
    }

    fn failures(&self) -> Vec<AssertFailure> {
        self.spec.failures()
    }

    fn display_failures(&self) -> Vec<String> {
        self.spec.display_failures()
    }
}

impl<S, R> DoFail for RecursiveComparison<'_, S, R>
where
    R: FailingStrategy,
{
    fn do_fail_with(&mut self, failures: impl IntoIterator<Item = AssertFailure>) {
        self.spec.do_fail_with(failures);
    }

    fn do_fail_with_message(&mut self, message: impl Into<String>) {
        self.spec.do_fail_with_message(message);
    }
}

impl<S> SoftPanic for RecursiveComparison<'_, S, CollectFailures> {
    fn soft_panic(&self) {
        self.spec.soft_panic();
    }
}

impl<'a, S, R> RecursiveComparison<'a, S, R> {
    pub(crate) fn new(spec: Spec<'a, S, R>) -> Self {
        Self {
            spec,
            compared_fields: vec![],
            ignored_fields: vec![],
            ignore_not_expected_fields: false,
        }
    }

    /// Adds one field that shall be compared in a field-by-field recursive
    /// comparison.
    ///
    /// This method can be called multiple times to add several paths to the
    /// list of paths to be compared. Each call of this method adds the given
    /// field-path to the list of compared paths.
    ///
    /// Fields are addressed by their path. To learn how to specify a path and
    /// its syntax, see the documentation of the [`Path`] struct.
    ///
    /// If the same path is added to the list of ignored paths, this path is
    /// effectively ignored. Ignored paths take precedence over compared ones.
    #[must_use = "the returned `RecursiveComparison` does nothing unless an assertion method like `is_equal_to` is called"]
    pub fn comparing_only_field(mut self, field_path: impl Into<Path<'a>>) -> Self {
        self.compared_fields.push(field_path.into());
        self
    }

    /// Adds multiple fields to the list of fields to be compared in a
    /// field-by-field recursive comparison.
    ///
    /// This method can be called multiple times. Each call of this method
    /// extends the list of compared fields with the given paths.
    ///
    /// Fields are addressed by their path. To learn how to specify a path and
    /// its syntax, see the documentation of the [`Path`] struct.
    ///
    /// If the same path is added to the list of ignored paths, this path is
    /// effectively ignored. Ignored paths take precedence over compared ones.
    #[must_use = "the returned `RecursiveComparison` does nothing unless an assertion method like `is_equal_to` is called"]
    pub fn comparing_only_fields<P>(
        mut self,
        list_of_field_path: impl IntoIterator<Item = P>,
    ) -> Self
    where
        P: Into<Path<'a>>,
    {
        self.compared_fields
            .extend(list_of_field_path.into_iter().map(Into::into));
        self
    }

    /// Adds one field that shall be ignored in a field-by-field recursive
    /// comparison.
    ///
    /// This method can be called multiple times to add several paths to the
    /// list of paths to be ignored. Each call of this method adds the given
    /// field-path to the list of ignored paths.
    ///
    /// Fields are addressed by their path. To learn how to specify a path and
    /// its syntax, see the documentation of the [`Path`] struct.
    ///
    /// If the same path is added to the list of compared paths, this path is
    /// effectively ignored. Ignored paths take precedence over compared paths.
    #[must_use = "the returned `RecursiveComparison` does nothing unless an assertion method like `is_equal_to` is called"]
    pub fn ignoring_field(mut self, field_path: impl Into<Path<'a>>) -> Self {
        self.ignored_fields.push(field_path.into());
        self
    }

    /// Adds multiple fields to the list of fields to be ignored in a
    /// field-by-field recursive comparison.
    ///
    /// This method can be called multiple times. Each call of this method
    /// extends the list of ignored fields with the given paths.
    ///
    /// Fields are addressed by their path. To learn how to specify a path and
    /// its syntax, see the documentation of the [`Path`] struct.
    ///
    /// If the same path is added to the list of compared paths, this path is
    /// effectively ignored. Ignored paths take precedence over compared paths.
    #[must_use = "the returned `RecursiveComparison` does nothing unless an assertion method like `is_equal_to` is called"]
    pub fn ignoring_fields<P>(mut self, list_of_field_path: impl IntoIterator<Item = P>) -> Self
    where
        P: Into<Path<'a>>,
    {
        self.ignored_fields
            .extend(list_of_field_path.into_iter().map(Into::into));
        self
    }

    /// Specifies that the recursive comparison shall ignore fields that are
    /// not present in the expected value.
    ///
    /// By default, the recursive comparison tries to compare all fields of the
    /// actual value (subject). If a field of the actual value is not present in
    /// the expected value, the assertion fails.
    ///
    /// With this option, we can tell the recursive comparison to ignore fields
    /// that are not present in the expected value. This is useful when not all
    /// fields are relevant to be compared for a specific test case.
    #[must_use = "the returned `RecursiveComparison` does nothing unless an assertion method like `is_equal_to` is called"]
    pub fn ignoring_not_expected_fields(mut self) -> Self {
        self.ignore_not_expected_fields = true;
        self
    }

    fn compare<'b>(&self, actual: &'b Value, expected: &'b Value) -> ComparisonResult<'b> {
        let mut ignored = Vec::new();
        let mut not_expected = Vec::new();
        let mut non_equal = Vec::new();

        for (actual_path, actual_value) in actual.depth_first_iter() {
            if self
                .ignored_fields
                .iter()
                .any(|ignored| actual_path.starts_with(ignored))
                || (!self.compared_fields.is_empty()
                    && !self
                        .compared_fields
                        .iter()
                        .any(|compared| actual_path.starts_with(compared)))
            {
                ignored.push(actual_path);
                continue;
            }
            if let Some(expected_value) = expected.get_path(&actual_path) {
                if actual_value != expected_value {
                    non_equal.push(NonEqual {
                        path: actual_path,
                        actual_value,
                        expected_value,
                    });
                }
            } else if self.ignore_not_expected_fields {
                ignored.push(actual_path);
            } else {
                not_expected.push(NotExpected {
                    path: actual_path,
                    value: actual_value,
                });
            }
        }

        ComparisonResult {
            ignored,
            non_equal,
            not_expected,
        }
    }
}

struct NotExpected<'a> {
    path: Path<'a>,
    value: &'a Value,
}

impl Display for NotExpected<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let NotExpected { path, value } = self;
        write!(f, "{path}: {value:?}")
    }
}

struct NonEqual<'a> {
    path: Path<'a>,
    actual_value: &'a Value,
    expected_value: &'a Value,
}

impl Display for NonEqual<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let NonEqual {
            path,
            actual_value,
            expected_value,
        } = self;
        let debug_actual = format!("{actual_value:?}");
        let debug_expected = format!("{expected_value:?}");
        if debug_actual == debug_expected {
            let actual_type = actual_value.type_name();
            let expected_type = expected_value.type_name();
            write!(f, "{path}: value <{actual_value:?}> was equal, but type was <{actual_type}> and expected type is <{expected_type}>")
        } else {
            write!(
                f,
                "{path}: expected <{expected_value:?}> but was <{actual_value:?}>",
            )
        }
    }
}

struct ComparisonResult<'a> {
    ignored: Vec<Path<'a>>,
    non_equal: Vec<NonEqual<'a>>,
    not_expected: Vec<NotExpected<'a>>,
}

impl ComparisonResult<'_> {
    fn has_failure(&self) -> bool {
        !self.non_equal.is_empty() || !self.not_expected.is_empty()
    }
}

impl Display for ComparisonResult<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.non_equal.is_empty() {
            writeln!(f, "\n  non equal fields:")?;
            for a_non_equal in &self.non_equal {
                writeln!(f, "    {a_non_equal}")?;
            }
        }
        if !self.not_expected.is_empty() {
            writeln!(f, "\n  the following fields were not expected:")?;
            for a_not_expected in &self.not_expected {
                writeln!(f, "    {a_not_expected}")?;
            }
        }
        if !self.ignored.is_empty() {
            writeln!(f, "\n  the following fields were ignored:")?;
            for an_ignored in &self.ignored {
                writeln!(f, "    {an_ignored}")?;
            }
        }
        Ok(())
    }
}

impl<S, E, R> AssertEquality<E> for RecursiveComparison<'_, S, R>
where
    S: Serialize,
    E: Serialize,
    R: FailingStrategy,
{
    fn is_equal_to(mut self, expected: E) -> Self {
        let expression = self.spec.expression();
        let actual = to_recursive_value(self.spec.subject())
            .unwrap_or_else(|err| panic!("failed to serialize the subject, reason: {err}"));
        let expected = to_recursive_value(&expected)
            .unwrap_or_else(|err| panic!("failed to serialize the expected value, reason: {err}"));

        let compared = self.compare(&actual, &expected);

        if compared.has_failure() {
            self.do_fail_with_message(format!(
                r"expected {expression} to be equal to {expected:?} (using recursive comparison)
   but was: {actual:?}
  expected: {expected:?}
{compared}"
            ));
        }
        self
    }

    fn is_not_equal_to(self, expected: E) -> Self {
        todo!()
    }
}

impl<S, R> AssertEquivalence<Value> for RecursiveComparison<'_, S, R>
where
    S: Serialize,
    R: FailingStrategy,
{
    fn is_equivalent_to(mut self, expected: Value) -> Self {
        let expression = self.spec.expression();
        let actual = to_recursive_value(self.spec.subject())
            .unwrap_or_else(|err| panic!("failed to serialize the subject, reason: {err}"));

        let compared = self.compare(&actual, &expected);

        if compared.has_failure() {
            self.do_fail_with_message(format!(
                r"expected {expression} to be equivalent to {expected:?} (using recursive comparison)
   but was: {actual:?}
  expected: {expected:?}
{compared}"
            ));
        }
        self
    }

    fn is_not_equivalent_to(self, expected: Value) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests;
