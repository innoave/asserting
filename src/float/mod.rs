use crate::properties::{
    AdditiveIdentityProperty, InfinityProperty, IsNanProperty, MultiplicativeIdentityProperty,
    SignumProperty,
};

impl SignumProperty for f32 {
    fn is_negative_property(&self) -> bool {
        self.is_sign_negative()
    }

    fn is_positive_property(&self) -> bool {
        self.is_sign_positive() && *self != 0.
    }
}

impl SignumProperty for f64 {
    fn is_negative_property(&self) -> bool {
        self.is_sign_negative()
    }

    fn is_positive_property(&self) -> bool {
        self.is_sign_positive() && *self != 0.
    }
}

macro_rules! impl_additive_identity_property {
    ($type:ty) => {
        impl AdditiveIdentityProperty for $type {
            fn additive_identity() -> Self {
                0.
            }
        }

        impl AdditiveIdentityProperty for &$type {
            fn additive_identity() -> Self {
                &0.
            }
        }
    };
}

impl_additive_identity_property!(f32);
impl_additive_identity_property!(f64);

macro_rules! impl_multiplicative_identity_property {
    ($type:ty) => {
        impl MultiplicativeIdentityProperty for $type {
            fn multiplicative_identity() -> Self {
                1.
            }
        }

        impl MultiplicativeIdentityProperty for &$type {
            fn multiplicative_identity() -> Self {
                &1.
            }
        }
    };
}

impl_multiplicative_identity_property!(f32);
impl_multiplicative_identity_property!(f64);

impl InfinityProperty for f32 {
    fn is_infinite_property(&self) -> bool {
        self.is_infinite()
    }

    fn is_finite_property(&self) -> bool {
        self.is_finite()
    }
}

impl InfinityProperty for f64 {
    fn is_infinite_property(&self) -> bool {
        self.is_infinite()
    }

    fn is_finite_property(&self) -> bool {
        self.is_finite()
    }
}

impl IsNanProperty for f32 {
    fn is_nan_property(&self) -> bool {
        self.is_nan()
    }
}

impl IsNanProperty for f64 {
    fn is_nan_property(&self) -> bool {
        self.is_nan()
    }
}

#[cfg(feature = "float-cmp")]
mod cmp {
    use crate::assertions::{AssertIsCloseToWithDefaultMargin, AssertIsCloseToWithinMargin};
    use crate::colored::mark_diff;
    use crate::expectations::{IsCloseTo, Not};
    use crate::spec::{DiffFormat, Expectation, Expression, FailingStrategy, Invertible, Spec};
    use crate::std::{format, string::String};
    use float_cmp::{ApproxEq, F32Margin, F64Margin};

    impl<R> AssertIsCloseToWithDefaultMargin<f32> for Spec<'_, f32, R>
    where
        R: FailingStrategy,
    {
        fn is_close_to(self, expected: f32) -> Self {
            self.expecting(
                IsCloseTo::<_, F32Margin>::new(expected).within_margin((4. * f32::EPSILON, 4)),
            )
        }

        fn is_not_close_to(self, expected: f32) -> Self {
            self.expecting(Not(
                IsCloseTo::<_, F32Margin>::new(expected).within_margin((4. * f32::EPSILON, 4))
            ))
        }
    }

    impl<R> AssertIsCloseToWithinMargin<f32, F32Margin> for Spec<'_, f32, R>
    where
        R: FailingStrategy,
    {
        fn is_close_to_with_margin(self, expected: f32, margin: impl Into<F32Margin>) -> Self {
            self.expecting(IsCloseTo::new(expected).within_margin(margin))
        }

        fn is_not_close_to_with_margin(self, expected: f32, margin: impl Into<F32Margin>) -> Self {
            self.expecting(Not(IsCloseTo::new(expected).within_margin(margin)))
        }
    }

    impl<R> AssertIsCloseToWithDefaultMargin<f64> for Spec<'_, f64, R>
    where
        R: FailingStrategy,
    {
        fn is_close_to(self, expected: f64) -> Self {
            self.expecting(
                IsCloseTo::<_, F64Margin>::new(expected).within_margin((4. * f64::EPSILON, 4)),
            )
        }

        fn is_not_close_to(self, expected: f64) -> Self {
            self.expecting(Not(
                IsCloseTo::<_, F64Margin>::new(expected).within_margin((4. * f64::EPSILON, 4))
            ))
        }
    }

    impl<R> AssertIsCloseToWithinMargin<f64, F64Margin> for Spec<'_, f64, R>
    where
        R: FailingStrategy,
    {
        fn is_close_to_with_margin(self, expected: f64, margin: impl Into<F64Margin>) -> Self {
            self.expecting(IsCloseTo::new(expected).within_margin(margin))
        }

        fn is_not_close_to_with_margin(self, expected: f64, margin: impl Into<F64Margin>) -> Self {
            self.expecting(Not(IsCloseTo::new(expected).within_margin(margin)))
        }
    }

    impl Expectation<f32> for IsCloseTo<f32, F32Margin> {
        fn test(&mut self, subject: &f32) -> bool {
            subject.approx_eq(self.expected, self.margin)
        }

        fn message(
            &self,
            expression: &Expression<'_>,
            actual: &f32,
            inverted: bool,
            format: &DiffFormat,
        ) -> String {
            let not = if inverted { "not " } else { "" };
            let (marked_actual, marked_expected) = mark_diff(actual, &self.expected, format);
            format!("expected {expression} to be {not}close to {:?}\n  within a margin of epsilon={:e} and ulps={}\n   but was: {marked_actual}\n  expected: {marked_expected}",
                &self.expected, self.margin.epsilon, self.margin.ulps
            )
        }
    }

    impl Invertible for IsCloseTo<f32, F32Margin> {}

    impl Expectation<f64> for IsCloseTo<f64, F64Margin> {
        fn test(&mut self, subject: &f64) -> bool {
            subject.approx_eq(self.expected, self.margin)
        }

        fn message(
            &self,
            expression: &Expression<'_>,
            actual: &f64,
            inverted: bool,
            format: &DiffFormat,
        ) -> String {
            let not = if inverted { "not " } else { "" };
            let (marked_actual, marked_expected) = mark_diff(actual, &self.expected, format);
            format!("expected {expression} to be {not}close to {:?}\n  within a margin of epsilon={:e} and ulps={}\n   but was: {marked_actual}\n  expected: {marked_expected}",
                &self.expected, self.margin.epsilon, self.margin.ulps
            )
        }
    }

    impl Invertible for IsCloseTo<f64, F64Margin> {}
}

#[cfg(test)]
mod tests;
