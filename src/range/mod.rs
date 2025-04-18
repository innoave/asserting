//! Implementation of assertions for `Range` and `RangeInclusive` values.

use crate::assertions::AssertInRange;
use crate::colored::{mark_missing, mark_unexpected};
use crate::expectations::{IsInRange, IsNotInRange};
use crate::properties::IsEmptyProperty;
use crate::spec::{DiffFormat, Expectation, Expression, FailingStrategy, Spec};
use crate::std::fmt::Debug;
use crate::std::ops::{Range, RangeBounds, RangeInclusive};
use crate::std::{format, string::String};

impl<T> IsEmptyProperty for Range<T>
where
    T: PartialEq,
{
    fn is_empty_property(&self) -> bool {
        self.start == self.end
    }
}

impl<T> IsEmptyProperty for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn is_empty_property(&self) -> bool {
        self.start() > self.end()
    }
}

impl<S, E, R> AssertInRange<E> for Spec<'_, S, R>
where
    S: PartialOrd<E> + Debug,
    E: PartialOrd<S> + Debug,
    R: FailingStrategy,
{
    fn is_in_range(self, range: RangeInclusive<E>) -> Self {
        self.expecting(IsInRange {
            expected_range: range,
        })
    }

    fn is_not_in_range(self, range: RangeInclusive<E>) -> Self {
        self.expecting(IsNotInRange {
            expected_range: range,
        })
    }
}

impl<S, E> Expectation<S> for IsInRange<E>
where
    S: PartialOrd<E> + Debug,
    E: PartialOrd<S> + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        self.expected_range.contains(subject)
    }

    fn message(&self, expression: Expression<'_>, actual: &S, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected_start = if actual < self.expected_range.start() {
            mark_missing(self.expected_range.start(), format)
        } else {
            format!("{:?}", self.expected_range.start())
        };
        let marked_expected_end = if actual > self.expected_range.end() {
            mark_missing(self.expected_range.end(), format)
        } else {
            format!("{:?}", self.expected_range.end())
        };
        format!(
            "expected {expression} is within range of {:?}\n   but was: {marked_actual}\n  expected: {marked_expected_start} <= x <= {marked_expected_end}",
            self.expected_range,
        )
    }
}

impl<S, E> Expectation<S> for IsNotInRange<E>
where
    S: PartialOrd<E> + Debug,
    E: PartialOrd<S> + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        !self.expected_range.contains(subject)
    }

    fn message(&self, expression: Expression<'_>, actual: &S, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected_start = mark_missing(self.expected_range.start(), format);
        let marked_expected_end = mark_missing(self.expected_range.end(), format);
        format!(
            "expected {expression} is not within range of {:?}\n   but was: {marked_actual}\n  expected: x < {marked_expected_start} || x > {marked_expected_end}",
            self.expected_range,
        )
    }
}

#[cfg(test)]
mod tests;
