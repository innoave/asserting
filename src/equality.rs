//! Implementation of the equality assertions.

use crate::assertions::AssertEquality;
use crate::color::mark_diff;
use crate::expectations::{IsEqualTo, IsNotEqualTo};
use crate::spec::{DiffFormat, Expectation, Expression, FailingStrategy, Spec};
use crate::std::fmt::Debug;
use crate::std::{format, string::String};

impl<S, E, R> AssertEquality<E> for Spec<'_, S, R>
where
    S: PartialEq<E> + Debug,
    E: Debug,
    R: FailingStrategy,
{
    fn is_equal_to(self, expected: E) -> Self {
        self.expecting(IsEqualTo { expected })
    }

    fn is_not_equal_to(self, expected: E) -> Self {
        self.expecting(IsNotEqualTo { expected })
    }
}

impl<S, E> Expectation<S> for IsEqualTo<E>
where
    S: PartialEq<E> + Debug,
    E: Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject == &self.expected
    }

    fn message(&self, expression: Expression<'_>, actual: &S, format: &DiffFormat) -> String {
        let expected = &self.expected;
        let (marked_actual, marked_expected) = mark_diff(actual, expected, format);
        format!(
            "expected {expression} is equal to {expected:?}\n   but was: {marked_actual}\n  expected: {marked_expected}",
        )
    }
}

impl<S, E> Expectation<S> for IsNotEqualTo<E>
where
    S: PartialEq<E> + Debug,
    E: Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject != &self.expected
    }

    fn message(&self, expression: Expression<'_>, actual: &S, _format: &DiffFormat) -> String {
        format!(
            "expected {expression} is not equal to {:?}\n   but was: {actual:?}\n  expected: {:?}",
            &self.expected, &self.expected
        )
    }
}
