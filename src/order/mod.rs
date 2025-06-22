//! Implementation of order assertions.

use crate::assertions::AssertOrder;
use crate::colored::{mark_missing, mark_unexpected};
use crate::expectations::{
    is_after, is_at_least, is_at_most, is_before, is_between, is_greater_than, is_less_than,
    IsAfter, IsAtLeast, IsAtMost, IsBefore, IsBetween, IsGreaterThan, IsLessThan,
};
use crate::spec::{DiffFormat, Expectation, Expression, FailingStrategy, Invertible, Spec};
use crate::std::fmt::Debug;
use crate::std::{format, string::String};

impl<S, E, R> AssertOrder<E> for Spec<'_, S, R>
where
    S: PartialOrd<E> + Debug,
    E: Debug,
    R: FailingStrategy,
{
    fn is_less_than(self, expected: E) -> Self {
        self.expecting(is_less_than(expected))
    }

    fn is_greater_than(self, expected: E) -> Self {
        self.expecting(is_greater_than(expected))
    }

    fn is_at_most(self, expected: E) -> Self {
        self.expecting(is_at_most(expected))
    }

    fn is_at_least(self, expected: E) -> Self {
        self.expecting(is_at_least(expected))
    }

    fn is_before(self, expected: E) -> Self {
        self.expecting(is_before(expected))
    }

    fn is_after(self, expected: E) -> Self {
        self.expecting(is_after(expected))
    }

    fn is_between(self, min: E, max: E) -> Self {
        self.expecting(is_between(min, max))
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

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let (not, cmp) = if inverted { ("not ", ">=") } else { ("", "<") };
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing(&self.expected, format);
        format!(
            "expected {expression} to be {not}less than {:?}\n   but was: {marked_actual}\n  expected: {cmp} {marked_expected}",
            self.expected,
        )
    }
}

impl<E> Invertible for IsLessThan<E> {}

impl<S, E> Expectation<S> for IsAtMost<E>
where
    S: PartialOrd<E> + Debug,
    E: Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject <= &self.expected
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let (not, cmp) = if inverted { ("not ", ">") } else { ("", "<=") };
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing(&self.expected, format);
        format!(
            "expected {expression} to be {not}at most {:?}\n   but was: {marked_actual}\n  expected: {cmp} {marked_expected}",
            self.expected,
        )
    }
}

impl<E> Invertible for IsAtMost<E> {}

impl<S, E> Expectation<S> for IsGreaterThan<E>
where
    S: PartialOrd<E> + Debug,
    E: Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject > &self.expected
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let (not, cmp) = if inverted { ("not ", "<=") } else { ("", ">") };
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing(&self.expected, format);
        format!(
            "expected {expression} to be {not}greater than {:?}\n   but was: {marked_actual}\n  expected: {cmp} {marked_expected}",
            self.expected,
        )
    }
}

impl<E> Invertible for IsGreaterThan<E> {}

impl<S, E> Expectation<S> for IsAtLeast<E>
where
    S: PartialOrd<E> + Debug,
    E: Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject >= &self.expected
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let (not, cmp) = if inverted { ("not ", "<") } else { ("", ">=") };
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing(&self.expected, format);
        format!(
            "expected {expression} to be {not}at least {:?}\n   but was: {marked_actual}\n  expected: {cmp} {marked_expected}",
            self.expected,
        )
    }
}

impl<E> Invertible for IsAtLeast<E> {}

impl<S, E> Expectation<S> for IsBefore<E>
where
    S: PartialOrd<E> + Debug,
    E: Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject < &self.expected
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let (not, cmp) = if inverted { ("not ", ">=") } else { ("", "<") };
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing(&self.expected, format);
        format!(
            "expected {expression} to be {not}before {:?}\n   but was: {marked_actual}\n  expected: {cmp} {marked_expected}",
            self.expected,
        )
    }
}

impl<E> Invertible for IsBefore<E> {}

impl<S, E> Expectation<S> for IsAfter<E>
where
    S: PartialOrd<E> + Debug,
    E: Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject > &self.expected
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let (not, cmp) = if inverted { ("not ", "<=") } else { ("", ">") };
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing(&self.expected, format);
        format!(
            "expected {expression} to be {not}after {:?}\n   but was: {marked_actual}\n  expected: {cmp} {marked_expected}",
            self.expected,
        )
    }
}

impl<E> Invertible for IsAfter<E> {}

impl<S, E> Expectation<S> for IsBetween<E>
where
    S: PartialOrd<E> + Debug,
    E: Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject >= &self.min && subject <= &self.max
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let (not, cmp) = if inverted {
            ("not ", "> x or x >")
        } else {
            ("", "<= x <=")
        };
        let marked_actual = mark_unexpected(actual, format);
        let marked_start = if (actual < &self.min) || inverted {
            mark_missing(&self.min, format)
        } else {
            format!("{:?}", &self.min)
        };
        let marked_end = if (actual > &self.max) || inverted {
            mark_missing(&self.max, format)
        } else {
            format!("{:?}", &self.max)
        };
        format!(
            "expected {expression} to be {not}between {:?} and {:?}\n   but was: {marked_actual}\n  expected: {marked_start} {cmp} {marked_end}",
            self.min, self.max
        )
    }
}

impl<E> Invertible for IsBetween<E> {}

#[cfg(test)]
mod tests;
