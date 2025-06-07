use crate::prelude::{AdditiveIdentityProperty, MultiplicativeIdentityProperty};
use crate::properties::SignumProperty;
use crate::std::vec;
use lazy_static::lazy_static;
use num_bigint::{BigInt, Sign};

static BIGINT_ZERO: BigInt = BigInt::ZERO;

lazy_static! {
    static ref BIGINT_ONE: BigInt = bigint_one();
}

#[inline]
fn bigint_one() -> BigInt {
    BigInt::new(Sign::Plus, vec![1])
}

impl SignumProperty for BigInt {
    fn is_negative_property(&self) -> bool {
        self.sign() == Sign::Minus
    }

    fn is_positive_property(&self) -> bool {
        self.sign() == Sign::Plus
    }
}

impl AdditiveIdentityProperty for BigInt {
    fn additive_identity() -> Self {
        Self::ZERO
    }
}

impl AdditiveIdentityProperty for &BigInt {
    fn additive_identity() -> Self {
        &BIGINT_ZERO
    }
}

impl MultiplicativeIdentityProperty for BigInt {
    fn multiplicative_identity() -> Self {
        bigint_one()
    }
}

impl MultiplicativeIdentityProperty for &BigInt {
    fn multiplicative_identity() -> Self {
        &BIGINT_ONE
    }
}

#[cfg(test)]
mod tests;
