#![allow(clippy::wrong_self_convention)]

pub trait IsEqualTo<'e, E, R>
where
    E: 'e,
{
    fn is_equal_to(self, expected: E) -> R;

    fn is_not_equal_to(self, expected: E) -> R;
}

pub trait IsTrue<R> {
    fn is_true(self) -> R;

    fn is_false(self) -> R;
}
