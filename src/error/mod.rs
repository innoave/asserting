use crate::assertions::AssertErrorHasSource;
use crate::colored::{mark_missing, mark_missing_substr, mark_unexpected, mark_unexpected_substr};
use crate::expectations::{ErrorHasNoSource, ErrorHasSource, ErrorHasSourceMessage};
use crate::spec::{DiffFormat, Expectation, Expression, FailingStrategy, Spec};
use crate::std::error::Error;
use crate::std::format;
use crate::std::string::{String, ToString};

impl<'a, S, R> AssertErrorHasSource<'a, R> for Spec<'a, S, R>
where
    S: Error,
    R: FailingStrategy,
{
    fn has_no_source(self) -> Self {
        self.expecting(ErrorHasNoSource)
    }

    fn has_source(self) -> Self {
        self.expecting(ErrorHasSource)
    }

    fn has_source_message(
        self,
        expected_source_message: impl Into<String>,
    ) -> Spec<'a, Option<String>, R> {
        let expected_source_message = expected_source_message.into();
        self.expecting(ErrorHasSourceMessage {
            expected_source_message,
        })
        .mapping(|err| err.source().map(ToString::to_string))
    }
}

impl<S> Expectation<S> for ErrorHasNoSource
where
    S: Error,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.source().is_none()
    }

    fn message(&self, expression: &Expression<'_>, actual: &S, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing_substr("<error with no source>", format);
        format!("expected {expression} has no source\n   but was: {marked_actual}\n  expected: {marked_expected}")
    }
}

impl<S> Expectation<S> for ErrorHasSource
where
    S: Error,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.source().is_some()
    }

    fn message(&self, expression: &Expression<'_>, actual: &S, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing_substr("<error with some source>", format);
        format!("expected {expression} has a source\n   but was: {marked_actual}\n  expected: {marked_expected}")
    }
}

impl<S> Expectation<S> for ErrorHasSourceMessage
where
    S: Error,
{
    fn test(&mut self, subject: &S) -> bool {
        subject
            .source()
            .is_some_and(|msg| msg.to_string() == self.expected_source_message)
    }

    fn message(&self, expression: &Expression<'_>, actual: &S, format: &DiffFormat) -> String {
        let expected = &self.expected_source_message;
        if let Some(actual_source) = actual.source() {
            let marked_actual = mark_unexpected_substr(&actual_source.to_string(), format);
            let marked_expected = mark_missing_substr(expected, format);
            format!("expected {expression} has source message \"{expected}\"\n   but was: \"{marked_actual}\"\n  expected: \"{marked_expected}\"")
        } else {
            let mut marked_actual = mark_unexpected(actual, format);
            marked_actual.push_str(" - which has no source");
            let marked_expected = mark_missing(expected, format);
            format!("expected {expression} has source message \"{expected}\"\n   but was: {marked_actual}\n  expected: {marked_expected}")
        }
    }
}

#[cfg(test)]
mod tests;
