use crate::assertions::{AssertHasValue, AssertOption};
use crate::spec::{Assertion, AssertionStrategy, Subject, Unknown};
use crate::std::fmt::Debug;
#[cfg(not(any(feature = "std", test)))]
use alloc::format;

impl<'a, U, R> AssertOption<'a, R> for Subject<'a, Option<U>, R>
where
    U: 'a,
    Assertion<'a, Option<U>, Option<Unknown>, R>: AssertionStrategy<R>,
{
    fn is_some(self) -> R {
        if self.subject().is_some() {
            self.assertion_with("is some", Some(Unknown)).passed()
        } else {
            self.assertion_with("is some", Some(Unknown)).failed()
        }
    }

    fn is_none(self) -> R {
        if self.subject().is_none() {
            self.assertion_with("is none", None).passed()
        } else {
            self.assertion_with("is none", None).failed()
        }
    }
}

impl<'a, U, E, R> AssertHasValue<'a, E, R> for Subject<'a, Option<U>, R>
where
    E: Debug,
    U: 'a + PartialEq<E>,
    Assertion<'a, Option<U>, Option<E>, R>: AssertionStrategy<R>,
{
    fn has_value(self, expected: E) -> R {
        if self
            .subject()
            .as_ref()
            .is_some_and(|value| value == &expected)
        {
            self.assertion_with(format!("has some value {expected:?}"), Some(expected))
                .passed()
        } else {
            self.assertion_with(format!("has some value {expected:?}"), Some(expected))
                .failed()
        }
    }
}

#[cfg(test)]
mod tests;
