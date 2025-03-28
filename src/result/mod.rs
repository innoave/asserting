//! Implementation of assertions for `Result` values.

use crate::assertions::{AssertHasErrorMessage, AssertResult, AssertResultValue};
use crate::expectations::{HasError, HasValue, IsEqualTo, IsErr, IsOk};
use crate::prelude::{AssertHasError, AssertHasValue};
use crate::spec::{Expectation, Expression, FailingStrategy, Spec, Unknown};
use crate::std::fmt::{Debug, Display};
#[cfg(not(feature = "std"))]
use alloc::{
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

    fn message(&self, expression: Expression<'_>, actual: &Result<T, E>) -> String {
        format!(
            "expected {expression} is {:?}\n   but was: {actual:?}\n  expected: {:?}",
            Ok::<_, Unknown>(Unknown),
            Ok::<_, Unknown>(Unknown),
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

    fn message(&self, expression: Expression<'_>, actual: &Result<T, E>) -> String {
        format!(
            "expected {expression} is {:?}\n   but was: {actual:?}\n  expected: {:?}",
            Err::<Unknown, Unknown>(Unknown),
            Err::<Unknown, Unknown>(Unknown),
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

    fn message(&self, expression: Expression<'_>, actual: &Result<T, E>) -> String {
        format!(
            "expected {expression} is ok containing {:?}\n   but was: {actual:?}\n  expected: {:?}",
            &self.expected,
            Ok::<_, E>(&self.expected),
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

    fn message(&self, expression: Expression<'_>, actual: &Result<T, E>) -> String {
        format!(
            "expected {expression} is error containing {:?}\n   but was: {actual:?}\n  expected: {:?}",
            &self.expected,
            Err::<T, _>(&self.expected),
        )
    }
}

#[cfg(test)]
mod tests;
