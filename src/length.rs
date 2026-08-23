//! Implementations of the emptiness and length assertions.

use crate::assertions::{AssertEmptiness, AssertHasLength};
use crate::colored::{mark_missing, mark_unexpected};
use crate::expectations::{
    HasAtLeastLength, HasAtMostLength, HasLength, HasLengthGreaterThan, HasLengthInRange,
    HasLengthLessThan, IsEmpty, has_at_least_length, has_at_most_length, has_length,
    has_length_greater_than, has_length_in_range, has_length_less_than, is_empty, not,
};
use crate::properties::{IsEmptyProperty, LengthProperty};
use crate::spec::{
    DiffFormat, Expectation, Expecting, Expression, FailingStrategy, Invertible, Represent,
    Represented, Spec,
};
use crate::std::ops::RangeBounds;
use crate::std::{format, string::String};

impl<S, D, R> AssertEmptiness for Spec<'_, S, D, R>
where
    S: IsEmptyProperty,
    D: Represent<S>,
    R: FailingStrategy,
{
    fn is_empty(self) -> Self {
        self.expecting(is_empty())
    }

    fn is_not_empty(self) -> Self {
        self.expecting(not(is_empty()))
    }
}

impl<S, D> Expectation<S, D> for IsEmpty
where
    S: IsEmptyProperty,
    D: Represent<S>,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.is_empty_property()
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let (not, expected) = if inverted {
            ("not ", "<non-empty>")
        } else {
            ("", "<empty>")
        };
        let marked_actual = mark_unexpected(actual, representation, format);
        format!(
            "expected {expression} to be {not}empty\n   but was: {marked_actual}\n  expected: {expected}"
        )
    }
}

impl Invertible for IsEmpty {}

impl<S, D, R> AssertHasLength<usize, D> for Spec<'_, S, D, R>
where
    S: LengthProperty,
    D: Represent<usize>,
    R: FailingStrategy,
{
    fn has_length(self, expected_length: usize) -> Self {
        self.expecting(has_length(expected_length))
    }

    fn has_length_in_range<U>(self, expected_range: U) -> Self
    where
        U: RangeBounds<usize>,
        D: Represent<U>,
    {
        self.expecting(has_length_in_range(expected_range))
    }

    fn has_length_less_than(self, expected_length: usize) -> Self {
        self.expecting(has_length_less_than(expected_length))
    }

    fn has_length_greater_than(self, expected_length: usize) -> Self {
        self.expecting(has_length_greater_than(expected_length))
    }

    fn has_at_most_length(self, expected_length: usize) -> Self {
        self.expecting(has_at_most_length(expected_length))
    }

    fn has_at_least_length(self, expected_length: usize) -> Self {
        self.expecting(has_at_least_length(expected_length))
    }
}

impl<S, D> Expectation<S, D> for HasLength<usize>
where
    S: LengthProperty,
    D: Represent<usize>,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.length_property() == self.expected_length
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let not = if inverted { "not " } else { "" };
        let marked_actual = mark_unexpected(&actual.length_property(), representation, format);
        let marked_expected = mark_missing(&self.expected_length, representation, format);
        let expected_length = Represented::from((&self.expected_length, representation));
        format!(
            "expected {expression} to {not}have a length of {expected_length}\n   but was: {marked_actual}\n  expected: {not}{marked_expected}",
        )
    }
}

impl Invertible for HasLength<usize> {}

impl<S, R, D> Expectation<S, D> for HasLengthInRange<R, usize>
where
    S: LengthProperty,
    R: RangeBounds<usize>,
    D: Represent<usize> + Represent<R>,
{
    fn test(&mut self, subject: &S) -> bool {
        self.expected_range.contains(&subject.length_property())
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let not = if inverted { "not " } else { "" };
        let marked_actual = mark_unexpected(&actual.length_property(), representation, format);
        let marked_expected = mark_missing(&self.expected_range, representation, format);
        let expected_range = Represented::from((&self.expected_range, representation));
        format!(
            "expected {expression} to {not}have a length within range {expected_range:?}\n   but was: {marked_actual}\n  expected: {not}{marked_expected}",
        )
    }
}

impl<R> Invertible for HasLengthInRange<R, usize> {}

impl<S, D> Expectation<S, D> for HasLengthLessThan<usize>
where
    S: LengthProperty,
    D: Represent<usize>,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.length_property() < self.expected_length
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
        let marked_actual = mark_unexpected(&actual.length_property(), representation, format);
        let marked_expected = mark_missing(&self.expected_length, representation, format);
        let expected_length = Represented::from((&self.expected_length, representation));
        format!(
            "expected {expression} to {not}have a length less than {expected_length:?}\n   but was: {marked_actual}\n  expected: {cmp} {marked_expected}",
        )
    }
}

impl Invertible for HasLengthLessThan<usize> {}

impl<S, D> Expectation<S, D> for HasLengthGreaterThan<usize>
where
    S: LengthProperty,
    D: Represent<usize>,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.length_property() > self.expected_length
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
        let marked_actual = mark_unexpected(&actual.length_property(), representation, format);
        let marked_expected = mark_missing(&self.expected_length, representation, format);
        let expected_length = Represented::from((&self.expected_length, representation));
        format!(
            "expected {expression} to {not}have a length greater than {expected_length:?}\n   but was: {marked_actual}\n  expected: {cmp} {marked_expected}",
        )
    }
}

impl Invertible for HasLengthGreaterThan<usize> {}

impl<S, D> Expectation<S, D> for HasAtMostLength<usize>
where
    S: LengthProperty,
    D: Represent<usize>,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.length_property() <= self.expected_length
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
        let marked_actual = mark_unexpected(&actual.length_property(), representation, format);
        let marked_expected = mark_missing(&self.expected_length, representation, format);
        let expected_length = Represented::from((&self.expected_length, representation));
        format!(
            "expected {expression} to {not}have at most a length of {expected_length:?}\n   but was: {marked_actual}\n  expected: {cmp} {marked_expected}",
        )
    }
}

impl Invertible for HasAtMostLength<usize> {}

impl<S, D> Expectation<S, D> for HasAtLeastLength<usize>
where
    S: LengthProperty,
    D: Represent<usize>,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.length_property() >= self.expected_length
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
        let marked_actual = mark_unexpected(&actual.length_property(), representation, format);
        let marked_expected = mark_missing(&self.expected_length, representation, format);
        let expected_length = Represented::from((&self.expected_length, representation));
        format!(
            "expected {expression} to {not}have at least a length of {expected_length:?}\n   but was: {marked_actual}\n  expected: {cmp} {marked_expected}",
        )
    }
}

impl Invertible for HasAtLeastLength<usize> {}
