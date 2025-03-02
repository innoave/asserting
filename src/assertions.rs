#![allow(clippy::wrong_self_convention, clippy::return_self_not_must_use)]

use crate::std::ops::RangeInclusive;

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
    fn is_less_than_or_equal_to(self, expected: E) -> Self;

    #[track_caller]
    fn is_greater_than_or_equal_to(self, expected: E) -> Self;
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

pub trait AssertHasLength {
    #[track_caller]
    fn has_length(self, expected: usize) -> Self;

    #[track_caller]
    fn has_length_in_range(self, range: RangeInclusive<usize>) -> Self;
}

pub trait AssertContains<E> {
    #[track_caller]
    fn contains(self, pattern: E) -> Self;
}

pub trait AssertContainsAnyOf<E> {
    #[track_caller]
    fn contains_any_of(self, pattern: E) -> Self;
}

pub trait AssertStartsWith<E> {
    #[track_caller]
    fn starts_with(self, pattern: E) -> Self;
}

pub trait AssertEndsWith<E> {
    #[track_caller]
    fn ends_with(self, pattern: E) -> Self;
}

#[cfg(feature = "panic")]
pub trait AssertPanics {
    #[track_caller]
    fn does_not_panic(self) -> Self;

    #[track_caller]
    fn panics(self) -> Self;

    #[track_caller]
    fn panics_with_message(self, message: impl Into<String>) -> Self;
}
