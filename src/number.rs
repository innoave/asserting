//! Implementations of assertions specific for numbers.

use crate::assertions::{
    AssertDecimalNumber, AssertInfinity, AssertNotANumber, AssertNumericIdentity, AssertSignum,
};
use crate::colored::{mark_missing, mark_missing_substr, mark_unexpected};
use crate::expectations::{
    HasPrecisionOf, HasScaleOf, IsANumber, IsFinite, IsInfinite, IsInteger, IsNegative,
    IsNotANumber, IsNotNegative, IsNotPositive, IsOne, IsPositive, IsZero,
};
use crate::properties::{
    AdditiveIdentityProperty, DecimalProperties, InfinityProperty, IsNanProperty,
    MultiplicativeIdentityProperty, SignumProperty,
};
use crate::spec::{DiffFormat, Expectation, Expression, FailingStrategy, Spec};
use crate::std::fmt::Debug;
use crate::std::format;
use crate::std::string::String;

impl<S, R> AssertSignum for Spec<'_, S, R>
where
    S: SignumProperty + Debug,
    R: FailingStrategy,
{
    fn is_negative(self) -> Self {
        self.expecting(IsNegative)
    }

    fn is_not_negative(self) -> Self {
        self.expecting(IsNotNegative)
    }

    fn is_positive(self) -> Self {
        self.expecting(IsPositive)
    }

    fn is_not_positive(self) -> Self {
        self.expecting(IsNotPositive)
    }
}

impl<S> Expectation<S> for IsNegative
where
    S: SignumProperty + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.is_negative_property()
    }

    fn message(&self, expression: &Expression<'_>, actual: &S, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing_substr("< 0", format);
        format!("expected {expression} is negative\n   but was: {marked_actual}\n  expected: {marked_expected}")
    }
}

impl<S> Expectation<S> for IsNotNegative
where
    S: SignumProperty + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        !subject.is_negative_property()
    }

    fn message(&self, expression: &Expression<'_>, actual: &S, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing_substr(">= 0", format);
        format!("expected {expression} is not negative\n   but was: {marked_actual}\n  expected: {marked_expected}")
    }
}

impl<S> Expectation<S> for IsPositive
where
    S: SignumProperty + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.is_positive_property()
    }

    fn message(&self, expression: &Expression<'_>, actual: &S, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing_substr("> 0", format);
        format!("expected {expression} is positive\n   but was: {marked_actual}\n  expected: {marked_expected}")
    }
}

impl<S> Expectation<S> for IsNotPositive
where
    S: SignumProperty + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        !subject.is_positive_property()
    }

    fn message(&self, expression: &Expression<'_>, actual: &S, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing_substr("<= 0", format);
        format!("expected {expression} is not positive\n   but was: {marked_actual}\n  expected: {marked_expected}")
    }
}

impl<S, R> AssertNumericIdentity for Spec<'_, S, R>
where
    S: AdditiveIdentityProperty + MultiplicativeIdentityProperty + PartialEq + Debug,
    R: FailingStrategy,
{
    fn is_zero(self) -> Self {
        self.expecting(IsZero)
    }

    fn is_one(self) -> Self {
        self.expecting(IsOne)
    }
}

impl<S> Expectation<S> for IsZero
where
    S: AdditiveIdentityProperty + PartialEq + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        *subject == <S as AdditiveIdentityProperty>::additive_identity()
    }

    fn message(&self, expression: &Expression<'_>, actual: &S, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected(&actual, format);
        let marked_expected = mark_missing(&S::additive_identity(), format);
        format!("expected {expression} is zero\n   but was: {marked_actual}\n  expected: {marked_expected}")
    }
}

impl<S> Expectation<S> for IsOne
where
    S: MultiplicativeIdentityProperty + PartialEq + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        *subject == <S as MultiplicativeIdentityProperty>::multiplicative_identity()
    }

    fn message(&self, expression: &Expression<'_>, actual: &S, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing(&S::multiplicative_identity(), format);
        format!("expected {expression} is one\n   but was: {marked_actual}\n  expected: {marked_expected}")
    }
}

impl<S, R> AssertInfinity for Spec<'_, S, R>
where
    S: InfinityProperty + Debug,
    R: FailingStrategy,
{
    fn is_infinite(self) -> Self {
        self.expecting(IsInfinite)
    }

    fn is_finite(self) -> Self {
        self.expecting(IsFinite)
    }
}

impl<S> Expectation<S> for IsFinite
where
    S: InfinityProperty + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.is_finite_property()
    }

    fn message(&self, expression: &Expression<'_>, actual: &S, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing_substr("a finite number", format);
        format!("expected {expression} is finite\n   but was: {marked_actual}\n  expected: {marked_expected}")
    }
}

impl<S> Expectation<S> for IsInfinite
where
    S: InfinityProperty + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.is_infinite_property()
    }

    fn message(&self, expression: &Expression<'_>, actual: &S, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing_substr("an infinite number", format);
        format!("expected {expression} is infinite\n   but was: {marked_actual}\n  expected: {marked_expected}")
    }
}

impl<S, R> AssertNotANumber for Spec<'_, S, R>
where
    S: IsNanProperty + Debug,
    R: FailingStrategy,
{
    fn is_not_a_number(self) -> Self {
        self.expecting(IsNotANumber)
    }

    fn is_a_number(self) -> Self {
        self.expecting(IsANumber)
    }
}

impl<S> Expectation<S> for IsANumber
where
    S: IsNanProperty + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        !subject.is_nan_property()
    }

    fn message(&self, expression: &Expression<'_>, actual: &S, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing_substr("a number", format);
        format!("expected {expression} is a number\n   but was: {marked_actual}\n  expected: {marked_expected}")
    }
}

impl<S> Expectation<S> for IsNotANumber
where
    S: IsNanProperty + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.is_nan_property()
    }

    fn message(&self, expression: &Expression<'_>, actual: &S, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing_substr("NaN", format);
        format!("expected {expression} is not a number (NaN)\n   but was: {marked_actual}\n  expected: {marked_expected}")
    }
}

impl<S, R> AssertDecimalNumber for Spec<'_, S, R>
where
    S: DecimalProperties + Debug,
    R: FailingStrategy,
{
    fn has_scale_of(self, expected_scale: i64) -> Self {
        self.expecting(HasScaleOf { expected_scale })
    }

    fn has_precision_of(self, expected_precision: u64) -> Self {
        self.expecting(HasPrecisionOf { expected_precision })
    }

    fn is_integer(self) -> Self {
        self.expecting(IsInteger)
    }
}

impl<S> Expectation<S> for HasScaleOf
where
    S: DecimalProperties + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.scale_property() == self.expected_scale
    }

    fn message(&self, expression: &Expression<'_>, actual: &S, format: &DiffFormat) -> String {
        let expected_scale = self.expected_scale;
        let marked_actual = mark_unexpected(&actual.scale_property(), format);
        let marked_expected = mark_missing(&expected_scale, format);
        format!("expected {expression} to have a scale of {expected_scale}\n   but was: {marked_actual}\n  expected: {marked_expected}")
    }
}

impl<S> Expectation<S> for HasPrecisionOf
where
    S: DecimalProperties + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.precision_property() == self.expected_precision
    }

    fn message(&self, expression: &Expression<'_>, actual: &S, format: &DiffFormat) -> String {
        let expected_precision = self.expected_precision;
        let marked_actual = mark_unexpected(&actual.precision_property(), format);
        let marked_expected = mark_missing(&expected_precision, format);
        format!("expected {expression} to have a precision of {expected_precision}\n   but was: {marked_actual}\n  expected: {marked_expected}")
    }
}

impl<S> Expectation<S> for IsInteger
where
    S: DecimalProperties + Debug,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.is_integer_property()
    }

    fn message(&self, expression: &Expression<'_>, actual: &S, format: &DiffFormat) -> String {
        let marked_actual = mark_unexpected(&actual, format);
        let marked_expected = mark_missing_substr("an integer value", format);
        format!("expected {expression} to be an integer value\n   but was: {marked_actual}\n  expected: {marked_expected}")
    }
}
