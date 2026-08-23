//! Implementation of the predicate assertion.

use crate::expectations::Predicate;
use crate::spec::{DiffFormat, Expectation, Expression, Invertible, Represent, Represented};
use crate::std::{format, string::String};

impl<S, P, D> Expectation<S, D> for Predicate<P>
where
    P: Fn(&S) -> bool,
    D: Represent<S>,
{
    fn test(&mut self, subject: &S) -> bool {
        (self.predicate)(subject)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        representation: &D,
        _format: &DiffFormat,
    ) -> String {
        let represented_actual = Represented::from((actual, representation));
        self.message.clone().unwrap_or_else(|| {
            format!("expected {expression} to satisfy the given predicate, but returned {inverted}\n  actual: {represented_actual}")
        })
    }
}

impl<P> Invertible for Predicate<P> {}

#[cfg(test)]
mod tests;
