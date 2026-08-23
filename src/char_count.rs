//! Implementations of the character count assertions.

use crate::assertions::AssertHasCharCount;
use crate::colored::{mark_missing, mark_unexpected};
use crate::expectations::{
    HasAtLeastCharCount, HasAtMostCharCount, HasCharCount, HasCharCountGreaterThan,
    HasCharCountInRange, HasCharCountLessThan, has_at_least_char_count, has_at_most_char_count,
    has_char_count, has_char_count_greater_than, has_char_count_in_range, has_char_count_less_than,
};
use crate::properties::CharCountProperty;
use crate::spec::{
    DiffFormat, Expectation, Expecting, Expression, FailingStrategy, Represent, Represented, Spec,
};
use crate::std::format;
use crate::std::ops::RangeBounds;
use crate::std::string::String;

impl<S, D, R> AssertHasCharCount<usize, D> for Spec<'_, S, D, R>
where
    S: CharCountProperty,
    D: Represent<usize>,
    R: FailingStrategy,
{
    fn has_char_count(self, expected_char_count: usize) -> Self {
        self.expecting(has_char_count(expected_char_count))
    }

    fn has_char_count_in_range<U>(self, expected_range: U) -> Self
    where
        U: RangeBounds<usize>,
        D: Represent<U>,
    {
        self.expecting(has_char_count_in_range(expected_range))
    }

    fn has_char_count_less_than(self, expected_char_count: usize) -> Self {
        self.expecting(has_char_count_less_than(expected_char_count))
    }

    fn has_char_count_greater_than(self, expected_char_count: usize) -> Self {
        self.expecting(has_char_count_greater_than(expected_char_count))
    }

    fn has_at_most_char_count(self, expected_char_count: usize) -> Self {
        self.expecting(has_at_most_char_count(expected_char_count))
    }

    fn has_at_least_char_count(self, expected_char_count: usize) -> Self {
        self.expecting(has_at_least_char_count(expected_char_count))
    }
}

impl<S, D> Expectation<S, D> for HasCharCount<usize>
where
    S: CharCountProperty,
    D: Represent<usize>,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.char_count_property() == self.expected_char_count
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let not = if inverted { "not in " } else { "" };
        let marked_actual = mark_unexpected(&actual.char_count_property(), representation, format);
        let marked_expected = mark_missing(&self.expected_char_count, representation, format);
        let expected_char_count = Represented::from((&self.expected_char_count, representation));
        format!(
            "expected {expression} to {not}have a char count of {expected_char_count:?}\n   but was: {marked_actual}\n  expected: {not}{marked_expected}",
        )
    }
}

impl<S, D, R> Expectation<S, D> for HasCharCountInRange<R, usize>
where
    S: CharCountProperty,
    R: RangeBounds<usize>,
    D: Represent<usize> + Represent<R>,
{
    fn test(&mut self, subject: &S) -> bool {
        self.expected_range.contains(&subject.char_count_property())
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let not = if inverted { "not in " } else { "" };
        let marked_actual = mark_unexpected(&actual.char_count_property(), representation, format);
        let marked_expected = mark_missing(&self.expected_range, representation, format);
        let expected_range = Represented::from((&self.expected_range, representation));
        format!(
            "expected {expression} to {not}have a char count within {expected_range:?}\n   but was: {marked_actual}\n  expected: {not}{marked_expected}",
        )
    }
}

impl<S, D> Expectation<S, D> for HasCharCountLessThan<usize>
where
    S: CharCountProperty,
    D: Represent<usize>,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.char_count_property() < self.expected_char_count
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let (not, cmp) = if inverted { ("not ", ">=") } else { ("", "<") };
        let marked_actual = mark_unexpected(&actual.char_count_property(), representation, format);
        let marked_expected = mark_missing(&self.expected_char_count, representation, format);
        format!(
            "expected {expression} to {not}have a char count less than {:?}\n   but was: {marked_actual}\n  expected: {cmp} {marked_expected}",
            self.expected_char_count,
        )
    }
}

impl<S, D> Expectation<S, D> for HasCharCountGreaterThan<usize>
where
    S: CharCountProperty,
    D: Represent<usize>,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.char_count_property() > self.expected_char_count
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let (not, cmp) = if inverted { ("not ", "<=") } else { ("", ">") };
        let marked_actual = mark_unexpected(&actual.char_count_property(), representation, format);
        let marked_expected = mark_missing(&self.expected_char_count, representation, format);
        let expected_char_count = Represented::from((&self.expected_char_count, representation));
        format!(
            "expected {expression} to {not}have a char count greater than {expected_char_count:?}\n   but was: {marked_actual}\n  expected: {cmp} {marked_expected}",
        )
    }
}

impl<S, D> Expectation<S, D> for HasAtMostCharCount<usize>
where
    S: CharCountProperty,
    D: Represent<usize>,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.char_count_property() <= self.expected_char_count
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let (not, cmp) = if inverted { ("not ", ">") } else { ("", "<=") };
        let marked_actual = mark_unexpected(&actual.char_count_property(), representation, format);
        let marked_expected = mark_missing(&self.expected_char_count, representation, format);
        let expected_char_count = Represented::from((&self.expected_char_count, representation));
        format!(
            "expected {expression} to {not}have at most a char count of {expected_char_count:?}\n   but was: {marked_actual}\n  expected: {cmp} {marked_expected}",
        )
    }
}

impl<S, D> Expectation<S, D> for HasAtLeastCharCount<usize>
where
    S: CharCountProperty,
    D: Represent<usize>,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.char_count_property() >= self.expected_char_count
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let (not, cmp) = if inverted { ("not ", "<") } else { ("", ">=") };
        let marked_actual = mark_unexpected(&actual.char_count_property(), representation, format);
        let marked_expected = mark_missing(&self.expected_char_count, representation, format);
        let expected_char_count = Represented::from((&self.expected_char_count, representation));
        format!(
            "expected {expression} to {not}have at least a char count of {expected_char_count:?}\n   but was: {marked_actual}\n  expected: {cmp} {marked_expected}",
        )
    }
}
