//! Implementation of assertions for `String` and `str` values.

use crate::assertions::{AssertStringContainsAnyOf, AssertStringPattern};
use crate::colored::{
    mark_missing, mark_missing_char, mark_missing_string,
    mark_selected_chars_in_string_as_unexpected, mark_selected_items_in_collection,
    mark_unexpected_char_in_string, mark_unexpected_string, mark_unexpected_substring_in_string,
};
use crate::expectations::{
    Not, StringContains, StringContainsAnyOf, StringEndsWith, StringStartWith,
};
use crate::properties::{CharCountProperty, DefinedOrderProperty, IsEmptyProperty, LengthProperty};
use crate::spec::{DiffFormat, Expectation, Expression, FailingStrategy, Invertible, Spec};
use crate::std::fmt::Debug;
use crate::std::str::Chars;
use crate::std::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use hashbrown::HashSet;

impl IsEmptyProperty for &str {
    fn is_empty_property(&self) -> bool {
        self.is_empty()
    }
}

impl IsEmptyProperty for String {
    fn is_empty_property(&self) -> bool {
        self.is_empty()
    }
}

impl LengthProperty for &str {
    fn length_property(&self) -> usize {
        self.len()
    }
}

impl LengthProperty for String {
    fn length_property(&self) -> usize {
        self.len()
    }
}

impl CharCountProperty for &str {
    fn char_count_property(&self) -> usize {
        self.chars().count()
    }
}

impl CharCountProperty for String {
    fn char_count_property(&self) -> usize {
        self.chars().count()
    }
}

impl DefinedOrderProperty for Chars<'_> {}

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

    fn does_not_contain(self, pattern: &'a str) -> Self {
        self.expecting(Not(StringContains { expected: pattern }))
    }

    fn starts_with(self, pattern: &str) -> Self {
        self.expecting(StringStartWith { expected: pattern })
    }

    fn does_not_start_with(self, pattern: &'a str) -> Self {
        self.expecting(Not(StringStartWith { expected: pattern }))
    }

    fn ends_with(self, pattern: &str) -> Self {
        self.expecting(StringEndsWith { expected: pattern })
    }

    fn does_not_end_with(self, pattern: &'a str) -> Self {
        self.expecting(Not(StringEndsWith { expected: pattern }))
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

    fn does_not_contain(self, pattern: String) -> Self {
        self.expecting(Not(StringContains { expected: pattern }))
    }

    fn starts_with(self, pattern: String) -> Self {
        self.expecting(StringStartWith { expected: pattern })
    }

    fn does_not_start_with(self, pattern: String) -> Self {
        self.expecting(Not(StringStartWith { expected: pattern }))
    }

    fn ends_with(self, pattern: String) -> Self {
        self.expecting(StringEndsWith { expected: pattern })
    }

    fn does_not_end_with(self, pattern: String) -> Self {
        self.expecting(Not(StringEndsWith { expected: pattern }))
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

    fn does_not_contain(self, pattern: char) -> Self {
        self.expecting(Not(StringContains { expected: pattern }))
    }

    fn starts_with(self, expected: char) -> Self {
        self.expecting(StringStartWith { expected })
    }

    fn does_not_start_with(self, pattern: char) -> Self {
        self.expecting(Not(StringStartWith { expected: pattern }))
    }

    fn ends_with(self, pattern: char) -> Self {
        self.expecting(StringEndsWith { expected: pattern })
    }

    fn does_not_end_with(self, pattern: char) -> Self {
        self.expecting(Not(StringEndsWith { expected: pattern }))
    }
}

impl<S> Expectation<S> for StringContains<&str>
where
    S: AsRef<str> + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.as_ref().contains(self.expected)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let (not, marked_actual) = if inverted {
            let marked_actual =
                mark_unexpected_substring_in_string(actual.as_ref(), self.expected, format);
            ("not ", marked_actual)
        } else {
            let marked_actual = mark_unexpected_string(actual.as_ref(), format);
            ("", marked_actual)
        };
        let marked_expected = mark_missing_string(self.expected, format);
        format!(
            "expected {expression} to {not}contain {:?}\n   but was: \"{marked_actual}\"\n  expected: {not}\"{marked_expected}\"",
            self.expected,
        )
    }
}

impl Invertible for StringContains<&str> {}

impl<S> Expectation<S> for StringContains<String>
where
    S: AsRef<str> + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.as_ref().contains(&self.expected)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let (not, marked_actual) = if inverted {
            let marked_actual =
                mark_unexpected_substring_in_string(actual.as_ref(), &self.expected, format);
            ("not ", marked_actual)
        } else {
            let marked_actual = mark_unexpected_string(actual.as_ref(), format);
            ("", marked_actual)
        };
        let marked_expected = mark_missing_string(&self.expected, format);
        format!(
            "expected {expression} to {not}contain {:?}\n   but was: \"{marked_actual}\"\n  expected: {not}\"{marked_expected}\"",
            self.expected,
        )
    }
}

impl Invertible for StringContains<String> {}

impl<S> Expectation<S> for StringContains<char>
where
    S: AsRef<str> + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.as_ref().contains(self.expected)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let (not, marked_actual) = if inverted {
            let marked_actual =
                mark_unexpected_char_in_string(actual.as_ref(), self.expected, format);
            ("not ", marked_actual)
        } else {
            let marked_actual = mark_unexpected_string(actual.as_ref(), format);
            ("", marked_actual)
        };
        let marked_expected = mark_missing_char(self.expected, format);
        format!(
            "expected {expression} to {not}contain {:?}\n   but was: \"{marked_actual}\"\n  expected: {not}'{marked_expected}'",
            self.expected,
        )
    }
}

impl Invertible for StringContains<char> {}

impl<S> Expectation<S> for StringStartWith<&str>
where
    S: AsRef<str> + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.as_ref().starts_with(self.expected)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let not = if inverted { "not " } else { "" };
        let expected_char_len = self.expected.chars().count();
        let actual_start = actual
            .as_ref()
            .chars()
            .take(expected_char_len)
            .collect::<String>();
        let actual_rest = actual
            .as_ref()
            .chars()
            .skip(expected_char_len)
            .collect::<String>();
        let marked_actual_start = mark_unexpected_string(&actual_start, format);
        let marked_expected = mark_missing_string(self.expected, format);
        format!(
            "expected {expression} to {not}start with {:?}\n   but was: \"{marked_actual_start}{actual_rest}\"\n  expected: {not}\"{marked_expected}\"",
            self.expected,
        )
    }
}

impl Invertible for StringStartWith<&str> {}

impl<S> Expectation<S> for StringStartWith<String>
where
    S: AsRef<str> + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.as_ref().starts_with(&self.expected)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let not = if inverted { "not " } else { "" };
        let expected_char_len = self.expected.chars().count();
        let actual_start = actual
            .as_ref()
            .chars()
            .take(expected_char_len)
            .collect::<String>();
        let actual_rest = actual
            .as_ref()
            .chars()
            .skip(expected_char_len)
            .collect::<String>();
        let marked_actual_start = mark_unexpected_string(&actual_start, format);
        let marked_expected = mark_missing_string(&self.expected, format);
        format!(
            "expected {expression} to {not}start with {:?}\n   but was: \"{marked_actual_start}{actual_rest}\"\n  expected: {not}\"{marked_expected}\"",
            self.expected,
        )
    }
}

impl Invertible for StringStartWith<String> {}

impl<S> Expectation<S> for StringStartWith<char>
where
    S: AsRef<str> + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.as_ref().starts_with(self.expected)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let not = if inverted { "not " } else { "" };
        let actual_first_char = actual.as_ref().chars().take(1).collect::<String>();
        let actual_rest = actual.as_ref().chars().skip(1).collect::<String>();
        let marked_actual_start = mark_unexpected_string(&actual_first_char, format);
        let marked_expected = mark_missing_char(self.expected, format);
        format!(
            "expected {expression} to {not}start with {:?}\n   but was: \"{marked_actual_start}{actual_rest}\"\n  expected: {not}'{marked_expected}'",
            self.expected,
        )
    }
}

impl Invertible for StringStartWith<char> {}

impl<S> Expectation<S> for StringEndsWith<&str>
where
    S: AsRef<str> + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.as_ref().ends_with(self.expected)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let not = if inverted { "not " } else { "" };
        let actual_char_len = actual.as_ref().chars().count();
        let expected_char_len = self.expected.chars().count();
        let split_point = actual_char_len.saturating_sub(expected_char_len);
        let actual_start = actual
            .as_ref()
            .chars()
            .take(split_point)
            .collect::<String>();
        let actual_end = actual
            .as_ref()
            .chars()
            .skip(split_point)
            .collect::<String>();
        let marked_actual_end = mark_unexpected_string(&actual_end, format);
        let marked_expected = mark_missing_string(self.expected, format);
        format!(
            "expected {expression} to {not}end with {:?}\n   but was: \"{actual_start}{marked_actual_end}\"\n  expected: {not}\"{marked_expected}\"",
            self.expected,
        )
    }
}

impl Invertible for StringEndsWith<&str> {}

impl<S> Expectation<S> for StringEndsWith<String>
where
    S: AsRef<str> + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.as_ref().ends_with(&self.expected)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let not = if inverted { "not " } else { "" };
        let actual_char_len = actual.as_ref().chars().count();
        let expected_char_len = self.expected.chars().count();
        let split_point = actual_char_len.saturating_sub(expected_char_len);
        let actual_start = actual
            .as_ref()
            .chars()
            .take(split_point)
            .collect::<String>();
        let actual_end = actual
            .as_ref()
            .chars()
            .skip(split_point)
            .collect::<String>();
        let marked_actual_end = mark_unexpected_string(&actual_end, format);
        let marked_expected = mark_missing_string(&self.expected, format);
        format!(
            "expected {expression} to {not}end with {:?}\n   but was: \"{actual_start}{marked_actual_end}\"\n  expected: {not}\"{marked_expected}\"",
            self.expected,
        )
    }
}

impl Invertible for StringEndsWith<String> {}

impl<S> Expectation<S> for StringEndsWith<char>
where
    S: AsRef<str> + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.as_ref().ends_with(self.expected)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let not = if inverted { "not " } else { "" };
        let actual_last_char = actual
            .as_ref()
            .chars()
            .last()
            .map(|c| c.to_string())
            .unwrap_or_default();
        let mut actual_start = actual.as_ref().to_string();
        actual_start.pop();
        let marked_actual_end = mark_unexpected_string(&actual_last_char, format);
        let marked_expected = mark_missing_char(self.expected, format);
        format!(
            "expected {expression} to {not}end with {:?}\n   but was: \"{actual_start}{marked_actual_end}\"\n  expected: {not}'{marked_expected}'",
            self.expected,
        )
    }
}

impl Invertible for StringEndsWith<char> {}

// When string slices' `contains` function is used with an array of chars or
// slice of chars, it checks if any of the chars in the array/slice is contained
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

    fn does_not_contain_any_of(self, expected: &'a [char]) -> Self {
        self.expecting(Not(StringContainsAnyOf { expected }))
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

    fn does_not_contain_any_of(self, expected: [char; N]) -> Self {
        self.expecting(Not(StringContainsAnyOf { expected }))
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

    fn does_not_contain_any_of(self, expected: &'a [char; N]) -> Self {
        self.expecting(Not(StringContainsAnyOf { expected }))
    }
}

impl<S> Expectation<S> for StringContainsAnyOf<&[char]>
where
    S: AsRef<str> + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.as_ref().contains(self.expected)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let (not, marked_actual, marked_expected) = if inverted {
            let actual = actual.as_ref();
            let mut found_in_actual = HashSet::new();
            let mut found_in_expected = HashSet::new();
            for (exp_idx, expected_char) in self.expected.iter().enumerate() {
                let found = actual
                    .chars()
                    .enumerate()
                    .filter_map(|(idx, chr)| {
                        if chr == *expected_char {
                            Some(idx)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                if !found.is_empty() {
                    found_in_actual.extend(found);
                    found_in_expected.insert(exp_idx);
                }
            }
            let marked_actual =
                mark_selected_chars_in_string_as_unexpected(actual, &found_in_actual, format);
            let marked_expected = mark_selected_items_in_collection(
                self.expected,
                &found_in_expected,
                format,
                mark_missing,
            );
            ("not ", marked_actual, marked_expected)
        } else {
            let marked_actual = mark_unexpected_string(actual.as_ref(), format);
            let marked_expected = mark_missing(&self.expected, format);
            ("", marked_actual, marked_expected)
        };
        format!(
            "expected {expression} to {not}contain any of {:?}\n   but was: \"{marked_actual}\"\n  expected: {not}{marked_expected}",
            self.expected,
        )
    }
}

impl Invertible for StringContainsAnyOf<&[char]> {}

impl<S, const N: usize> Expectation<S> for StringContainsAnyOf<[char; N]>
where
    S: AsRef<str> + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.as_ref().contains(self.expected)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let (not, marked_actual, marked_expected) = if inverted {
            let actual = actual.as_ref();
            let mut found_in_actual = HashSet::new();
            let mut found_in_expected = HashSet::new();
            for (exp_idx, expected_char) in self.expected.iter().enumerate() {
                let found = actual
                    .chars()
                    .enumerate()
                    .filter_map(|(idx, chr)| {
                        if chr == *expected_char {
                            Some(idx)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                if !found.is_empty() {
                    found_in_actual.extend(found);
                    found_in_expected.insert(exp_idx);
                }
            }
            let marked_actual =
                mark_selected_chars_in_string_as_unexpected(actual, &found_in_actual, format);
            let marked_expected = mark_selected_items_in_collection(
                &self.expected,
                &found_in_expected,
                format,
                mark_missing,
            );
            ("not ", marked_actual, marked_expected)
        } else {
            let marked_actual = mark_unexpected_string(actual.as_ref(), format);
            let marked_expected = mark_missing(&self.expected, format);
            ("", marked_actual, marked_expected)
        };
        format!(
            "expected {expression} to {not}contain any of {:?}\n   but was: \"{marked_actual}\"\n  expected: {not}{marked_expected}",
            self.expected,
        )
    }
}

impl<const N: usize> Invertible for StringContainsAnyOf<[char; N]> {}

impl<S, const N: usize> Expectation<S> for StringContainsAnyOf<&[char; N]>
where
    S: AsRef<str> + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.as_ref().contains(self.expected)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let (not, marked_actual, marked_expected) = if inverted {
            let actual = actual.as_ref();
            let mut found_in_actual = HashSet::new();
            let mut found_in_expected = HashSet::new();
            for (exp_idx, expected_char) in self.expected.iter().enumerate() {
                let found = actual
                    .chars()
                    .enumerate()
                    .filter_map(|(idx, chr)| {
                        if chr == *expected_char {
                            Some(idx)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                if !found.is_empty() {
                    found_in_actual.extend(found);
                    found_in_expected.insert(exp_idx);
                }
            }
            let marked_actual =
                mark_selected_chars_in_string_as_unexpected(actual, &found_in_actual, format);
            let marked_expected = mark_selected_items_in_collection(
                self.expected,
                &found_in_expected,
                format,
                mark_missing,
            );
            ("not ", marked_actual, marked_expected)
        } else {
            let marked_actual = mark_unexpected_string(actual.as_ref(), format);
            let marked_expected = mark_missing(&self.expected, format);
            ("", marked_actual, marked_expected)
        };
        format!(
            "expected {expression} to {not}contain any of {:?}\n   but was: \"{marked_actual}\"\n  expected: {not}{marked_expected}",
            self.expected,
        )
    }
}

impl<const N: usize> Invertible for StringContainsAnyOf<&[char; N]> {}

#[cfg(feature = "regex")]
mod regex {
    use crate::assertions::AssertStringMatches;
    use crate::colored::{mark_missing_string, mark_unexpected_string};
    use crate::expectations::{Not, StringMatches};
    use crate::spec::{DiffFormat, Expectation, Expression, FailingStrategy, Invertible, Spec};
    use crate::std::fmt::Debug;

    impl<S, R> AssertStringMatches for Spec<'_, S, R>
    where
        S: AsRef<str> + Debug,
        R: FailingStrategy,
    {
        fn matches(self, regex_pattern: &str) -> Self {
            self.expecting(StringMatches::new(regex_pattern))
        }

        fn does_not_match(self, regex_pattern: &str) -> Self {
            self.expecting(Not(StringMatches::new(regex_pattern)))
        }
    }

    impl<S> Expectation<S> for StringMatches<'_>
    where
        S: AsRef<str> + Debug,
    {
        fn test(&mut self, subject: &S) -> bool {
            self.regex.is_match(subject.as_ref())
        }

        fn message(
            &self,
            expression: &Expression<'_>,
            actual: &S,
            inverted: bool,
            format: &DiffFormat,
        ) -> String {
            let (not, does_not_match) = if inverted {
                ("not ", "    does match")
            } else {
                ("", "does not match")
            };
            let regex = self.regex.as_str();
            let marked_actual = mark_unexpected_string(actual.as_ref(), format);
            let marked_expected = mark_missing_string(regex, format);
            format!("expected {expression} to {not}match the regex {regex}\n               but was: {marked_actual}\n  {does_not_match} regex: {marked_expected}")
        }
    }

    impl Invertible for StringMatches<'_> {}
}

#[cfg(test)]
mod tests;
