use core::fmt::Debug;
use core::marker::PhantomData;
use core::ops::Deref;

pub type Subject<'a, T> = Oob<'a, T>;
pub type Expected<'a, T> = Oob<'a, T>;

/// An owned or borrowed value of type `T`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Oob<'a, T> {
    Owned(T),
    Borrowed(&'a T),
}

impl<'a, T> From<&'a T> for Oob<'a, T> {
    fn from(value: &'a T) -> Self {
        Self::Borrowed(value)
    }
}

impl<T> From<T> for Oob<'_, T> {
    fn from(value: T) -> Self {
        Self::Owned(value)
    }
}

impl<T> Deref for Oob<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Oob::Owned(value) => value,
            Oob::Borrowed(value) => value,
        }
    }
}

/// Code location.
///
/// # Related
/// - [`core::panic::Location`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Location<'a> {
    file: &'a str,
    line: u32,
    column: u32,
}

#[must_use = "a spec does nothing unless an assertion method is called"]
pub struct Spec<'s, S, R> {
    subject: Subject<'s, S>,
    subject_name: Option<&'s str>,
    location: Option<Location<'s>>,
    description: Option<&'s str>,
    return_type: PhantomData<R>,
}

impl<'a, S, R> Spec<'a, S, R> {
    pub fn new(subject: impl Into<Subject<'a, S>>) -> Self {
        Self {
            subject: subject.into(),
            subject_name: None,
            location: None,
            description: None,
            return_type: PhantomData,
        }
    }

    pub const fn at_location(mut self, location: Location<'a>) -> Self {
        self.location = Some(location);
        self
    }

    pub const fn asserting(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    pub fn subject(&self) -> &S {
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

    pub fn assertion_with<E>(self, expected: Expected<'a, E>) -> Assertion<'a, S, E, R> {
        Assertion {
            subject: self.subject,
            subject_name: self.subject_name,
            location: self.location,
            description: self.description,
            expected,
            return_type: self.return_type,
        }
    }
}

#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssertionResult<'a, S, E> {
    Passed,
    Failed(Asserted<'a, S, E>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Asserted<'a, S, E> {
    actual: Subject<'a, S>,
    expected: Expected<'a, E>,
}

impl<S, E> Asserted<'_, S, E> {
    pub fn actual(&self) -> &S {
        &self.actual
    }

    pub fn expected(&self) -> &E {
        &self.expected
    }
}

pub struct Assertion<'a, S, E, R> {
    subject: Subject<'a, S>,
    subject_name: Option<&'a str>,
    location: Option<Location<'a>>,
    description: Option<&'a str>,
    expected: Expected<'a, E>,
    return_type: PhantomData<R>,
}

pub trait AssertionStrategy<R> {
    fn passed(self) -> R;

    fn failed(self) -> R;
}

impl<'a, S, E> AssertionStrategy<AssertionResult<'a, S, E>>
    for Assertion<'a, S, E, AssertionResult<'a, S, E>>
{
    fn passed(self) -> AssertionResult<'a, S, E> {
        AssertionResult::Passed
    }

    fn failed(self) -> AssertionResult<'a, S, E> {
        AssertionResult::Failed(Asserted {
            actual: self.subject,
            expected: self.expected,
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
        panic_for_failed_assertion::<S, E>(&Asserted {
            actual: self.subject,
            expected: self.expected,
        });
    }
}

fn panic_for_failed_assertion<S, E>(error: &Asserted<'_, S, E>)
where
    S: Debug,
    E: Debug,
{
    panic!(
        "{:?} is not equal to {:?}\n  expected: {:?}\n    actual: {:?}\n",
        &error.actual, &error.expected, &error.expected, &error.actual
    )
}

pub fn assert_that<'a, T>(subject: impl Into<Subject<'a, T>>) -> Spec<'a, T, ()> {
    Spec::new(subject)
}

pub fn check_that<'a, S, E, R>(
    subject: impl Into<Subject<'a, S>>,
) -> Spec<'a, S, Asserted<'a, S, E>> {
    Spec::new(subject)
}
