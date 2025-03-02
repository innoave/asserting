use crate::expectations::Predicate;
use crate::spec::{Expectation, Expression};
#[cfg(not(any(feature = "std", test)))]
use alloc::{format, string::String};

impl<S, P> Expectation<S> for Predicate<P>
where
    P: Fn(&S) -> bool,
{
    fn test(&self, subject: &S) -> bool {
        (self.predicate)(subject)
    }

    fn message(&self, expression: Expression<'_>, _actual: &S) -> String {
        self.message.clone().unwrap_or_else(|| {
            format!("expected {expression} to satisfy predicate, but returned false")
        })
    }
}
