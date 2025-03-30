use crate::assertions::{AssertIsCloseToWithDefaultMargin, AssertIsCloseToWithinMargin};
use crate::expectations::{IsCloseTo, IsNotCloseTo};
use crate::spec::{DiffFormat, Expectation, Expression, FailingStrategy, Spec};
#[cfg(not(feature = "std"))]
use alloc::{format, string::String};
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

    fn message(&self, expression: Expression<'_>, actual: &f32, _format: &DiffFormat) -> String {
        format!("expected {expression} is close to {:?}\n   but was: {actual:?}\n  expected: {:?} within a margin of epsilon={:e} and ulps={}",
            &self.expected, &self.expected, self.margin.epsilon, self.margin.ulps
        )
    }
}

impl Expectation<f32> for IsNotCloseTo<f32, F32Margin> {
    fn test(&mut self, subject: &f32) -> bool {
        !subject.approx_eq(self.expected, self.margin)
    }

    fn message(&self, expression: Expression<'_>, actual: &f32, _format: &DiffFormat) -> String {
        format!("expected {expression} is not close to {:?}\n   but was: {actual:?}\n  expected: {:?} within a margin of epsilon={:e} and ulps={}",
            &self.expected, &self.expected, self.margin.epsilon, self.margin.ulps
        )
    }
}

impl Expectation<f64> for IsCloseTo<f64, F64Margin> {
    fn test(&mut self, subject: &f64) -> bool {
        subject.approx_eq(self.expected, self.margin)
    }

    fn message(&self, expression: Expression<'_>, actual: &f64, _format: &DiffFormat) -> String {
        format!("expected {expression} is close to {:?}\n   but was: {actual:?}\n  expected: {:?} within a margin of epsilon={:e} and ulps={}",
            &self.expected, &self.expected, self.margin.epsilon, self.margin.ulps
        )
    }
}

impl Expectation<f64> for IsNotCloseTo<f64, F64Margin> {
    fn test(&mut self, subject: &f64) -> bool {
        !subject.approx_eq(self.expected, self.margin)
    }

    fn message(&self, expression: Expression<'_>, actual: &f64, _format: &DiffFormat) -> String {
        format!("expected {expression} is not close to {:?}\n   but was: {actual:?}\n  expected: {:?} within a margin of epsilon={:e} and ulps={}",
            &self.expected, &self.expected, self.margin.epsilon, self.margin.ulps
        )
    }
}

#[cfg(test)]
mod tests;
