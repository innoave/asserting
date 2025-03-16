//! This is the core of the `asserting` crate.

use crate::expectations::Predicate;
use crate::std::fmt::{self, Debug, Display};
use crate::std::ops::Deref;
#[cfg(not(feature = "std"))]
use alloc::{
    borrow::ToOwned,
    string::{String, ToString},
    vec,
    vec::Vec,
};

/// Starts an assertion for the given subject or expression in the
/// [`PanicOnFail`] mode.
///
/// It wraps the subject into a [`Spec`] and sets the name of the expression and
/// the code location of the assertion in the [`Spec`]. On the [`Spec`] any
/// assertion method that is implemented for the subject's type can be called.
///
/// Assertions started with `assert_that!` will panic on the first failing
/// assertion.
///
/// ## Example
///
/// ```
/// use asserting::prelude::*;
///
/// assert_that!(7 * 6).is_equal_to(42);
/// ```
///
/// This call of the macro expands to:
///
/// ```
/// # use asserting::prelude::*;
/// assert_that(7 * 6)
///     .named("7 * 6")
///     .located_at(Location { file: file!(), line: line!(), column: column!() })
///     .is_equal_to(42);
/// ```
#[macro_export]
macro_rules! assert_that {
    ($subject:expr) => {
        $crate::prelude::assert_that($subject)
            .named(&stringify!($subject).replace("\n", " "))
            .located_at($crate::prelude::Location {
                file: file!(),
                line: line!(),
                column: column!(),
            })
    };
}

/// Starts an assertion for the given subject or expression in the
/// [`CollectFailures`] mode.
///
/// It wraps the subject into a [`Spec`] and sets the name of the expression and
/// the code location of the assertion in the [`Spec`]. On the [`Spec`] any
/// assertion method that is implemented for the subject's type can be called.
///
/// Assertions started with `verify_that!` will collect [`AssertFailure`]s for
/// all failing assertions. The collected failures can be queried by calling one
/// of the methods [`failures`](Spec::failures) or
/// [`display_failures`](Spec::display_failures) on the [`Spec`].
///
/// ## Example
///
/// ```
/// use asserting::prelude::*;
/// let some_text = "vel tempor augue delenit".to_string();
/// let failures = verify_that!(some_text)
///     .starts_with("nibh")
///     .ends_with("magna")
///     .failures();
///
/// assert_that!(failures).has_length(2);
/// ```
///
/// This call of the macro expands to:
///
/// ```
/// # use asserting::prelude::*;
/// # let some_text = "vel tempor augue delenit".to_string();
/// let failures = verify_that(some_text)
///     .named("some_text")
///     .located_at(Location { file: file!(), line: line!(), column: column!() })
///     .starts_with("nibh")
///     .ends_with("magna")
///     .failures();
/// ```
#[macro_export]
macro_rules! verify_that {
    ($subject:expr) => {
        $crate::prelude::verify_that($subject)
            .named(&stringify!($subject).replace("\n", " "))
            .located_at($crate::prelude::Location {
                file: file!(),
                line: line!(),
                column: column!(),
            })
    };
}

/// Starts an assertion for some piece of code in the [`PanicOnFail`] mode.
///
/// It takes a closure and wraps it into a [`Spec`]. On the [`Spec`] any
/// assertion method that is implemented for closures can be called.
///
/// Assertions started with `assert_that_code!` will panic on the first failing
/// assertion.
///
/// ## Examples
///
/// ```
/// use asserting::prelude::*;
///
/// fn divide(a: i32, b: i32) -> i32 {
///     a / b
/// }
///
/// assert_that_code!(|| { divide(7, 0); })
///     .panics_with_message("attempt to divide by zero");
///
/// assert_that_code!(|| { divide(7, 3); }).does_not_panic();
/// ```
#[cfg(feature = "panic")]
#[cfg_attr(feature = "panic", macro_export)]
macro_rules! assert_that_code {
    ($subject:expr) => {
        $crate::prelude::assert_that_code($subject)
            .named(&stringify!($subject).replace("\n", " "))
            .located_at($crate::prelude::Location {
                file: file!(),
                line: line!(),
                column: column!(),
            })
    };
}

/// Starts an assertion for some piece of code in the [`CollectFailures`] mode.
///
/// It takes a closure and wraps it into a [`Spec`]. On the [`Spec`] any
/// assertion method that is implemented for closures can be called.
///
/// Assertions started with `verify_that_code!` will collect [`AssertFailure`]s
/// for all failing assertions. The collected failures can be queried by calling
/// one of the methods [`failures`](Spec::failures) or
/// [`display_failures`](Spec::display_failures) on the [`Spec`].
///
/// ## Examples
///
/// ```
/// use asserting::prelude::*;
///
/// fn divide(a: i32, b: i32) -> i32 {
///     a / b
/// }
///
/// let failures = verify_that_code!(|| { divide(7, 3); })
///     .does_not_panic()
///     .failures();
///
/// assert_that!(failures).is_empty();
///
/// let failures = verify_that_code!(|| { divide(7, 0); })
///     .does_not_panic()
///     .display_failures();
///
/// assert_that!(failures).contains_exactly([
///     r#"assertion failed: expected || { divide(7, 0); } to not panic, but did panic
///   with message: "attempt to divide by zero"
/// "#
/// ]);
///
/// let failures = verify_that_code!(|| { divide(7, 0); })
///     .panics_with_message("division by zero")
///     .display_failures();
///
/// assert_that!(failures).contains_exactly([
///     r#"assertion failed: expected || { divide(7, 0); } to panic with message "division by zero"
///    but was: "attempt to divide by zero"
///   expected: "division by zero"
/// "#
/// ]);
/// ```
#[cfg(feature = "panic")]
#[cfg_attr(feature = "panic", macro_export)]
macro_rules! verify_that_code {
    ($subject:expr) => {
        $crate::prelude::verify_that_code($subject)
            .named(&stringify!($subject).replace("\n", " "))
            .located_at($crate::prelude::Location {
                file: file!(),
                line: line!(),
                column: column!(),
            })
    };
}

/// Starts an assertion for the given subject or expression in the
/// [`PanicOnFail`] mode.
///
/// It wraps the subject into a [`Spec`]. On the [`Spec`] any
/// assertion method that is implemented for the subject's type can be called.
///
/// Assertions started with `assert_that()` will panic on the first failing
/// assertion.
///
/// In comparison to using the macro [`assert_that!`](crate::assert_that)
/// calling this function does not set a name for the expression and does not
/// set the code location of the assertion. In failure messages the generic word
/// "subject" is used. To set a specific text for the expression the method
/// [`named`](Spec::named) must be called explicitly.
///
/// Note: It is not necessary to set the code location explicitly as this
/// function is annotated with `#[track_caller]`.
///
/// ## Examples
///
/// ```
/// use asserting::prelude::*;
///
/// assert_that(7 * 6).is_equal_to(42);
/// ```
///
/// or with setting a name for the expression:
///
/// ```
/// use asserting::prelude::*;
///
/// assert_that(7 * 6)
///     .named("7 * 6")
///     .is_equal_to(42);
/// ```
#[track_caller]
pub const fn assert_that<'a, S>(subject: S) -> Spec<'a, S, PanicOnFail> {
    Spec::new(subject, PanicOnFail)
}

/// Starts an assertion for the given subject or expression in the
/// [`CollectFailures`] mode.
///
/// It wraps the subject into a [`Spec`]. On the [`Spec`] any
/// assertion method that is implemented for the subject's type can be called.
///
/// Assertions started with `verify_that()` will collect [`AssertFailure`]s
/// for all failing assertions. The collected failures can be queried by calling
/// one of the methods [`failures`](Spec::failures) or the
/// [`display_failures`](Spec::display_failures) on the [`Spec`].
///
/// In comparison to using the macro [`verify_that!`](crate::verify_that) calling
/// this function does not set a name for the expression and does not set the
/// code location of the assertion. In failure messages the generic word
/// "subject" is used. To set a specific text for the expression the method
/// [`named`](Spec::named) must be called explicitly.
///
/// ## Examples
///
/// ```
/// use asserting::prelude::*;
///
/// let some_text = "vel tempor augue delenit".to_string();
///
/// let failures = verify_that(some_text).named("my_thing")
///     .starts_with("nibh")
///     .ends_with("magna")
///     .failures();
///
/// assert_that!(failures).has_length(2);
/// ```
///
/// or with querying the failures as formated text:
///
/// ```
/// use asserting::prelude::*;
///
/// let some_text = "vel tempor augue delenit".to_string();
///
/// let failures = verify_that(some_text).named("my_thing")
///     .starts_with("nibh")
///     .ends_with("magna")
///     .display_failures();
///
/// assert_that!(failures).contains_exactly([
///     r#"assertion failed: expected my_thing to start with "nibh"
///    but was: "vel tempor augue delenit"
///   expected: "nibh"
/// "#,
///     r#"assertion failed: expected my_thing to end with "magna"
///    but was: "vel tempor augue delenit"
///   expected: "magna"
/// "#,
/// ]);
/// ```
#[track_caller]
pub const fn verify_that<'a, S>(subject: S) -> Spec<'a, S, CollectFailures> {
    Spec::new(subject, CollectFailures)
}

/// Starts an assertion for some piece of code in the [`PanicOnFail`] mode.
///
/// It takes a closure and wraps it into a [`Spec`]. On the [`Spec`] any
/// assertion method that is implemented for closures can be called.
///
/// Assertions started with `assert_that_code()` will panic on the first failing
/// assertion.
///
/// In comparison to using the macro [`assert_that_code!`](crate::assert_that_code)
/// calling this function does not set a name for the expression and does not
/// set the code location of the assertion. In failure messages the generic word
/// "the closure" is used. To set a specific text for the expression the method
/// [`named`](Spec::named) must be called explicitly.
///
/// ## Examples
///
/// ```
/// use asserting::prelude::*;
///
/// fn divide(a: i32, b: i32) -> i32 {
///     a / b
/// }
///
/// assert_that_code(|| { divide(7, 0); })
///     .panics_with_message("attempt to divide by zero");
///
/// assert_that_code(|| { divide(7, 3); }).does_not_panic();
/// ```
#[cfg(feature = "panic")]
pub fn assert_that_code<'a, S>(code: S) -> Spec<'a, Code<S>, PanicOnFail>
where
    S: FnOnce(),
{
    Spec::new(Code::from(code), PanicOnFail).named("the closure")
}

/// Starts an assertion for some piece of code in the [`CollectFailures`] mode.
///
/// It takes a closure and wraps it into a [`Spec`]. On the [`Spec`] any
/// assertion method that is implemented for closures can be called.
///
/// Assertions started with `verify_that_code()` will collect [`AssertFailure`]s
/// for all failing assertions. The collected failures can be queried by calling
/// one of the methods [`failures`](Spec::failures) or
/// [`display_failures`](Spec::display_failures) on the [`Spec`].
///
/// In comparison to using the macro [`verify_that_code!`](crate::verify_that_code)
/// calling this function does not set a name for the expression and does not
/// set the code location of the assertion. In failure messages the generic word
/// "the closure" is used. To set a specific text for the expression the method
/// [`named`](Spec::named) must be called explicitly.
///
/// ## Examples
///
/// ```
/// use asserting::prelude::*;
///
/// fn divide(a: i32, b: i32) -> i32 {
///     a / b
/// }
///
/// let failures = verify_that_code(|| { divide(7, 3); })
///     .does_not_panic()
///     .failures();
///
/// assert_that!(failures).is_empty();
///
/// let failures = verify_that_code(|| { divide(7, 0); })
///     .does_not_panic()
///     .display_failures();
///
/// assert_that!(failures).contains_exactly([
///     r#"assertion failed: expected the closure to not panic, but did panic
///   with message: "attempt to divide by zero"
/// "#
/// ]);
///
/// let failures = verify_that_code(|| { divide(7, 0); })
///     .panics_with_message("division by zero")
///     .display_failures();
///
/// assert_that!(failures).contains_exactly([
///     r#"assertion failed: expected the closure to panic with message "division by zero"
///    but was: "attempt to divide by zero"
///   expected: "division by zero"
/// "#
/// ]);
/// ```
#[cfg(feature = "panic")]
pub fn verify_that_code<'a, S>(code: S) -> Spec<'a, Code<S>, CollectFailures>
where
    S: FnOnce(),
{
    Spec::new(Code::from(code), CollectFailures).named("the closure")
}

pub trait Expectation<S: ?Sized> {
    fn test(&mut self, subject: &S) -> bool;

    fn message(&self, expression: Expression<'_>, actual: &S) -> String;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Expression<'a>(pub &'a str);

impl Default for Expression<'_> {
    fn default() -> Self {
        Self("subject")
    }
}

impl Display for Expression<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Expression<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0
    }
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
        #[cfg(not(test))]
        let file = self.file;
        #[cfg(test)]
        let file = self.file.replace('\\', "/");
        write!(f, "{file}:{}:{}", self.line, self.column)
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

/// Code location.
///
/// # Related
/// - [`core::panic::Location`]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OwnedLocation {
    pub file: String,
    pub line: u32,
    pub column: u32,
}

impl Display for OwnedLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(not(test))]
        let file = self.file.clone();
        #[cfg(test)]
        let file = self.file.replace('\\', "/");
        write!(f, "{file}:{}:{}", self.line, self.column)
    }
}

impl OwnedLocation {
    #[must_use]
    pub fn new(file: impl Into<String>, line: u32, column: u32) -> Self {
        Self {
            file: file.into(),
            line,
            column,
        }
    }
}

impl From<Location<'_>> for OwnedLocation {
    fn from(value: Location<'_>) -> Self {
        Self {
            file: value.file.into(),
            line: value.line,
            column: value.column,
        }
    }
}

impl OwnedLocation {
    #[must_use]
    pub fn file(&self) -> &str {
        &self.file
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
    expression: Option<Expression<'a>>,
    description: Option<&'a str>,
    location: Option<Location<'a>>,
    failures: Vec<AssertFailure>,
    failing_strategy: R,
}

impl<S, R> Spec<'_, S, R> {
    #[must_use = "a spec does nothing unless an assertion method is called"]
    pub const fn new(subject: S, failing_strategy: R) -> Self {
        Self {
            subject,
            expression: None,
            description: None,
            location: None,
            failures: vec![],
            failing_strategy,
        }
    }

    pub const fn subject(&self) -> &S {
        &self.subject
    }

    pub const fn expression(&self) -> Option<Expression<'_>> {
        self.expression
    }

    pub const fn location(&self) -> Option<Location<'_>> {
        self.location
    }

    pub const fn description(&self) -> Option<&str> {
        self.description
    }

    pub const fn failing_strategy(&self) -> &R {
        &self.failing_strategy
    }

    pub fn failures(&self) -> Vec<AssertFailure> {
        self.failures.clone()
    }

    pub fn display_failures(&self) -> Vec<String> {
        self.failures.iter().map(ToString::to_string).collect()
    }
}

impl<'a, S, R> Spec<'a, S, R> {
    #[must_use = "a spec does nothing unless an assertion method is called"]
    pub const fn named(mut self, subject_name: &'a str) -> Self {
        self.expression = Some(Expression(subject_name));
        self
    }

    #[must_use = "a spec does nothing unless an assertion method is called"]
    pub const fn described_as(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    #[must_use = "a spec does nothing unless an assertion method is called"]
    pub const fn located_at(mut self, location: Location<'a>) -> Self {
        self.location = Some(location);
        self
    }

    pub fn extracting<F, U>(self, extractor: F) -> Spec<'a, U, R>
    where
        F: FnOnce(S) -> U,
    {
        self.mapping(extractor)
    }

    pub fn mapping<F, U>(self, mapper: F) -> Spec<'a, U, R>
    where
        F: FnOnce(S) -> U,
    {
        Spec {
            subject: mapper(self.subject),
            expression: self.expression,
            description: self.description,
            location: self.location,
            failures: self.failures,
            failing_strategy: self.failing_strategy,
        }
    }
}

impl<S, R> Spec<'_, S, R>
where
    R: FailingStrategy,
{
    #[allow(clippy::needless_pass_by_value, clippy::return_self_not_must_use)]
    #[track_caller]
    pub fn expecting(mut self, mut expectation: impl Expectation<S>) -> Self {
        if !expectation.test(&self.subject) {
            let expression = self.expression.unwrap_or_default();
            let message = expectation.message(expression, &self.subject);
            self.do_fail_with_message(message);
        }
        self
    }

    #[allow(clippy::return_self_not_must_use)]
    #[track_caller]
    pub fn satisfies<P>(self, predicate: P) -> Self
    where
        P: Fn(&S) -> bool,
    {
        self.expecting(Predicate {
            predicate,
            message: None,
        })
    }

    #[allow(clippy::return_self_not_must_use)]
    #[track_caller]
    pub fn satisfies_with_message<P>(self, message: impl Into<String>, predicate: P) -> Self
    where
        P: Fn(&S) -> bool,
    {
        self.expecting(Predicate {
            predicate,
            message: Some(message.into()),
        })
    }

    #[track_caller]
    fn do_fail_with_message(&mut self, message: impl Into<String>) {
        let message = message.into();
        let failure = AssertFailure {
            description: self.description.map(ToOwned::to_owned),
            message,
            location: self.location.map(OwnedLocation::from),
        };
        self.failures.push(failure);
        self.failing_strategy.do_fail_with(&self.failures);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssertFailure {
    description: Option<String>,
    message: String,
    location: Option<OwnedLocation>,
}

impl Display for AssertFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.description {
            None => {
                writeln!(f, "assertion failed: {}", &self.message)?;
            },
            Some(description) => {
                writeln!(f, "assertion failed: {description}\n{}", &self.message)?;
            },
        }
        Ok(())
    }
}

#[allow(clippy::must_use_candidate)]
impl AssertFailure {
    pub const fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub const fn location(&self) -> Option<&OwnedLocation> {
        self.location.as_ref()
    }
}

pub trait FailingStrategy {
    fn do_fail_with(&self, failures: &[AssertFailure]);
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PanicOnFail;

impl FailingStrategy for PanicOnFail {
    #[track_caller]
    fn do_fail_with(&self, failures: &[AssertFailure]) {
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
    fn do_fail_with(&self, _failures: &[AssertFailure]) {
        // do nothing by design
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

#[cfg(feature = "panic")]
pub use code::Code;

#[cfg(feature = "panic")]
mod code {
    use std::cell::RefCell;
    use std::rc::Rc;

    pub struct Code<F>(Rc<RefCell<Option<F>>>);

    impl<F> From<F> for Code<F>
    where
        F: FnOnce(),
    {
        fn from(value: F) -> Self {
            Self(Rc::new(RefCell::new(Some(value))))
        }
    }

    impl<F> Code<F> {
        #[must_use]
        pub fn take(&self) -> Option<F> {
            self.0.borrow_mut().take()
        }
    }
}

#[cfg(test)]
mod tests;
