//! Definitions of "properties" that are used by implementations of certain
//! assertions.
//!
//! A property can be some kind of information about a type's value or a certain
//! behavior of the type. A property is defined by a trait. Each trait defines
//! one property. Assertions around a specific property can be implemented once
//! for all types that implement the related property trait.
//!
//! Property traits that define access to a specific kind of information of a
//! type's value define an accessor method to query this information.
//!
//! For example the trait [`LengthProperty`] defines that all implementing types
//! have a length. The trait defines the accessor method `length_property` to
//! query the length of a value of this type. For all types that implement the
//! [`LengthProperty`] trait the assertions around a types length defined by
//! [`AssertHasLength`](crate::assertions::AssertHasLength) trait can be used.
//!
//! Property traits that define a certain behavior of a type are often marker
//! traits.
//!
//! An example for a behavior property is the [`DefinedOrder`] trait. It
//! specifies that a collection's iterator yields the items in a well-defined
//! order.

/// Any type that implements this trait provides access to its `is_empty` method
/// to be used by the implementation of the
/// [`AsssertEmptiness`](crate::assertions::AssertEmptiness) assertions.
pub trait IsEmptyProperty {
    /// Provides access to the `is_empty` property.
    fn is_empty_property(&self) -> bool;
}

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

/// Any type that implements this trait provides access to its `len` method
/// to be used by the implementation of the
/// [`AssertHasLength`](crate::assertions::AssertHasLength) assertion.
pub trait LengthProperty {
    /// Provides access to the `len` property.
    fn length_property(&self) -> usize;
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

/// Marker trait to specify whether a collection or iterator iterates over its
/// elements in a well-defined order.
pub trait DefinedOrder {}

impl<C> DefinedOrder for &C where C: DefinedOrder + ?Sized {}
impl<C> DefinedOrder for &mut C where C: DefinedOrder + ?Sized {}
