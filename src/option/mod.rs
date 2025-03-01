use crate::expectations::{AssertHasValue, AssertOption};
use crate::spec::{Expectation, Expression, FailingStrategy, Spec, Unknown};
use crate::std::fmt::Debug;
#[cfg(not(any(feature = "std", test)))]
use alloc::{format, string::String};

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

struct IsSome;

impl<T> Expectation<Option<T>> for IsSome
where
    T: Debug,
{
    fn test(&self, subject: &Option<T>) -> bool {
        subject.is_some()
    }

    fn message(&self, expression: Expression<'_>, actual: &Option<T>) -> String {
        format!(
            "expected {expression} is {:?}\n   but was: {actual:?}\n  expected: {:?}",
            Some(Unknown),
            Some(Unknown)
        )
    }
}

struct IsNone;

impl<T> Expectation<Option<T>> for IsNone
where
    T: Debug,
{
    fn test(&self, subject: &Option<T>) -> bool {
        subject.is_none()
    }

    fn message(&self, expression: Expression<'_>, actual: &Option<T>) -> String {
        format!(
            "expected {expression} is {:?}\n   but was: {actual:?}\n  expected: {:?}",
            None::<Unknown>, None::<Unknown>
        )
    }
}

struct HasValue<E> {
    expected: E,
}

impl<T, E> Expectation<Option<T>> for HasValue<E>
where
    T: PartialEq<E> + Debug,
    E: Debug,
{
    fn test(&self, subject: &Option<T>) -> bool {
        subject
            .as_ref()
            .is_some_and(|value| value == &self.expected)
    }

    fn message(&self, expression: Expression<'_>, actual: &Option<T>) -> String {
        format!("expected {expression} is some containing {:?}\n   but was: {actual:?}\n  expected: {:?}",
            &self.expected,
            Some(&self.expected),
        )
    }
}

#[cfg(test)]
mod tests;
