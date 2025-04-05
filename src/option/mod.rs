//! Implementation of assertions for `Option` values.

use crate::assertions::{AssertHasValue, AssertOption, AssertOptionValue};
use crate::colored::{mark_missing, mark_unexpected};
use crate::expectations::{HasValue, IsNone, IsSome};
use crate::spec::{DiffFormat, Expectation, Expression, FailingStrategy, Spec, Unknown};
use crate::std::fmt::Debug;
use crate::std::{format, string::String};

impl<S, R> AssertOption for Spec<'_, Option<S>, R>
where
    S: Debug,
    R: FailingStrategy,
{
    fn is_some(self) -> Self {
        self.expecting(IsSome)
    }

    fn is_none(self) -> Self {
        self.expecting(IsNone)
    }
}

impl<'a, T, R> AssertOptionValue<'a, T, R> for Spec<'a, Option<T>, R>
where
    R: FailingStrategy,
{
    fn some(self) -> Spec<'a, T, R> {
        self.mapping(|subject| match subject {
            None => {
                panic!("assertion failed: expected the subject to be `Some(_)`, but was `None`")
            },
            Some(value) => value,
        })
    }
}

impl<S, E, R> AssertHasValue<E> for Spec<'_, Option<S>, R>
where
    S: PartialEq<E> + Debug,
    E: Debug,
    R: FailingStrategy,
{
    fn has_value(self, expected: E) -> Self {
        self.expecting(HasValue { expected })
    }
}

impl<T> Expectation<Option<T>> for IsSome
where
    T: Debug,
{
    fn test(&mut self, subject: &Option<T>) -> bool {
        subject.is_some()
    }

    fn message(
        &self,
        expression: Expression<'_>,
        actual: &Option<T>,
        format: &DiffFormat,
    ) -> String {
        let expected = Some(Unknown);
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing(&expected, format);
        format!(
            "expected {expression} is {expected:?}\n   but was: {marked_actual}\n  expected: {marked_expected}"
        )
    }
}

impl<T> Expectation<Option<T>> for IsNone
where
    T: Debug,
{
    fn test(&mut self, subject: &Option<T>) -> bool {
        subject.is_none()
    }

    fn message(
        &self,
        expression: Expression<'_>,
        actual: &Option<T>,
        format: &DiffFormat,
    ) -> String {
        let expected = None::<Unknown>;
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing(&expected, format);
        format!(
            "expected {expression} is {expected:?}\n   but was: {marked_actual}\n  expected: {marked_expected}"
        )
    }
}

impl<T, E> Expectation<Option<T>> for HasValue<E>
where
    T: PartialEq<E> + Debug,
    E: Debug,
{
    fn test(&mut self, subject: &Option<T>) -> bool {
        subject
            .as_ref()
            .is_some_and(|value| value == &self.expected)
    }

    fn message(
        &self,
        expression: Expression<'_>,
        actual: &Option<T>,
        format: &DiffFormat,
    ) -> String {
        let expected = &self.expected;
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing(&Some(expected), format);
        format!("expected {expression} is some containing {expected:?}\n   but was: {marked_actual}\n  expected: {marked_expected}")
    }
}

#[cfg(test)]
mod tests;
