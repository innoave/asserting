use crate::expectations::AssertEquality;
use crate::spec::{Expectation, Expression, FailingStrategy, Spec};
use crate::std::fmt::Debug;
#[cfg(not(any(feature = "std", test)))]
use alloc::{format, string::String};

impl<S, E, R> AssertEquality<E> for Spec<'_, S, R>
where
    S: PartialEq<E> + Debug,
    E: Debug,
    R: FailingStrategy,
{
    fn is_equal_to(self, expected: E) -> Self {
        self.expecting(IsEqualTo { expected })
    }

    fn is_not_equal_to(self, expected: E) -> Self {
        self.expecting(IsNotEqualTo { expected })
    }
}

struct IsEqualTo<E> {
    expected: E,
}

impl<S, E> Expectation<S> for IsEqualTo<E>
where
    S: PartialEq<E> + Debug,
    E: Debug,
{
    fn test(&self, subject: &S) -> bool {
        subject == &self.expected
    }

    fn message(&self, expression: Expression<'_>, actual: &S) -> String {
        format!(
            "expected {expression} is equal to {:?}\n   but was: {actual:?}\n  expected: {:?}",
            &self.expected, &self.expected
        )
    }
}

struct IsNotEqualTo<E> {
    expected: E,
}

impl<S, E> Expectation<S> for IsNotEqualTo<E>
where
    S: PartialEq<E> + Debug,
    E: Debug,
{
    fn test(&self, subject: &S) -> bool {
        subject != &self.expected
    }

    fn message(&self, expression: Expression<'_>, actual: &S) -> String {
        format!(
            "expected {expression} is not equal to {:?}\n   but was: {actual:?}\n  expected: {:?}",
            &self.expected, &self.expected
        )
    }
}

#[cfg(test)]
mod tests;
