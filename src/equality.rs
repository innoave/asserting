//! Implementation of the equality assertions.

use crate::assertions::AssertEquality;
use crate::colored::mark_diff;
use crate::expectations::{IsEqualTo, Not};
use crate::spec::{DiffFormat, Expectation, Expression, FailingStrategy, Invertible, Spec};
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
        self.expecting(Not(IsEqualTo { expected }))
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

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let not = if inverted { "not " } else { "" };
        let expected = &self.expected;
        let (marked_actual, marked_expected) = mark_diff(actual, expected, format);
        format!(
            "expected {expression} to be {not}equal to {expected:?}\n   but was: {marked_actual}\n  expected: {not}{marked_expected}",
        )
    }
}

impl<E> Invertible for IsEqualTo<E> {}
