use crate::properties::{AdditiveIdentityProperty, MultiplicativeIdentityProperty, SignumProperty};
use bigdecimal::num_bigint::Sign;
use bigdecimal::{BigDecimal, BigDecimalRef, One, Zero};
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
        self.sign() == Sign::Minus
    }

    fn is_positive_property(&self) -> bool {
        self.sign() == Sign::Plus
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

impl SignumProperty for BigDecimalRef<'_> {
    fn is_negative_property(&self) -> bool {
        self.sign() == Sign::Minus
    }

    fn is_positive_property(&self) -> bool {
        self.sign() == Sign::Plus
    }
}

impl AdditiveIdentityProperty for BigDecimalRef<'_> {
    fn additive_identity() -> Self {
        BIGDECIMAL_ZERO.to_ref()
    }
}

impl MultiplicativeIdentityProperty for BigDecimalRef<'_> {
    fn multiplicative_identity() -> Self {
        BIGDECIMAL_ONE.to_ref()
    }
}

#[cfg(test)]
mod tests;
