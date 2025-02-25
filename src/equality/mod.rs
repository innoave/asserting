use crate::assertions::IsEqualTo;
use crate::spec::{Assertion, AssertionStrategy, Subject};

impl<'a, S, E, R> IsEqualTo<'a, E, R> for Subject<'a, S, R>
where
    S: 'a + PartialEq<E>,
    E: 'a,
    Assertion<'a, S, E, R>: AssertionStrategy<R>,
{
    fn is_equal_to(self, expected: E) -> R {
        if self.subject().eq(&expected) {
            self.assertion_with("is equal to", expected).passed()
        } else {
            self.assertion_with("is equal to", expected).failed()
        }
    }

    fn is_not_equal_to(self, expected: E) -> R {
        if self.subject().ne(&expected) {
            self.assertion_with("is not equal to", expected).passed()
        } else {
            self.assertion_with("is not equal to", expected).failed()
        }
    }
}

#[cfg(test)]
mod tests;
