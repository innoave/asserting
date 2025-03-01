use crate::expectations::{AssertContains, AssertContainsAnyOf};
use crate::spec::{Expectation, Expression, FailingStrategy, Spec};
use crate::std::fmt::Debug;
#[cfg(not(any(feature = "std", test)))]
use alloc::{format, string::String};

// We implement `AssertContains` for different `Pattern` types as the
// [`core::str::pattern`] API is not stabilized as of February 2025;
// see issue [#27721](https://github.com/rust-lang/rust/issues/27721).
// Maybe we keep the implementations for a long time to support an earlier MSRV.

impl<'a, S, R> AssertContains<&'a str> for Spec<'a, S, R>
where
    S: 'a + AsRef<str> + Debug,
    R: FailingStrategy,
{
    fn contains(self, pattern: &'a str) -> Self {
        self.expecting(ContainsStrPattern { pattern })
    }
}

impl<'a, S, R> AssertContains<String> for Spec<'a, S, R>
where
    S: 'a + AsRef<str> + Debug,
    R: FailingStrategy,
{
    fn contains(self, pattern: String) -> Self {
        self.expecting(ContainsStringPattern { pattern })
    }
}

impl<'a, S, R> AssertContains<char> for Spec<'a, S, R>
where
    S: 'a + AsRef<str> + Debug,
    R: FailingStrategy,
{
    fn contains(self, expected: char) -> Self {
        self.expecting(ContainsChar { expected })
    }
}

struct ContainsStrPattern<'a> {
    pattern: &'a str,
}

impl<S> Expectation<S> for ContainsStrPattern<'_>
where
    S: AsRef<str> + Debug,
{
    fn test(&self, subject: &S) -> bool {
        subject.as_ref().contains(self.pattern)
    }

    fn message(&self, expression: Expression<'_>, actual: &S) -> String {
        format!(
            "expected {expression} to contain {:?}\n   but was: {actual:?}\n  expected: {:?}",
            self.pattern, self.pattern
        )
    }
}

struct ContainsStringPattern {
    pattern: String,
}

impl<S> Expectation<S> for ContainsStringPattern
where
    S: AsRef<str> + Debug,
{
    fn test(&self, subject: &S) -> bool {
        subject.as_ref().contains(&self.pattern)
    }

    fn message(&self, expression: Expression<'_>, actual: &S) -> String {
        format!(
            "expected {expression} to contain {:?}\n   but was: {actual:?}\n  expected: {:?}",
            self.pattern, self.pattern
        )
    }
}

struct ContainsChar {
    expected: char,
}

impl<S> Expectation<S> for ContainsChar
where
    S: AsRef<str> + Debug,
{
    fn test(&self, subject: &S) -> bool {
        subject.as_ref().contains(self.expected)
    }

    fn message(&self, expression: Expression<'_>, actual: &S) -> String {
        format!(
            "expected {expression} to contain {:?}\n   but was: {actual:?}\n  expected: {:?}",
            self.expected, self.expected
        )
    }
}

// When string slices' `contains` function is used with an array of chars or
// slice of chars it checks if any of the chars in the array/slice is contained
// in the string slice. Therefore, we implement the [`AssertContainsAnyOf`]
// assertion for array/slice of chars as expected value, but not the
// [`AssertContains`] assertion.

impl<'a, S, R> AssertContainsAnyOf<&'a [char]> for Spec<'a, S, R>
where
    S: 'a + AsRef<str> + Debug,
    R: FailingStrategy,
{
    fn contains_any_of(self, expected: &'a [char]) -> Self {
        self.expecting(ContainsAnyOfCharSlice { expected })
    }
}

impl<'a, S, R, const N: usize> AssertContainsAnyOf<[char; N]> for Spec<'a, S, R>
where
    S: 'a + AsRef<str> + Debug,
    R: FailingStrategy,
{
    fn contains_any_of(self, expected: [char; N]) -> Self {
        self.expecting(ContainsAnyOfCharArray { expected })
    }
}

impl<'a, S, R, const N: usize> AssertContainsAnyOf<&'a [char; N]> for Spec<'a, S, R>
where
    S: 'a + AsRef<str> + Debug,
    R: FailingStrategy,
{
    fn contains_any_of(self, expected: &'a [char; N]) -> Self {
        self.expecting(ContainsAnyOfBorrowedCharArray { expected })
    }
}

struct ContainsAnyOfCharSlice<'a> {
    expected: &'a [char],
}

impl<S> Expectation<S> for ContainsAnyOfCharSlice<'_>
where
    S: AsRef<str> + Debug,
{
    fn test(&self, subject: &S) -> bool {
        subject.as_ref().contains(self.expected)
    }

    fn message(&self, expression: Expression<'_>, actual: &S) -> String {
        format!(
            "expected {expression} to contain any of {:?}\n   but was: {actual:?}\n  expected: {:?}",
            self.expected, self.expected
        )
    }
}

struct ContainsAnyOfCharArray<const N: usize> {
    expected: [char; N],
}

impl<S, const N: usize> Expectation<S> for ContainsAnyOfCharArray<N>
where
    S: AsRef<str> + Debug,
{
    fn test(&self, subject: &S) -> bool {
        subject.as_ref().contains(self.expected)
    }

    fn message(&self, expression: Expression<'_>, actual: &S) -> String {
        format!(
            "expected {expression} to contain any of {:?}\n   but was: {actual:?}\n  expected: {:?}",
            self.expected, self.expected
        )
    }
}

struct ContainsAnyOfBorrowedCharArray<'a, const N: usize> {
    expected: &'a [char; N],
}

impl<S, const N: usize> Expectation<S> for ContainsAnyOfBorrowedCharArray<'_, N>
where
    S: AsRef<str> + Debug,
{
    fn test(&self, subject: &S) -> bool {
        subject.as_ref().contains(self.expected)
    }

    fn message(&self, expression: Expression<'_>, actual: &S) -> String {
        format!(
            "expected {expression} to contain any of {:?}\n   but was: {actual:?}\n  expected: {:?}",
            self.expected, self.expected
        )
    }
}

#[cfg(test)]
mod tests;
