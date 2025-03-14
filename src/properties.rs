pub trait IsEmptyProperty {
    fn is_empty_property(&self) -> bool;
}

pub trait LengthProperty {
    fn length_property(&self) -> usize;
}

/// Marker trait to specify whether a collection or iterator iterates over its
/// elements in a well-defined order.
pub trait DefinedOrder {}

impl<C> DefinedOrder for &C where C: DefinedOrder + ?Sized {}
impl<C> DefinedOrder for &mut C where C: DefinedOrder + ?Sized {}

impl<T> IsEmptyProperty for &T
where
    T: IsEmptyProperty + ?Sized,
{
    fn is_empty_property(&self) -> bool {
        <T as IsEmptyProperty>::is_empty_property(self)
    }
}

impl<T> IsEmptyProperty for &mut T
where
    T: IsEmptyProperty + ?Sized,
{
    fn is_empty_property(&self) -> bool {
        <T as IsEmptyProperty>::is_empty_property(self)
    }
}

impl<T> LengthProperty for &T
where
    T: LengthProperty + ?Sized,
{
    fn length_property(&self) -> usize {
        <T as LengthProperty>::length_property(self)
    }
}

impl<T> LengthProperty for &mut T
where
    T: LengthProperty + ?Sized,
{
    fn length_property(&self) -> usize {
        <T as LengthProperty>::length_property(self)
    }
}
