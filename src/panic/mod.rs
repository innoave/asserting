//! Implementation of assertions for code that should or should not panic.

use crate::assertions::AssertCodePanics;
use crate::colored::{mark_missing_substr, mark_unexpected_substr};
use crate::expectations::{DoesNotPanic, DoesPanic};
use crate::spec::{Code, DiffFormat, Expectation, Expression, FailingStrategy, Spec};
use crate::std::any::Any;
use crate::std::panic;

const ONLY_ONE_EXPECTATION: &str = "only one expectation allowed when asserting closures!";
const UNKNOWN_PANIC_MESSAGE: &str = "<unknown panic message>";

impl<S, R> AssertCodePanics for Spec<'_, Code<S>, R>
where
    S: FnOnce(),
    R: FailingStrategy,
{
    fn does_not_panic(self) -> Self {
        self.expecting(DoesNotPanic::default())
    }

    fn panics(self) -> Self {
        self.expecting(DoesPanic::default())
    }

    fn panics_with_message(self, message: impl Into<String>) -> Self {
        self.expecting(DoesPanic::with_message(message))
    }
}

impl<S> Expectation<Code<S>> for DoesNotPanic
where
    S: FnOnce(),
{
    fn test(&mut self, subject: &Code<S>) -> bool {
        if let Some(function) = subject.take() {
            let result = panic::catch_unwind(panic::AssertUnwindSafe(function));
            match result {
                Ok(()) => true,
                Err(panic_message) => {
                    self.actual_message = Some(panic_message);
                    false
                },
            }
        } else {
            self.actual_message = Some(Box::new(ONLY_ONE_EXPECTATION));
            false
        }
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        _actual: &Code<S>,
        format: &DiffFormat,
    ) -> String {
        let panic_message = read_panic_message(self.actual_message.as_ref())
            .unwrap_or_else(|| UNKNOWN_PANIC_MESSAGE.to_string());

        if panic_message == ONLY_ONE_EXPECTATION {
            format!("error in test assertion: {ONLY_ONE_EXPECTATION}")
        } else {
            let marked_did_panic = mark_unexpected_substr("did panic", format);
            let marked_panic_message = mark_unexpected_substr(&panic_message, format);
            format!(
                "expected {expression} to not panic, but {marked_did_panic}\n  with message: \"{marked_panic_message}\""
            )
        }
    }
}

impl<S> Expectation<Code<S>> for DoesPanic
where
    S: FnOnce(),
{
    fn test(&mut self, subject: &Code<S>) -> bool {
        if let Some(function) = subject.take() {
            let result = panic::catch_unwind(panic::AssertUnwindSafe(function));
            match result {
                Ok(()) => false,
                Err(panic_message) => {
                    let panic_message = read_panic_message(Some(panic_message).as_ref())
                        .unwrap_or_else(|| UNKNOWN_PANIC_MESSAGE.to_string());
                    let test_result = if let Some(expected_message) = &self.expected_message {
                        &panic_message == expected_message
                    } else {
                        // did panic - panic message should not be asserted
                        true
                    };
                    self.actual_message = Some(panic_message);
                    test_result
                },
            }
        } else {
            self.actual_message = Some(ONLY_ONE_EXPECTATION.to_string());
            false
        }
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        _actual: &Code<S>,
        format: &DiffFormat,
    ) -> String {
        if let Some(actual_message) = self.actual_message.as_ref() {
            if actual_message == ONLY_ONE_EXPECTATION {
                format!("error in test assertion: {ONLY_ONE_EXPECTATION}")
            } else if let Some(expected_message) = &self.expected_message {
                let marked_expected_message = mark_missing_substr(expected_message, format);
                let marked_actual_message = mark_unexpected_substr(actual_message, format);
                format!("expected {expression} to panic with message {expected_message:?}\n   but was: \"{marked_actual_message}\"\n  expected: \"{marked_expected_message}\"")
            } else {
                // should be unreachable
                format!("expected {expression} to panic, but did not panic")
            }
        } else if let Some(expected_message) = &self.expected_message {
            let marked_did_not_panic = mark_unexpected_substr("did not panic", format);
            format!("expected {expression} to panic with message {expected_message:?},\n  but {marked_did_not_panic}")
        } else {
            let marked_did_not_panic = mark_unexpected_substr("did not panic", format);
            format!("expected {expression} to panic, but {marked_did_not_panic}")
        }
    }
}

fn read_panic_message(error: Option<&Box<dyn Any + Send>>) -> Option<String> {
    error.and_then(|message| {
        message
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| message.downcast_ref::<&str>().map(ToString::to_string))
    })
}

#[cfg(test)]
mod tests;
