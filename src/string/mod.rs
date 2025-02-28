use crate::assertions::AssertContains;
use crate::spec::{Assertion, AssertionStrategy, Subject};
#[cfg(not(any(feature = "std", test)))]
use alloc::{format, string::String};

// We implement `AssertContains` for different `Pattern` types as the
// `core::str::pattern` API is not stabilized as of February 2025;
// see issue [#27721](https://github.com/rust-lang/rust/issues/27721).
// Maybe we keep the implementations for a long time to support an earlier MSRV.

impl<'a, S, R> AssertContains<'a, &'a str, R> for Subject<'a, S, R>
where
    S: 'a + AsRef<str>,
    Assertion<'a, S, &'a str, R>: AssertionStrategy<R>,
{
    fn contains(self, pattern: &'a str) -> R {
        if self.subject().as_ref().contains(pattern) {
            self.assertion_with(format!("contains {pattern:?}"), pattern)
                .passed()
        } else {
            self.assertion_with(format!("contains {pattern:?}"), pattern)
                .failed()
        }
    }
}

impl<'a, S, R> AssertContains<'a, String, R> for Subject<'a, S, R>
where
    S: 'a + AsRef<str>,
    Assertion<'a, S, String, R>: AssertionStrategy<R>,
{
    fn contains(self, pattern: String) -> R {
        if self.subject().as_ref().contains(&pattern) {
            self.assertion_with(format!("contains {pattern:?}"), pattern)
                .passed()
        } else {
            self.assertion_with(format!("contains {pattern:?}"), pattern)
                .failed()
        }
    }
}

impl<'a, S, R> AssertContains<'a, &'a [char], R> for Subject<'a, S, R>
where
    S: 'a + AsRef<str>,
    Assertion<'a, S, &'a [char], R>: AssertionStrategy<R>,
{
    fn contains(self, pattern: &'a [char]) -> R {
        if self.subject().as_ref().contains(pattern) {
            self.assertion_with(format!("contains {pattern:?}"), pattern)
                .passed()
        } else {
            self.assertion_with(format!("contains {pattern:?}"), pattern)
                .failed()
        }
    }
}

impl<'a, S, R, const N: usize> AssertContains<'a, [char; N], R> for Subject<'a, S, R>
where
    S: 'a + AsRef<str>,
    Assertion<'a, S, [char; N], R>: AssertionStrategy<R>,
{
    fn contains(self, pattern: [char; N]) -> R {
        if self.subject().as_ref().contains(pattern) {
            self.assertion_with(format!("contains {pattern:?}"), pattern)
                .passed()
        } else {
            self.assertion_with(format!("contains {pattern:?}"), pattern)
                .failed()
        }
    }
}

impl<'a, S, R, const N: usize> AssertContains<'a, &'a [char; N], R> for Subject<'a, S, R>
where
    S: 'a + AsRef<str>,
    Assertion<'a, S, &'a [char; N], R>: AssertionStrategy<R>,
{
    fn contains(self, pattern: &'a [char; N]) -> R {
        if self.subject().as_ref().contains(pattern) {
            self.assertion_with(format!("contains {pattern:?}"), pattern)
                .passed()
        } else {
            self.assertion_with(format!("contains {pattern:?}"), pattern)
                .failed()
        }
    }
}

#[cfg(test)]
mod tests;
