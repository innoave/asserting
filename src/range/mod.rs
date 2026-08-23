//! Implementation of assertions for `Range` and `RangeInclusive` values.

use crate::assertions::AssertInRange;
use crate::colored::{mark_missing, mark_unexpected};
use crate::expectations::{IsInRange, is_in_range, not};
use crate::properties::IsEmptyProperty;
use crate::spec::{
    DiffFormat, DisplayRepresentation, Expectation, Expecting, Expression, FailingStrategy,
    Invertible, Represent, Represented, Spec,
};
use crate::std::format;
use crate::std::ops::{Bound, Range, RangeBounds, RangeInclusive};
use crate::std::string::String;

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

impl<S, E, D, R> AssertInRange<E, D> for Spec<'_, S, D, R>
where
    S: PartialOrd<E>,
    E: PartialOrd<S>,
    D: Represent<S> + Represent<E>,
    R: FailingStrategy,
{
    fn is_in_range<U>(self, range: U) -> Self
    where
        U: RangeBounds<E>,
        D: Represent<U>,
    {
        self.expecting(is_in_range(range))
    }

    fn is_not_in_range<U>(self, range: U) -> Self
    where
        U: RangeBounds<E>,
        D: Represent<U>,
    {
        self.expecting(not(is_in_range(range)))
    }
}

impl<S, E, D, R> Expectation<S, D> for IsInRange<R, E>
where
    S: PartialOrd<E>,
    E: PartialOrd<S>,
    R: RangeBounds<E>,
    D: Represent<S> + Represent<E> + Represent<R>,
{
    fn test(&mut self, subject: &S) -> bool {
        self.expected_range.contains(subject)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let marked_actual = mark_unexpected(actual, representation, format);
        let (not, marked_expected) = if inverted {
            let marked_expected_start = match self.expected_range.start_bound() {
                Bound::Included(start) => {
                    format!("< {}", mark_missing(start, representation, format))
                },
                Bound::Excluded(start) => {
                    format!("<= {}", mark_missing(start, representation, format))
                },
                Bound::Unbounded => {
                    format!("< {}", mark_missing("..", &DisplayRepresentation, format))
                },
            };
            let marked_expected_end = match self.expected_range.end_bound() {
                Bound::Included(end) => format!("> {}", mark_missing(end, representation, format)),
                Bound::Excluded(end) => format!(">= {}", mark_missing(end, representation, format)),
                Bound::Unbounded => {
                    format!("> {}", mark_missing("..", &DisplayRepresentation, format))
                },
            };

            (
                "not ",
                format!("x {marked_expected_start} || x {marked_expected_end}"),
            )
        } else {
            let marked_expected_start = match self.expected_range.start_bound() {
                Bound::Included(start) => {
                    if actual < start {
                        format!("{} <=", mark_missing(start, representation, format))
                    } else {
                        let represented_start = Represented::from((start, representation));
                        format!("{represented_start:?} <=")
                    }
                },
                Bound::Excluded(start) => {
                    if actual <= start {
                        format!("{} <", mark_missing(start, representation, format))
                    } else {
                        let represented_start = Represented::from((start, representation));
                        format!("{represented_start:?} <")
                    }
                },
                Bound::Unbounded => {
                    format!("{} <", mark_missing("..", &DisplayRepresentation, format))
                },
            };
            let marked_expected_end = match self.expected_range.end_bound() {
                Bound::Included(end) => {
                    if actual > end {
                        format!("<= {}", mark_missing(end, representation, format))
                    } else {
                        let represented_end = Represented::from((end, representation));
                        format!("<= {represented_end:?}")
                    }
                },
                Bound::Excluded(end) => {
                    if actual >= end {
                        format!("< {}", mark_missing(end, representation, format))
                    } else {
                        let represented_end = Represented::from((end, representation));
                        format!("< {represented_end:?}")
                    }
                },
                Bound::Unbounded => {
                    format!("< {}", mark_missing("..", &DisplayRepresentation, format))
                },
            };

            (
                "",
                format!("{marked_expected_start} x {marked_expected_end}"),
            )
        };

        let represented_range = Represented::from((&self.expected_range, representation));
        format!(
            "expected {expression} to be {not}within range of {represented_range:?}\n   but was: {marked_actual}\n  expected: {marked_expected}",
        )
    }
}

impl<R, E> Invertible for IsInRange<R, E> {}

#[cfg(test)]
mod tests;
