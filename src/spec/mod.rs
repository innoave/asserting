use crate::std::fmt::{self, Debug, Display};
use crate::std::marker::PhantomData;
#[cfg(not(any(feature = "std", test)))]
use alloc::string::String;

#[macro_export]
macro_rules! assert_that {
    ($subject:expr) => {
        $crate::prelude::assert_that::<'_>($subject)
            .named(&stringify!($subject).replace("\n", " "))
            .at_location($crate::prelude::Location {
                file: file!(),
                line: line!(),
                column: column!(),
            })
    };
}

/// Code location.
///
/// # Related
/// - [`core::panic::Location`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Location<'a> {
    pub file: &'a str,
    pub line: u32,
    pub column: u32,
}

impl Display for Location<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}

impl<'a> Location<'a> {
    #[must_use]
    pub const fn new(file: &'a str, line: u32, column: u32) -> Self {
        Self { file, line, column }
    }
}

impl Location<'_> {
    #[must_use]
    pub const fn file(&self) -> &str {
        self.file
    }

    #[must_use]
    pub const fn line(&self) -> u32 {
        self.line
    }

    #[must_use]
    pub const fn column(&self) -> u32 {
        self.column
    }
}

#[must_use = "a subject does nothing unless an assertion method is called"]
#[derive(Debug, Clone, Copy)]
pub struct Subject<'a, S, R> {
    subject: S,
    subject_name: Option<&'a str>,
    location: Option<Location<'a>>,
    description: Option<&'a str>,
    return_type: PhantomData<R>,
}

impl<'a, S, R> Subject<'a, S, R> {
    pub const fn new(subject: S) -> Self {
        Self {
            subject,
            subject_name: None,
            location: None,
            description: None,
            return_type: PhantomData,
        }
    }

    pub const fn named(mut self, subject_name: &'a str) -> Self {
        self.subject_name = Some(subject_name);
        self
    }

    pub const fn at_location(mut self, location: Location<'a>) -> Self {
        self.location = Some(location);
        self
    }

    pub const fn asserting(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    pub const fn subject(&self) -> &S {
        &self.subject
    }

    pub const fn subject_name(&self) -> Option<&str> {
        self.subject_name
    }

    pub const fn location(&self) -> Option<Location<'a>> {
        self.location
    }

    pub const fn description(&self) -> Option<&'a str> {
        self.description
    }

    pub fn assertion_with<E>(
        self,
        assertion_phrase: impl Into<String>,
        expected: E,
    ) -> Assertion<'a, S, E, R> {
        Assertion {
            subject: self.subject,
            subject_name: self.subject_name,
            location: self.location,
            description: self.description,
            assertion_phrase: assertion_phrase.into(),
            expected,
            return_type: self.return_type,
        }
    }
}

pub struct Assertion<'a, S, E, R> {
    subject: S,
    subject_name: Option<&'a str>,
    location: Option<Location<'a>>,
    description: Option<&'a str>,
    assertion_phrase: String,
    expected: E,
    return_type: PhantomData<R>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Asserted<'a, S, E> {
    description: Option<&'a str>,
    assertion_phrase: String,
    location: Option<Location<'a>>,
    subject_name: Option<&'a str>,
    actual: S,
    expected: E,
}

impl<'a, S, E, R> From<Assertion<'a, S, E, R>> for Asserted<'a, S, E> {
    fn from(assertion: Assertion<'a, S, E, R>) -> Self {
        Self {
            description: assertion.description,
            assertion_phrase: assertion.assertion_phrase,
            location: assertion.location,
            subject_name: assertion.subject_name,
            actual: assertion.subject,
            expected: assertion.expected,
        }
    }
}

impl<S, E> Asserted<'_, S, E> {
    pub const fn description(&self) -> Option<&str> {
        self.description
    }

    pub fn assertion_phrase(&self) -> &str {
        &self.assertion_phrase
    }

    pub const fn location(&self) -> Option<Location<'_>> {
        self.location
    }

    pub const fn subject_name(&self) -> Option<&str> {
        self.subject_name
    }

    pub const fn actual(&self) -> &S {
        &self.actual
    }

    pub const fn expected(&self) -> &E {
        &self.expected
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssertionStatus {
    Passed,
    Failed,
}

#[must_use]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssertionResult<'a, S, E> {
    asserted: Asserted<'a, S, E>,
    status: AssertionStatus,
}

pub trait AssertionStrategy<R> {
    fn passed(self) -> R;

    fn failed(self) -> R;
}

impl<'a, S, E> AssertionStrategy<AssertionResult<'a, S, E>>
    for Assertion<'a, S, E, AssertionResult<'a, S, E>>
{
    fn passed(self) -> AssertionResult<'a, S, E> {
        AssertionResult {
            asserted: self.into(),
            status: AssertionStatus::Passed,
        }
    }

    fn failed(self) -> AssertionResult<'a, S, E> {
        AssertionResult {
            asserted: self.into(),
            status: AssertionStatus::Failed,
        }
    }
}

impl<S, E> AssertionStrategy<()> for Assertion<'_, S, E, ()>
where
    S: Debug,
    E: Debug,
{
    fn passed(self) {
        // do nothing
    }

    fn failed(self) {
        panic!(
            "{}",
            AssertionResult {
                asserted: self.into(),
                status: AssertionStatus::Failed,
            }
        );
    }
}

impl<S, E> Display for AssertionResult<'_, S, E>
where
    S: Debug,
    E: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Asserted {
            description,
            assertion_phrase,
            location,
            subject_name,
            actual,
            expected,
        } = &self.asserted;

        match self.status {
            AssertionStatus::Passed => {
                writeln!(f, "assertion passed: ")?;
            },
            AssertionStatus::Failed => {
                write!(f, "assertion failed: ")?;
            },
        }

        match (description, subject_name) {
            (Some(description), Some(subject_name)) => {
                writeln!(f, "{description} {subject_name} {assertion_phrase}")?;
            },
            (Some(description), None) => {
                writeln!(f, "{description}")?;
            },
            (None, Some(subject_name)) => {
                writeln!(f, "expected {subject_name} {assertion_phrase}")?;
            },
            (None, None) => {
                writeln!(f, "{assertion_phrase}")?;
            },
        }

        if self.status != AssertionStatus::Passed {
            writeln!(f, "   but was: {actual:?}")?;
            writeln!(f, "  expected: {expected:?}")?;
            if let Some(location) = location {
                writeln!(f, "at location: {location}")?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Unknown;

impl Debug for Unknown {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "_")
    }
}

#[track_caller]
pub const fn assert_that<'a, S>(subject: S) -> Subject<'a, S, ()> {
    Subject::new(subject)
}

pub const fn check_that<'a, S, E>(subject: S) -> Subject<'a, S, AssertionResult<'a, S, E>> {
    Subject::new(subject)
}

#[cfg(test)]
mod tests;
