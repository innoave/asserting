use crate::properties::{AdditiveIdentityProperty, MultiplicativeIdentityProperty, SignumProperty};
use rust_decimal::Decimal;

impl SignumProperty for Decimal {
    fn is_negative_property(&self) -> bool {
        self.is_sign_negative()
    }

    fn is_positive_property(&self) -> bool {
        self.is_sign_positive() && !self.is_zero()
    }
}

impl AdditiveIdentityProperty for Decimal {
    fn additive_identity() -> Self {
        Self::ZERO
    }
}

impl AdditiveIdentityProperty for &Decimal {
    fn additive_identity() -> Self {
        &Decimal::ZERO
    }
}

impl MultiplicativeIdentityProperty for Decimal {
    fn multiplicative_identity() -> Self {
        Self::ONE
    }
}

impl MultiplicativeIdentityProperty for &Decimal {
    fn multiplicative_identity() -> Self {
        &Decimal::ONE
    }
}

#[cfg(test)]
mod tests;
