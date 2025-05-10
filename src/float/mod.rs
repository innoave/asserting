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

impl AdditiveIdentityProperty for f32 {
    const ADDITIVE_IDENTITY: Self = 0.;
}

impl AdditiveIdentityProperty for f64 {
    const ADDITIVE_IDENTITY: Self = 0.;
}

impl MultiplicativeIdentityProperty for f32 {
    const MULTIPLICATIVE_IDENTITY: Self = 1.;
}

impl MultiplicativeIdentityProperty for f64 {
    const MULTIPLICATIVE_IDENTITY: Self = 1.;
}

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
    use crate::expectations::{IsCloseTo, IsNotCloseTo};
    use crate::spec::{DiffFormat, Expectation, Expression, FailingStrategy, Spec};
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
            self.expecting(
                IsNotCloseTo::<_, F32Margin>::new(expected).within_margin((4. * f32::EPSILON, 4)),
            )
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
            self.expecting(IsNotCloseTo::new(expected).within_margin(margin))
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
            self.expecting(
                IsNotCloseTo::<_, F64Margin>::new(expected).within_margin((4. * f64::EPSILON, 4)),
            )
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
            self.expecting(IsNotCloseTo::new(expected).within_margin(margin))
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
            format: &DiffFormat,
        ) -> String {
            let (marked_actual, marked_expected) = mark_diff(actual, &self.expected, format);
            format!("expected {expression} is close to {:?}\n  within a margin of epsilon={:e} and ulps={}\n   but was: {marked_actual}\n  expected: {marked_expected}",
                &self.expected, self.margin.epsilon, self.margin.ulps
            )
        }
    }

    impl Expectation<f32> for IsNotCloseTo<f32, F32Margin> {
        fn test(&mut self, subject: &f32) -> bool {
            !subject.approx_eq(self.expected, self.margin)
        }

        fn message(
            &self,
            expression: &Expression<'_>,
            actual: &f32,
            _format: &DiffFormat,
        ) -> String {
            format!("expected {expression} is not close to {:?}\n  within a margin of epsilon={:e} and ulps={}\n   but was: {actual:?}\n  expected: {:?}",
                &self.expected, self.margin.epsilon, self.margin.ulps, &self.expected
            )
        }
    }

    impl Expectation<f64> for IsCloseTo<f64, F64Margin> {
        fn test(&mut self, subject: &f64) -> bool {
            subject.approx_eq(self.expected, self.margin)
        }

        fn message(
            &self,
            expression: &Expression<'_>,
            actual: &f64,
            format: &DiffFormat,
        ) -> String {
            let (marked_actual, marked_expected) = mark_diff(actual, &self.expected, format);
            format!("expected {expression} is close to {:?}\n  within a margin of epsilon={:e} and ulps={}\n   but was: {marked_actual}\n  expected: {marked_expected}",
                &self.expected, self.margin.epsilon, self.margin.ulps
            )
        }
    }

    impl Expectation<f64> for IsNotCloseTo<f64, F64Margin> {
        fn test(&mut self, subject: &f64) -> bool {
            !subject.approx_eq(self.expected, self.margin)
        }

        fn message(
            &self,
            expression: &Expression<'_>,
            actual: &f64,
            _format: &DiffFormat,
        ) -> String {
            format!("expected {expression} is not close to {:?}\n  within a margin of epsilon={:e} and ulps={}\n   but was: {actual:?}\n  expected: {:?}",
                &self.expected, self.margin.epsilon, self.margin.ulps, &self.expected
            )
        }
    }
}

#[cfg(test)]
mod tests;
