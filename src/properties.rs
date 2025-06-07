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
//! For example, the trait [`LengthProperty`] defines that all implementing
//! types have a length. The trait defines the accessor method `length_property`
//! to query the length of a value of this type. For all types that implement
//! the [`LengthProperty`] trait, the assertions around a type's length defined
//! by [`AssertHasLength`](crate::assertions::AssertHasLength) trait can be
//! used.
//!
//! Property traits that define a certain behavior of a type are often marker
//! traits.
//!
//! An example for a behavior property is the [`DefinedOrderProperty`] trait. It
//! specifies that a collection's iterator yields the items in a well-defined
//! order.

use crate::std::iter::Iterator;

/// The "empty" property of a collection-like type.
///
/// This property is used by the implementation of the
/// [`AsssertEmptiness`](crate::assertions::AssertEmptiness) assertions.
pub trait IsEmptyProperty {
    /// Returns whether the collection-like value is empty.
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

/// The length property of a collection-like type.
///
/// Collection-like types are, for example, `Vec`, slice, array, `HashSet`,
/// `HashMap` and strings.
///
/// This property is used by the implementation of the
/// [`AssertHasLength`](crate::assertions::AssertHasLength) assertion.
pub trait LengthProperty {
    /// Returns the length of a collection-like value.
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
pub trait DefinedOrderProperty {}

impl<C> DefinedOrderProperty for &C where C: DefinedOrderProperty + ?Sized {}
impl<C> DefinedOrderProperty for &mut C where C: DefinedOrderProperty + ?Sized {}

/// Property for types that contain characters.
pub trait CharCountProperty {
    /// Returns the number of characters contained in this type.
    fn char_count_property(&self) -> usize;
}

impl<T> CharCountProperty for &T
where
    T: CharCountProperty + ?Sized,
{
    fn char_count_property(&self) -> usize {
        <T as CharCountProperty>::char_count_property(self)
    }
}

impl<T> CharCountProperty for &mut T
where
    T: CharCountProperty + ?Sized,
{
    fn char_count_property(&self) -> usize {
        <T as CharCountProperty>::char_count_property(self)
    }
}

/// The additive identity property of a numeric type.
pub trait AdditiveIdentityProperty {
    /// The additive identity (zero).
    fn additive_identity() -> Self;
}

/// The multiplicative identity property of a numeric type.
pub trait MultiplicativeIdentityProperty {
    /// The multiplicative identity (one).
    fn multiplicative_identity() -> Self;
}

/// A property of numeric types that can have negative and positive values.
pub trait SignumProperty {
    /// Returns whether this value is negative.
    fn is_negative_property(&self) -> bool;

    /// Returns whether this value is positive.
    fn is_positive_property(&self) -> bool;
}

impl<T> SignumProperty for &T
where
    T: SignumProperty + ?Sized,
{
    fn is_negative_property(&self) -> bool {
        <T as SignumProperty>::is_negative_property(self)
    }

    fn is_positive_property(&self) -> bool {
        <T as SignumProperty>::is_positive_property(self)
    }
}

impl<T> SignumProperty for &mut T
where
    T: SignumProperty + ?Sized,
{
    fn is_negative_property(&self) -> bool {
        <T as SignumProperty>::is_negative_property(self)
    }

    fn is_positive_property(&self) -> bool {
        <T as SignumProperty>::is_positive_property(self)
    }
}

/// A property of floating point numbers that may have infinite or finite
/// values.
pub trait InfinityProperty {
    /// Returns whether this value is infinite.
    fn is_infinite_property(&self) -> bool;

    /// Returns whether this value is finite.
    fn is_finite_property(&self) -> bool;
}

impl<T> InfinityProperty for &T
where
    T: InfinityProperty + ?Sized,
{
    fn is_infinite_property(&self) -> bool {
        <T as InfinityProperty>::is_infinite_property(self)
    }

    fn is_finite_property(&self) -> bool {
        <T as InfinityProperty>::is_finite_property(self)
    }
}

impl<T> InfinityProperty for &mut T
where
    T: InfinityProperty + ?Sized,
{
    fn is_infinite_property(&self) -> bool {
        <T as InfinityProperty>::is_infinite_property(self)
    }

    fn is_finite_property(&self) -> bool {
        <T as InfinityProperty>::is_finite_property(self)
    }
}

/// The not-a-number property of floating point numbers.
pub trait IsNanProperty {
    /// Returns whether this value is not a number.
    fn is_nan_property(&self) -> bool;
}

impl<T> IsNanProperty for &T
where
    T: IsNanProperty + ?Sized,
{
    fn is_nan_property(&self) -> bool {
        <T as IsNanProperty>::is_nan_property(self)
    }
}

impl<T> IsNanProperty for &mut T
where
    T: IsNanProperty + ?Sized,
{
    fn is_nan_property(&self) -> bool {
        <T as IsNanProperty>::is_nan_property(self)
    }
}

/// The properties of a map-like type.
pub trait MapProperties {
    /// The type of the keys in this map.
    type Key;

    /// The type of the values in this map.
    type Value;

    /// Returns an iterator over the keys in this map.
    fn keys_property(&self) -> impl Iterator<Item = &Self::Key>;

    /// Returns an iterator over the values in this map.
    fn values_property(&self) -> impl Iterator<Item = &Self::Value>;

    /// Returns an iterator over the key/value-pairs in this map.
    fn entries_property(&self) -> impl Iterator<Item = (&Self::Key, &Self::Value)>;
}
