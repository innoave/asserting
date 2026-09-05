//! Implementation of order assertions.

use crate::assertions::AssertOrder;
use crate::colored::{mark_missing, mark_unexpected};
use crate::expectations::{
    IsAfter, IsAtLeast, IsAtMost, IsBefore, IsBetween, IsGreaterThan, IsLessThan, is_after,
    is_at_least, is_at_most, is_before, is_between, is_greater_than, is_less_than,
};
use crate::spec::{
    DiffFormat, Expectation, Expecting, Expression, FailingStrategy, Invertible, Represent,
    Represented, Spec,
};
use crate::std::{format, string::String};

impl<S, E, D, R> AssertOrder<E> for Spec<'_, S, D, R>
where
    S: PartialOrd<E>,
    D: Represent<S> + Represent<E>,
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

impl<S, E, D> Expectation<S, D> for IsLessThan<E>
where
    S: PartialOrd<E>,
    D: Represent<S> + Represent<E>,
{
    fn test(&mut self, subject: &S) -> bool {
        subject < &self.expected
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let (not, cmp) = if inverted { ("not ", ">=") } else { ("", "<") };
        let marked_actual = mark_unexpected(actual, representation, format);
        let marked_expected = mark_missing(&self.expected, representation, format);
        let represented_expected = Represented::from((&self.expected, representation));
        format!(
            "expected {expression} to be {not}less than {represented_expected:?}\n   but was: {marked_actual}\n  expected: {cmp} {marked_expected}",
        )
    }
}

impl<E> Invertible for IsLessThan<E> {}

impl<S, E, D> Expectation<S, D> for IsAtMost<E>
where
    S: PartialOrd<E>,
    D: Represent<S> + Represent<E>,
{
    fn test(&mut self, subject: &S) -> bool {
        subject <= &self.expected
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let (not, cmp) = if inverted { ("not ", ">") } else { ("", "<=") };
        let marked_actual = mark_unexpected(actual, representation, format);
        let marked_expected = mark_missing(&self.expected, representation, format);
        let represented_expected = Represented::from((&self.expected, representation));
        format!(
            "expected {expression} to be {not}at most {represented_expected:?}\n   but was: {marked_actual}\n  expected: {cmp} {marked_expected}",
        )
    }
}

impl<E> Invertible for IsAtMost<E> {}

impl<S, E, D> Expectation<S, D> for IsGreaterThan<E>
where
    S: PartialOrd<E>,
    D: Represent<S> + Represent<E>,
{
    fn test(&mut self, subject: &S) -> bool {
        subject > &self.expected
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let (not, cmp) = if inverted { ("not ", "<=") } else { ("", ">") };
        let marked_actual = mark_unexpected(actual, representation, format);
        let marked_expected = mark_missing(&self.expected, representation, format);
        let represented_expected = Represented::from((&self.expected, representation));
        format!(
            "expected {expression} to be {not}greater than {represented_expected:?}\n   but was: {marked_actual}\n  expected: {cmp} {marked_expected}",
        )
    }
}

impl<E> Invertible for IsGreaterThan<E> {}

impl<S, E, D> Expectation<S, D> for IsAtLeast<E>
where
    S: PartialOrd<E>,
    D: Represent<S> + Represent<E>,
{
    fn test(&mut self, subject: &S) -> bool {
        subject >= &self.expected
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let (not, cmp) = if inverted { ("not ", "<") } else { ("", ">=") };
        let marked_actual = mark_unexpected(actual, representation, format);
        let marked_expected = mark_missing(&self.expected, representation, format);
        let represented_expected = Represented::from((&self.expected, representation));
        format!(
            "expected {expression} to be {not}at least {represented_expected:?}\n   but was: {marked_actual}\n  expected: {cmp} {marked_expected}",
        )
    }
}

impl<E> Invertible for IsAtLeast<E> {}

impl<S, E, D> Expectation<S, D> for IsBefore<E>
where
    S: PartialOrd<E>,
    D: Represent<S> + Represent<E>,
{
    fn test(&mut self, subject: &S) -> bool {
        subject < &self.expected
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let (not, cmp) = if inverted { ("not ", ">=") } else { ("", "<") };
        let marked_actual = mark_unexpected(actual, representation, format);
        let marked_expected = mark_missing(&self.expected, representation, format);
        let represented_expected = Represented::from((&self.expected, representation));
        format!(
            "expected {expression} to be {not}before {represented_expected:?}\n   but was: {marked_actual}\n  expected: {cmp} {marked_expected}",
        )
    }
}

impl<E> Invertible for IsBefore<E> {}

impl<S, E, D> Expectation<S, D> for IsAfter<E>
where
    S: PartialOrd<E>,
    D: Represent<S> + Represent<E>,
{
    fn test(&mut self, subject: &S) -> bool {
        subject > &self.expected
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let (not, cmp) = if inverted { ("not ", "<=") } else { ("", ">") };
        let marked_actual = mark_unexpected(actual, representation, format);
        let marked_expected = mark_missing(&self.expected, representation, format);
        let represented_expected = Represented::from((&self.expected, representation));
        format!(
            "expected {expression} to be {not}after {represented_expected:?}\n   but was: {marked_actual}\n  expected: {cmp} {marked_expected}",
        )
    }
}

impl<E> Invertible for IsAfter<E> {}

impl<S, E, D> Expectation<S, D> for IsBetween<E>
where
    S: PartialOrd<E>,
    D: Represent<S> + Represent<E>,
{
    fn test(&mut self, subject: &S) -> bool {
        subject >= &self.min && subject <= &self.max
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let (not, cmp) = if inverted {
            ("not ", "> x or x >")
        } else {
            ("", "<= x <=")
        };
        let expected_min = Represented::from((&self.min, representation));
        let expected_max = Represented::from((&self.max, representation));
        let marked_actual = mark_unexpected(actual, representation, format);
        let marked_start = if (actual < &self.min) || inverted {
            mark_missing(&self.min, representation, format)
        } else {
            format!("{expected_min:?}")
        };
        let marked_end = if (actual > &self.max) || inverted {
            mark_missing(&self.max, representation, format)
        } else {
            format!("{expected_max:?}")
        };
        format!(
            "expected {expression} to be {not}between {expected_min:?} and {expected_max:?}\n   but was: {marked_actual}\n  expected: {marked_start} {cmp} {marked_end}",
        )
    }
}

impl<E> Invertible for IsBetween<E> {}

#[cfg(test)]
mod tests;
