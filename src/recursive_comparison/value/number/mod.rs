//! Defines the [`Number`] type which represents numbers in the [`Value`]
//! data structure used for recursive comparison.
//!
//! [`Value`]: crate::recursive_comparison::value::Value

use crate::std::borrow::Cow;
use crate::std::fmt::{self, Debug, Display};

/// Represents a number in the [`Value`] data structure used for recursive
/// comparison.
///
/// It can hold integer and float values of different sizes.
///
/// # Note on `usize` and `isize`
///
/// The `serde` crate does not explicitly support `usize` and `isize`, because
/// the size of those types is platform-dependent and cannot be safely
/// serialized on one platform and deserialized on another.
///
/// For recursive comparison purposes we try to convert `usize` and `isize`
/// values to `u64` and `i64` values respectively. If the conversion fails, we
/// try to convert them to `u128` and `i128` values. If both conversions fail,
/// we panic.
///
/// # Note on `f32` and `f64`
///
/// The [`Value`] type implements the `Eq`, `Ord` and `Hash` traits. Although
/// these traits cannot be implemented for `f32` and `f64` in a mathematically
/// correct way, for the purpose of asserting whether two values are equal,
/// the naive implementation we are using is perfectly fine.
///
/// [`Value`]: crate::recursive_comparison::value::Value
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Number {
    /// An `i8` signed integer
    I8(i8),
    /// An `i16` signed integer
    I16(i16),
    /// An `i32` signed integer
    I32(i32),
    /// An `i64` signed integer
    I64(i64),
    /// An `i128` signed integer
    I128(i128),
    /// An `u8` unsigned integer
    U8(u8),
    /// An `u16` unsigned integer
    U16(u16),
    /// An `u32` unsigned integer
    U32(u32),
    /// An `u64` unsigned integer
    U64(u64),
    /// An `u128` unsigned integer
    U128(u128),
    /// A `f32` floating point number
    F32(F32),
    /// A `f64` floating point number
    F64(F64),
}

impl Number {
    /// Returns the type name of the number variant as a string.
    pub fn type_name(&self) -> Cow<'static, str> {
        match self {
            Self::I8(_) => Cow::Borrowed("i8"),
            Self::I16(_) => Cow::Borrowed("i16"),
            Self::I32(_) => Cow::Borrowed("i32"),
            Self::I64(_) => Cow::Borrowed("i64"),
            Self::I128(_) => Cow::Borrowed("i128"),
            Self::U8(_) => Cow::Borrowed("u8"),
            Self::U16(_) => Cow::Borrowed("u16"),
            Self::U32(_) => Cow::Borrowed("u32"),
            Self::U64(_) => Cow::Borrowed("u64"),
            Self::U128(_) => Cow::Borrowed("u128"),
            Self::F32(_) => Cow::Borrowed("f32"),
            Self::F64(_) => Cow::Borrowed("f64"),
        }
    }
}

impl Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::I8(value) => write!(f, "{value:?}"),
            Self::I16(value) => write!(f, "{value:?}"),
            Self::I32(value) => write!(f, "{value:?}"),
            Self::I64(value) => write!(f, "{value:?}"),
            Self::I128(value) => write!(f, "{value:?}"),
            Self::U8(value) => write!(f, "{value:?}"),
            Self::U16(value) => write!(f, "{value:?}"),
            Self::U32(value) => write!(f, "{value:?}"),
            Self::U64(value) => write!(f, "{value:?}"),
            Self::U128(value) => write!(f, "{value:?}"),
            Self::F32(value) => write!(f, "{value:?}"),
            Self::F64(value) => write!(f, "{value:?}"),
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::I8(value) => write!(f, "{value}"),
            Self::I16(value) => write!(f, "{value}"),
            Self::I32(value) => write!(f, "{value}"),
            Self::I64(value) => write!(f, "{value}"),
            Self::I128(value) => write!(f, "{value}"),
            Self::U8(value) => write!(f, "{value}"),
            Self::U16(value) => write!(f, "{value}"),
            Self::U32(value) => write!(f, "{value}"),
            Self::U64(value) => write!(f, "{value}"),
            Self::U128(value) => write!(f, "{value}"),
            Self::F32(value) => write!(f, "{value}"),
            Self::F64(value) => write!(f, "{value}"),
        }
    }
}

macro_rules! define_float_type {
    ($ty:ident($float:ty)) => {
        /// A wrapper around a float value.
        ///
        /// This wrapper enables us to provide implementations for the
        /// `Eq`, `Ord`, and `Hash` traits.
        #[derive(Clone, Copy)]
        pub struct $ty(pub $float);

        impl $ty {
            /// Returns the underlying float value.
            pub fn val(self) -> $float {
                self.0
            }
        }

        impl crate::std::convert::From<$float> for $ty {
            fn from(val: $float) -> Self {
                Self(val)
            }
        }

        impl crate::std::borrow::Borrow<$float> for $ty {
            fn borrow(&self) -> &$float {
                &self.0
            }
        }

        impl crate::std::ops::Deref for $ty {
            type Target = $float;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl crate::std::ops::DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl crate::std::fmt::Debug for $ty {
            fn fmt(&self, f: &mut crate::std::fmt::Formatter<'_>) -> crate::std::fmt::Result {
                crate::std::write!(f, "{:?}", self.0)
            }
        }

        impl crate::std::hash::Hash for $ty {
            fn hash<H>(&self, state: &mut H)
            where
                H: crate::std::hash::Hasher,
            {
                if self.0 == -0. {
                    let zero: $float = 0.;
                    return zero.to_bits().hash(state);
                }
                self.0.to_bits().hash(state)
            }
        }

        impl crate::std::cmp::Ord for $ty {
            fn cmp(&self, other: &Self) -> crate::std::cmp::Ordering {
                self.0.total_cmp(&other.0)
            }
        }

        impl crate::std::cmp::PartialOrd for $ty {
            fn partial_cmp(&self, other: &Self) -> Option<crate::std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl crate::std::cmp::PartialEq for $ty {
            fn eq(&self, other: &Self) -> bool {
                if self.0 == 0. && other.0 == -0. {
                    return true;
                }
                self.cmp(other).is_eq()
            }
        }

        impl Eq for $ty {}

        impl crate::std::fmt::Display for $ty {
            fn fmt(&self, f: &mut crate::std::fmt::Formatter<'_>) -> crate::std::fmt::Result {
                crate::std::write!(f, "{}", self.0)
            }
        }
    };
}

define_float_type! { F32(f32) }
define_float_type! { F64(f64) }

macro_rules! impl_number_from_float {
    ($ty:ty => $variant:ident) => {
        impl crate::std::convert::From<$ty> for Number {
            fn from(value: $ty) -> Self {
                Self::$variant(value.into())
            }
        }
    };
}

impl_number_from_float! { f32 => F32 }
impl_number_from_float! { f64 => F64 }

macro_rules! impl_number_from_integer {
    ($ty:ty => $variant:ident) => {
        impl crate::std::convert::From<$ty> for Number {
            fn from(value: $ty) -> Self {
                Self::$variant(value)
            }
        }
    };
}

impl_number_from_integer! { i8 => I8 }
impl_number_from_integer! { i16 => I16 }
impl_number_from_integer! { i32 => I32 }
impl_number_from_integer! { i64 => I64 }
impl_number_from_integer! { i128 => I128 }
impl_number_from_integer! { u8 => U8 }
impl_number_from_integer! { u16 => U16 }
impl_number_from_integer! { u32 => U32 }
impl_number_from_integer! { u64 => U64 }
impl_number_from_integer! { u128 => U128 }

#[cfg(test)]
mod tests;
