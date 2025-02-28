#![allow(clippy::wrong_self_convention)]

pub trait AssertEquality<'a, E, R>
where
    E: 'a,
{
    fn is_equal_to(self, expected: E) -> R;

    fn is_not_equal_to(self, expected: E) -> R;
}

pub trait AssertBoolean<R> {
    fn is_true(self) -> R;

    fn is_false(self) -> R;
}

pub trait AssertOption<'a, R> {
    fn is_some(self) -> R;

    fn is_none(self) -> R;
}

pub trait AssertHasValue<'a, E, R> {
    fn has_value(self, expected: E) -> R;
}

pub trait AssertHasError<'a, E, R> {
    fn has_error(self, expected: E) -> R;
}

pub trait AssertContains<'a, E, R> {
    fn contains(self, pattern: E) -> R;
}

pub trait AssertContainsAnyOf<'a, E, R> {
    fn contains_any_of(self, pattern: E) -> R;
}
