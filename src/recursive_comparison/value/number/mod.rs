use crate::std::fmt::{self, Debug, Display};
use std::borrow::Cow;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Number {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    F32(F32),
    F64(F64),
}

impl Number {
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
        #[derive(Clone, Copy)]
        pub struct $ty(pub $float);

        impl $ty {
            #[allow(dead_code)]
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
