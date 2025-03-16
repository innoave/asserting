//! This is the core of the `asserting` crate.

use crate::expectations::Predicate;
use crate::std::error::Error as StdError;
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
/// # Example
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
/// # Example
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
/// # Examples
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
/// # Examples
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
/// # Examples
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
/// # Examples
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
/// # Examples
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
/// # Examples
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

/// An expectation defines a test for a property of the asserted subject.
///
/// It requires two methods: a `test()` method and a `message()` method.
/// The `test()` method is called to verify whether an actual subject meets the
/// expected property. In case the test of the expectation fails the `message()`
/// method is called to form an expectation specific failure message.
pub trait Expectation<S: ?Sized> {
    /// Verifies whether the actual subject fulfills the expected property.
    fn test(&mut self, subject: &S) -> bool;

    /// Forms a failure message for this expectation.
    fn message(&self, expression: Expression<'_>, actual: &S) -> String;
}

/// A textual representation of the expression or subject that is being
/// asserted.
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

/// The location of an assertion in the source code respectively test code.
///
/// # Related
/// - [`core::panic::Location`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Location<'a> {
    /// The file path of the source file where the assertion is located.
    pub file: &'a str,

    /// The line number within the source file where the assertion is located.
    pub line: u32,

    /// The column number on the line within the source file where the assertion
    /// is located.
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
    /// Constructs a new `Location` with the given file, line and column.
    #[must_use]
    pub const fn new(file: &'a str, line: u32, column: u32) -> Self {
        Self { file, line, column }
    }
}

impl Location<'_> {
    /// Returns the file path of this location.
    #[must_use]
    pub const fn file(&self) -> &str {
        self.file
    }

    /// Returns the line number of this location.
    #[must_use]
    pub const fn line(&self) -> u32 {
        self.line
    }

    /// Returns the column number of this location.
    #[must_use]
    pub const fn column(&self) -> u32 {
        self.column
    }
}

/// An owned location in the source code respectively test code.
///
/// It is basically the same as [`Location`] but uses owned types instead of
/// borrowed types for its fields.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OwnedLocation {
    /// The file path of the source file where the assertion is located.
    pub file: String,

    /// The line number within the source file where the assertion is located.
    pub line: u32,

    /// The column number on the line within the source file where the assertion
    /// is located.
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
    /// Constructs a new `OwnedLocation` with the given file, line and column.
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
    /// Returns the file path of this location.
    #[must_use]
    pub fn file(&self) -> &str {
        &self.file
    }

    #[must_use]
    /// Returns the line number of this location.
    pub const fn line(&self) -> u32 {
        self.line
    }

    #[must_use]
    /// Returns the column number of this location.
    pub const fn column(&self) -> u32 {
        self.column
    }
}

/// Data of an actual assertion.
///
/// It holds the data needed to execute an assertion such as the subject,
/// the name of the subject or expression, an optional description of the
/// current assertion and the location of the assertion in the source code
/// respectively test code.
///
/// It also holds the concrete [`FailingStrategy`] on how to behave in case
/// an assertion fails.
///
/// In case of the [`CollectFailures`] failing strategy the [`AssertFailure`]s
/// are collected in this struct.
pub struct Spec<'a, S, R> {
    subject: S,
    expression: Option<Expression<'a>>,
    description: Option<&'a str>,
    location: Option<Location<'a>>,
    failures: Vec<AssertFailure>,
    failing_strategy: R,
}

impl<S, R> Spec<'_, S, R> {
    /// Constructs a new `Spec` for the given subject and with the specified
    /// failing strategy.
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

    /// Returns the subject.
    pub const fn subject(&self) -> &S {
        &self.subject
    }

    /// Returns the expression (or subject name) if one has been set.
    pub const fn expression(&self) -> Option<Expression<'_>> {
        self.expression
    }

    /// Returns the location in source code or test code if it has been set.
    pub const fn location(&self) -> Option<Location<'_>> {
        self.location
    }

    /// Returns the description or the assertion if it has been set.
    pub const fn description(&self) -> Option<&str> {
        self.description
    }

    /// Returns the failing strategy that is used in case an assertion fails.
    pub const fn failing_strategy(&self) -> &R {
        &self.failing_strategy
    }

    /// Returns the assertion failures that have been collected so far.
    pub fn failures(&self) -> Vec<AssertFailure> {
        self.failures.clone()
    }

    /// Returns the assertion failures collected so far formatted as text.
    pub fn display_failures(&self) -> Vec<String> {
        self.failures.iter().map(ToString::to_string).collect()
    }
}

impl<'a, S, R> Spec<'a, S, R> {
    /// Sets the subject name or expression for this assertion.
    #[must_use = "a spec does nothing unless an assertion method is called"]
    pub const fn named(mut self, subject_name: &'a str) -> Self {
        self.expression = Some(Expression(subject_name));
        self
    }

    /// Sets a custom description about what is being asserted.
    #[must_use = "a spec does nothing unless an assertion method is called"]
    pub const fn described_as(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    /// Sets the location of the assertion in the source code respectively test
    /// code.
    #[must_use = "a spec does nothing unless an assertion method is called"]
    pub const fn located_at(mut self, location: Location<'a>) -> Self {
        self.location = Some(location);
        self
    }

    /// Maps the current subject to some other value.
    ///
    /// It takes a closure that maps the current subject to a new subject and
    /// returns a new `Spec` with the value returned by the closure as the new
    /// subject. The new subject may have a different type than the original
    /// subject. All other data like expression, description and location are
    /// taken over from this `Spec` into the returned `Spec`.
    ///
    /// This function is useful when having a custom type and some specific
    /// property of this type shall be asserted only.
    ///
    /// This is an alias function to the [`mapping()`](Spec::mapping) function.
    /// Both functions do exactly the same. The idea is to provide different
    /// names to be able to express the intent more clearly when used in
    /// assertions.
    ///
    /// # Example
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// struct MyStruct {
    ///     important_property: String,
    ///     other_property: f64,
    /// }
    ///
    /// let some_thing = MyStruct {
    ///     important_property: "imperdiet aliqua zzril eiusmod".into(),
    ///     other_property: 99.9,
    /// };
    ///
    /// assert_that!(some_thing).extracting(|s| s.important_property)
    ///     .is_equal_to("imperdiet aliqua zzril eiusmod");
    ///
    /// ```
    pub fn extracting<F, U>(self, extractor: F) -> Spec<'a, U, R>
    where
        F: FnOnce(S) -> U,
    {
        self.mapping(extractor)
    }

    /// Maps the current subject to some other value.
    ///
    /// It takes a closure that maps the current subject to a new subject and
    /// returns a new `Spec` with the value returned by the closure as the new
    /// subject. The new subject may have a different type than the original
    /// subject. All other data like expression, description and location are
    /// taken over from this `Spec` into the returned `Spec`.
    ///
    /// This function is useful if some type does not implement a trait that is
    /// required for an assertion.
    ///
    /// `Spec` also provides the [`extracting()`](Spec::extracting) function,
    /// which is an alias to this function. Both functions do exactly the same.
    /// Choose that function of which its name expresses the intent more
    /// clearly.
    ///
    /// # Example
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// struct Point {
    ///     x: i64,
    ///     y: i64,
    /// }
    ///
    /// let target = Point { x: 12, y: -64 };
    ///
    /// assert_that!(target).mapping(|s| (s.x, s.y)).is_equal_to((12, -64));
    /// ```
    ///
    /// The custom type `Point` does not implement the `PartialEq` trait nor
    /// the `Debug` trait, which are both required for an `is_equal_to`
    /// assertion. So we map the subject of the type `Point` to a tuple of its
    /// fields.
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
    /// Asserts the given expectation.
    ///
    /// In case the expectation is not meet it does fail according the current
    /// failing strategy of this `Spec`.
    ///
    /// This method is called from the implementations of the assertion traits
    /// defined in the [`assertions`](crate::assertions) module. Implementations
    /// of custom assertions will call this method with a proper expectation.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::expectations::{IsEmpty, IsEqualTo};
    /// use asserting::prelude::*;
    ///
    /// assert_that!(7 * 6).expecting(IsEqualTo {expected: 42 });
    ///
    /// assert_that!("").expecting(IsEmpty);
    /// ```
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

    /// Asserts whether the given predicate is meet.
    ///
    /// This method takes a predicate function and calls it as an expectation.
    /// In case the predicate function returns false, it does fail with a
    /// generic failure message and according to the current failing strategy of
    /// this `Spec`.
    ///
    /// This method can be used to do simple custom assertions without
    /// implementing an [`Expectation`] and an assertion trait.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// fn is_odd(value: &i32) -> bool {
    ///     value & 1 == 1
    /// }
    ///
    /// assert_that!(37).satisfies(is_odd);
    ///
    /// let failures = verify_that!(22).satisfies(is_odd).display_failures();
    ///
    /// assert_that!(failures).contains_exactly([
    ///     "assertion failed: expected 22 to satisfy the given predicate, but returned false\n"
    /// ]);
    /// ```
    ///
    /// To assert a predicate with a custom failure message instead of the
    /// generic one use the method
    /// [`satisfies_with_message`](Spec::satisfies_with_message).
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
    /// Asserts whether the given predicate is meet.
    ///
    /// This method takes a predicate function and calls it as an expectation.
    /// In case the predicate function returns false, it does fail with the
    /// provided failure message and according to the current failing strategy
    /// of this `Spec`.
    ///
    /// This method can be used to do simple custom assertions without
    /// implementing an [`Expectation`] and an assertion trait.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// fn is_odd(value: &i32) -> bool {
    ///     value & 1 == 1
    /// }
    ///
    /// assert_that!(37).satisfies_with_message("expected my number to be odd", is_odd);
    ///
    /// let failures = verify_that!(22)
    ///         .satisfies_with_message("expected my number to be odd", is_odd)
    ///         .display_failures();
    ///
    /// assert_that!(failures).contains_exactly([
    ///     "assertion failed: expected my number to be odd\n"
    /// ]);
    /// ```
    ///
    /// To assert a predicate with a generic failure message instead of
    /// providing one use the method
    /// [`satisfies`](Spec::satisfies).
    pub fn satisfies_with_message<P>(self, message: impl Into<String>, predicate: P) -> Self
    where
        P: Fn(&S) -> bool,
    {
        self.expecting(Predicate {
            predicate,
            message: Some(message.into()),
        })
    }

    /// Fails the assertion according the current failing strategy of this
    /// `Spec`.
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

/// An error describing a failed assertion.
///
/// This struct implements the [`std::error::Error`] trait.
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

impl StdError for AssertFailure {}

#[allow(clippy::must_use_candidate)]
impl AssertFailure {
    /// Returns the description of the assertion that failed.
    pub const fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Returns the failure message of the assertion that failed.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the location of the assertion in the source code / test code if
    /// it has been set in the [`Spec`].
    pub const fn location(&self) -> Option<&OwnedLocation> {
        self.location.as_ref()
    }
}

/// Defines the behavior when an assertion fails.
///
/// This crate provides two implementations:
///
/// * [`PanicOnFail`] - panics when an assertion fails
/// * [`CollectFailures`] - collects [`AssertFailure`]s of assertions that have failed.
pub trait FailingStrategy {
    /// Reacts to an assertion that has failed with the [`AssertFailure`]s given
    /// as argument.
    fn do_fail_with(&self, failures: &[AssertFailure]);
}

/// [`FailingStrategy`] that panics when an assertion fails.
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

/// [`FailingStrategy`] that collects the failures from failing assertions.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct CollectFailures;

impl FailingStrategy for CollectFailures {
    fn do_fail_with(&self, _failures: &[AssertFailure]) {
        // do nothing by design
    }
}

/// Used with generic types in expectations where the concrete type is not
/// relevant for the failure message.
///
/// This type implements the std format trait [`std::fmt::Debug`] and
/// [`std::fmt::Display`] which both format the value as "_".
///
/// ```
/// # use asserting::prelude::*;
/// # use asserting::spec::Unknown;
/// assert_that!(format!("{:?}", Unknown)).is_equal_to("_");
/// assert_that!(format!("{}", Unknown)).is_equal_to("_");
/// ```
///
/// # Examples
///
/// This type is used to implement the expectations
///
/// * [`IsSome`](crate::expectations::IsSome)
/// * [`IsNone`](crate::expectations::IsNone)
/// * [`IsOk`](crate::expectations::IsOk)
/// * [`IsErr`](crate::expectations::IsErr)
///
/// For example for implementing the function [`Expectation::message()`] for the
/// [`IsOk`](crate::expectations::IsOk) expectation for `Result<T, E>` the
/// concrete types for `T` and `E` are not relevant. The implementation of the
/// trait looks like this:
///
/// ```no_run
/// # use std::fmt::Debug;
/// # use asserting::spec::Expectation;
/// # use asserting::spec::Expression;
/// # use asserting::spec::Unknown;
/// # struct IsOk;
/// impl<T, E> Expectation<Result<T, E>> for IsOk
/// where
///     T: Debug,
///     E: Debug,
/// {
///     fn test(&mut self, subject: &Result<T, E>) -> bool {
///         subject.is_ok()
///     }
///
///     fn message(&self, expression: Expression<'_>, actual: &Result<T, E>) -> String {
///         format!(
///             "expected {expression} is {:?}\n   but was: {actual:?}\n  expected: {:?}",
///             Ok::<_, Unknown>(Unknown),
///             Ok::<_, Unknown>(Unknown),
///         )
///     }
/// }
/// ```
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

    /// Wrapper type that holds a closure as code snippet.
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
        /// Takes the closure out of this `Code` leaving it empty.
        #[must_use]
        pub fn take(&self) -> Option<F> {
            self.0.borrow_mut().take()
        }
    }
}

#[cfg(test)]
mod tests;
