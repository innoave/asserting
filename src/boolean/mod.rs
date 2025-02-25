use crate::assertions::IsTrue;
use crate::spec::{Assertion, AssertionStrategy, Subject};

impl<'a, R> IsTrue<R> for Subject<'a, bool, R>
where
    Assertion<'a, bool, bool, R>: AssertionStrategy<R>,
{
    fn is_true(self) -> R {
        if *self.subject() {
            self.assertion_with("is true", true).passed()
        } else {
            self.assertion_with("is true", true).failed()
        }
    }

    fn is_false(self) -> R {
        if *self.subject() {
            self.assertion_with("is false", true).failed()
        } else {
            self.assertion_with("is false", true).passed()
        }
    }
}

#[cfg(test)]
mod tests;
