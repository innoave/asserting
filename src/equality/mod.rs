use crate::assertion::IsEqualTo;
use crate::specification::{Assertion, AssertionStrategy, Expected, Spec};

impl<'a, S, E, R> IsEqualTo<'a, E, R> for Spec<'a, S, R>
where
    S: 'a + PartialEq<E>,
    E: 'a,
    Assertion<'a, S, E, R>: AssertionStrategy<R>,
{
    fn is_equal_to(self, expected: impl Into<Expected<'a, E>>) -> R {
        let expected = expected.into();
        if self.subject().eq(&expected) {
            self.assertion_with(expected).passed()
        } else {
            self.assertion_with(expected).failed()
        }
    }

    fn is_not_equal_to(self, expected: impl Into<Expected<'a, E>>) -> R {
        let expected = expected.into();
        if self.subject().ne(&expected) {
            self.assertion_with(expected).passed()
        } else {
            self.assertion_with(expected).failed()
        }
    }
}

#[cfg(test)]
mod tests;
