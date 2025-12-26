//! Definitions of the expectations that are provided by this crate.

#![allow(missing_docs)]
#![warn(clippy::return_self_not_must_use)]

use crate::std::marker::PhantomData;
use crate::std::{string::String, vec::Vec};
use hashbrown::HashSet;
#[cfg(feature = "regex")]
use regex::Regex;

/// Creates a [`Not`] expectation combinator wrapping the given expectation.
///
/// # Examples
///
/// ```
/// use asserting::expectations::{not, HasLength, IsEmpty, IsEqualTo, IsNegative, StringContains};
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
/// use asserting::expectations::{all, IsAtMost, IsPositive};
/// use asserting::prelude::*;
///
/// let custom_expectation = all((IsPositive, IsAtMost { expected: 99 }));
///
/// assert_that!(42).expecting(custom_expectation);
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

/// Creates an [`Any`] expectation combinator from a tuple of expectations.
///
/// # Examples
///
/// ```
/// use asserting::expectations::{any, not, IsEmpty, StringContains};
/// use asserting::prelude::*;
///
/// let custom_expectation = any((not(IsEmpty), StringContains { expected: "unfugiaty" }));
///
/// assert_that!("elit fugiat dolores").expecting(custom_expectation);
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

/// Creates a [`Rec`] expectation combinator that wraps the given expectation.
///
/// This is a convenience function that is equivalent to `Rec::new()`.
pub fn rec<E>(expectations: E) -> Rec<E> {
    Rec::new(expectations)
}

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
/// assert_that!(expectation.is_failure()).is_false();
/// assert_that!(expectation.is_success()).is_false();
///
/// // once the `test` method has been called, the result can be queried at a
/// // later time.
/// _ = expectation.test(&-42);  // returns true
/// assert_that!(expectation.is_success()).is_true();
/// assert_that!(expectation.is_failure()).is_false();
///
/// // once the `test` method has been called, the result can be queried at a
/// // later time.
/// _= expectation.test(&42);  // returns false
/// assert_that!(expectation.is_success()).is_false();
/// assert_that!(expectation.is_failure()).is_true();
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

/// Creates a [`Predicate`] expectation from a predicate function.
///
/// The failure message will contain a generic description of the expectation.
/// To specify a custom description for the expectation, use the method [`Predicate::with_message`]
/// on the newly constructed expectation.
///
/// # Examples
///
/// ```
/// use asserting::expectations::satisfies;
/// use asserting::prelude::*;
///
/// fn is_odd(number: &i32) -> bool {
///     *number & 1 == 1
/// }
///
/// // with a generic description
/// assert_that!(5).expecting(satisfies(is_odd));
///
/// // with a custom description
/// assert_that!(5).expecting(
///     satisfies(is_odd).with_message("my number to be odd")
/// );
/// ```
pub fn satisfies<F>(predicate: F) -> Predicate<F> {
    Predicate {
        predicate,
        message: None,
    }
}

#[must_use]
pub struct Predicate<F> {
    pub predicate: F,
    pub message: Option<String>,
}

impl<F> Predicate<F> {
    /// Sets a custom description of the expectation.
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }
}

/// Creates an [`IsTrue`] expectation.
pub fn is_true() -> IsTrue {
    IsTrue
}

#[must_use]
pub struct IsTrue;

/// Creates a [`IsFalse`] expectation.
pub fn is_false() -> IsFalse {
    IsFalse
}

#[must_use]
pub struct IsFalse;

/// Creates an [`IsEqualTo`] expectation.
pub fn is_equal_to<E>(expected: E) -> IsEqualTo<E> {
    IsEqualTo { expected }
}

#[must_use]
pub struct IsEqualTo<E> {
    pub expected: E,
}

/// Creates an [`IsSameAs`] expectation.
pub fn is_same_as<E>(expected: E) -> IsSameAs<E> {
    IsSameAs { expected }
}

pub struct IsSameAs<E> {
    pub expected: E,
}

/// Creates an [`IsCloseTo`] expectation.
///
/// The margin is set to a default value. To define a custom margin, use the
/// method [`IsCloseTo::within_margin`] on the newly constructed expectation.
///
/// # Examples
///
/// ```
/// # #[cfg(not(feature = "float-cmp"))]
/// # fn main() {}
/// # #[cfg(feature = "float-cmp")]
/// # fn main() {
/// use asserting::expectations::is_close_to;
/// use asserting::prelude::*;
///
/// // using the default margin
/// assert_that!(-2.453).expecting(is_close_to(-2.453));
///
/// // with custom margin
/// assert_that!(-2.453_f32)
///     .expecting(is_close_to(-2.453).within_margin((0.001, 4)));
/// # }
/// ```
pub fn is_close_to<E, M>(expected: E) -> IsCloseTo<E, M>
where
    M: Default,
{
    IsCloseTo {
        expected,
        margin: M::default(),
    }
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
    #[deprecated = "use the function [`is_close_to`] instead"]
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

/// Creates an [`IsLessThan`] expectation.
pub fn is_less_than<E>(expected: E) -> IsLessThan<E> {
    IsLessThan { expected }
}

#[must_use]
pub struct IsLessThan<E> {
    pub expected: E,
}

/// Creates an [`IsAtMost`] expectation.
pub fn is_at_most<E>(expected: E) -> IsAtMost<E> {
    IsAtMost { expected }
}

#[must_use]
pub struct IsAtMost<E> {
    pub expected: E,
}

/// Creates an [`IsGreaterThan`] expectation.
pub fn is_greater_than<E>(expected: E) -> IsGreaterThan<E> {
    IsGreaterThan { expected }
}

#[must_use]
pub struct IsGreaterThan<E> {
    pub expected: E,
}

/// Creates an [`IsAtLeast`] expectation.
pub fn is_at_least<E>(expected: E) -> IsAtLeast<E> {
    IsAtLeast { expected }
}

#[must_use]
pub struct IsAtLeast<E> {
    pub expected: E,
}

/// Creates an [`IsBefore`] expectation.
pub fn is_before<E>(expected: E) -> IsBefore<E> {
    IsBefore { expected }
}

#[must_use]
pub struct IsBefore<E> {
    pub expected: E,
}

/// Creates an [`IsAfter`] expectation.
pub fn is_after<E>(expected: E) -> IsAfter<E> {
    IsAfter { expected }
}

#[must_use]
pub struct IsAfter<E> {
    pub expected: E,
}

/// Creates an [`IsBetween`] expectation.
pub fn is_between<E>(min: E, max: E) -> IsBetween<E> {
    IsBetween { min, max }
}

#[must_use]
pub struct IsBetween<E> {
    pub min: E,
    pub max: E,
}

/// Creates an [`IsInRange`] expectation.
pub fn is_in_range<R, E>(expected_range: R) -> IsInRange<R, E> {
    IsInRange {
        expected_range,
        _element_type: PhantomData,
    }
}

#[must_use]
pub struct IsInRange<R, E> {
    pub expected_range: R,
    _element_type: PhantomData<E>,
}

impl<R, E> IsInRange<R, E> {
    #[deprecated = "use the function [`is_in_range`] instead"]
    pub fn new(expected_range: R) -> Self {
        Self {
            expected_range,
            _element_type: PhantomData,
        }
    }
}

/// Creates an [`IsNegative`] expectation.
pub fn is_negative() -> IsNegative {
    IsNegative
}

#[must_use]
pub struct IsNegative;

/// Creates an [`IsPositive`] expectation.
pub fn is_positive() -> IsPositive {
    IsPositive
}

#[must_use]
pub struct IsPositive;

/// Creates an [`IsZero`] expectation.
pub fn is_zero() -> IsZero {
    IsZero
}

#[must_use]
pub struct IsZero;

/// Creates an [`IsOne`] expectation.
pub fn is_one() -> IsOne {
    IsOne
}

#[must_use]
pub struct IsOne;

/// Creates an [`IsFinite`] expectation.
pub fn is_finite() -> IsFinite {
    IsFinite
}

#[must_use]
pub struct IsFinite;

/// Creates an [`IsInfinite`] expectation.
pub fn is_infinite() -> IsInfinite {
    IsInfinite
}

#[must_use]
pub struct IsInfinite;

/// Creates an [`IsANumber`] expectation.
pub fn is_a_number() -> IsANumber {
    IsANumber
}

#[must_use]
pub struct IsANumber;

/// Creates a [`HasPrecisionOf`] expectation.
pub fn has_precision_of(expected_precision: u64) -> HasPrecisionOf {
    HasPrecisionOf { expected_precision }
}

#[must_use]
pub struct HasPrecisionOf {
    pub expected_precision: u64,
}

/// Creates a [`HasScaleOf`] expectation.
pub fn has_scale_of(expected_scale: i64) -> HasScaleOf {
    HasScaleOf { expected_scale }
}

#[must_use]
pub struct HasScaleOf {
    pub expected_scale: i64,
}

/// Creates an [`IsInteger`] expectation.
pub fn is_integer() -> IsInteger {
    IsInteger
}

#[must_use]
pub struct IsInteger;

/// Creates an [`IsLowerCase`] expectation.
pub fn is_lower_case() -> IsLowerCase {
    IsLowerCase
}

#[must_use]
pub struct IsLowerCase;

/// Creates an [`IsUpperCase`] expectation.
pub fn is_upper_case() -> IsUpperCase {
    IsUpperCase
}

#[must_use]
pub struct IsUpperCase;

/// Creates an [`IsAscii`] expectation.
pub fn is_ascii() -> IsAscii {
    IsAscii
}

#[must_use]
pub struct IsAscii;

/// Creates an [`IsAlphabetic`] expectation.
pub fn is_alphabetic() -> IsAlphabetic {
    IsAlphabetic
}

#[must_use]
pub struct IsAlphabetic;

/// Creates an [`IsAlphanumeric`] expectation.
pub fn is_alphanumeric() -> IsAlphanumeric {
    IsAlphanumeric
}

#[must_use]
pub struct IsAlphanumeric;

/// Creates an [`IsControlChar`] expectation.
pub fn is_control_char() -> IsControlChar {
    IsControlChar
}

#[must_use]
pub struct IsControlChar;

/// Creates an [`IsDigit`] expectation.
pub fn is_digit(radix: u32) -> IsDigit {
    IsDigit { radix }
}

#[must_use]
pub struct IsDigit {
    pub radix: u32,
}

/// Creates an [`IsWhitespace`] expectation.
pub fn is_whitespace() -> IsWhitespace {
    IsWhitespace
}

#[must_use]
pub struct IsWhitespace;

/// Creates an [`IsSome`] expectation.
pub fn is_some() -> IsSome {
    IsSome
}

#[must_use]
pub struct IsSome;

/// Creates an [`IsNone`] expectation.
pub fn is_none() -> IsNone {
    IsNone
}

#[must_use]
pub struct IsNone;

/// Creates a [`HasValue`] expectation.
pub fn has_value<E>(expected: E) -> HasValue<E> {
    HasValue { expected }
}

#[must_use]
pub struct HasValue<E> {
    pub expected: E,
}

/// Creates an [`IsOk`] expectation.
pub fn is_ok() -> IsOk {
    IsOk
}

#[must_use]
pub struct IsOk;

/// Creates an [`IsErr`] expectation.
pub fn is_err() -> IsErr {
    IsErr
}

#[must_use]
pub struct IsErr;

/// Creates a [`HasError`] expectation.
pub fn has_error<E>(expected: E) -> HasError<E> {
    HasError { expected }
}

#[must_use]
pub struct HasError<E> {
    pub expected: E,
}

/// Creates an [`ErrorHasSource`] expectation.
pub fn error_has_source() -> ErrorHasSource {
    ErrorHasSource
}

#[must_use]
pub struct ErrorHasSource;

/// Creates an [`ErrorHasSourceMessage`] expectation.
pub fn error_has_source_message(
    expected_source_message: impl Into<String>,
) -> ErrorHasSourceMessage {
    ErrorHasSourceMessage {
        expected_source_message: expected_source_message.into(),
    }
}

#[must_use]
pub struct ErrorHasSourceMessage {
    pub expected_source_message: String,
}

/// Creates a [`HasDebugMessage`] expectation.
pub fn has_debug_message<E>(expected: E) -> HasDebugMessage<E> {
    HasDebugMessage { expected }
}

pub struct HasDebugMessage<E> {
    pub expected: E,
}

/// Creates a [`HasDisplayMessage`] expectation.
pub fn has_display_message<E>(expected: E) -> HasDisplayMessage<E> {
    HasDisplayMessage { expected }
}

pub struct HasDisplayMessage<E> {
    pub expected: E,
}

/// Creates an [`IsEmpty`] expectation.
pub fn is_empty() -> IsEmpty {
    IsEmpty
}

#[must_use]
pub struct IsEmpty;

/// Creates a [`HasLength`] expectation.
pub fn has_length<E>(expected_length: E) -> HasLength<E> {
    HasLength { expected_length }
}

#[must_use]
pub struct HasLength<E> {
    pub expected_length: E,
}

/// Creates a [`HasLengthInRange`] expectation.
pub fn has_length_in_range<R, E>(expected_range: R) -> HasLengthInRange<R, E> {
    HasLengthInRange {
        expected_range,
        _element_type: PhantomData,
    }
}

#[must_use]
pub struct HasLengthInRange<R, E> {
    pub expected_range: R,
    _element_type: PhantomData<E>,
}

impl<R, E> HasLengthInRange<R, E> {
    #[deprecated = "use the function [`has_length_in_range`] instead"]
    pub fn new(expected_range: R) -> Self {
        Self {
            expected_range,
            _element_type: PhantomData,
        }
    }
}

/// Creates a [`HasLengthLessThan`] expectation.
pub fn has_length_less_than<E>(expected_length: E) -> HasLengthLessThan<E> {
    HasLengthLessThan { expected_length }
}

#[must_use]
pub struct HasLengthLessThan<E> {
    pub expected_length: E,
}

/// Creates a [`HasLengthGreaterThan`] expectation.
pub fn has_length_greater_than<E>(expected_length: E) -> HasLengthGreaterThan<E> {
    HasLengthGreaterThan { expected_length }
}

#[must_use]
pub struct HasLengthGreaterThan<E> {
    pub expected_length: E,
}

/// Creates a [`HasAtMostLength`] expectation.
pub fn has_at_most_length<E>(expected_length: E) -> HasAtMostLength<E> {
    HasAtMostLength { expected_length }
}

#[must_use]
pub struct HasAtMostLength<E> {
    pub expected_length: E,
}

/// Creates a [`HasAtLeastLength`] expectation.
pub fn has_at_least_length<E>(expected_length: E) -> HasAtLeastLength<E> {
    HasAtLeastLength { expected_length }
}

#[must_use]
pub struct HasAtLeastLength<E> {
    pub expected_length: E,
}

/// Creates a [`HasCharCount`] expectation.
pub fn has_char_count<E>(expected_char_count: E) -> HasCharCount<E> {
    HasCharCount {
        expected_char_count,
    }
}

#[must_use]
pub struct HasCharCount<E> {
    pub expected_char_count: E,
}

/// Creates a [`HasCharCountInRange`] expectation.
pub fn has_char_count_in_range<R, E>(expected_range: R) -> HasCharCountInRange<R, E> {
    HasCharCountInRange {
        expected_range,
        _element_type: PhantomData,
    }
}

#[must_use]
pub struct HasCharCountInRange<R, E> {
    pub expected_range: R,
    _element_type: PhantomData<E>,
}

impl<R, E> HasCharCountInRange<R, E> {
    #[deprecated = "use the function [`has_char_count_in_range`] instead"]
    pub fn new(expected_range: R) -> Self {
        Self {
            expected_range,
            _element_type: PhantomData,
        }
    }
}

/// Creates a [`HasCharCountLessThan`] expectation.
pub fn has_char_count_less_than<E>(expected_char_count: E) -> HasCharCountLessThan<E> {
    HasCharCountLessThan {
        expected_char_count,
    }
}

#[must_use]
pub struct HasCharCountLessThan<E> {
    pub expected_char_count: E,
}

/// Creates a [`HasCharCount`] expectation.
pub fn has_char_count_greater_than<E>(expected_char_count: E) -> HasCharCountGreaterThan<E> {
    HasCharCountGreaterThan {
        expected_char_count,
    }
}

#[must_use]
pub struct HasCharCountGreaterThan<E> {
    pub expected_char_count: E,
}

/// Creates a [`HasAtMostCharCount`] expectation.
pub fn has_at_most_char_count<E>(expected_char_count: E) -> HasAtMostCharCount<E> {
    HasAtMostCharCount {
        expected_char_count,
    }
}

#[must_use]
pub struct HasAtMostCharCount<E> {
    pub expected_char_count: E,
}

/// Creates a [`HasAtLeastCharCount`] expectation.
pub fn has_at_least_char_count<E>(expected_char_count: E) -> HasAtLeastCharCount<E> {
    HasAtLeastCharCount {
        expected_char_count,
    }
}

#[must_use]
pub struct HasAtLeastCharCount<E> {
    pub expected_char_count: E,
}

/// Creates a [`StringContains`] expectation.
pub fn string_contains<E>(expected: E) -> StringContains<E> {
    StringContains { expected }
}

#[must_use]
pub struct StringContains<E> {
    pub expected: E,
}

/// Creates a [`StringContainsAnyOf`] expectation.
pub fn string_contains_any_of<E>(expected: E) -> StringContainsAnyOf<E> {
    StringContainsAnyOf { expected }
}

#[must_use]
pub struct StringContainsAnyOf<E> {
    pub expected: E,
}

/// Creates a [`StringStartWith`] expectation.
pub fn string_starts_with<E>(expected: E) -> StringStartWith<E> {
    StringStartWith { expected }
}

#[must_use]
pub struct StringStartWith<E> {
    pub expected: E,
}

/// Creates a [`StringEndsWith`] expectation.
pub fn string_ends_with<E>(expected: E) -> StringEndsWith<E> {
    StringEndsWith { expected }
}

#[must_use]
pub struct StringEndsWith<E> {
    pub expected: E,
}

/// Creates a [`StringMatches`] expectation.
///
/// # Panics
///
/// Panics, if the regex pattern is invalid or exceeds the size limit.
#[cfg(feature = "regex")]
#[cfg_attr(docsrs, doc(cfg(feature = "regex")))]
pub fn string_matches(regex_pattern: &str) -> StringMatches<'_> {
    let regex = Regex::new(regex_pattern)
        .unwrap_or_else(|err| panic!("failed to match string with regex: {err}"));
    StringMatches {
        pattern: regex_pattern,
        regex,
    }
}

#[cfg(feature = "regex")]
#[cfg_attr(docsrs, doc(cfg(feature = "regex")))]
#[must_use]
pub struct StringMatches<'a> {
    pub pattern: &'a str,
    pub regex: Regex,
}

#[cfg(feature = "regex")]
#[cfg_attr(docsrs, doc(cfg(feature = "regex")))]
impl<'a> StringMatches<'a> {
    /// Creates a new `StringMatches`-expectation.
    ///
    /// # Panics
    ///
    /// Panics, if the regex pattern is invalid or exceeds the size limit.
    #[deprecated = "use the function [`string_matches`] instead"]
    pub fn new(regex_pattern: &'a str) -> Self {
        let regex = Regex::new(regex_pattern)
            .unwrap_or_else(|err| panic!("failed to match string with regex: {err}"));
        Self {
            pattern: regex_pattern,
            regex,
        }
    }
}

/// Creates an [`IteratorContains`] expectation.
pub fn iterator_contains<E>(expected: E) -> IteratorContains<E> {
    IteratorContains { expected }
}

#[must_use]
pub struct IteratorContains<E> {
    pub expected: E,
}

/// Creates an [`IteratorContainsExactlyInAnyOrder`] expectation.
pub fn iterator_contains_exactly_in_any_order<E>(
    expected: impl IntoIterator<Item = E>,
) -> IteratorContainsExactlyInAnyOrder<E> {
    IteratorContainsExactlyInAnyOrder {
        expected: Vec::from_iter(expected),
        missing: HashSet::new(),
        extra: HashSet::new(),
    }
}

#[must_use]
pub struct IteratorContainsExactlyInAnyOrder<E> {
    pub expected: Vec<E>,
    pub missing: HashSet<usize>,
    pub extra: HashSet<usize>,
}

impl<E> IteratorContainsExactlyInAnyOrder<E> {
    #[deprecated = "use the function [`iterator_contains_exactly_in_any_order`] instead"]
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            missing: HashSet::new(),
            extra: HashSet::new(),
        }
    }
}

/// Creates an [`IteratorContainsAnyOf`] expectation.
pub fn iterator_contains_any_of<E>(
    expected: impl IntoIterator<Item = E>,
) -> IteratorContainsAnyOf<E> {
    IteratorContainsAnyOf {
        expected: Vec::from_iter(expected),
    }
}

#[must_use]
pub struct IteratorContainsAnyOf<E> {
    pub expected: Vec<E>,
}

/// Creates an [`IteratorContainsAllOf`] expectation.
pub fn iterator_contains_all_of<E>(
    expected: impl IntoIterator<Item = E>,
) -> IteratorContainsAllOf<E> {
    IteratorContainsAllOf {
        expected: Vec::from_iter(expected),
        missing: HashSet::new(),
    }
}

#[must_use]
pub struct IteratorContainsAllOf<E> {
    pub expected: Vec<E>,
    pub missing: HashSet<usize>,
}

impl<E> IteratorContainsAllOf<E> {
    #[deprecated = "use the function [`iterator_contains_all_of`] instead"]
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            missing: HashSet::new(),
        }
    }
}

/// Creates an [`IteratorContainsOnly`] expectation.
pub fn iterator_contains_only<E>(expected: impl IntoIterator<Item = E>) -> IteratorContainsOnly<E> {
    IteratorContainsOnly {
        expected: Vec::from_iter(expected),
        extra: HashSet::new(),
    }
}

#[must_use]
pub struct IteratorContainsOnly<E> {
    pub expected: Vec<E>,
    pub extra: HashSet<usize>,
}

impl<E> IteratorContainsOnly<E> {
    #[deprecated = "use the function [`iterator_contains_only`] instead"]
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            extra: HashSet::new(),
        }
    }
}

/// Creates an [`IteratorContainsOnlyOnce`] expectation.
pub fn iterator_contains_only_once<E>(
    expected: impl IntoIterator<Item = E>,
) -> IteratorContainsOnlyOnce<E> {
    IteratorContainsOnlyOnce {
        expected: Vec::from_iter(expected),
        extra: HashSet::new(),
        duplicates: HashSet::new(),
    }
}

#[must_use]
pub struct IteratorContainsOnlyOnce<E> {
    pub expected: Vec<E>,
    pub extra: HashSet<usize>,
    pub duplicates: HashSet<usize>,
}

impl<E> IteratorContainsOnlyOnce<E> {
    #[deprecated = "use the function [`iterator_contains_only_once`] instead"]
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            extra: HashSet::new(),
            duplicates: HashSet::new(),
        }
    }
}

/// Creates an [`IteratorContainsExactly`] expectation.
pub fn iterator_contains_exactly<E>(
    expected: impl IntoIterator<Item = E>,
) -> IteratorContainsExactly<E> {
    IteratorContainsExactly {
        expected: Vec::from_iter(expected),
        missing: HashSet::new(),
        extra: HashSet::new(),
        out_of_order: HashSet::new(),
    }
}

#[must_use]
pub struct IteratorContainsExactly<E> {
    pub expected: Vec<E>,
    pub missing: HashSet<usize>,
    pub extra: HashSet<usize>,
    pub out_of_order: HashSet<usize>,
}

impl<E> IteratorContainsExactly<E> {
    #[deprecated = "use the function [`iterator_contains_exactly`] instead"]
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            missing: HashSet::new(),
            extra: HashSet::new(),
            out_of_order: HashSet::new(),
        }
    }
}

/// Creates an [`IteratorContainsAnyOf`] expectation.
pub fn iterator_contains_sequence<E>(
    expected: impl IntoIterator<Item = E>,
) -> IteratorContainsSequence<E> {
    IteratorContainsSequence {
        expected: Vec::from_iter(expected),
        missing: HashSet::new(),
        extra: HashSet::new(),
    }
}

#[must_use]
pub struct IteratorContainsSequence<E> {
    pub expected: Vec<E>,
    pub missing: HashSet<usize>,
    pub extra: HashSet<usize>,
}

impl<E> IteratorContainsSequence<E> {
    #[deprecated = "use the function [`iterator_contains_sequence`] instead"]
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            missing: HashSet::new(),
            extra: HashSet::new(),
        }
    }
}

/// Creates an [`IteratorContainsAllInOrder`] expectation.
pub fn iterator_contains_all_in_order<E>(
    expected: impl IntoIterator<Item = E>,
) -> IteratorContainsAllInOrder<E> {
    IteratorContainsAllInOrder {
        expected: Vec::from_iter(expected),
        missing: HashSet::new(),
    }
}

#[must_use]
pub struct IteratorContainsAllInOrder<E> {
    pub expected: Vec<E>,
    pub missing: HashSet<usize>,
}

impl<E> IteratorContainsAllInOrder<E> {
    #[deprecated = "use the function [`iterator_contains_all_in_order`] instead"]
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            missing: HashSet::new(),
        }
    }
}

/// Creates an [`IteratorStartsWith`] expectation.
pub fn iterator_starts_with<E>(expected: impl IntoIterator<Item = E>) -> IteratorStartsWith<E> {
    IteratorStartsWith {
        expected: Vec::from_iter(expected),
        missing: HashSet::new(),
        extra: HashSet::new(),
    }
}

#[must_use]
pub struct IteratorStartsWith<E> {
    pub expected: Vec<E>,
    pub missing: HashSet<usize>,
    pub extra: HashSet<usize>,
}

impl<E> IteratorStartsWith<E> {
    #[deprecated = "use the function [`iterator_starts_with`] instead"]
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            missing: HashSet::new(),
            extra: HashSet::new(),
        }
    }
}

/// Creates an [`IteratorEndsWith`] expectation.
pub fn iterator_ends_with<E>(expected: impl IntoIterator<Item = E>) -> IteratorEndsWith<E> {
    IteratorEndsWith {
        expected: Vec::from_iter(expected),
        missing: HashSet::new(),
        extra: HashSet::new(),
    }
}

#[must_use]
pub struct IteratorEndsWith<E> {
    pub expected: Vec<E>,
    pub missing: HashSet<usize>,
    pub extra: HashSet<usize>,
}

impl<E> IteratorEndsWith<E> {
    #[deprecated = "use the function [`iterator_ends_with`] instead"]
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            missing: HashSet::new(),
            extra: HashSet::new(),
        }
    }
}

pub fn has_single_element() -> HasSingleElement {
    HasSingleElement
}

#[must_use]
pub struct HasSingleElement;

pub fn has_at_least_number_of_elements(
    expected_number_of_elements: usize,
) -> HasAtLeastNumberOfElements {
    HasAtLeastNumberOfElements {
        expected_number_of_elements,
    }
}

#[must_use]
pub struct HasAtLeastNumberOfElements {
    pub expected_number_of_elements: usize,
}

/// Creates a [`MapContainsKey`] expectation.
pub fn map_contains_key<E>(expected_key: E) -> MapContainsKey<E> {
    MapContainsKey { expected_key }
}

#[must_use]
pub struct MapContainsKey<E> {
    pub expected_key: E,
}

/// Creates a [`MapContainsValue`] expectation.
pub fn map_contains_value<E>(expected_value: E) -> MapContainsValue<E> {
    MapContainsValue { expected_value }
}

#[must_use]
pub struct MapContainsValue<E> {
    pub expected_value: E,
}

/// Creates a [`MapContainsKeys`] expectation.
pub fn map_contains_keys<E>(expected_keys: impl IntoIterator<Item = E>) -> MapContainsKeys<E> {
    MapContainsKeys {
        expected_keys: Vec::from_iter(expected_keys),
        missing: HashSet::new(),
    }
}

#[must_use]
pub struct MapContainsKeys<E> {
    pub expected_keys: Vec<E>,
    pub missing: HashSet<usize>,
}

impl<E> MapContainsKeys<E> {
    #[deprecated = "use the function [`map_contains_keys`] instead"]
    pub fn new(expected_keys: impl IntoIterator<Item = E>) -> Self {
        Self {
            expected_keys: Vec::from_iter(expected_keys),
            missing: HashSet::new(),
        }
    }
}

/// Creates a [`MapDoesNotContainKeys`] expectation.
pub fn map_does_not_contain_keys<E>(
    expected_keys: impl IntoIterator<Item = E>,
) -> MapDoesNotContainKeys<E> {
    MapDoesNotContainKeys {
        expected_keys: Vec::from_iter(expected_keys),
        extra: HashSet::new(),
    }
}

#[must_use]
pub struct MapDoesNotContainKeys<E> {
    pub expected_keys: Vec<E>,
    pub extra: HashSet<usize>,
}

impl<E> MapDoesNotContainKeys<E> {
    #[deprecated = "use the function [`map_does_not_contain_keys`] instead"]
    pub fn new(expected_keys: impl IntoIterator<Item = E>) -> Self {
        Self {
            expected_keys: Vec::from_iter(expected_keys),
            extra: HashSet::new(),
        }
    }
}

/// Creates a [`MapContainsValues`] expectation.
pub fn map_contains_values<E>(
    expected_values: impl IntoIterator<Item = E>,
) -> MapContainsValues<E> {
    MapContainsValues {
        expected_values: Vec::from_iter(expected_values),
        missing: HashSet::new(),
    }
}

#[must_use]
pub struct MapContainsValues<E> {
    pub expected_values: Vec<E>,
    pub missing: HashSet<usize>,
}

impl<E> MapContainsValues<E> {
    #[deprecated = "use the function [`map_contains_values`] instead"]
    pub fn new(expected_values: impl IntoIterator<Item = E>) -> Self {
        Self {
            expected_values: Vec::from_iter(expected_values),
            missing: HashSet::new(),
        }
    }
}

/// Creates a [`MapDoesNotContainValues`] expectation.
pub fn map_does_not_contain_values<E>(
    expected_values: impl IntoIterator<Item = E>,
) -> MapDoesNotContainValues<E> {
    MapDoesNotContainValues {
        expected_values: Vec::from_iter(expected_values),
        extra: HashSet::new(),
    }
}

#[must_use]
pub struct MapDoesNotContainValues<E> {
    pub expected_values: Vec<E>,
    pub extra: HashSet<usize>,
}

impl<E> MapDoesNotContainValues<E> {
    #[deprecated = "use the function [`map_does_not_contain_values`] instead"]
    pub fn new(expected_values: impl IntoIterator<Item = E>) -> Self {
        Self {
            expected_values: Vec::from_iter(expected_values),
            extra: HashSet::new(),
        }
    }
}

/// Creates a [`MapContainsExactlyKeys`] expectation.
pub fn map_contains_exactly_keys<E>(
    expected_keys: impl IntoIterator<Item = E>,
) -> MapContainsExactlyKeys<E> {
    MapContainsExactlyKeys {
        expected_keys: Vec::from_iter(expected_keys),
        missing: HashSet::new(),
        extra: HashSet::new(),
    }
}

#[must_use]
pub struct MapContainsExactlyKeys<E> {
    pub expected_keys: Vec<E>,
    pub missing: HashSet<usize>,
    pub extra: HashSet<usize>,
}

impl<E> MapContainsExactlyKeys<E> {
    #[deprecated = "use the function [`map_contains_exactly_keys`] instead"]
    pub fn new(expected_keys: impl IntoIterator<Item = E>) -> Self {
        Self {
            expected_keys: Vec::from_iter(expected_keys),
            missing: HashSet::new(),
            extra: HashSet::new(),
        }
    }
}

/// Creates a [`DoesPanic`] expectation.
///
/// The panic message is not being asserted. To expect to panic with a specific
/// message, use the [`DoesPanic::with_message`] method on the newly constructed
/// expectation.
///
/// # Examples
///
/// ```
/// use asserting::expectations::{does_panic};
/// use asserting::prelude::*;
///
/// // expect to panic with any message
/// assert_that_code!(|| {panic!("we have a problem!");})
///     .expecting(does_panic());
///
/// // expect to panic with a specific message
/// assert_that_code!(|| {panic!("we have a problem!");})
///     .expecting(does_panic().with_message("we have a problem!"));
/// ```
#[cfg(feature = "panic")]
#[cfg_attr(docsrs, doc(cfg(feature = "panic")))]
pub fn does_panic() -> DoesPanic {
    DoesPanic {
        expected_message: None,
        actual_message: None,
    }
}

#[cfg(feature = "panic")]
#[cfg_attr(docsrs, doc(cfg(feature = "panic")))]
#[must_use]
pub struct DoesPanic {
    pub expected_message: Option<String>,
    pub actual_message: Option<String>,
}

#[cfg(feature = "panic")]
impl DoesPanic {
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.expected_message = Some(message.into());
        self
    }
}

/// Creates a [`DoesNotPanic`] expectation.
#[cfg(feature = "panic")]
#[cfg_attr(docsrs, doc(cfg(feature = "panic")))]
pub fn does_not_panic() -> DoesNotPanic {
    DoesNotPanic {
        actual_message: None,
    }
}

#[cfg(feature = "panic")]
#[cfg_attr(docsrs, doc(cfg(feature = "panic")))]
#[must_use]
pub struct DoesNotPanic {
    pub actual_message: Option<Box<dyn std::any::Any + Send>>,
}
