#![allow(clippy::wrong_self_convention)]

pub trait AssertEquality<'a, E> {
    fn is_equal_to(self, expected: E) -> Self;

    fn is_not_equal_to(self, expected: E) -> Self;
}
