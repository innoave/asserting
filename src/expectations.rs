#![allow(clippy::wrong_self_convention, clippy::return_self_not_must_use)]

pub trait AssertEquality<E> {
    #[track_caller]
    fn is_equal_to(self, expected: E) -> Self;

    #[track_caller]
    fn is_not_equal_to(self, expected: E) -> Self;
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

pub trait AssertHasValue<E> {
    #[track_caller]
    fn has_value(self, expected: E) -> Self;
}

pub trait AssertHasError<E> {
    #[track_caller]
    fn has_error(self, expected: E) -> Self;
}

pub trait AssertContains<E> {
    #[track_caller]
    fn contains(self, pattern: E) -> Self;
}

pub trait AssertContainsAnyOf<E> {
    #[track_caller]
    fn contains_any_of(self, pattern: E) -> Self;
}
