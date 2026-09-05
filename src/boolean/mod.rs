//! Implementation of assertions for values of type `bool`.

use crate::assertions::AssertBoolean;
use crate::colored::{mark_missing, mark_unexpected};
use crate::expectations::{IsFalse, IsTrue, is_false, is_true};
use crate::spec::{
    DiffFormat, Expectation, Expecting, Expression, FailingStrategy, Invertible, Represent,
    Represented, Spec,
};
use crate::std::format;
use crate::std::string::String;

impl<D, R> AssertBoolean for Spec<'_, bool, D, R>
where
    D: Represent<bool>,
    R: FailingStrategy,
{
    fn is_true(self) -> Self {
        self.expecting(is_true())
    }

    fn is_false(self) -> Self {
        self.expecting(is_false())
    }
}

impl<D> Expectation<bool, D> for IsTrue
where
    D: Represent<bool>,
{
    fn test(&mut self, subject: &bool) -> bool {
        *subject
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &bool,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let marked_actual = mark_unexpected(actual, representation, format);
        let marked_expected = mark_missing(&!inverted, representation, format);
        let represented_expected = Represented::from((&true, representation));
        format!(
            "expected {expression} to be {represented_expected:?}\n   but was: {marked_actual}\n  expected: {marked_expected}",
        )
    }
}

impl Invertible for IsTrue {}

impl<D> Expectation<bool, D> for IsFalse
where
    D: Represent<bool>,
{
    fn test(&mut self, subject: &bool) -> bool {
        !*subject
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &bool,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let marked_actual = mark_unexpected(actual, representation, format);
        let marked_expected = mark_missing(&inverted, representation, format);
        let represented_expected = Represented::from((&false, representation));
        format!(
            "expected {expression} to be {represented_expected:?}\n   but was: {marked_actual}\n  expected: {marked_expected}",
        )
    }
}

impl Invertible for IsFalse {}

#[cfg(test)]
mod tests;
