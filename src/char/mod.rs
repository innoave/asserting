use crate::assertions::AssertChar;
use crate::colored::{mark_missing, mark_unexpected};
use crate::expectations::{
    IsAlphabetic, IsAlphanumeric, IsAscii, IsControlChar, IsDigit, IsLowerCase, IsUpperCase,
    IsWhitespace, is_alphabetic, is_alphanumeric, is_ascii, is_control_char, is_digit,
    is_lower_case, is_upper_case, is_whitespace,
};
use crate::spec::{
    DiffFormat, DisplayRepresentation, Expectation, Expecting, Expression, FailingStrategy,
    Invertible, Represent, Spec,
};
use crate::std::format;
use crate::std::string::{String, ToString};

impl<D, R> AssertChar for Spec<'_, char, D, R>
where
    D: Represent<char> + Represent<str>,
    R: FailingStrategy,
{
    fn is_lowercase(self) -> Self {
        self.expecting(is_lower_case())
    }

    fn is_uppercase(self) -> Self {
        self.expecting(is_upper_case())
    }

    fn is_ascii(self) -> Self {
        self.expecting(is_ascii())
    }

    fn is_alphabetic(self) -> Self {
        self.expecting(is_alphabetic())
    }

    fn is_alphanumeric(self) -> Self {
        self.expecting(is_alphanumeric())
    }

    fn is_control_char(self) -> Self {
        self.expecting(is_control_char())
    }

    fn is_digit(self, radix: u32) -> Self {
        self.expecting(is_digit(radix))
    }

    fn is_whitespace(self) -> Self {
        self.expecting(is_whitespace())
    }
}

impl<D, R> AssertChar for Spec<'_, &char, D, R>
where
    D: Represent<char> + Represent<str>,
    R: FailingStrategy,
{
    fn is_lowercase(self) -> Self {
        self.expecting(is_lower_case())
    }

    fn is_uppercase(self) -> Self {
        self.expecting(is_upper_case())
    }

    fn is_ascii(self) -> Self {
        self.expecting(is_ascii())
    }

    fn is_alphabetic(self) -> Self {
        self.expecting(is_alphabetic())
    }

    fn is_alphanumeric(self) -> Self {
        self.expecting(is_alphanumeric())
    }

    fn is_control_char(self) -> Self {
        self.expecting(is_control_char())
    }

    fn is_digit(self, radix: u32) -> Self {
        self.expecting(is_digit(radix))
    }

    fn is_whitespace(self) -> Self {
        self.expecting(is_whitespace())
    }
}

impl<D> Expectation<char, D> for IsLowerCase
where
    D: Represent<char>,
{
    fn test(&mut self, subject: &char) -> bool {
        subject.is_lowercase()
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &char,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let (not, expected) = if inverted {
            ("not ", actual.to_uppercase().to_string())
        } else {
            ("", actual.to_lowercase().to_string())
        };
        let marked_actual = mark_unexpected(actual, representation, format);
        let marked_expected = mark_missing(&expected, &DisplayRepresentation, format);
        format!(
            "expected {expression} to be {not}lowercase\n   but was: {marked_actual}\n  expected: '{marked_expected}'"
        )
    }
}

impl Invertible for IsLowerCase {}

impl<D> Expectation<&char, D> for IsLowerCase
where
    D: Represent<char>,
{
    fn test(&mut self, subject: &&char) -> bool {
        <Self as Expectation<char, D>>::test(self, subject)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &&char,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        <Self as Expectation<char, D>>::message(
            self,
            expression,
            actual,
            inverted,
            representation,
            format,
        )
    }
}

impl<D> Expectation<char, D> for IsUpperCase
where
    D: Represent<char>,
{
    fn test(&mut self, subject: &char) -> bool {
        subject.is_uppercase()
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &char,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let (not, expected) = if inverted {
            ("not ", actual.to_lowercase().to_string())
        } else {
            ("", actual.to_uppercase().to_string())
        };
        let marked_actual = mark_unexpected(actual, representation, format);
        let marked_expected = mark_missing(&expected, &DisplayRepresentation, format);
        format!(
            "expected {expression} to be {not}uppercase\n   but was: {marked_actual}\n  expected: '{marked_expected}'"
        )
    }
}

impl Invertible for IsUpperCase {}

impl<D> Expectation<&char, D> for IsUpperCase
where
    D: Represent<char>,
{
    fn test(&mut self, subject: &&char) -> bool {
        <Self as Expectation<char, D>>::test(self, subject)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &&char,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        <Self as Expectation<char, D>>::message(
            self,
            expression,
            actual,
            inverted,
            representation,
            format,
        )
    }
}

impl<D> Expectation<char, D> for IsAscii
where
    D: Represent<char>,
{
    fn test(&mut self, subject: &char) -> bool {
        subject.is_ascii()
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &char,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let not = if inverted { "not " } else { "" };
        let marked_actual = mark_unexpected(actual, representation, format);
        format!(
            "expected {expression} to be {not}an ASCII character\n   but was: {marked_actual}\n  expected: {not}an ASCII character"
        )
    }
}

impl Invertible for IsAscii {}

impl<D> Expectation<&char, D> for IsAscii
where
    D: Represent<char>,
{
    fn test(&mut self, subject: &&char) -> bool {
        <Self as Expectation<char, D>>::test(self, subject)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &&char,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        <Self as Expectation<char, D>>::message(
            self,
            expression,
            actual,
            inverted,
            representation,
            format,
        )
    }
}

impl<D> Expectation<char, D> for IsAlphabetic
where
    D: Represent<char>,
{
    fn test(&mut self, subject: &char) -> bool {
        subject.is_alphabetic()
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &char,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let not = if inverted { "not " } else { "" };
        let marked_actual = mark_unexpected(actual, representation, format);
        format!(
            "expected {expression} to be {not}an alphabetic character\n   but was: {marked_actual}\n  expected: {not}an alphabetic character"
        )
    }
}

impl Invertible for IsAlphabetic {}

impl<D> Expectation<&char, D> for IsAlphabetic
where
    D: Represent<char>,
{
    fn test(&mut self, subject: &&char) -> bool {
        <Self as Expectation<char, D>>::test(self, subject)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &&char,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        <Self as Expectation<char, D>>::message(
            self,
            expression,
            actual,
            inverted,
            representation,
            format,
        )
    }
}

impl<D> Expectation<char, D> for IsAlphanumeric
where
    D: Represent<char>,
{
    fn test(&mut self, subject: &char) -> bool {
        subject.is_alphanumeric()
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &char,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let not = if inverted { "not " } else { "" };
        let marked_actual = mark_unexpected(actual, representation, format);
        format!(
            "expected {expression} to be {not}an alphanumeric character\n   but was: {marked_actual}\n  expected: {not}an alphanumeric character"
        )
    }
}

impl Invertible for IsAlphanumeric {}

impl<D> Expectation<&char, D> for IsAlphanumeric
where
    D: Represent<char>,
{
    fn test(&mut self, subject: &&char) -> bool {
        <Self as Expectation<char, D>>::test(self, subject)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &&char,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        <Self as Expectation<char, D>>::message(
            self,
            expression,
            actual,
            inverted,
            representation,
            format,
        )
    }
}

impl<D> Expectation<char, D> for IsControlChar
where
    D: Represent<char>,
{
    fn test(&mut self, subject: &char) -> bool {
        subject.is_control()
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &char,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let not = if inverted { "not " } else { "" };
        let marked_actual = mark_unexpected(actual, representation, format);
        format!(
            "expected {expression} to be {not}a control character\n   but was: {marked_actual}\n  expected: {not}a control character"
        )
    }
}

impl Invertible for IsControlChar {}

impl<D> Expectation<&char, D> for IsControlChar
where
    D: Represent<char>,
{
    fn test(&mut self, subject: &&char) -> bool {
        <Self as Expectation<char, D>>::test(self, subject)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &&char,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        <Self as Expectation<char, D>>::message(
            self,
            expression,
            actual,
            inverted,
            representation,
            format,
        )
    }
}

impl<D> Expectation<char, D> for IsDigit
where
    D: Represent<char>,
{
    fn test(&mut self, subject: &char) -> bool {
        subject.is_digit(self.radix)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &char,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let not = if inverted { "not " } else { "" };
        let radix = self.radix;
        let marked_actual = mark_unexpected(actual, representation, format);
        format!(
            "expected {expression} to be {not}a digit in the radix {radix}\n   but was: {marked_actual}\n  expected: {not}a digit in the radix {radix}"
        )
    }
}

impl Invertible for IsDigit {}

impl<D> Expectation<&char, D> for IsDigit
where
    D: Represent<char>,
{
    fn test(&mut self, subject: &&char) -> bool {
        <Self as Expectation<char, D>>::test(self, subject)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &&char,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        <Self as Expectation<char, D>>::message(
            self,
            expression,
            actual,
            inverted,
            representation,
            format,
        )
    }
}

impl<D> Expectation<char, D> for IsWhitespace
where
    D: Represent<char>,
{
    fn test(&mut self, subject: &char) -> bool {
        subject.is_whitespace()
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &char,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let not = if inverted { "not " } else { "" };
        let marked_actual = mark_unexpected(actual, representation, format);
        format!(
            "expected {expression} to be {not}whitespace\n   but was: {marked_actual}\n  expected: {not}whitespace"
        )
    }
}

impl Invertible for IsWhitespace {}

impl<D> Expectation<&char, D> for IsWhitespace
where
    D: Represent<char>,
{
    fn test(&mut self, subject: &&char) -> bool {
        <Self as Expectation<char, D>>::test(self, subject)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &&char,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        <Self as Expectation<char, D>>::message(
            self,
            expression,
            actual,
            inverted,
            representation,
            format,
        )
    }
}

#[cfg(test)]
mod tests;
