//! Implementation of assertions for integer values.

use crate::properties::{AdditiveIdentityProperty, MultiplicativeIdentityProperty, SignumProperty};

macro_rules! impl_signum_property {
    ($type:ty) => {
        impl SignumProperty for $type {
            fn is_negative_property(&self) -> bool {
                self.is_negative()
            }

            fn is_positive_property(&self) -> bool {
                self.is_positive()
            }
        }
    };
}

impl_signum_property!(i8);
impl_signum_property!(i16);
impl_signum_property!(i32);
impl_signum_property!(i64);
impl_signum_property!(i128);
impl_signum_property!(isize);

macro_rules! impl_additive_identity_property {
    ($type:ty) => {
        impl AdditiveIdentityProperty for $type {
            fn additive_identity() -> Self {
                0
            }
        }

        impl AdditiveIdentityProperty for &$type {
            fn additive_identity() -> Self {
                &0
            }
        }
    };
}

impl_additive_identity_property!(i8);
impl_additive_identity_property!(i16);
impl_additive_identity_property!(i32);
impl_additive_identity_property!(i64);
impl_additive_identity_property!(i128);
impl_additive_identity_property!(isize);

impl_additive_identity_property!(u8);
impl_additive_identity_property!(u16);
impl_additive_identity_property!(u32);
impl_additive_identity_property!(u64);
impl_additive_identity_property!(u128);
impl_additive_identity_property!(usize);

macro_rules! impl_multiplicative_identity_property {
    ($type:ty) => {
        impl MultiplicativeIdentityProperty for $type {
            fn multiplicative_identity() -> Self {
                1
            }
        }

        impl MultiplicativeIdentityProperty for &$type {
            fn multiplicative_identity() -> Self {
                &1
            }
        }
    };
}

impl_multiplicative_identity_property!(i8);
impl_multiplicative_identity_property!(i16);
impl_multiplicative_identity_property!(i32);
impl_multiplicative_identity_property!(i64);
impl_multiplicative_identity_property!(i128);
impl_multiplicative_identity_property!(isize);

impl_multiplicative_identity_property!(u8);
impl_multiplicative_identity_property!(u16);
impl_multiplicative_identity_property!(u32);
impl_multiplicative_identity_property!(u64);
impl_multiplicative_identity_property!(u128);
impl_multiplicative_identity_property!(usize);

#[cfg(test)]
mod tests;
