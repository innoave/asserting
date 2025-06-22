//! Implementation of assertions for values of type `bool`.

use crate::assertions::AssertBoolean;
use crate::colored::{mark_missing, mark_unexpected};
use crate::expectations::{is_false, is_true, IsFalse, IsTrue};
use crate::spec::{DiffFormat, Expectation, Expression, FailingStrategy, Invertible, Spec};
use crate::std::format;
use crate::std::string::String;

impl<R> AssertBoolean for Spec<'_, bool, R>
where
    R: FailingStrategy,
{
    fn is_true(self) -> Self {
        self.expecting(is_true())
    }

    fn is_false(self) -> Self {
        self.expecting(is_false())
    }
}

impl Expectation<bool> for IsTrue {
    fn test(&mut self, subject: &bool) -> bool {
        *subject
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &bool,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let marked_actual = mark_unexpected(&actual, format);
        let marked_expected = mark_missing(&!inverted, format);
        format!(
            "expected {expression} to be {:?}\n   but was: {marked_actual}\n  expected: {marked_expected}",
            true
        )
    }
}

impl Invertible for IsTrue {}

impl Expectation<bool> for IsFalse {
    fn test(&mut self, subject: &bool) -> bool {
        !*subject
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &bool,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing(&inverted, format);
        format!(
            "expected {expression} to be {:?}\n   but was: {marked_actual}\n  expected: {marked_expected}",
            false
        )
    }
}

impl Invertible for IsFalse {}

#[cfg(test)]
mod tests;
