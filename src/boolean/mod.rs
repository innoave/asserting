//! Implementation of assertions for values of type `bool`.

use crate::assertions::AssertBoolean;
use crate::expectations::{IsFalse, IsTrue};
use crate::spec::{DiffFormat, Expectation, Expression, FailingStrategy, Spec};
#[cfg(not(feature = "std"))]
use alloc::{format, string::String};

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

    fn message(&self, expression: Expression<'_>, actual: &bool, _format: &DiffFormat) -> String {
        format!(
            "expected {expression} is {:?}\n   but was: {actual:?}\n  expected: {:?}",
            true, true
        )
    }
}

impl Expectation<bool> for IsFalse {
    fn test(&mut self, subject: &bool) -> bool {
        !*subject
    }

    fn message(&self, expression: Expression<'_>, actual: &bool, _format: &DiffFormat) -> String {
        format!(
            "expected {expression} is {:?}\n   but was: {actual:?}\n  expected: {:?}",
            false, false
        )
    }
}

#[cfg(test)]
mod tests;
