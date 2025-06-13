//! Implementations of the emptiness and length assertions.

use crate::assertions::{AssertEmptiness, AssertHasLength};
use crate::colored::{mark_missing, mark_unexpected};
use crate::expectations::{
    HasAtLeastLength, HasAtMostLength, HasLength, HasLengthGreaterThan, HasLengthInRange,
    HasLengthLessThan, IsEmpty, Not,
};
use crate::properties::{IsEmptyProperty, LengthProperty};
use crate::spec::{DiffFormat, Expectation, Expression, FailingStrategy, Invertible, Spec};
use crate::std::fmt::Debug;
use crate::std::ops::RangeBounds;
use crate::std::{format, string::String};

impl<S, R> AssertEmptiness for Spec<'_, S, R>
where
    S: IsEmptyProperty + Debug,
    R: FailingStrategy,
{
    fn is_empty(self) -> Self {
        self.expecting(IsEmpty)
    }

    fn is_not_empty(self) -> Self {
        self.expecting(Not(IsEmpty))
    }
}

impl<S> Expectation<S> for IsEmpty
where
    S: IsEmptyProperty + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.is_empty_property()
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let (not, expected) = if inverted {
            ("not ", "<non-empty>")
        } else {
            ("", "<empty>")
        };
        let marked_actual = mark_unexpected(actual, format);
        format!("expected {expression} to be {not}empty\n   but was: {marked_actual}\n  expected: {expected}")
    }
}

impl Invertible for IsEmpty {}

impl<S, R> AssertHasLength<usize> for Spec<'_, S, R>
where
    S: LengthProperty + Debug,
    R: FailingStrategy,
{
    fn has_length(self, expected_length: usize) -> Self {
        self.expecting(HasLength { expected_length })
    }

    fn has_length_in_range<U>(self, expected_range: U) -> Self
    where
        U: RangeBounds<usize> + Debug,
    {
        self.expecting(HasLengthInRange::new(expected_range))
    }

    fn has_length_less_than(self, expected_length: usize) -> Self {
        self.expecting(HasLengthLessThan { expected_length })
    }

    fn has_length_greater_than(self, expected_length: usize) -> Self {
        self.expecting(HasLengthGreaterThan { expected_length })
    }

    fn has_at_most_length(self, expected_length: usize) -> Self {
        self.expecting(HasAtMostLength { expected_length })
    }

    fn has_at_least_length(self, expected_length: usize) -> Self {
        self.expecting(HasAtLeastLength { expected_length })
    }
}

impl<S> Expectation<S> for HasLength<usize>
where
    S: LengthProperty + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.length_property() == self.expected_length
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let not = if inverted { "not " } else { "" };
        let marked_actual = mark_unexpected(&actual.length_property(), format);
        let marked_expected = mark_missing(&self.expected_length, format);
        format!(
            "expected {expression} to {not}have a length of {}\n   but was: {marked_actual}\n  expected: {not}{marked_expected}",
            self.expected_length,
        )
    }
}

impl Invertible for HasLength<usize> {}

impl<S, R> Expectation<S> for HasLengthInRange<R, usize>
where
    S: LengthProperty + Debug,
    R: RangeBounds<usize> + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        self.expected_range.contains(&subject.length_property())
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let not = if inverted { "not " } else { "" };
        let marked_actual = mark_unexpected(&actual.length_property(), format);
        let marked_expected = mark_missing(&self.expected_range, format);
        format!(
            "expected {expression} to {not}have a length within range {:?}\n   but was: {marked_actual}\n  expected: {not}{marked_expected}",
            self.expected_range,
        )
    }
}

impl<R> Invertible for HasLengthInRange<R, usize> {}

impl<S> Expectation<S> for HasLengthLessThan<usize>
where
    S: LengthProperty + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.length_property() < self.expected_length
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let (not, cmp) = if inverted { ("not ", ">=") } else { ("", "<") };
        let marked_actual = mark_unexpected(&actual.length_property(), format);
        let marked_expected = mark_missing(&self.expected_length, format);
        format!(
            "expected {expression} to {not}have a length less than {:?}\n   but was: {marked_actual}\n  expected: {cmp} {marked_expected}",
            self.expected_length,
        )
    }
}

impl Invertible for HasLengthLessThan<usize> {}

impl<S> Expectation<S> for HasLengthGreaterThan<usize>
where
    S: LengthProperty + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.length_property() > self.expected_length
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let (not, cmp) = if inverted { ("not ", "<=") } else { ("", ">") };
        let marked_actual = mark_unexpected(&actual.length_property(), format);
        let marked_expected = mark_missing(&self.expected_length, format);
        format!(
            "expected {expression} to {not}have a length greater than {:?}\n   but was: {marked_actual}\n  expected: {cmp} {marked_expected}",
            self.expected_length,
        )
    }
}

impl Invertible for HasLengthGreaterThan<usize> {}

impl<S> Expectation<S> for HasAtMostLength<usize>
where
    S: LengthProperty + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.length_property() <= self.expected_length
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let (not, cmp) = if inverted { ("not ", ">") } else { ("", "<=") };
        let marked_actual = mark_unexpected(&actual.length_property(), format);
        let marked_expected = mark_missing(&self.expected_length, format);
        format!(
            "expected {expression} to {not}have at most a length of {:?}\n   but was: {marked_actual}\n  expected: {cmp} {marked_expected}",
            self.expected_length,
        )
    }
}

impl Invertible for HasAtMostLength<usize> {}

impl<S> Expectation<S> for HasAtLeastLength<usize>
where
    S: LengthProperty + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.length_property() >= self.expected_length
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let (not, cmp) = if inverted { ("not ", "<") } else { ("", ">=") };
        let marked_actual = mark_unexpected(&actual.length_property(), format);
        let marked_expected = mark_missing(&self.expected_length, format);
        format!(
            "expected {expression} to {not}have at least a length of {:?}\n   but was: {marked_actual}\n  expected: {cmp} {marked_expected}",
            self.expected_length,
        )
    }
}

impl Invertible for HasAtLeastLength<usize> {}
