use crate::assertions::AssertIteratorContains;
use crate::expectations::IterContains;
use crate::spec::{Expectation, Expression, FailingStrategy, Spec};
use crate::std::fmt::Debug;
#[cfg(not(any(feature = "std", test)))]
use alloc::{format, string::String, vec::Vec};

impl<'a, S, T, E, R> AssertIteratorContains<'a, Vec<T>, E, R> for Spec<'a, S, R>
where
    S: IntoIterator<Item = T>,
    T: PartialEq<E> + Debug,
    E: Debug,
    R: FailingStrategy,
{
    fn contains(self, expected: E) -> Spec<'a, Vec<T>, R> {
        self.map(Vec::from_iter)
            .expecting(IterContains { expected })
    }
}

impl<T, E> Expectation<Vec<T>> for IterContains<E>
where
    T: PartialEq<E> + Debug,
    E: Debug,
{
    fn test(&self, subject: &Vec<T>) -> bool {
        subject.iter().any(|e| e == &self.expected)
    }

    fn message(&self, expression: Expression<'_>, actual: &Vec<T>) -> String {
        format!(
            "expected {expression} to contain {:?}\n   but was: {actual:?}\n  expected: {:?}",
            &self.expected, &self.expected
        )
    }
}

#[cfg(test)]
mod tests;
