//! Implementations of the character count assertions.

use crate::assertions::AssertHasCharCount;
use crate::colored::{mark_missing, mark_unexpected};
use crate::expectations::{
    HasAtLeastCharCount, HasAtMostCharCount, HasCharCount, HasCharCountGreaterThan,
    HasCharCountInRange, HasCharCountLessThan,
};
use crate::properties::CharCountProperty;
use crate::spec::{DiffFormat, Expectation, Expression, FailingStrategy, Spec};
use crate::std::fmt::Debug;
use crate::std::format;
use crate::std::ops::RangeInclusive;
use crate::std::string::String;

impl<S, R> AssertHasCharCount<usize> for Spec<'_, S, R>
where
    S: CharCountProperty + Debug,
    R: FailingStrategy,
{
    fn has_char_count(self, expected_char_count: usize) -> Self {
        self.expecting(HasCharCount {
            expected_char_count,
        })
    }

    fn has_char_count_in_range(self, range: RangeInclusive<usize>) -> Self {
        self.expecting(HasCharCountInRange {
            expected_range: range,
        })
    }

    fn has_char_count_less_than(self, expected_char_count: usize) -> Self {
        self.expecting(HasCharCountLessThan {
            expected_char_count,
        })
    }

    fn has_char_count_greater_than(self, expected_char_count: usize) -> Self {
        self.expecting(HasCharCountGreaterThan {
            expected_char_count,
        })
    }

    fn has_at_most_char_count(self, expected_char_count: usize) -> Self {
        self.expecting(HasAtMostCharCount {
            expected_char_count,
        })
    }

    fn has_at_least_char_count(self, expected_char_count: usize) -> Self {
        self.expecting(HasAtLeastCharCount {
            expected_char_count,
        })
    }
}

impl<S> Expectation<S> for HasCharCount<usize>
where
    S: CharCountProperty + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.char_count_property() == self.expected_char_count
    }

    fn message(&self, expression: Expression<'_>, actual: &S, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected(&actual.char_count_property(), format);
        let marked_expected = mark_missing(&self.expected_char_count, format);
        format!(
            "expected {expression} has a char count of {:?}\n   but was: {marked_actual}\n  expected: {marked_expected}",
            self.expected_char_count
        )
    }
}

impl<S> Expectation<S> for HasCharCountInRange<usize>
where
    S: CharCountProperty + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        self.expected_range.contains(&subject.char_count_property())
    }

    fn message(&self, expression: Expression<'_>, actual: &S, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected(&actual.char_count_property(), format);
        let marked_expected = mark_missing(&self.expected_range, format);
        format!(
            "expected {expression} has a char count of {:?}\n   but was: {marked_actual}\n  expected: {marked_expected}",
            self.expected_range,
        )
    }
}

impl<S> Expectation<S> for HasCharCountLessThan<usize>
where
    S: CharCountProperty + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.char_count_property() < self.expected_char_count
    }

    fn message(&self, expression: Expression<'_>, actual: &S, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected(&actual.char_count_property(), format);
        let marked_expected = mark_missing(&self.expected_char_count, format);
        format!(
            "expected {expression} has a char count less than {:?}\n   but was: {marked_actual}\n  expected: < {marked_expected}",
            self.expected_char_count,
        )
    }
}

impl<S> Expectation<S> for HasCharCountGreaterThan<usize>
where
    S: CharCountProperty + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.char_count_property() > self.expected_char_count
    }

    fn message(&self, expression: Expression<'_>, actual: &S, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected(&actual.char_count_property(), format);
        let marked_expected = mark_missing(&self.expected_char_count, format);
        format!(
            "expected {expression} has a char count greater than {:?}\n   but was: {marked_actual}\n  expected: > {marked_expected}",
            self.expected_char_count,
        )
    }
}

impl<S> Expectation<S> for HasAtMostCharCount<usize>
where
    S: CharCountProperty + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.char_count_property() <= self.expected_char_count
    }

    fn message(&self, expression: Expression<'_>, actual: &S, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected(&actual.char_count_property(), format);
        let marked_expected = mark_missing(&self.expected_char_count, format);
        format!(
            "expected {expression} has at most a char count of {:?}\n   but was: {marked_actual}\n  expected: <= {marked_expected}",
            self.expected_char_count,
        )
    }
}

impl<S> Expectation<S> for HasAtLeastCharCount<usize>
where
    S: CharCountProperty,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.char_count_property() >= self.expected_char_count
    }

    fn message(&self, expression: Expression<'_>, actual: &S, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected(&actual.char_count_property(), format);
        let marked_expected = mark_missing(&self.expected_char_count, format);
        format!(
            "expected {expression} has at least a char count of {:?}\n   but was: {marked_actual}\n  expected: >= {marked_expected}",
            self.expected_char_count,
        )
    }
}
