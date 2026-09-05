//! Implementation of the equality assertions.

use crate::assertions::{
    AssertEquality, AssertHasDebugString, AssertHasDisplayString, AssertSameAs,
};
use crate::colored::{mark_diff, mark_diff_str};
use crate::expectations::{
    HasDebugString, HasDisplayString, IsEqualTo, IsSameAs, has_debug_string, has_display_string,
    is_equal_to, is_same_as, not,
};
use crate::spec::{
    DiffFormat, Expectation, Expecting, Expression, FailingStrategy, Invertible, Represent,
    Represented, Spec,
};
use crate::std::fmt::{Debug, Display};
use crate::std::format;
use crate::std::string::{String, ToString};

impl<S, E, D, R> AssertEquality<E> for Spec<'_, S, D, R>
where
    S: PartialEq<E>,
    D: Represent<S> + Represent<E>,
    R: FailingStrategy,
{
    fn is_equal_to(self, expected: E) -> Self {
        self.expecting(is_equal_to(expected))
    }

    fn is_not_equal_to(self, expected: E) -> Self {
        self.expecting(not(is_equal_to(expected)))
    }
}

impl<S, D, E> Expectation<S, D> for IsEqualTo<E>
where
    S: PartialEq<E>,
    D: Represent<S> + Represent<E>,
{
    fn test(&mut self, subject: &S) -> bool {
        subject == &self.expected
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
        let expected = &self.expected;
        let represented_expected = Represented::from((expected, representation));
        let (marked_actual, marked_expected) = mark_diff(actual, expected, representation, format);
        format!(
            "expected {expression} to be {not}equal to {represented_expected:?}\n   but was: {marked_actual}\n  expected: {not}{marked_expected}",
        )
    }
}

impl<E> Invertible for IsEqualTo<E> {}

impl<S, D, R> AssertSameAs<S> for Spec<'_, S, D, R>
where
    S: PartialEq,
    R: FailingStrategy,
    D: Represent<S>,
{
    fn is_same_as(self, expected: S) -> Self {
        self.expecting(is_same_as(expected))
    }

    fn is_not_same_as(self, expected: S) -> Self {
        self.expecting(not(is_same_as(expected)))
    }
}

impl<S, D> Expectation<S, D> for IsSameAs<S>
where
    S: PartialEq,
    D: Represent<S>,
{
    fn test(&mut self, subject: &S) -> bool {
        subject == &self.expected
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
        let expected = &self.expected;
        let represented_expected = Represented::from((expected, representation));
        let (marked_actual, marked_expected) = mark_diff(actual, expected, representation, format);
        format!(
            "expected {expression} to be {not}the same as {represented_expected:?}\n   but was: {marked_actual}\n  expected: {not}{marked_expected}",
        )
    }
}

impl<E> Invertible for IsSameAs<E> {}

impl<S, E, D, R> AssertHasDebugString<E> for Spec<'_, S, D, R>
where
    S: Debug,
    E: AsRef<str>,
    R: FailingStrategy,
{
    fn has_debug_string(self, expected: E) -> Self {
        self.expecting(has_debug_string(expected))
    }

    fn does_not_have_debug_string(self, expected: E) -> Self {
        self.expecting(not(has_debug_string(expected)))
    }
}

impl<S, E, D> Expectation<S, D> for HasDebugString<E>
where
    S: Debug,
    E: AsRef<str>,
{
    fn test(&mut self, subject: &S) -> bool {
        format!("{subject:?}") == self.expected.as_ref()
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        _representation: &D,
        format: &DiffFormat,
    ) -> String {
        let not = if inverted { "not " } else { "" };
        let expected = self.expected.as_ref();
        let (marked_actual, marked_expected) =
            mark_diff_str(&format!("{actual:?}"), expected, format);
        format!(
            "expected {expression} to {not}have a debug string equal to {expected:?}\n   but was: {marked_actual}\n  expected: {not}{marked_expected}",
        )
    }
}

impl<E> Invertible for HasDebugString<E> {}

impl<S, E, D, R> AssertHasDisplayString<E> for Spec<'_, S, D, R>
where
    S: Display,
    E: AsRef<str>,
    R: FailingStrategy,
{
    fn has_display_string(self, expected: E) -> Self {
        self.expecting(has_display_string(expected))
    }

    fn does_not_have_display_string(self, expected: E) -> Self {
        self.expecting(not(has_display_string(expected)))
    }
}

impl<S, E, D> Expectation<S, D> for HasDisplayString<E>
where
    S: Display,
    E: AsRef<str>,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.to_string() == self.expected.as_ref()
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        _representation: &D,
        format: &DiffFormat,
    ) -> String {
        let not = if inverted { "not " } else { "" };
        let expected = self.expected.as_ref();
        let (marked_actual, marked_expected) = mark_diff_str(&actual.to_string(), expected, format);
        format!(
            "expected {expression} to {not}have a display string equal to {expected:?}\n   but was: \"{marked_actual}\"\n  expected: {not}\"{marked_expected}\"",
        )
    }
}

impl<E> Invertible for HasDisplayString<E> {}
