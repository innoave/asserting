use crate::properties::{AdditiveIdentityProperty, MultiplicativeIdentityProperty, SignumProperty};
use bigdecimal::{BigDecimal, One, Signed, Zero};
use lazy_static::lazy_static;

lazy_static! {
    static ref BIGDECIMAL_ZERO: BigDecimal = bigdecimal_zero();
    static ref BIGDECIMAL_ONE: BigDecimal = bigdecimal_one();
}

#[inline]
fn bigdecimal_zero() -> BigDecimal {
    BigDecimal::zero()
}

#[inline]
fn bigdecimal_one() -> BigDecimal {
    BigDecimal::one()
}

impl SignumProperty for BigDecimal {
    fn is_negative_property(&self) -> bool {
        self.is_negative()
    }

    fn is_positive_property(&self) -> bool {
        self.is_positive()
    }
}

impl AdditiveIdentityProperty for BigDecimal {
    fn additive_identity() -> Self {
        bigdecimal_zero()
    }
}

impl AdditiveIdentityProperty for &BigDecimal {
    fn additive_identity() -> Self {
        &BIGDECIMAL_ZERO
    }
}

impl MultiplicativeIdentityProperty for BigDecimal {
    fn multiplicative_identity() -> Self {
        bigdecimal_one()
    }
}

impl MultiplicativeIdentityProperty for &BigDecimal {
    fn multiplicative_identity() -> Self {
        &BIGDECIMAL_ONE
    }
}

#[cfg(test)]
mod tests;
