use crate::properties::{AdditiveIdentityProperty, MultiplicativeIdentityProperty, SignumProperty};
use crate::std::vec;
use num_bigint::{BigInt, BigUint, Sign};
use once_cell::sync::Lazy;

static BIGINT_ZERO: BigInt = BigInt::ZERO;
static BIGUINT_ZERO: BigUint = BigUint::ZERO;

#[allow(clippy::non_std_lazy_statics)]
static BIGINT_ONE: Lazy<BigInt> = Lazy::new(bigint_one);
#[allow(clippy::non_std_lazy_statics)]
static BIGUINT_ONE: Lazy<BigUint> = Lazy::new(biguint_one);

#[inline]
fn bigint_one() -> BigInt {
    BigInt::new(Sign::Plus, vec![1])
}

#[inline]
fn biguint_one() -> BigUint {
    BigUint::new(vec![1])
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

impl AdditiveIdentityProperty for BigUint {
    fn additive_identity() -> Self {
        Self::ZERO
    }
}

impl AdditiveIdentityProperty for &BigUint {
    fn additive_identity() -> Self {
        &BIGUINT_ZERO
    }
}

impl MultiplicativeIdentityProperty for BigUint {
    fn multiplicative_identity() -> Self {
        biguint_one()
    }
}

impl MultiplicativeIdentityProperty for &BigUint {
    fn multiplicative_identity() -> Self {
        &BIGUINT_ONE
    }
}

#[cfg(test)]
mod tests;
