//! Definitions of the expectations that are provided by this crate.

#![allow(missing_docs)]
#![warn(clippy::return_self_not_must_use)]

use crate::std::marker::PhantomData;
use crate::std::{string::String, vec::Vec};
use hashbrown::HashSet;

/// Creates a [`Not`] expectation combinator wrapping the given expectation.
///
/// # Examples
///
/// ```
/// use asserting::expectations::{HasLength, IsEmpty, IsEqualTo, IsNegative, StringContains};
/// use asserting::prelude::*;
///
/// assert_that!(41).expecting(not(IsEqualTo { expected: 42 }));
/// assert_that!([1, 2, 3]).expecting(not(IsEmpty));
/// assert_that!(37.9).expecting(not(IsNegative));
/// assert_that!([1, 2, 3]).expecting(not(HasLength { expected_length: 4 }));
/// assert_that!("almost").expecting(not(StringContains { expected: "entire" }));
/// ```
pub fn not<E>(expectation: E) -> Not<E> {
    Not(expectation)
}

/// A combinator expectation that inverts the wrapped expectation.
///
/// This combinator can only be used with expectations that implement the
/// [`Invertible`] trait (additional to the [`Expectation`] trait).
///
/// Most of the expectations provided by this crate do implement the
/// [`Invertible`] trait and thus can be used with the `Not` combinator.
///
/// Use the function [`not()`] to construct a `Not` combinator containing the
/// given expectation.
///
/// [`Expectation`]: crate::spec::Expectation
/// [`Invertible`]: crate::spec::Invertible
#[must_use]
pub struct Not<E>(pub E);

/// Creates an [`All`] expectation combinator from a tuple of expectations.
///
/// # Examples
///
/// ```
/// use asserting::expectations::{IsAtMost, IsPositive};
/// use asserting::prelude::*;
///
/// let custom_expectation = all((IsPositive, IsAtMost { expected: 99 }));
///
/// assert_that(42).expecting(custom_expectation);  
/// ```
pub fn all<A>(expectations: A) -> All<A::Output>
where
    A: IntoRec,
{
    All(expectations.into_rec())
}

/// A combinator expectation that verifies that all containing expectations are
/// met.
///
/// Use the function [`all()`] to construct an `All` combinator for a tuple of
/// expectations.
#[must_use]
pub struct All<E>(pub E);

/// Creates a [`Rec`] expectation combinator that wraps the given expectation.
///
/// This is a convenience function that is equivalent to `Rec::new()`.
pub fn rec<E>(expectations: E) -> Rec<E> {
    Rec::new(expectations)
}

/// Creates an [`Any`] expectation combinator from a tuple of expectations.
///
/// # Examples
///
/// ```
/// use asserting::expectations::{IsEmpty, StringContains};
/// use asserting::prelude::*;
///
/// let custom_expectation = any((not(IsEmpty), StringContains { expected: "unfugiaty" }));
///
/// assert_that("elit fugiat dolores").expecting(custom_expectation);
/// ```
pub fn any<A>(expectations: A) -> Any<A::Output>
where
    A: IntoRec,
{
    Any(expectations.into_rec())
}

/// A combinator expectation that verifies that any containing expectation is
/// met.
///
/// Use the function [`any()`] to construct an `Any` combinator for a tuple of
/// expectations.
pub struct Any<E>(pub E);

/// A combinator expectation that memorizes ("records") the result of the
/// wrapped expectation.
///
/// Use the function [`rec()`] to conveniently wrap an expectation into the
/// `Rec` combinator.
///
/// # Examples
///
/// ```
/// use asserting::prelude::*;
/// use asserting::expectations::{IsNegative, rec};
/// use asserting::spec::Expectation;
///
/// // the result of new `Rec` is neither `success` nor `failure`
/// let mut expectation = rec(IsNegative);
/// assert_that(expectation.is_failure()).is_false();
/// assert_that(expectation.is_success()).is_false();
///
/// // once the `test` method has been called, the result can be queried at a
/// // later time.
/// _ = expectation.test(&-42);  // returns true
/// assert_that(expectation.is_success()).is_true();
/// assert_that(expectation.is_failure()).is_false();
///
/// // once the `test` method has been called, the result can be queried at a
/// // later time.
/// _= expectation.test(&42);  // returns false
/// assert_that(expectation.is_success()).is_false();
/// assert_that(expectation.is_failure()).is_true();
/// ```
#[must_use]
pub struct Rec<E> {
    pub expectation: E,
    pub result: Option<bool>,
}

impl<E> Rec<E> {
    /// Creates a new ìnstance of `Rec` that wraps the given expectation.
    pub fn new(expectation: E) -> Self {
        Self {
            expectation,
            result: None,
        }
    }

    /// Returns true if the `test` method has been called and the result of the
    /// wrapped expectation was true ("success") and false otherwise.
    pub fn is_success(&self) -> bool {
        self.result.is_some_and(|r| r)
    }

    /// Returns true if the `test` method has been called and the result of the
    /// wrapped expectation was false ("failure") and false otherwise.
    pub fn is_failure(&self) -> bool {
        self.result.is_some_and(|r| !r)
    }
}

/// Trait to convert a type into another type that wraps the contained
/// expectation(s) into `Rec`(s).
///
/// If this type contains multiple expectations like `Vec<E: Expectation>` or
/// tuples of expectations, each expectation should be wrapped into its own
/// `Rec`.
pub trait IntoRec {
    /// The result type with the expectation(s) wrapped into [`Rec`].
    type Output;

    /// Wraps an expectation of this type into [`Rec`].
    ///
    /// If this type contains multiple expectations like `Vec<E: Expectation` or
    /// tuples of expectations, each expectation should be wrapped into its own
    /// [`Rec`].
    fn into_rec(self) -> Self::Output;
}

#[must_use]
pub struct Predicate<F> {
    pub predicate: F,
    pub message: Option<String>,
}

#[must_use]
pub struct IsTrue;

#[must_use]
pub struct IsFalse;

#[must_use]
pub struct IsEqualTo<E> {
    pub expected: E,
}

#[must_use]
pub struct IsCloseTo<E, M> {
    pub expected: E,
    pub margin: M,
}

impl<E, M> IsCloseTo<E, M>
where
    M: Default,
{
    pub fn new(expected: E) -> Self {
        Self {
            expected,
            margin: M::default(),
        }
    }
}

impl<E, M> IsCloseTo<E, M> {
    pub fn within_margin(mut self, margin: impl Into<M>) -> Self {
        self.margin = margin.into();
        self
    }
}

#[must_use]
pub struct IsLessThan<E> {
    pub expected: E,
}

#[must_use]
pub struct IsAtMost<E> {
    pub expected: E,
}

#[must_use]
pub struct IsGreaterThan<E> {
    pub expected: E,
}

#[must_use]
pub struct IsAtLeast<E> {
    pub expected: E,
}

#[must_use]
pub struct IsBefore<E> {
    pub expected: E,
}

#[must_use]
pub struct IsAfter<E> {
    pub expected: E,
}

#[must_use]
pub struct IsBetween<E> {
    pub min: E,
    pub max: E,
}

#[must_use]
pub struct IsInRange<R, E> {
    pub expected_range: R,
    _element_type: PhantomData<E>,
}

impl<R, E> IsInRange<R, E> {
    pub fn new(expected_range: R) -> Self {
        Self {
            expected_range,
            _element_type: PhantomData,
        }
    }
}

#[must_use]
pub struct IsNegative;

#[must_use]
pub struct IsPositive;

#[must_use]
pub struct IsZero;

#[must_use]
pub struct IsOne;

#[must_use]
pub struct IsFinite;

#[must_use]
pub struct IsInfinite;

#[must_use]
pub struct IsANumber;

#[must_use]
pub struct HasPrecisionOf {
    pub expected_precision: u64,
}

#[must_use]
pub struct HasScaleOf {
    pub expected_scale: i64,
}

#[must_use]
pub struct IsInteger;

#[must_use]
pub struct IsLowerCase;

#[must_use]
pub struct IsUpperCase;

#[must_use]
pub struct IsAscii;

#[must_use]
pub struct IsAlphabetic;

#[must_use]
pub struct IsAlphanumeric;

#[must_use]
pub struct IsControlChar;

#[must_use]
pub struct IsDigit {
    pub radix: u32,
}

#[must_use]
pub struct IsWhitespace;

#[must_use]
pub struct IsSome;

#[must_use]
pub struct IsNone;

#[must_use]
pub struct HasValue<E> {
    pub expected: E,
}

#[must_use]
pub struct IsOk;

#[must_use]
pub struct IsErr;

#[must_use]
pub struct HasError<E> {
    pub expected: E,
}

#[must_use]
pub struct ErrorHasSource;

#[must_use]
pub struct ErrorHasSourceMessage {
    pub expected_source_message: String,
}

#[must_use]
pub struct IsEmpty;

#[must_use]
pub struct HasLength<E> {
    pub expected_length: E,
}

#[must_use]
pub struct HasLengthInRange<R, E> {
    pub expected_range: R,
    _element_type: PhantomData<E>,
}

impl<R, E> HasLengthInRange<R, E> {
    pub fn new(expected_range: R) -> Self {
        Self {
            expected_range,
            _element_type: PhantomData,
        }
    }
}

#[must_use]
pub struct HasLengthLessThan<E> {
    pub expected_length: E,
}

#[must_use]
pub struct HasLengthGreaterThan<E> {
    pub expected_length: E,
}

#[must_use]
pub struct HasAtMostLength<E> {
    pub expected_length: E,
}

#[must_use]
pub struct HasAtLeastLength<E> {
    pub expected_length: E,
}

#[must_use]
pub struct HasCharCount<E> {
    pub expected_char_count: E,
}

#[must_use]
pub struct HasCharCountInRange<R, E> {
    pub expected_range: R,
    _element_type: PhantomData<E>,
}

impl<R, E> HasCharCountInRange<R, E> {
    pub fn new(expected_range: R) -> Self {
        Self {
            expected_range,
            _element_type: PhantomData,
        }
    }
}

#[must_use]
pub struct HasCharCountLessThan<E> {
    pub expected_char_count: E,
}

#[must_use]
pub struct HasCharCountGreaterThan<E> {
    pub expected_char_count: E,
}

#[must_use]
pub struct HasAtMostCharCount<E> {
    pub expected_char_count: E,
}

#[must_use]
pub struct HasAtLeastCharCount<E> {
    pub expected_char_count: E,
}

#[must_use]
pub struct StringContains<E> {
    pub expected: E,
}

#[must_use]
pub struct StringContainsAnyOf<E> {
    pub expected: E,
}

#[must_use]
pub struct StringStartWith<E> {
    pub expected: E,
}

#[must_use]
pub struct StringEndsWith<E> {
    pub expected: E,
}

#[cfg(feature = "regex")]
#[cfg_attr(docsrs, doc(cfg(feature = "regex")))]
pub use regex::StringMatches;

#[cfg(feature = "regex")]
mod regex {
    use regex::Regex;

    #[must_use]
    pub struct StringMatches<'a> {
        pub pattern: &'a str,
        pub regex: Regex,
    }

    impl<'a> StringMatches<'a> {
        /// Creates a new `StringMatches`-expectation.
        ///
        /// # Panics
        ///
        /// Panics, if the regex pattern is invalid or exceeds the size limit.
        pub fn new(regex_pattern: &'a str) -> Self {
            let regex = Regex::new(regex_pattern)
                .unwrap_or_else(|err| panic!("failed to match string with regex: {err}"));
            Self {
                pattern: regex_pattern,
                regex,
            }
        }
    }
}

#[must_use]
pub struct IterContains<E> {
    pub expected: E,
}

#[must_use]
pub struct IterContainsExactlyInAnyOrder<E> {
    pub expected: Vec<E>,
    pub missing: HashSet<usize>,
    pub extra: HashSet<usize>,
}

impl<E> IterContainsExactlyInAnyOrder<E> {
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            missing: HashSet::new(),
            extra: HashSet::new(),
        }
    }
}

#[must_use]
pub struct IterContainsAnyOf<E> {
    pub expected: Vec<E>,
}

#[must_use]
pub struct IterContainsAllOf<E> {
    pub expected: Vec<E>,
    pub missing: HashSet<usize>,
}

impl<E> IterContainsAllOf<E> {
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            missing: HashSet::new(),
        }
    }
}

#[must_use]
pub struct IterContainsOnly<E> {
    pub expected: Vec<E>,
    pub extra: HashSet<usize>,
}

impl<E> IterContainsOnly<E> {
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            extra: HashSet::new(),
        }
    }
}

#[must_use]
pub struct IterContainsOnlyOnce<E> {
    pub expected: Vec<E>,
    pub extra: HashSet<usize>,
    pub duplicates: HashSet<usize>,
}

impl<E> IterContainsOnlyOnce<E> {
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            extra: HashSet::new(),
            duplicates: HashSet::new(),
        }
    }
}

#[must_use]
pub struct IterContainsExactly<E> {
    pub expected: Vec<E>,
    pub missing: HashSet<usize>,
    pub extra: HashSet<usize>,
    pub out_of_order: HashSet<usize>,
}

impl<E> IterContainsExactly<E> {
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            missing: HashSet::new(),
            extra: HashSet::new(),
            out_of_order: HashSet::new(),
        }
    }
}

#[must_use]
pub struct IterContainsSequence<E> {
    pub expected: Vec<E>,
    pub missing: HashSet<usize>,
    pub extra: HashSet<usize>,
}

impl<E> IterContainsSequence<E> {
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            missing: HashSet::new(),
            extra: HashSet::new(),
        }
    }
}

#[must_use]
pub struct IterContainsAllInOrder<E> {
    pub expected: Vec<E>,
    pub missing: HashSet<usize>,
}

impl<E> IterContainsAllInOrder<E> {
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            missing: HashSet::new(),
        }
    }
}

#[must_use]
pub struct IterStartsWith<E> {
    pub expected: Vec<E>,
    pub missing: HashSet<usize>,
    pub extra: HashSet<usize>,
}

impl<E> IterStartsWith<E> {
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            missing: HashSet::new(),
            extra: HashSet::new(),
        }
    }
}

#[must_use]
pub struct IterEndsWith<E> {
    pub expected: Vec<E>,
    pub missing: HashSet<usize>,
    pub extra: HashSet<usize>,
}

impl<E> IterEndsWith<E> {
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            missing: HashSet::new(),
            extra: HashSet::new(),
        }
    }
}

#[must_use]
pub struct MapContainsKey<E> {
    pub expected_key: E,
}

#[must_use]
pub struct MapContainsValue<E> {
    pub expected_value: E,
}

#[must_use]
pub struct MapContainsKeys<E> {
    pub expected_keys: Vec<E>,
    pub missing: HashSet<usize>,
}

impl<E> MapContainsKeys<E> {
    pub fn new(expected_keys: impl IntoIterator<Item = E>) -> Self {
        Self {
            expected_keys: Vec::from_iter(expected_keys),
            missing: HashSet::new(),
        }
    }
}

#[must_use]
pub struct MapDoesNotContainKeys<E> {
    pub expected_keys: Vec<E>,
    pub extra: HashSet<usize>,
}

impl<E> MapDoesNotContainKeys<E> {
    pub fn new(expected_keys: impl IntoIterator<Item = E>) -> Self {
        Self {
            expected_keys: Vec::from_iter(expected_keys),
            extra: HashSet::new(),
        }
    }
}

#[must_use]
pub struct MapContainsValues<E> {
    pub expected_values: Vec<E>,
    pub missing: HashSet<usize>,
}

impl<E> MapContainsValues<E> {
    pub fn new(expected_values: impl IntoIterator<Item = E>) -> Self {
        Self {
            expected_values: Vec::from_iter(expected_values),
            missing: HashSet::new(),
        }
    }
}

#[must_use]
pub struct MapDoesNotContainValues<E> {
    pub expected_values: Vec<E>,
    pub extra: HashSet<usize>,
}

impl<E> MapDoesNotContainValues<E> {
    pub fn new(expected_values: impl IntoIterator<Item = E>) -> Self {
        Self {
            expected_values: Vec::from_iter(expected_values),
            extra: HashSet::new(),
        }
    }
}

#[must_use]
pub struct MapContainsExactlyKeys<E> {
    pub expected_keys: Vec<E>,
    pub missing: HashSet<usize>,
    pub extra: HashSet<usize>,
}

impl<E> MapContainsExactlyKeys<E> {
    pub fn new(expected_keys: impl IntoIterator<Item = E>) -> Self {
        Self {
            expected_keys: Vec::from_iter(expected_keys),
            missing: HashSet::new(),
            extra: HashSet::new(),
        }
    }
}

#[cfg(feature = "panic")]
#[cfg_attr(docsrs, doc(cfg(feature = "panic")))]
pub use panic::{DoesNotPanic, DoesPanic};

#[cfg(feature = "panic")]
mod panic {
    use std::any::Any;

    #[must_use]
    pub struct DoesPanic {
        pub expected_message: Option<String>,
        pub actual_message: Option<String>,
    }

    impl DoesPanic {
        pub fn with_any_message() -> Self {
            Self {
                expected_message: None,
                actual_message: None,
            }
        }

        pub fn with_message(message: impl Into<String>) -> Self {
            Self {
                expected_message: Some(message.into()),
                actual_message: None,
            }
        }
    }

    #[must_use]
    #[derive(Default)]
    pub struct DoesNotPanic {
        pub actual_message: Option<Box<dyn Any + Send>>,
    }
}
