use crate::assertions::{AssertHasValue, AssertOption};
use crate::prelude::{Assertion, AssertionStrategy};
use crate::spec::Subject;

impl<'a, U, R> AssertOption<'a, R> for Subject<'a, Option<U>, R>
where
    Assertion<'a, Option<U>, (), R>: AssertionStrategy<R>,
{
    fn is_some(self) -> R {
        if self.subject().is_some() {
            self.assertion_with("is some", ()).passed()
        } else {
            self.assertion_with("is some", ()).failed()
        }
    }

    fn is_none(self) -> R {
        if self.subject().is_none() {
            self.assertion_with("is none", ()).passed()
        } else {
            self.assertion_with("is none", ()).failed()
        }
    }
}

impl<'a, U, E, R> AssertHasValue<'a, E, R> for Subject<'a, Option<U>, R>
where
    U: 'a + PartialEq<E>,
    Assertion<'a, Option<U>, E, R>: AssertionStrategy<R>,
{
    fn has_value(self, expected: E) -> R {
        if self
            .subject()
            .as_ref()
            .is_some_and(|value| value == &expected)
        {
            self.assertion_with("has some value", expected).passed()
        } else {
            self.assertion_with("has none value", expected).failed()
        }
    }
}

#[cfg(test)]
mod tests;
