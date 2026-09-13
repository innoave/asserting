//! Implementation of the equality assertions.

use crate::assertions::{
    AssertEquality, AssertHasDebugString, AssertHasDisplayString, AssertIsIn, AssertSameAs,
};
use crate::colored::{
    mark_all_items_in_collection, mark_diff, mark_diff_str, mark_missing, mark_unexpected,
};
use crate::expectations::{
    HasDebugString, HasDisplayString, IsEqualTo, IsIn, IsSameAs, has_debug_string,
    has_display_string, is_equal_to, is_in, is_same_as, not,
};
use crate::spec::{
    DiffFormat, Expectation, Expecting, Expression, FailingStrategy, Invertible, Represent,
    Represented, Spec,
};
use crate::std::{
    fmt::{Debug, Display},
    format,
    string::{String, ToString},
    vec::Vec,
};

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

impl<S, I, E, D, R> AssertIsIn<I, E> for Spec<'_, S, D, R>
where
    I: IntoIterator<Item = E>,
    S: PartialEq<E>,
    D: Represent<S> + Represent<E>,
    R: FailingStrategy,
{
    fn is_in(self, expected_values: I) -> Self {
        self.expecting(is_in(expected_values))
    }
}

impl<S, E, D> Expectation<S, D> for IsIn<E>
where
    S: PartialEq<E>,
    D: Represent<S> + Represent<E>,
{
    fn test(&mut self, subject: &S) -> bool {
        self.expected_values
            .iter()
            .any(|expected| subject == expected)
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
        let expected_values = self
            .expected_values
            .iter()
            .map(|expected| Represented::from((expected, representation)))
            .collect::<Vec<_>>();
        let marked_actual = mark_unexpected(actual, representation, format);
        let marked_expected = mark_all_items_in_collection(
            &self.expected_values,
            representation,
            format,
            mark_missing,
        );
        format!(
            r"expected {expression} to {not}be in {expected_values:?}
          but was: {marked_actual}
  which is not in: {marked_expected}",
        )
    }
}

impl<E> Invertible for IsIn<E> {}
