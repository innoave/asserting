use crate::assertions::AssertEquality;
use crate::spec::{Assertion, AssertionStrategy, Subject};
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

#[cfg(test)]
mod tests;
