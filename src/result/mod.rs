//! Implementation of assertions for `Result` values.

use crate::assertions::{
    AssertHasError, AssertHasErrorMessage, AssertHasValue, AssertResult, AssertResultValue,
};
use crate::colored::{mark_missing, mark_unexpected};
use crate::expectations::{HasError, HasValue, IsEqualTo, IsErr, IsOk};
use crate::spec::{DiffFormat, Expectation, Expression, FailingStrategy, Spec, Unknown};
use crate::std::fmt::{Debug, Display};
use crate::std::{
    format,
    string::{String, ToString},
};

impl<T, E, R> AssertResult for Spec<'_, Result<T, E>, R>
where
    T: Debug,
    E: Debug,
    R: FailingStrategy,
{
    fn is_ok(self) -> Self {
        self.expecting(IsOk)
    }

    fn is_err(self) -> Self {
        self.expecting(IsErr)
    }
}

impl<'a, T, E, R> AssertResultValue<'a, T, E, R> for Spec<'a, Result<T, E>, R>
where
    T: Debug,
    E: Debug,
{
    fn ok(self) -> Spec<'a, T, R> {
        self.mapping(|subject| match subject {
            Ok(value) => value,
            Err(error) => {
                panic!("assertion failed: expected the subject to be `Ok(_)`, but was `Err({error:?})`")
            },
        })
    }

    fn err(self) -> Spec<'a, E, R> {
        self.mapping(|subject| match subject {
            Ok(value) => {
                panic!("assertion failed: expected the subject to be `Err(_)`, but was `Ok({value:?})`")
            },
            Err(error) => error,
        })
    }
}

impl<T, E, X, R> AssertHasValue<X> for Spec<'_, Result<T, E>, R>
where
    T: PartialEq<X> + Debug,
    E: Debug,
    X: Debug,
    R: FailingStrategy,
{
    fn has_value(self, expected: X) -> Self {
        self.expecting(HasValue { expected })
    }
}

impl<T, E, X, R> AssertHasError<X> for Spec<'_, Result<T, E>, R>
where
    T: Debug,
    E: PartialEq<X> + Debug,
    X: Debug,
    R: FailingStrategy,
{
    fn has_error(self, expected: X) -> Self {
        self.expecting(HasError { expected })
    }
}

impl<'a, T, E, X, R> AssertHasErrorMessage<'a, X, R> for Spec<'a, Result<T, E>, R>
where
    T: Debug,
    E: Display,
    X: Debug,
    String: PartialEq<X>,
    R: FailingStrategy,
{
    fn has_error_message(self, expected: X) -> Spec<'a, String, R> {
        self.mapping(|result| match result {
            Ok(value) => panic!(
                r"assertion failed: expected the subject to be `Err(_)` with message {expected:?}, but was `Ok({value:?})`"
            ),
            Err(error) => {
                error.to_string()
            },
        }).expecting(IsEqualTo {expected})
    }
}

impl<T, E> Expectation<Result<T, E>> for IsOk
where
    T: Debug,
    E: Debug,
{
    fn test(&mut self, subject: &Result<T, E>) -> bool {
        subject.is_ok()
    }

    fn message(
        &self,
        expression: Expression<'_>,
        actual: &Result<T, E>,
        format: &DiffFormat,
    ) -> String {
        let expected = Ok::<_, Unknown>(Unknown);
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing(&expected, format);
        format!(
            "expected {expression} is {expected:?}\n   but was: {marked_actual}\n  expected: {marked_expected}"
        )
    }
}

impl<T, E> Expectation<Result<T, E>> for IsErr
where
    T: Debug,
    E: Debug,
{
    fn test(&mut self, subject: &Result<T, E>) -> bool {
        subject.is_err()
    }

    fn message(
        &self,
        expression: Expression<'_>,
        actual: &Result<T, E>,
        format: &DiffFormat,
    ) -> String {
        let expected = Err::<Unknown, Unknown>(Unknown);
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing(&expected, format);
        format!(
            "expected {expression} is {expected:?}\n   but was: {marked_actual}\n  expected: {marked_expected}"
        )
    }
}

impl<T, E, X> Expectation<Result<T, E>> for HasValue<X>
where
    T: PartialEq<X> + Debug,
    E: Debug,
    X: Debug,
{
    fn test(&mut self, subject: &Result<T, E>) -> bool {
        subject.as_ref().is_ok_and(|value| value == &self.expected)
    }

    fn message(
        &self,
        expression: Expression<'_>,
        actual: &Result<T, E>,
        format: &DiffFormat,
    ) -> String {
        let expected = &self.expected;
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing(&Ok::<_, E>(expected), format);
        format!(
            "expected {expression} is ok containing {expected:?}\n   but was: {marked_actual}\n  expected: {marked_expected}"
        )
    }
}

impl<T, E, X> Expectation<Result<T, E>> for HasError<X>
where
    T: Debug,
    E: PartialEq<X> + Debug,
    X: Debug,
{
    fn test(&mut self, subject: &Result<T, E>) -> bool {
        subject.as_ref().is_err_and(|err| err == &self.expected)
    }

    fn message(
        &self,
        expression: Expression<'_>,
        actual: &Result<T, E>,
        format: &DiffFormat,
    ) -> String {
        let expected = &self.expected;
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing(&Err::<T, _>(expected), format);
        format!(
            "expected {expression} is error containing {expected:?}\n   but was: {marked_actual}\n  expected: {marked_expected}"
        )
    }
}

#[cfg(test)]
mod tests;
