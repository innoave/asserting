//! Implementation of assertions for `Range` and `RangeInclusive` values.

use crate::assertions::AssertInRange;
use crate::colored::{mark_missing, mark_missing_substr, mark_unexpected};
use crate::expectations::{IsInRange, Not};
use crate::properties::IsEmptyProperty;
use crate::spec::{DiffFormat, Expectation, Expression, FailingStrategy, Invertible, Spec};
use crate::std::fmt::Debug;
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

impl<S, E, R> AssertInRange<E> for Spec<'_, S, R>
where
    S: PartialOrd<E> + Debug,
    E: PartialOrd<S> + Debug,
    R: FailingStrategy,
{
    fn is_in_range<U>(self, range: U) -> Self
    where
        U: RangeBounds<E> + Debug,
    {
        self.expecting(IsInRange::new(range))
    }

    fn is_not_in_range<U>(self, range: U) -> Self
    where
        U: RangeBounds<E> + Debug,
    {
        self.expecting(Not(IsInRange::new(range)))
    }
}

impl<S, E, R> Expectation<S> for IsInRange<R, E>
where
    S: PartialOrd<E> + Debug,
    E: PartialOrd<S> + Debug,
    R: RangeBounds<E> + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        self.expected_range.contains(subject)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let marked_actual = mark_unexpected(actual, format);
        let (not, marked_expected) = if inverted {
            let marked_expected_start = match self.expected_range.start_bound() {
                Bound::Included(start) => format!("< {}", mark_missing(start, format)),
                Bound::Excluded(start) => format!("<= {}", mark_missing(start, format)),
                Bound::Unbounded => format!("< {}", mark_missing_substr("..", format)),
            };
            let marked_expected_end = match self.expected_range.end_bound() {
                Bound::Included(end) => format!("> {}", mark_missing(end, format)),
                Bound::Excluded(end) => format!(">= {}", mark_missing(end, format)),
                Bound::Unbounded => format!("> {}", mark_missing_substr("..", format)),
            };

            (
                "not ",
                format!("x {marked_expected_start} || x {marked_expected_end}"),
            )
        } else {
            let marked_expected_start = match self.expected_range.start_bound() {
                Bound::Included(start) => {
                    if actual < start {
                        format!("{} <=", mark_missing(start, format))
                    } else {
                        format!("{start:?} <=")
                    }
                },
                Bound::Excluded(start) => {
                    if actual <= start {
                        format!("{} <", mark_missing(start, format))
                    } else {
                        format!("{start:?} <")
                    }
                },
                Bound::Unbounded => format!("{} <", mark_missing_substr("..", format)),
            };
            let marked_expected_end = match self.expected_range.end_bound() {
                Bound::Included(end) => {
                    if actual > end {
                        format!("<= {}", mark_missing(end, format))
                    } else {
                        format!("<= {end:?}")
                    }
                },
                Bound::Excluded(end) => {
                    if actual >= end {
                        format!("< {}", mark_missing(end, format))
                    } else {
                        format!("< {end:?}")
                    }
                },
                Bound::Unbounded => format!("< {}", mark_missing_substr("..", format)),
            };

            (
                "",
                format!("{marked_expected_start} x {marked_expected_end}"),
            )
        };

        format!(
            "expected {expression} to be {not}within range of {:?}\n   but was: {marked_actual}\n  expected: {marked_expected}",
            self.expected_range,
        )
    }
}

impl<R, E> Invertible for IsInRange<R, E> {}

#[cfg(test)]
mod tests;
