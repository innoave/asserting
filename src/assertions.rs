#![allow(clippy::wrong_self_convention, clippy::return_self_not_must_use)]

use crate::spec::Spec;
use crate::std::ops::RangeInclusive;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Order {
    #[default]
    Ascending,
    Descending,
}

pub trait AssertEquality<E> {
    #[track_caller]
    fn is_equal_to(self, expected: E) -> Self;

    #[track_caller]
    fn is_not_equal_to(self, expected: E) -> Self;
}

pub trait AssertOrder<E> {
    #[track_caller]
    fn is_less_than(self, expected: E) -> Self;

    #[track_caller]
    fn is_greater_than(self, expected: E) -> Self;

    #[track_caller]
    fn is_at_most(self, expected: E) -> Self;

    #[track_caller]
    fn is_at_least(self, expected: E) -> Self;
}

pub trait AssertInRange<E> {
    #[track_caller]
    fn is_in_range(self, range: RangeInclusive<E>) -> Self;

    #[track_caller]
    fn is_not_in_range(self, range: RangeInclusive<E>) -> Self;
}

pub trait AssertEmptiness {
    #[track_caller]
    fn is_empty(self) -> Self;

    fn is_not_empty(self) -> Self;
}

pub trait AssertBoolean {
    #[track_caller]
    fn is_true(self) -> Self;

    #[track_caller]
    fn is_false(self) -> Self;
}

pub trait AssertOption {
    #[track_caller]
    fn is_some(self) -> Self;

    #[track_caller]
    fn is_none(self) -> Self;
}

pub trait AssertResult {
    #[track_caller]
    fn is_ok(self) -> Self;

    fn is_err(self) -> Self;
}

pub trait AssertHasValue<E> {
    #[track_caller]
    fn has_value(self, expected: E) -> Self;
}

pub trait AssertHasError<E> {
    #[track_caller]
    fn has_error(self, expected: E) -> Self;
}

pub trait AssertHasLength<E> {
    #[track_caller]
    fn has_length(self, expected: E) -> Self;

    #[track_caller]
    fn has_length_in_range(self, range: RangeInclusive<E>) -> Self;
}

pub trait AssertStringPattern<E> {
    #[track_caller]
    fn contains(self, pattern: E) -> Self;

    #[track_caller]
    fn starts_with(self, pattern: E) -> Self;

    #[track_caller]
    fn ends_with(self, pattern: E) -> Self;
}

pub trait AssertStringContainsAnyOf<E> {
    #[track_caller]
    fn contains_any_of(self, pattern: E) -> Self;
}

pub trait AssertIteratorContains<'a, U, E, R> {
    #[track_caller]
    fn contains(self, element: E) -> Spec<'a, U, R>;
}

/// Assert values in a collection.
///
/// These assertions do not rely on the order in which the collection iterates
/// over its values.
pub trait AssertIteratorContainsInAnyOrder<'a, S, E, R> {
    /// Verifies that the actual collection/iterator contains exactly the given
    /// values and nothing else in any order.
    #[track_caller]
    fn contains_exactly_in_any_order(self, expected: E) -> Spec<'a, S, R>;

    /// Verifies that the actual collection/iterator contains at least one of
    /// the given values.
    #[track_caller]
    fn contains_any_of(self, expected: E) -> Spec<'a, S, R>;

    /// Verifies that the actual collection/iterator contains the given values
    /// in any order.
    ///
    /// The collection/iterator may contain more values than the given ones, but
    /// at least all the specified ones.
    #[track_caller]
    fn contains_all_of(self, expected: E) -> Spec<'a, S, R>;

    /// Verifies that the actual collection/iterator contains only the given
    /// values and nothing else in any order and ignoring duplicates.
    ///
    /// The collection may contain fewer values than the expected ones.
    #[track_caller]
    fn contains_only(self, expected: E) -> Spec<'a, S, R>;

    /// Verifies that the actual collection/iterator contains only the given
    /// values in any order and each of them only once.
    ///
    /// The collection may contain fewer values than the expected ones.
    #[track_caller]
    fn contains_only_once(self, expected: E) -> Spec<'a, S, R>;
}

/// Assert values in an ordered collection.
///
/// These assertions are applicable to collections which iterate over their
/// values in a defined order.
pub trait AssertIteratorContainsInOrder<'a, S, E, R> {
    /// Verifies that the actual collection/iterator contains exactly the given
    /// values and nothing else in the given order.
    #[track_caller]
    fn contains_exactly(self, expected: E) -> Spec<'a, S, R>;

    /// Verifies that the actual collection/iterator contains the given sequence
    /// of values in the given order and without extra values between the
    /// sequence values.
    ///
    /// May contain more values as in the given sequence before and after the
    /// sequence.
    #[track_caller]
    fn contains_sequence(self, expected: E) -> Spec<'a, S, R>;

    /// Verifies that the actual collection/iterator contains all the given
    /// values and in the given order, possible with other values between them.
    #[track_caller]
    fn contains_all_in_order(self, expected: E) -> Spec<'a, S, R>;

    /// Verifies that the actual collection/iterator contains the given values
    /// as the first elements in order.
    #[track_caller]
    fn starts_with(self, expected: E) -> Spec<'a, S, R>;

    /// Verifies that the actual collection/iterator contains the given values
    /// as the last elements in order.
    #[track_caller]
    fn ends_with(self, expected: E) -> Spec<'a, S, R>;
}

/// Assert the order of the values within a collection.
///
/// These assertions are applicable to ordered collections only.
pub trait AssertIsSorted {
    /// Verifies that the actual collection is sorted.
    #[track_caller]
    fn is_sorted(self, order: Order) -> Self;
}

/// Assert that the code under test panics, panics with a certain message or
/// does not panic.
#[cfg(feature = "panic")]
pub trait AssertCodePanics {
    /// Verifies that the actual code under test does not panic.
    #[track_caller]
    fn does_not_panic(self) -> Self;

    /// Verifies that the actual code under test panics with any message.
    #[track_caller]
    fn panics(self) -> Self;

    /// Verifies that the actual code under test panics with the given
    /// message.
    #[track_caller]
    fn panics_with_message(self, message: impl Into<String>) -> Self;
}
