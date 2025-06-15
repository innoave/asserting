use crate::expectations::Not;
use crate::spec::{DiffFormat, Expectation, Expression, Invertible};
use crate::std::string::String;

impl<S, E> Expectation<S> for Not<E>
where
    E: Invertible + Expectation<S>,
{
    fn test(&mut self, subject: &S) -> bool {
        !self.0.test(subject)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        self.0.message(expression, actual, !inverted, format)
    }
}

#[cfg(test)]
mod tests;
