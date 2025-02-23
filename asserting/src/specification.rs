use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Deref;

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
            subject_name: self.subject_name,
            location: self.location,
            description: self.description,
            result: AssertionResult {
                actual: self.subject,
                expected,
            },
            return_type: self.return_type,
        }
    }
}

pub struct AssertionResult<'a, S, E> {
    actual: Subject<'a, S>,
    expected: Expected<'a, E>,
}

pub struct Assertion<'a, S, E, R> {
    subject_name: Option<&'a str>,
    location: Option<Location<'a>>,
    description: Option<&'a str>,
    result: AssertionResult<'a, S, E>,
    return_type: PhantomData<R>,
}

pub trait AssertionStrategy<R> {
    fn passed(self) -> R;

    fn failed(self) -> R;
}

impl<'a, S, E> AssertionStrategy<Result<(), AssertionResult<'a, S, E>>>
    for Assertion<'a, S, E, Result<(), AssertionResult<'a, S, E>>>
{
    fn passed(self) -> Result<(), AssertionResult<'a, S, E>> {
        Ok(())
    }

    fn failed(self) -> Result<(), AssertionResult<'a, S, E>> {
        Err(self.result)
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
        panic_for_failed_assertion::<S, E, ()>(&self.result);
    }
}

pub fn panic_for_failed_assertion<S, E, R>(error: &AssertionResult<'_, S, E>)
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
) -> Spec<'a, S, AssertionResult<'a, S, E>> {
    Spec::new(subject)
}
