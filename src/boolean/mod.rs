//! Implementation of assertions for values of type `bool`.

use crate::assertions::AssertBoolean;
use crate::colored::{mark_missing, mark_unexpected};
use crate::expectations::{IsFalse, IsTrue};
use crate::spec::{DiffFormat, Expectation, Expression, FailingStrategy, Spec};
use crate::std::format;
use crate::std::string::String;

impl<R> AssertBoolean for Spec<'_, bool, R>
where
    R: FailingStrategy,
{
    fn is_true(self) -> Self {
        self.expecting(IsTrue)
    }

    fn is_false(self) -> Self {
        self.expecting(IsFalse)
    }
}

impl Expectation<bool> for IsTrue {
    fn test(&mut self, subject: &bool) -> bool {
        *subject
    }

    fn message(&self, expression: Expression<'_>, actual: &bool, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing(&true, format);
        format!(
            "expected {expression} is {:?}\n   but was: {marked_actual}\n  expected: {marked_expected}",
            true
        )
    }
}

impl Expectation<bool> for IsFalse {
    fn test(&mut self, subject: &bool) -> bool {
        !*subject
    }

    fn message(&self, expression: Expression<'_>, actual: &bool, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing(&false, format);
        format!(
            "expected {expression} is {:?}\n   but was: {marked_actual}\n  expected: {marked_expected}",
            false
        )
    }
}

#[cfg(test)]
mod tests;
