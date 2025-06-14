//! Definitions of the expectations that are provided by this crate.

#![allow(missing_docs)]
#![warn(clippy::return_self_not_must_use)]

use crate::std::marker::PhantomData;
use crate::std::{string::String, vec::Vec};
use hashbrown::HashSet;

/// Combinator for expectations that inverts the contained expectation.
///
/// This combinator can only be used with expectations that implement the
/// [`Invertible`] trait (additional to the [`Expectation`] trait).
///
/// Most of the expectations provided by this crate do implement the
/// [`Invertible`] trait and thus can be used with the [`Not`] combinator.
///
/// # Examples
///
/// ```
/// use asserting::expectations::{HasLength, IsEmpty, IsEqualTo, IsNegative, StringContains};
/// use asserting::prelude::*;
///
/// assert_that!(41).expecting(Not(IsEqualTo { expected: 42 }));
/// assert_that!([1, 2, 3]).expecting(Not(IsEmpty));
/// assert_that!(37.9).expecting(Not(IsNegative));
/// assert_that!([1, 2, 3]).expecting(Not(HasLength { expected_length: 4 }));
/// assert_that!("almost").expecting(Not(StringContains { expected: "entire" }));
/// ```
///
/// [`Expectation`]: crate::spec::Expectation
/// [`Invertible`]: crate::spec::Invertible
#[must_use]
pub struct Not<E>(pub E);

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

mod combinators {
    use crate::expectations::Not;
    use crate::spec::{DiffFormat, Expectation, Expression, Invertible};
    use crate::std::string::String;

    impl<S, E> Expectation<S> for Not<E>
    where
        E: Invertible + Expectation<S>,
    {
        fn test(&mut self, subject: &S) -> bool {
            !self.0.test(subject)
        }

        fn message(
            &self,
            expression: &Expression<'_>,
            actual: &S,
            inverted: bool,
            format: &DiffFormat,
        ) -> String {
            self.0.message(expression, actual, !inverted, format)
        }
    }
}
