use crate::specification::Expected;

pub trait IsEqualTo<'e, E, R>
where
    E: 'e,
{
    fn is_equal_to(self, expected: impl Into<Expected<'e, E>>) -> R;
}
