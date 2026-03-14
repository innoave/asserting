use crate::properties::{
    AdditiveIdentityProperty, DecimalProperties, MultiplicativeIdentityProperty, SignumProperty,
};
use bigdecimal::num_bigint::Sign;
use bigdecimal::{BigDecimal, BigDecimalRef, One, Zero};
use once_cell::sync::Lazy;

#[allow(clippy::non_std_lazy_statics)]
static BIGDECIMAL_ZERO: Lazy<BigDecimal> = Lazy::new(bigdecimal_zero);
#[allow(clippy::non_std_lazy_statics)]
static BIGDECIMAL_ONE: Lazy<BigDecimal> = Lazy::new(bigdecimal_one);

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

impl DecimalProperties for BigDecimal {
    fn precision_property(&self) -> u64 {
        self.digits()
    }

    fn scale_property(&self) -> i64 {
        self.fractional_digit_count()
    }

    fn is_integer_property(&self) -> bool {
        self.is_integer()
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
