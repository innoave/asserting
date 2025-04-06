//! Implementations of the emptiness and length assertions.

use crate::assertions::{AssertEmptiness, AssertHasLength};
use crate::colored::mark_unexpected;
use crate::expectations::{HasLength, HasLengthInRange, IsEmpty, IsNotEmpty};
use crate::properties::{IsEmptyProperty, LengthProperty};
use crate::spec::{DiffFormat, Expectation, Expression, FailingStrategy, Spec};
use crate::std::fmt::Debug;
use crate::std::ops::RangeInclusive;
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
        self.expecting(IsNotEmpty)
    }
}

impl<S> Expectation<S> for IsEmpty
where
    S: IsEmptyProperty + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.is_empty_property()
    }

    fn message(&self, expression: Expression<'_>, actual: &S, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected(actual, format);
        format!("expected {expression} is empty\n   but was: {marked_actual}\n  expected: <empty>")
    }
}

impl<S> Expectation<S> for IsNotEmpty
where
    S: IsEmptyProperty + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        !subject.is_empty_property()
    }

    fn message(&self, expression: Expression<'_>, actual: &S, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected(actual, format);
        format!(
            "expected {expression} is not empty\n   but was: {marked_actual}\n  expected: <non-empty>",
        )
    }
}

impl<S, R> AssertHasLength<usize> for Spec<'_, S, R>
where
    S: LengthProperty + Debug,
    R: FailingStrategy,
{
    fn has_length(self, expected_length: usize) -> Self {
        self.expecting(HasLength { expected_length })
    }

    fn has_length_in_range(self, range: RangeInclusive<usize>) -> Self {
        self.expecting(HasLengthInRange {
            expected_range: range,
        })
    }
}

impl<S> Expectation<S> for HasLength<usize>
where
    S: LengthProperty + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.length_property() == self.expected_length
    }

    fn message(&self, expression: Expression<'_>, actual: &S, _format: &DiffFormat) -> String {
        format!(
            "expected {expression} has length {}\n   but was: {}\n  expected: {}",
            self.expected_length,
            actual.length_property(),
            self.expected_length
        )
    }
}

impl<S> Expectation<S> for HasLengthInRange<usize>
where
    S: LengthProperty + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        self.expected_range.contains(&subject.length_property())
    }

    fn message(&self, expression: Expression<'_>, actual: &S, _format: &DiffFormat) -> String {
        format!(
            "expected {expression} has length in range {:?}\n   but was: {}\n  expected: {:?}",
            self.expected_range,
            actual.length_property(),
            self.expected_range
        )
    }
}
