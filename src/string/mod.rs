use crate::assertions::{AssertStringContainsAnyOf, AssertStringPattern};
use crate::expectations::{StringContains, StringContainsAnyOf, StringEndsWith, StringStartWith};
use crate::prelude::DefinedOrder;
use crate::properties::{IsEmptyProperty, LengthProperty};
use crate::spec::{Expectation, Expression, FailingStrategy, Spec};
use crate::std::fmt::Debug;
use crate::std::str::Chars;
#[cfg(not(feature = "std"))]
use alloc::{format, string::String};

impl IsEmptyProperty for &str {
    fn is_empty_property(&self) -> bool {
        self.is_empty()
    }
}

impl LengthProperty for &str {
    fn length_property(&self) -> usize {
        self.len()
    }
}

impl IsEmptyProperty for String {
    fn is_empty_property(&self) -> bool {
        self.is_empty()
    }
}

impl LengthProperty for String {
    fn length_property(&self) -> usize {
        self.len()
    }
}

#[cfg(feature = "std")]
mod os_string {
    use crate::properties::{IsEmptyProperty, LengthProperty};
    use crate::std::ffi::{OsStr, OsString};

    impl IsEmptyProperty for OsString {
        fn is_empty_property(&self) -> bool {
            self.is_empty()
        }
    }

    impl LengthProperty for OsString {
        fn length_property(&self) -> usize {
            self.len()
        }
    }

    impl IsEmptyProperty for &OsStr {
        fn is_empty_property(&self) -> bool {
            self.is_empty()
        }
    }

    impl LengthProperty for &OsStr {
        fn length_property(&self) -> usize {
            self.len()
        }
    }
}

#[cfg(feature = "std")]
mod c_string {
    use crate::properties::IsEmptyProperty;
    use crate::std::ffi::{CStr, CString};

    impl IsEmptyProperty for CString {
        fn is_empty_property(&self) -> bool {
            self.is_empty()
        }
    }

    impl IsEmptyProperty for &CStr {
        fn is_empty_property(&self) -> bool {
            self.is_empty()
        }
    }
}

impl DefinedOrder for Chars<'_> {}

// We implement `AssertContains` for different `Pattern` types as the
// [`core::str::pattern`] API is not stabilized as of February 2025;
// see issue [#27721](https://github.com/rust-lang/rust/issues/27721).
// Maybe we keep the implementations for a long time to support an earlier MSRV.

impl<'a, S, R> AssertStringPattern<&'a str> for Spec<'a, S, R>
where
    S: 'a + AsRef<str> + Debug,
    R: FailingStrategy,
{
    fn contains(self, pattern: &'a str) -> Self {
        self.expecting(StringContains { expected: pattern })
    }

    fn starts_with(self, pattern: &str) -> Self {
        self.expecting(StringStartWith { expected: pattern })
    }

    fn ends_with(self, pattern: &str) -> Self {
        self.expecting(StringEndsWith { expected: pattern })
    }
}

impl<'a, S, R> AssertStringPattern<String> for Spec<'a, S, R>
where
    S: 'a + AsRef<str> + Debug,
    R: FailingStrategy,
{
    fn contains(self, pattern: String) -> Self {
        self.expecting(StringContains { expected: pattern })
    }

    fn starts_with(self, pattern: String) -> Self {
        self.expecting(StringStartWith { expected: pattern })
    }

    fn ends_with(self, pattern: String) -> Self {
        self.expecting(StringEndsWith { expected: pattern })
    }
}

impl<'a, S, R> AssertStringPattern<char> for Spec<'a, S, R>
where
    S: 'a + AsRef<str> + Debug,
    R: FailingStrategy,
{
    fn contains(self, expected: char) -> Self {
        self.expecting(StringContains { expected })
    }

    fn starts_with(self, expected: char) -> Self {
        self.expecting(StringStartWith { expected })
    }

    fn ends_with(self, pattern: char) -> Self {
        self.expecting(StringEndsWith { expected: pattern })
    }
}

impl<S> Expectation<S> for StringContains<&str>
where
    S: AsRef<str> + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.as_ref().contains(self.expected)
    }

    fn message(&self, expression: Expression<'_>, actual: &S) -> String {
        format!(
            "expected {expression} to contain {:?}\n   but was: {actual:?}\n  expected: {:?}",
            self.expected, self.expected
        )
    }
}

impl<S> Expectation<S> for StringContains<String>
where
    S: AsRef<str> + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.as_ref().contains(&self.expected)
    }

    fn message(&self, expression: Expression<'_>, actual: &S) -> String {
        format!(
            "expected {expression} to contain {:?}\n   but was: {actual:?}\n  expected: {:?}",
            self.expected, self.expected
        )
    }
}

impl<S> Expectation<S> for StringContains<char>
where
    S: AsRef<str> + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.as_ref().contains(self.expected)
    }

    fn message(&self, expression: Expression<'_>, actual: &S) -> String {
        format!(
            "expected {expression} to contain {:?}\n   but was: {actual:?}\n  expected: {:?}",
            self.expected, self.expected
        )
    }
}

impl<S> Expectation<S> for StringStartWith<&str>
where
    S: AsRef<str> + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.as_ref().starts_with(self.expected)
    }

    fn message(&self, expression: Expression<'_>, actual: &S) -> String {
        format!(
            "expected {expression} to start with {:?}\n   but was: {actual:?}\n  expected: {:?}",
            self.expected, self.expected
        )
    }
}

impl<S> Expectation<S> for StringStartWith<String>
where
    S: AsRef<str> + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.as_ref().starts_with(&self.expected)
    }

    fn message(&self, expression: Expression<'_>, actual: &S) -> String {
        format!(
            "expected {expression} to start with {:?}\n   but was: {actual:?}\n  expected: {:?}",
            self.expected, self.expected
        )
    }
}

impl<S> Expectation<S> for StringStartWith<char>
where
    S: AsRef<str> + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.as_ref().starts_with(self.expected)
    }

    fn message(&self, expression: Expression<'_>, actual: &S) -> String {
        format!(
            "expected {expression} to start with {:?}\n   but was: {actual:?}\n  expected: {:?}",
            self.expected, self.expected
        )
    }
}

impl<S> Expectation<S> for StringEndsWith<&str>
where
    S: AsRef<str> + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.as_ref().ends_with(self.expected)
    }

    fn message(&self, expression: Expression<'_>, actual: &S) -> String {
        format!(
            "expected {expression} to start with {:?}\n   but was: {actual:?}\n  expected: {:?}",
            self.expected, self.expected
        )
    }
}

impl<S> Expectation<S> for StringEndsWith<String>
where
    S: AsRef<str> + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.as_ref().ends_with(&self.expected)
    }

    fn message(&self, expression: Expression<'_>, actual: &S) -> String {
        format!(
            "expected {expression} to start with {:?}\n   but was: {actual:?}\n  expected: {:?}",
            self.expected, self.expected
        )
    }
}

impl<S> Expectation<S> for StringEndsWith<char>
where
    S: AsRef<str> + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.as_ref().ends_with(self.expected)
    }

    fn message(&self, expression: Expression<'_>, actual: &S) -> String {
        format!(
            "expected {expression} to start with {:?}\n   but was: {actual:?}\n  expected: {:?}",
            self.expected, self.expected
        )
    }
}

// When string slices' `contains` function is used with an array of chars or
// slice of chars it checks if any of the chars in the array/slice is contained
// in the string slice. Therefore, we implement the [`AssertContainsAnyOf`]
// assertion for array/slice of chars as expected value, but not the
// [`AssertContains`] assertion.

impl<'a, S, R> AssertStringContainsAnyOf<&'a [char]> for Spec<'a, S, R>
where
    S: 'a + AsRef<str> + Debug,
    R: FailingStrategy,
{
    fn contains_any_of(self, expected: &'a [char]) -> Self {
        self.expecting(StringContainsAnyOf { expected })
    }
}

impl<'a, S, R, const N: usize> AssertStringContainsAnyOf<[char; N]> for Spec<'a, S, R>
where
    S: 'a + AsRef<str> + Debug,
    R: FailingStrategy,
{
    fn contains_any_of(self, expected: [char; N]) -> Self {
        self.expecting(StringContainsAnyOf { expected })
    }
}

impl<'a, S, R, const N: usize> AssertStringContainsAnyOf<&'a [char; N]> for Spec<'a, S, R>
where
    S: 'a + AsRef<str> + Debug,
    R: FailingStrategy,
{
    fn contains_any_of(self, expected: &'a [char; N]) -> Self {
        self.expecting(StringContainsAnyOf { expected })
    }
}

impl<S> Expectation<S> for StringContainsAnyOf<&[char]>
where
    S: AsRef<str> + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.as_ref().contains(self.expected)
    }

    fn message(&self, expression: Expression<'_>, actual: &S) -> String {
        format!(
            "expected {expression} to contain any of {:?}\n   but was: {actual:?}\n  expected: {:?}",
            self.expected, self.expected
        )
    }
}

impl<S, const N: usize> Expectation<S> for StringContainsAnyOf<[char; N]>
where
    S: AsRef<str> + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.as_ref().contains(self.expected)
    }

    fn message(&self, expression: Expression<'_>, actual: &S) -> String {
        format!(
            "expected {expression} to contain any of {:?}\n   but was: {actual:?}\n  expected: {:?}",
            self.expected, self.expected
        )
    }
}

impl<S, const N: usize> Expectation<S> for StringContainsAnyOf<&[char; N]>
where
    S: AsRef<str> + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
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
