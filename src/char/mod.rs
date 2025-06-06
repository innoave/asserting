use crate::assertions::AssertChar;
use crate::colored::{mark_missing_substr, mark_unexpected_char};
use crate::expectations::{
    IsAlphabetic, IsAlphanumeric, IsAscii, IsControlChar, IsDigit, IsLowerCase, IsUpperCase,
    IsWhitespace,
};
use crate::spec::{DiffFormat, Expectation, Expression, FailingStrategy, Spec};
use crate::std::format;
use crate::std::string::{String, ToString};

impl<R> AssertChar for Spec<'_, char, R>
where
    R: FailingStrategy,
{
    fn is_lowercase(self) -> Self {
        self.expecting(IsLowerCase)
    }

    fn is_uppercase(self) -> Self {
        self.expecting(IsUpperCase)
    }

    fn is_ascii(self) -> Self {
        self.expecting(IsAscii)
    }

    fn is_alphabetic(self) -> Self {
        self.expecting(IsAlphabetic)
    }

    fn is_alphanumeric(self) -> Self {
        self.expecting(IsAlphanumeric)
    }

    fn is_control_char(self) -> Self {
        self.expecting(IsControlChar)
    }

    fn is_digit(self, radix: u32) -> Self {
        self.expecting(IsDigit { radix })
    }

    fn is_whitespace(self) -> Self {
        self.expecting(IsWhitespace)
    }
}

impl<R> AssertChar for Spec<'_, &char, R>
where
    R: FailingStrategy,
{
    fn is_lowercase(self) -> Self {
        self.expecting(IsLowerCase)
    }

    fn is_uppercase(self) -> Self {
        self.expecting(IsUpperCase)
    }

    fn is_ascii(self) -> Self {
        self.expecting(IsAscii)
    }

    fn is_alphabetic(self) -> Self {
        self.expecting(IsAlphabetic)
    }

    fn is_alphanumeric(self) -> Self {
        self.expecting(IsAlphanumeric)
    }

    fn is_control_char(self) -> Self {
        self.expecting(IsControlChar)
    }

    fn is_digit(self, radix: u32) -> Self {
        self.expecting(IsDigit { radix })
    }

    fn is_whitespace(self) -> Self {
        self.expecting(IsWhitespace)
    }
}

impl Expectation<char> for IsLowerCase {
    fn test(&mut self, subject: &char) -> bool {
        subject.is_lowercase()
    }

    fn message(&self, expression: &Expression<'_>, actual: &char, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected_char(*actual, format);
        let marked_expected = mark_missing_substr(&actual.to_lowercase().to_string(), format);
        format!("expected {expression} is lowercase\n   but was: {marked_actual}\n  expected: {marked_expected}")
    }
}

impl Expectation<&char> for IsLowerCase {
    fn test(&mut self, subject: &&char) -> bool {
        subject.is_lowercase()
    }

    fn message(&self, expression: &Expression<'_>, actual: &&char, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected_char(**actual, format);
        let marked_expected = mark_missing_substr(&actual.to_lowercase().to_string(), format);
        format!("expected {expression} is lowercase\n   but was: {marked_actual}\n  expected: {marked_expected}")
    }
}

impl Expectation<char> for IsUpperCase {
    fn test(&mut self, subject: &char) -> bool {
        subject.is_uppercase()
    }

    fn message(&self, expression: &Expression<'_>, actual: &char, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected_char(*actual, format);
        let marked_expected = mark_missing_substr(&actual.to_uppercase().to_string(), format);
        format!("expected {expression} is uppercase\n   but was: {marked_actual}\n  expected: {marked_expected}")
    }
}

impl Expectation<&char> for IsUpperCase {
    fn test(&mut self, subject: &&char) -> bool {
        subject.is_uppercase()
    }

    fn message(&self, expression: &Expression<'_>, actual: &&char, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected_char(**actual, format);
        let marked_expected = mark_missing_substr(&actual.to_uppercase().to_string(), format);
        format!("expected {expression} is uppercase\n   but was: {marked_actual}\n  expected: {marked_expected}")
    }
}

impl Expectation<char> for IsAscii {
    fn test(&mut self, subject: &char) -> bool {
        subject.is_ascii()
    }

    fn message(&self, expression: &Expression<'_>, actual: &char, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected_char(*actual, format);
        format!("expected {expression} is an ASCII character\n   but was: {marked_actual}\n  expected: an ASCII character")
    }
}

impl Expectation<&char> for IsAscii {
    fn test(&mut self, subject: &&char) -> bool {
        subject.is_ascii()
    }

    fn message(&self, expression: &Expression<'_>, actual: &&char, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected_char(**actual, format);
        format!("expected {expression} is an ASCII character\n   but was: {marked_actual}\n  expected: an ASCII character")
    }
}

impl Expectation<char> for IsAlphabetic {
    fn test(&mut self, subject: &char) -> bool {
        subject.is_alphabetic()
    }

    fn message(&self, expression: &Expression<'_>, actual: &char, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected_char(*actual, format);
        format!("expected {expression} is an alphabetic character\n   but was: {marked_actual}\n  expected: an alphabetic character")
    }
}

impl Expectation<&char> for IsAlphabetic {
    fn test(&mut self, subject: &&char) -> bool {
        subject.is_alphabetic()
    }

    fn message(&self, expression: &Expression<'_>, actual: &&char, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected_char(**actual, format);
        format!("expected {expression} is an alphabetic character\n   but was: {marked_actual}\n  expected: an alphabetic character")
    }
}

impl Expectation<char> for IsAlphanumeric {
    fn test(&mut self, subject: &char) -> bool {
        subject.is_alphanumeric()
    }

    fn message(&self, expression: &Expression<'_>, actual: &char, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected_char(*actual, format);
        format!("expected {expression} is an alphanumeric character\n   but was: {marked_actual}\n  expected: an alphanumeric character")
    }
}

impl Expectation<&char> for IsAlphanumeric {
    fn test(&mut self, subject: &&char) -> bool {
        subject.is_alphanumeric()
    }

    fn message(&self, expression: &Expression<'_>, actual: &&char, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected_char(**actual, format);
        format!("expected {expression} is an alphanumeric character\n   but was: {marked_actual}\n  expected: an alphanumeric character")
    }
}

impl Expectation<char> for IsControlChar {
    fn test(&mut self, subject: &char) -> bool {
        subject.is_control()
    }

    fn message(&self, expression: &Expression<'_>, actual: &char, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected_char(*actual, format);
        format!("expected {expression} is a control character\n   but was: {marked_actual}\n  expected: a control character")
    }
}

impl Expectation<&char> for IsControlChar {
    fn test(&mut self, subject: &&char) -> bool {
        subject.is_control()
    }

    fn message(&self, expression: &Expression<'_>, actual: &&char, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected_char(**actual, format);
        format!("expected {expression} is a control character\n   but was: {marked_actual}\n  expected: a control character")
    }
}

impl Expectation<char> for IsDigit {
    fn test(&mut self, subject: &char) -> bool {
        subject.is_digit(self.radix)
    }

    fn message(&self, expression: &Expression<'_>, actual: &char, format: &DiffFormat) -> String {
        let radix = self.radix;
        let marked_actual = mark_unexpected_char(*actual, format);
        format!("expected {expression} is a digit in the radix {radix}\n   but was: {marked_actual}\n  expected: a digit in the radix {radix}")
    }
}

impl Expectation<&char> for IsDigit {
    fn test(&mut self, subject: &&char) -> bool {
        subject.is_digit(self.radix)
    }

    fn message(&self, expression: &Expression<'_>, actual: &&char, format: &DiffFormat) -> String {
        let radix = self.radix;
        let marked_actual = mark_unexpected_char(**actual, format);
        format!("expected {expression} is a digit in the radix {radix}\n   but was: {marked_actual}\n  expected: a digit in the radix {radix}")
    }
}

impl Expectation<char> for IsWhitespace {
    fn test(&mut self, subject: &char) -> bool {
        subject.is_whitespace()
    }

    fn message(&self, expression: &Expression<'_>, actual: &char, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected_char(*actual, format);
        format!("expected {expression} is a whitespace\n   but was: {marked_actual}\n  expected: a whitespace")
    }
}

impl Expectation<&char> for IsWhitespace {
    fn test(&mut self, subject: &&char) -> bool {
        subject.is_whitespace()
    }

    fn message(&self, expression: &Expression<'_>, actual: &&char, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected_char(**actual, format);
        format!("expected {expression} is a whitespace\n   but was: {marked_actual}\n  expected: a whitespace")
    }
}

#[cfg(test)]
mod tests;
