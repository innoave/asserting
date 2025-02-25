#![allow(clippy::wrong_self_convention)]

pub trait IsEqualTo<'e, E, R>
where
    E: 'e,
{
    fn is_equal_to(self, expected: E) -> R;

    fn is_not_equal_to(self, expected: E) -> R;
}
