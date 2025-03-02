use crate::assertions::{AssertContains, AssertContainsAnyOf, AssertEmptiness, AssertHasLength};
use crate::expectations::{Contains, ContainsAnyOf, HasLength, IsEmpty, IsNotEmpty};
use crate::spec::{Expectation, Expression, FailingStrategy, Spec};
use crate::std::fmt::Debug;
#[cfg(not(any(feature = "std", test)))]
use alloc::{format, string::String};

impl<S, R> AssertEmptiness for Spec<'_, S, R>
where
    S: AsRef<str> + Debug,
    R: FailingStrategy,
{
    fn is_empty(self) -> Self {
        self.expecting(IsEmpty)
    }

    fn is_not_empty(self) -> Self {
        self.expecting(IsNotEmpty)
    }
}

impl<S> Expectation<S> for IsEmpty
where
    S: AsRef<str> + Debug,
{
    fn test(&self, subject: &S) -> bool {
        subject.as_ref().is_empty()
    }

    fn message(&self, expression: Expression<'_>, actual: &S) -> String {
        format!("expected {expression} is empty\n   but was: {actual:?}\n  expected: <empty>")
    }
}

impl<S> Expectation<S> for IsNotEmpty
where
    S: AsRef<str> + Debug,
{
    fn test(&self, subject: &S) -> bool {
        !subject.as_ref().is_empty()
    }

    fn message(&self, expression: Expression<'_>, actual: &S) -> String {
        format!(
            "expected {expression} is not empty\n   but was: {actual:?}\n  expected: <non-empty>",
        )
    }
}

impl<S, R> AssertHasLength for Spec<'_, S, R>
where
    S: AsRef<str>,
    R: FailingStrategy,
{
    fn has_length(self, expected: usize) -> Self {
        self.expecting(HasLength {
            expected_length: expected,
        })
    }
}

impl<S> Expectation<S> for HasLength
where
    S: AsRef<str>,
{
    fn test(&self, subject: &S) -> bool {
        subject.as_ref().len() == self.expected_length
    }

    fn message(&self, expression: Expression<'_>, actual: &S) -> String {
        format!(
            "expected {expression} has length {}\n   but was: {}\n  expected: {}",
            self.expected_length,
            actual.as_ref().len(),
            self.expected_length
        )
    }
}

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
        self.expecting(Contains { expected: pattern })
    }
}

impl<'a, S, R> AssertContains<String> for Spec<'a, S, R>
where
    S: 'a + AsRef<str> + Debug,
    R: FailingStrategy,
{
    fn contains(self, pattern: String) -> Self {
        self.expecting(Contains { expected: pattern })
    }
}

impl<'a, S, R> AssertContains<char> for Spec<'a, S, R>
where
    S: 'a + AsRef<str> + Debug,
    R: FailingStrategy,
{
    fn contains(self, expected: char) -> Self {
        self.expecting(Contains { expected })
    }
}

impl<S> Expectation<S> for Contains<&str>
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

impl<S> Expectation<S> for Contains<String>
where
    S: AsRef<str> + Debug,
{
    fn test(&self, subject: &S) -> bool {
        subject.as_ref().contains(&self.expected)
    }

    fn message(&self, expression: Expression<'_>, actual: &S) -> String {
        format!(
            "expected {expression} to contain {:?}\n   but was: {actual:?}\n  expected: {:?}",
            self.expected, self.expected
        )
    }
}

impl<S> Expectation<S> for Contains<char>
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
        self.expecting(ContainsAnyOf { expected })
    }
}

impl<'a, S, R, const N: usize> AssertContainsAnyOf<[char; N]> for Spec<'a, S, R>
where
    S: 'a + AsRef<str> + Debug,
    R: FailingStrategy,
{
    fn contains_any_of(self, expected: [char; N]) -> Self {
        self.expecting(ContainsAnyOf { expected })
    }
}

impl<'a, S, R, const N: usize> AssertContainsAnyOf<&'a [char; N]> for Spec<'a, S, R>
where
    S: 'a + AsRef<str> + Debug,
    R: FailingStrategy,
{
    fn contains_any_of(self, expected: &'a [char; N]) -> Self {
        self.expecting(ContainsAnyOf { expected })
    }
}

impl<S> Expectation<S> for ContainsAnyOf<&[char]>
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

impl<S, const N: usize> Expectation<S> for ContainsAnyOf<[char; N]>
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

impl<S, const N: usize> Expectation<S> for ContainsAnyOf<&[char; N]>
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
