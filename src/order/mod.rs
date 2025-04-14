//! Implementation of order assertions.

use crate::assertions::AssertOrder;
use crate::colored::{mark_missing, mark_unexpected};
use crate::expectations::{IsAtLeast, IsAtMost, IsGreaterThan, IsLessThan};
use crate::spec::{DiffFormat, Expectation, Expression, FailingStrategy, Spec};
use crate::std::fmt::Debug;
use crate::std::{format, string::String};

impl<S, E, R> AssertOrder<E> for Spec<'_, S, R>
where
    S: PartialOrd<E> + Debug,
    E: Debug,
    R: FailingStrategy,
{
    fn is_less_than(self, expected: E) -> Self {
        self.expecting(IsLessThan { expected })
    }

    fn is_greater_than(self, expected: E) -> Self {
        self.expecting(IsGreaterThan { expected })
    }

    fn is_at_most(self, expected: E) -> Self {
        self.expecting(IsAtMost { expected })
    }

    fn is_at_least(self, expected: E) -> Self {
        self.expecting(IsAtLeast { expected })
    }
}

impl<S, E> Expectation<S> for IsLessThan<E>
where
    S: PartialOrd<E> + Debug,
    E: Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject < &self.expected
    }

    fn message(&self, expression: Expression<'_>, actual: &S, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing(&self.expected, format);
        format!(
            "expected {expression} is less than {:?}\n   but was: {marked_actual}\n  expected: < {marked_expected}",
            self.expected,
        )
    }
}

impl<S, E> Expectation<S> for IsAtMost<E>
where
    S: PartialOrd<E> + Debug,
    E: Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject <= &self.expected
    }

    fn message(&self, expression: Expression<'_>, actual: &S, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing(&self.expected, format);
        format!(
            "expected {expression} is at most {:?}\n   but was: {marked_actual}\n  expected: <= {marked_expected}",
            self.expected,
        )
    }
}

impl<S, E> Expectation<S> for IsGreaterThan<E>
where
    S: PartialOrd<E> + Debug,
    E: Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject > &self.expected
    }

    fn message(&self, expression: Expression<'_>, actual: &S, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing(&self.expected, format);
        format!(
            "expected {expression} is greater than {:?}\n   but was: {marked_actual}\n  expected: > {marked_expected}",
            self.expected,
        )
    }
}

impl<S, E> Expectation<S> for IsAtLeast<E>
where
    S: PartialOrd<E> + Debug,
    E: Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject >= &self.expected
    }

    fn message(&self, expression: Expression<'_>, actual: &S, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing(&self.expected, format);
        format!(
            "expected {expression} is at least {:?}\n   but was: {marked_actual}\n  expected: >= {marked_expected}",
            self.expected,
        )
    }
}

#[cfg(test)]
mod tests;
