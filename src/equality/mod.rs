use crate::assertions::AssertEquality;
use crate::spec::{Assertion, AssertionStrategy, Expectation, FailingStrategy, Spec, Subject};
#[cfg(not(any(feature = "std", test)))]
use alloc::format;
use core::fmt::Debug;

impl<'a, S, E, R> AssertEquality<'a, E, R> for Subject<'a, S, R>
where
    S: 'a + PartialEq<E>,
    E: 'a + Debug,
    Assertion<'a, S, E, R>: AssertionStrategy<R>,
{
    fn is_equal_to(self, expected: E) -> R {
        if self.subject() == &expected {
            self.assertion_with(format!("is equal to {expected:?}"), expected)
                .passed()
        } else {
            self.assertion_with(format!("is equal to {expected:?}"), expected)
                .failed()
        }
    }

    fn is_not_equal_to(self, expected: E) -> R {
        if self.subject() != &expected {
            self.assertion_with(format!("is not equal to {expected:?}"), expected)
                .passed()
        } else {
            self.assertion_with(format!("is not equal to {expected:?}"), expected)
                .failed()
        }
    }
}

impl<'a, S, E, R> crate::expectations::AssertEquality<'a, E> for Spec<'a, S, R>
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

    fn message(&self, subject_name: Option<&str>, subject: &S) -> String {
        let subject_name = subject_name.unwrap_or("subject");
        format!(
            "expected {subject_name} is equal to {:?}\n   but was: {subject:?}\n  expected: {:?}",
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

    fn message(&self, subject_name: Option<&str>, subject: &S) -> String {
        let subject_name = subject_name.unwrap_or("subject");
        format!(
            "expected {subject_name} is not equal to {:?}\n   but was: {subject:?}\n  expected: {:?}",
            &self.expected, &self.expected
        )
    }
}

#[cfg(test)]
mod tests;
