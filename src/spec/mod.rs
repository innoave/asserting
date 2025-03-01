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

#[track_caller]
pub const fn assert_that<'a, S>(subject: S) -> Subject<'a, S, ()> {
    Subject::new(subject)
}

pub const fn check_that<'a, S, E>(subject: S) -> Subject<'a, S, AssertionResult<'a, S, E>> {
    Subject::new(subject)
}

pub const fn expect_that<'a, S>(subject: S) -> Spec<'a, S, PanicOnFail> {
    Spec::new(subject, PanicOnFail)
}

pub const fn verify_that<'a, S>(subject: S) -> Spec<'a, S, CollectFailures> {
    Spec::new(subject, CollectFailures)
}

pub trait Expectation<S> {
    fn test(&self, subject: &S) -> bool;

    fn message(&self, subject_name: Option<&str>, subject: &S) -> String;
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

pub struct Spec<'a, S, R> {
    subject: S,
    subject_name: Option<&'a str>,
    description: Option<&'a str>,
    location: Option<Location<'a>>,
    failures: Vec<AssertFailure<'a>>,
    failing_strategy: R,
}

impl<S, R> Spec<'_, S, R> {
    #[must_use = "a spec does nothing unless an assertion method is called"]
    pub const fn new(subject: S, failing_strategy: R) -> Self {
        Self {
            subject,
            subject_name: None,
            description: None,
            location: None,
            failures: vec![],
            failing_strategy,
        }
    }

    pub fn failures(&self) -> &[AssertFailure<'_>] {
        &self.failures
    }

    pub fn display_failures(&self) -> Vec<String> {
        self.failures.iter().map(ToString::to_string).collect()
    }
}

impl<'a, S, R> Spec<'a, S, R> {
    #[must_use = "a spec does nothing unless an assertion method is called"]
    pub const fn named(mut self, description: &'a str) -> Self {
        self.subject_name = Some(description);
        self
    }
}

impl<S, R> Spec<'_, S, R>
where
    R: FailingStrategy,
{
    #[allow(clippy::needless_pass_by_value)]
    #[must_use]
    pub fn expecting(mut self, expectation: impl Expectation<S>) -> Self {
        if !expectation.test(&self.subject) {
            let message = expectation.message(self.subject_name, &self.subject);
            let failure = AssertFailure {
                description: self.description,
                message,
                location: self.location,
            };
            self.failures.push(failure);
            self.failing_strategy.do_fail_with(&self.failures);
        }
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssertFailure<'a> {
    description: Option<&'a str>,
    message: String,
    location: Option<Location<'a>>,
}

impl Display for AssertFailure<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.description {
            None => {
                writeln!(f, "assertion failed: {}", &self.message)?;
            },
            Some(description) => {
                writeln!(f, "assertion failed: {description}\n{}", &self.message)?;
            },
        }
        if let Some(location) = self.location {
            writeln!(f, "located at: {location}")?;
        }
        Ok(())
    }
}

pub trait FailingStrategy {
    fn do_fail_with(&self, failures: &[AssertFailure<'_>]);
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PanicOnFail;

impl FailingStrategy for PanicOnFail {
    fn do_fail_with(&self, failures: &[AssertFailure<'_>]) {
        let message = failures
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n");
        panic!("{}", message);
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct CollectFailures;

impl FailingStrategy for CollectFailures {
    fn do_fail_with(&self, _failures: &[AssertFailure<'_>]) {
        // do nothing by design
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

#[must_use]
pub enum AssertionResult<'a, S, E> {
    Passed(Assertion<'a, S, E, Self>),
    Failed(AssertionFailure<'a, S, E>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssertionFailure<'a, S, E> {
    asserted: Asserted<'a, S, E>,
}

pub trait AssertionStrategy<R> {
    fn passed(self) -> R;

    fn failed(self) -> R;
}

impl<'a, S, E> AssertionStrategy<AssertionResult<'a, S, E>>
    for Assertion<'a, S, E, AssertionResult<'a, S, E>>
{
    fn passed(self) -> AssertionResult<'a, S, E> {
        AssertionResult::Passed(self)
    }

    fn failed(self) -> AssertionResult<'a, S, E> {
        AssertionResult::Failed(AssertionFailure {
            asserted: self.into(),
        })
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
            AssertionFailure {
                asserted: self.into(),
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
        match self {
            AssertionResult::Passed(assertion) => {
                write!(f, "{assertion}")
            },
            AssertionResult::Failed(failure) => {
                write!(f, "{failure}")
            },
        }
    }
}

impl<S, E, R> Display for Assertion<'_, S, E, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Assertion {
            subject_name,
            description,
            assertion_phrase,
            ..
        } = self;

        write!(f, "assertion passed: ")?;

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

        Ok(())
    }
}

impl<S, E> Display for AssertionFailure<'_, S, E>
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

        write!(f, "assertion failed: ")?;

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

        writeln!(f, "   but was: {actual:?}")?;
        writeln!(f, "  expected: {expected:?}")?;
        if let Some(location) = location {
            writeln!(f, "at location: {location}")?;
        }

        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Unknown;

impl Debug for Unknown {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl Display for Unknown {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "_")
    }
}

#[cfg(test)]
mod tests;
