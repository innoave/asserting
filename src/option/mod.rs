//! Implementation of assertions for `Option` values.

use crate::assertions::{AssertHasValue, AssertOption, AssertOptionValue};
use crate::colored::{mark_missing, mark_unexpected};
use crate::expectations::{HasValue, IsNone, IsSome, has_value, is_none, is_some};
use crate::spec::{
    DiffFormat, Expectation, Expecting, Expression, FailingStrategy, Invertible, Represent,
    Represented, RepresentedBy, Spec, Unknown,
};
use crate::std::{format, string::String};

impl<T, D, R> AssertOption for Spec<'_, Option<T>, D, R>
where
    D: Represent<Option<T>> + Represent<Option<Unknown>>,
    R: FailingStrategy,
{
    fn is_some(self) -> Self {
        self.expecting(is_some())
    }

    fn is_none(self) -> Self {
        self.expecting(is_none())
    }
}

impl<T, D, R> AssertOption for Spec<'_, &Option<T>, D, R>
where
    D: Represent<Option<T>> + Represent<Option<Unknown>>,
    R: FailingStrategy,
{
    fn is_some(self) -> Self {
        self.expecting(is_some())
    }

    fn is_none(self) -> Self {
        self.expecting(is_none())
    }
}

impl<'a, T, D, R> AssertOptionValue for Spec<'a, Option<T>, D, R>
where
    D: Represent<T> + Clone,
    R: FailingStrategy,
{
    type Some = Spec<'a, T, D, R>;

    fn some(self) -> Self::Some {
        let value_representation = self.representation().clone();
        self.mapping(|subject| match subject {
            None => {
                panic!("expected the subject to be `Some(_)`, but was `None`")
            },
            Some(value) => value,
        })
        .represented_by(value_representation)
    }
}

impl<'a, T, D, R> AssertOptionValue for Spec<'a, &'a Option<T>, D, R>
where
    D: Represent<T> + Clone,
    R: FailingStrategy,
{
    type Some = Spec<'a, &'a T, D, R>;

    fn some(self) -> Self::Some {
        let value_representation = self.representation().clone();
        self.mapping(|subject| match subject {
            None => {
                panic!("expected the subject to be `Some(_)`, but was `None`")
            },
            Some(value) => value,
        })
        .represented_by(value_representation)
    }
}

impl<T, E, D, R> AssertHasValue<E> for Spec<'_, Option<T>, D, R>
where
    T: PartialEq<E>,
    D: Represent<Option<T>> + Represent<Option<E>> + Represent<E>,
    R: FailingStrategy,
{
    fn has_value(self, expected: E) -> Self {
        self.expecting(has_value(expected))
    }
}

impl<T, E, D, R> AssertHasValue<E> for Spec<'_, &Option<T>, D, R>
where
    T: PartialEq<E>,
    D: Represent<Option<T>> + Represent<Option<E>> + Represent<E>,
    R: FailingStrategy,
{
    fn has_value(self, expected: E) -> Self {
        self.expecting(has_value(expected))
    }
}

impl<T, D> Expectation<Option<T>, D> for IsSome
where
    D: Represent<Option<T>> + Represent<Option<Unknown>>,
{
    fn test(&mut self, subject: &Option<T>) -> bool {
        subject.is_some()
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &Option<T>,
        _inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let expected = Some(Unknown);
        let marked_actual = mark_unexpected(actual, representation, format);
        let marked_expected = mark_missing(&expected, representation, format);
        format!(
            "expected {expression} to be {expected:?}\n   but was: {marked_actual}\n  expected: {marked_expected}"
        )
    }
}

impl<T, D> Expectation<&Option<T>, D> for IsSome
where
    D: Represent<Option<T>> + Represent<Option<Unknown>>,
{
    fn test(&mut self, subject: &&Option<T>) -> bool {
        <Self as Expectation<Option<T>, D>>::test(self, subject)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &&Option<T>,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        <Self as Expectation<Option<T>, D>>::message(
            self,
            expression,
            actual,
            inverted,
            representation,
            format,
        )
    }
}

impl<T, D> Expectation<Option<T>, D> for IsNone
where
    D: Represent<Option<T>> + Represent<Option<Unknown>>,
{
    fn test(&mut self, subject: &Option<T>) -> bool {
        subject.is_none()
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &Option<T>,
        _inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let expected = None::<Unknown>;
        let marked_actual = mark_unexpected(actual, representation, format);
        let marked_expected = mark_missing(&expected, representation, format);
        format!(
            "expected {expression} to be {expected:?}\n   but was: {marked_actual}\n  expected: {marked_expected}"
        )
    }
}

impl<T, D> Expectation<&Option<T>, D> for IsNone
where
    D: Represent<Option<T>> + Represent<Option<Unknown>>,
{
    fn test(&mut self, subject: &&Option<T>) -> bool {
        <Self as Expectation<Option<T>, D>>::test(self, subject)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &&Option<T>,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        <Self as Expectation<Option<T>, D>>::message(
            self,
            expression,
            actual,
            inverted,
            representation,
            format,
        )
    }
}

impl<T, E, D> Expectation<Option<T>, D> for HasValue<E>
where
    T: PartialEq<E>,
    D: Represent<Option<T>> + Represent<Option<E>> + Represent<E>,
{
    fn test(&mut self, subject: &Option<T>) -> bool {
        subject
            .as_ref()
            .is_some_and(|value| value == &self.expected)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &Option<T>,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let not = if inverted { "not " } else { "" };
        let expected = &self.expected;
        let marked_actual = mark_unexpected(actual, representation, format);
        let marked_expected = mark_missing(expected, representation, format);
        let represented_expected = Represented::from((expected, representation));
        format!(
            "expected {expression} to be some {not}containing {represented_expected:?}\n   but was: {marked_actual}\n  expected: {not}Some({marked_expected}))"
        )
    }
}

impl<E> Invertible for HasValue<E> {}

impl<T, E, D> Expectation<&Option<T>, D> for HasValue<E>
where
    T: PartialEq<E>,
    D: Represent<Option<T>> + Represent<Option<E>> + Represent<E>,
{
    fn test(&mut self, subject: &&Option<T>) -> bool {
        <Self as Expectation<Option<T>, D>>::test(self, subject)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &&Option<T>,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        <Self as Expectation<Option<T>, D>>::message(
            self,
            expression,
            actual,
            inverted,
            representation,
            format,
        )
    }
}

#[cfg(test)]
mod tests;
