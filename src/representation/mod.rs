//! The representation system enables custom formatting of actual and expected
//! values in failure reports.
//!
//! The presentation system serves two main purposes:
//!
//! 1. Custom representation of actual and expected values in failure reports
//! 2. Provide a representation for (foreign) types that do not implement the
//!    `std::fmt::Debug` trait.
//!
//! To define a custom representation for a subject and the expected value,
//! there are two possible ways:
//!
//! **Ad-hoc representation**: We call
//! [`represented_as`](RepresentedAs::represented_as) on a [`Spec`] or
//! [`DerivedSpec`] providing a function or closure that formats the subject
//! and expected values for this assertion.
//!
//! **Representation struct**: We define a so-called representation-struct and
//! configure it to be used with an assertion by calling
//! [`represented_by`](RepresentedBy::represented_by) on a [`Spec`] or
//! [`DerivedSpec`]. A representation struct is usually a unit struct that
//! implements the [`Represent`] for the type of the subject and the type of the
//! expected value (if they are not of the same type).
//!
//! The closure expected by the [`represented_as`](RepresentedAs::represented_as)
//! and the method defined by the [`Represent`] trait do have a similar
//! signature as the `std::fmt` traits.
//!
//! Examples for using the representation system can be found in the crate level
//! documentation chapter [Type formatting (aka Representation)].
//!
//! [`Spec`]: crate::spec::Spec
//! [`DerivedSpec`]: crate::derived_spec::DerivedSpec
//! [Type formatting (aka Representation)]: crate#type-formatting-aka-representation

use crate::std::{
    boxed::Box,
    fmt,
    fmt::{Debug, Display},
};

/// Specify a custom representation that `asserting` shall use to format the
/// subject and the expected value in failure reports.
pub trait RepresentedBy<P> {
    /// The output type of this trait.
    ///
    /// Usually this is a [`Spec`] or a [`DerivedSpec`].
    ///
    /// [`Spec`]: crate::spec::Spec
    /// [`DerivedSpec`]: crate::derived_spec::DerivedSpec
    type Output;

    /// Configure a custom representation that `asserting` shall use to format
    /// the subject and the expected value in failure reports.
    ///
    /// The representation is a type that implements the [`Represent`] trait
    /// for the type of the subject and the type of the expected value.
    /// See the docs of the [`Represent`] trait for an example of implementing
    /// a custom representation.
    fn represented_by(self, representation: P) -> Self::Output;
}

/// Specify an ad-hoc representation function or closure that `asserting` shall
/// use to format the subject and the expected value in failure reports.
pub trait RepresentedAs {
    /// The type of the subject that shall be formatted.
    type Subject;
    /// The output type of this trait.
    ///
    /// Usually this is a [`Spec`] or a [`DerivedSpec`].
    ///
    /// [`Spec`]: crate::spec::Spec
    /// [`DerivedSpec`]: crate::derived_spec::DerivedSpec
    type Output;

    /// Configure a representation function or closure that `asserting` shall
    /// use to format the subject and the expected value in failure reports.
    fn represented_as<F>(self, representation: F) -> Self::Output
    where
        F: Fn(&Self::Subject, &mut fmt::Formatter<'_>) -> fmt::Result + 'static;
}

/// A trait that defines how a type's value is printed in the failure report of
/// a failing assertion.
///
/// With the use of this trait we can define the representation of actual and
/// expected values in failure reports.
///
/// It defines one method: `represent`. The signature of this method is similar
/// to the `fmt`-method of the [`Debug`] and [`Display`] traits in the
/// standard library.
///
/// To implement the representation of a type `Foo`, we first define a struct
/// (usually a unit struct), e.g. `FooRepresentation`. Then we implement this
/// trait for the "representation" struct (in this example `FooRepresentation`)
/// with our custom type as a type parameter.
///
/// ```no_run
/// use asserting::prelude::*;
/// use core::fmt;
///
/// #[derive(PartialEq)]
/// struct Foo {
///     bar: String,
///     baz: u16,
/// }
///
/// #[derive(Clone, Copy)]
/// struct FooRepresentation;
///
/// impl Represent<Foo> for FooRepresentation {
///     fn represent(&self, value: &Foo, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         write!(f, "Foo {{ bar: {}, baz: {} }}", value.bar, value.baz)
///     }
/// }
/// ```
///
/// Assertions for values in containers (e.g. `Vec`) require that the
/// representation struct implements the `Clone` trait. That's why we derive
/// `Clone` and `Copy` for `FooRepresentation` in the example above.
///
/// To make use of this representation, we have to tell `asserting` to use it by
/// calling the [`represented_by`](RepresentedBy::represented_by) method on a
/// [`Spec`] or [`DerivedSpec`], like so:
///
/// ```
/// use asserting::prelude::*;
/// use core::fmt;
/// # #[derive(PartialEq)]
/// # struct Foo {
/// #     bar: String,
/// #     baz: u16,
/// # }
/// #
/// # #[derive(Clone, Copy)]
/// # struct FooRepresentation;
/// #
/// # impl Represent<Foo> for FooRepresentation {
/// #     fn represent(&self, value: &Foo, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/// #         write!(f, "Foo {{ bar: {}, baz: {} }}", value.bar, value.baz)
/// #     }
/// # }
///
/// let foo = Foo { bar: "bar".to_string(), baz: 42 };
///
/// assert_that!(foo)
///     .represented_by(FooRepresentation)
///     .is_equal_to(Foo { bar: "bar".to_string(), baz: 42 });
/// ```
///
/// This representation mechanic can be used to:
///
/// 1. define a custom representation for a type that already implements `Debug`
///    but for testing purposes we want a different representation.
/// 2. write assertions for types that do not implement `Debug` and there are
///    some reasons why we cannot implement it. E.g., foreign types and the
///    orphan rule.
///
/// It is only possible to configure one representation type per assertion
/// (that is per `assert_that!()...` statement). If the subject and the expected
/// value are not exactly of the same type (e.g., `String` and `str`), then
/// the representation must implement the [`Represent`] trait for both types,
/// the type of the subject and the type of the expected value.
///
/// This crate provides a [`DebugRepresentation`] which can represent any type
/// that implements the `Debug` trait, and a [`DisplayRepresentation`]
/// which can represent any type that implements the `fmt::Display` trait. The
/// [`DebugRepresentation`] is used when no other representation is specified
/// by calling [`represented_by`](RepresentedBy::represented_by) on a [`Spec`]
/// or [`DerivedSpec`].
///
/// For simple cases and/or we need a custom representation just for one or a
/// few tests, we can use an ad-hoc representation, which is a format function
/// given to the [`represented_as`](RepresentedAs::represented_as) method on a
/// [`Spec`] or [`DerivedSpec`].
///
/// [`Spec`]: crate::spec::Spec
/// [`DerivedSpec`]: crate::derived_spec::DerivedSpec
pub trait Represent<T: ?Sized> {
    /// Formats the given value of type `T` as it should be represented in
    /// failure reports.
    ///
    /// Implementations are very similar to those of the [`Debug`] and
    /// [`Display`] traits of the standard library.
    #[allow(clippy::missing_errors_doc)]
    fn represent(&self, value: &T, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

/// Combines a value and a representation for this value.
///
/// The representation can be a type that implements [`Represent`] or an
/// [`AdHocRepresentation`].
pub struct Represented<'t, 'd, T: ?Sized, D> {
    /// The value of type `T`.
    pub value: &'t T,
    /// The representation to be used for the value.
    pub representation: &'d D,
}

impl<T: ?Sized, D> Clone for Represented<'_, '_, T, D> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized, D> Copy for Represented<'_, '_, T, D> {}

impl<'t, 'd, T: ?Sized, D> From<(&'t T, &'d D)> for Represented<'t, 'd, T, D> {
    fn from((value, representation): (&'t T, &'d D)) -> Self {
        Represented {
            value,
            representation,
        }
    }
}

impl<T: ?Sized, D> Debug for Represented<'_, '_, T, D>
where
    D: Represent<T>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.representation.represent(self.value, f)
    }
}

impl<T: ?Sized, D> Display for Represented<'_, '_, T, D>
where
    D: Represent<T>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.representation.represent(self.value, f)
    }
}

/// An ad-hoc representation formats a value by a format function or closure.
///
/// Using a format function does not require defining a representation struct
/// and implementing the [`Represent`] trait for it. But the function has to
/// be written for all tests where an ad-hoc representation should be used.
///
/// This is useful if we need a custom representation only for one or a few
/// tests, or we want different representations for different test cases.
/// Usually we will not use this struct directly in tests. Instead, we call the
/// [`represented_as`](RepresentedAs::represented_as) method on a [`Spec`] or
/// [`DerivedSpec`]. `asserting` wraps the function into the struct to store
/// the representation function or closure internally.
///
/// The representation function has a similar signature as the
/// [`represent`](Represent::represent) method of the [`Represent`] trait.
///
/// [`Spec`]: crate::spec::Spec
/// [`DerivedSpec`]: crate::derived_spec::DerivedSpec
#[allow(clippy::type_complexity)]
pub struct AdHocRepresentation<T>(pub Box<dyn Fn(&T, &mut fmt::Formatter<'_>) -> fmt::Result>);

impl<T> Represent<T> for AdHocRepresentation<T> {
    fn represent(&self, value: &T, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0(value, f)
    }
}

/// A representation that can format all values of any type `T` that implements
/// the [`Debug`] trait of the standard library.
///
/// The implementation of the [`Represent`] for this representation just
/// delegates to the implementation of the `Debug` trait.
///
/// This is the default representation used by `asserting` as long as we do not
/// specify a different representation by calling
/// [`represented_by`](RepresentedBy::represented_by) or use an ad-hoc
/// representation by calling the
/// [`represented_as`](RepresentedAs::represented_as) method on a [`Spec`] or
/// [`DerivedSpec`].
///
/// [`Spec`]: crate::spec::Spec
/// [`DerivedSpec`]: crate::derived_spec::DerivedSpec
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DebugRepresentation;

impl<T> Represent<T> for DebugRepresentation
where
    T: ?Sized + Debug,
{
    fn represent(&self, value: &T, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(value, f)
    }
}

/// A representation that can format all values of any type `T` that implements
/// the [`Display`] trait of the standard library.
///
/// The implementation of the [`Represent`] for this representation just
/// delegates to the implementation of the `Display` trait.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DisplayRepresentation;

impl<T> Represent<T> for DisplayRepresentation
where
    T: ?Sized + Display,
{
    fn represent(&self, value: &T, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(value, f)
    }
}

#[cfg(test)]
mod tests;
