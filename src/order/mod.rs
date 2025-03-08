use crate::assertions::AssertOrder;
use crate::expectations::{IsAtLeast, IsAtMost, IsGreaterThan, IsLessThan};
use crate::spec::{Expectation, Expression, FailingStrategy, Spec};
use crate::std::fmt::Debug;
#[cfg(not(any(feature = "std", test)))]
use alloc::{format, string::String};

impl<S, E, R> AssertOrder<E> for Spec<'_, S, R>
where
    S: PartialOrd<E> + Debug,
    E: Debug,
    R: FailingStrategy,
{
    fn is_less_than(self, expected: E) -> Self {
        self.expecting(IsLessThan { expected })
    }

    fn is_greater_than(self, expected: E) -> Self {
        self.expecting(IsGreaterThan { expected })
    }

    fn is_at_most(self, expected: E) -> Self {
        self.expecting(IsAtMost { expected })
    }

    fn is_at_least(self, expected: E) -> Self {
        self.expecting(IsAtLeast { expected })
    }
}

impl<S, E> Expectation<S> for IsLessThan<E>
where
    S: PartialOrd<E> + Debug,
    E: Debug,
{
    fn test(&self, subject: &S) -> bool {
        subject < &self.expected
    }

    fn message(&self, expression: Expression<'_>, actual: &S) -> String {
        format!(
            "expected {expression} is less than {:?}\n   but was: {actual:?}\n  expected: < {:?}",
            self.expected, self.expected,
        )
    }
}

impl<S, E> Expectation<S> for IsAtMost<E>
where
    S: PartialOrd<E> + Debug,
    E: Debug,
{
    fn test(&self, subject: &S) -> bool {
        subject <= &self.expected
    }

    fn message(&self, expression: Expression<'_>, actual: &S) -> String {
        format!(
            "expected {expression} is at most {:?}\n   but was: {actual:?}\n  expected: <= {:?}",
            self.expected, self.expected,
        )
    }
}

impl<S, E> Expectation<S> for IsGreaterThan<E>
where
    S: PartialOrd<E> + Debug,
    E: Debug,
{
    fn test(&self, subject: &S) -> bool {
        subject > &self.expected
    }

    fn message(&self, expression: Expression<'_>, actual: &S) -> String {
        format!(
            "expected {expression} is greater than {:?}\n   but was: {actual:?}\n  expected: > {:?}",
            self.expected, self.expected,
        )
    }
}

impl<S, E> Expectation<S> for IsAtLeast<E>
where
    S: PartialOrd<E> + Debug,
    E: Debug,
{
    fn test(&self, subject: &S) -> bool {
        subject >= &self.expected
    }

    fn message(&self, expression: Expression<'_>, actual: &S) -> String {
        format!(
            "expected {expression} is at least {:?}\n   but was: {actual:?}\n  expected: >= {:?}",
            self.expected, self.expected,
        )
    }
}

#[cfg(test)]
mod tests;
