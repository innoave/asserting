//! Implementation of the predicate assertion.

use crate::expectations::Predicate;
use crate::spec::{Expectation, Expression};
use crate::std::{format, string::String};

impl<S, P> Expectation<S> for Predicate<P>
where
    P: Fn(&S) -> bool,
{
    fn test(&mut self, subject: &S) -> bool {
        (self.predicate)(subject)
    }

    fn message(&self, expression: Expression<'_>, _actual: &S) -> String {
        self.message.clone().unwrap_or_else(|| {
            format!("expected {expression} to satisfy the given predicate, but returned false")
        })
    }
}

#[cfg(test)]
mod tests;
