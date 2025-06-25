//! Implementation of the equality assertions.

use crate::assertions::{AssertEquality, AssertHasDebugMessage, AssertHasDisplayMessage};
use crate::colored::{mark_diff, mark_diff_str};
use crate::expectations::{
    has_debug_message, has_display_message, is_equal_to, not, HasDebugMessage, HasDisplayMessage,
    IsEqualTo,
};
use crate::spec::{DiffFormat, Expectation, Expression, FailingStrategy, Invertible, Spec};
use crate::std::fmt::{Debug, Display};
use crate::std::format;
use crate::std::string::{String, ToString};

impl<S, E, R> AssertEquality<E> for Spec<'_, S, R>
where
    S: PartialEq<E> + Debug,
    E: Debug,
    R: FailingStrategy,
{
    fn is_equal_to(self, expected: E) -> Self {
        self.expecting(is_equal_to(expected))
    }

    fn is_not_equal_to(self, expected: E) -> Self {
        self.expecting(not(is_equal_to(expected)))
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

impl<S, E, R> AssertHasDebugMessage<E> for Spec<'_, S, R>
where
    S: Debug,
    E: AsRef<str>,
    R: FailingStrategy,
{
    fn has_debug_message(self, expected: E) -> Self {
        self.expecting(has_debug_message(expected))
    }

    fn does_not_have_debug_message(self, expected: E) -> Self {
        self.expecting(not(has_debug_message(expected)))
    }
}

impl<S, E> Expectation<S> for HasDebugMessage<E>
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
        format: &DiffFormat,
    ) -> String {
        let not = if inverted { "not " } else { "" };
        let expected = self.expected.as_ref();
        let (marked_actual, marked_expected) =
            mark_diff_str(&format!("{actual:?}"), expected, format);
        format!(
            "expected {expression} to {not}have debug message {expected:?}\n   but was: {marked_actual}\n  expected: {not}{marked_expected}",
        )
    }
}

impl<E> Invertible for HasDebugMessage<E> {}

impl<S, E, R> AssertHasDisplayMessage<E> for Spec<'_, S, R>
where
    S: Display,
    E: AsRef<str>,
    R: FailingStrategy,
{
    fn has_display_message(self, expected: E) -> Self {
        self.expecting(has_display_message(expected))
    }

    fn does_not_have_display_message(self, expected: E) -> Self {
        self.expecting(not(has_display_message(expected)))
    }
}

impl<S, E> Expectation<S> for HasDisplayMessage<E>
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
        format: &DiffFormat,
    ) -> String {
        let not = if inverted { "not " } else { "" };
        let expected = self.expected.as_ref();
        let (marked_actual, marked_expected) = mark_diff_str(&actual.to_string(), expected, format);
        format!(
            "expected {expression} to {not}have display message {expected:?}\n   but was: \"{marked_actual}\"\n  expected: {not}\"{marked_expected}\"",
        )
    }
}

impl<E> Invertible for HasDisplayMessage<E> {}
