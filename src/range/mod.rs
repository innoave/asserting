use crate::assertions::AssertInRange;
use crate::expectations::{IsInRange, IsNotInRange};
use crate::properties::IsEmptyProperty;
use crate::spec::{Expectation, Expression, FailingStrategy, Spec};
use crate::std::fmt::Debug;
use crate::std::ops::{Range, RangeBounds, RangeInclusive};
#[cfg(not(feature = "std"))]
use alloc::{format, string::String};

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

    fn message(&self, expression: Expression<'_>, actual: &S) -> String {
        format!(
            "expected {expression} is within range of {:?}\n   but was: {actual:?}\n  expected: {:?} <= x <= {:?}",
            self.expected_range,
            self.expected_range.start(),
            self.expected_range.end(),
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

    fn message(&self, expression: Expression<'_>, actual: &S) -> String {
        format!(
            "expected {expression} is not within range of {:?}\n   but was: {actual:?}\n  expected: x < {:?} || x > {:?}",
            self.expected_range,
            self.expected_range.start(),
            self.expected_range.end(),
        )
    }
}

#[cfg(test)]
mod tests;
