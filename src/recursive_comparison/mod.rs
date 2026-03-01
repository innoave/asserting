mod path;
pub mod serialize;
pub mod value;

use crate::assertions::{AssertEquality, AssertEquivalence};
use crate::recursive_comparison::path::Path;
use crate::recursive_comparison::serialize::to_recursive_values;
use crate::recursive_comparison::value::{struct_, Field, Value};
use crate::spec::{
    AssertFailure, CollectFailures, DoFail, FailingStrategy, GetFailures, SoftPanic, Spec,
};
use crate::std::fmt::{self, Display};
use serde_core::Serialize;

impl<'a, S, R> Spec<'a, S, R> {
    #[cfg_attr(docsrs, doc(cfg(feature = "recursive")))]
    #[must_use = "the returned `RecursiveComparison` does nothing unless an assertion method like `is_equal_to` is called"]
    pub fn using_recursive_comparison(self) -> RecursiveComparison<'a, S, R> {
        RecursiveComparison::new(self)
    }
}

pub fn struct_with_fields<T>(fields: impl IntoIterator<Item = T>) -> Value
where
    T: Into<Field>,
{
    struct_("", fields)
}

pub struct RecursiveComparison<'a, S, R> {
    spec: Spec<'a, S, R>,
    compared_fields: Vec<Path<'a>>,
    ignored_fields: Vec<Path<'a>>,
    ignore_missing_expected_fields: bool,
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
    fn new(spec: Spec<'a, S, R>) -> Self {
        Self {
            spec,
            compared_fields: vec![],
            ignored_fields: vec![],
            ignore_missing_expected_fields: false,
        }
    }

    #[must_use = "the returned `RecursiveComparison` does nothing unless an assertion method like `is_equal_to` is called"]
    pub fn comparing_only_field(mut self, field_path: impl Into<Path<'a>>) -> Self {
        self.compared_fields.push(field_path.into());
        self
    }

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

    #[must_use = "the returned `RecursiveComparison` does nothing unless an assertion method like `is_equal_to` is called"]
    pub fn ignoring_field(mut self, field_path: impl Into<Path<'a>>) -> Self {
        self.ignored_fields.push(field_path.into());
        self
    }

    #[must_use = "the returned `RecursiveComparison` does nothing unless an assertion method like `is_equal_to` is called"]
    pub fn ignoring_fields<P>(mut self, list_of_field_path: impl IntoIterator<Item = P>) -> Self
    where
        P: Into<Path<'a>>,
    {
        self.ignored_fields
            .extend(list_of_field_path.into_iter().map(Into::into));
        self
    }

    #[must_use = "the returned `RecursiveComparison` does nothing unless an assertion method like `is_equal_to` is called"]
    pub fn ignoring_missing_expected_fields(mut self) -> Self {
        self.ignore_missing_expected_fields = true;
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
            } else if self.ignore_missing_expected_fields {
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
        let actual = to_recursive_values(self.spec.subject())
            .unwrap_or_else(|err| panic!("failed to serialize the subject, reason: {err}"));
        let expected = to_recursive_values(&expected)
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
        let actual = to_recursive_values(self.spec.subject())
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
