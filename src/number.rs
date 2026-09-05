//! Implementations of assertions specific for numbers.

use crate::assertions::{
    AssertDecimalNumber, AssertInfinity, AssertNotANumber, AssertNumericIdentity, AssertSignum,
};
use crate::colored::{mark_missing, mark_unexpected};
use crate::expectations::{
    HasPrecisionOf, HasScaleOf, IsANumber, IsFinite, IsInfinite, IsInteger, IsNegative, IsOne,
    IsPositive, IsZero, has_precision_of, has_scale_of, is_a_number, is_finite, is_infinite,
    is_integer, is_negative, is_one, is_positive, is_zero, not,
};
use crate::properties::{
    AdditiveIdentityProperty, DecimalProperties, InfinityProperty, IsNanProperty,
    MultiplicativeIdentityProperty, SignumProperty,
};
use crate::spec::{
    DiffFormat, DisplayRepresentation, Expectation, Expecting, Expression, FailingStrategy,
    Invertible, Represent, Represented, Spec,
};
use crate::std::format;
use crate::std::string::String;

impl<S, D, R> AssertSignum for Spec<'_, S, D, R>
where
    S: SignumProperty,
    D: Represent<S>,
    R: FailingStrategy,
{
    fn is_negative(self) -> Self {
        self.expecting(is_negative())
    }

    fn is_not_negative(self) -> Self {
        self.expecting(not(is_negative()))
    }

    fn is_positive(self) -> Self {
        self.expecting(is_positive())
    }

    fn is_not_positive(self) -> Self {
        self.expecting(not(is_positive()))
    }
}

impl<S, D> Expectation<S, D> for IsNegative
where
    S: SignumProperty,
    D: Represent<S>,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.is_negative_property()
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let (not, expected) = if inverted {
            ("not ", ">= 0")
        } else {
            ("", "< 0")
        };
        let marked_actual = mark_unexpected(actual, representation, format);
        let marked_expected = mark_missing(expected, &DisplayRepresentation, format);
        format!(
            "expected {expression} to be {not}negative\n   but was: {marked_actual}\n  expected: {marked_expected}"
        )
    }
}

impl Invertible for IsNegative {}

impl<S, D> Expectation<S, D> for IsPositive
where
    S: SignumProperty,
    D: Represent<S>,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.is_positive_property()
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let (not, expected) = if inverted {
            ("not ", "<= 0")
        } else {
            ("", "> 0")
        };
        let marked_actual = mark_unexpected(actual, representation, format);
        let marked_expected = mark_missing(expected, &DisplayRepresentation, format);
        format!(
            "expected {expression} to be {not}positive\n   but was: {marked_actual}\n  expected: {marked_expected}"
        )
    }
}

impl Invertible for IsPositive {}

impl<S, D, R> AssertNumericIdentity for Spec<'_, S, D, R>
where
    S: AdditiveIdentityProperty + MultiplicativeIdentityProperty + PartialEq,
    D: Represent<S>,
    R: FailingStrategy,
{
    fn is_zero(self) -> Self {
        self.expecting(is_zero())
    }

    fn is_one(self) -> Self {
        self.expecting(is_one())
    }
}

impl<S, D> Expectation<S, D> for IsZero
where
    S: AdditiveIdentityProperty + PartialEq,
    D: Represent<S>,
{
    fn test(&mut self, subject: &S) -> bool {
        *subject == <S as AdditiveIdentityProperty>::additive_identity()
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let not = if inverted { "not " } else { "" };
        let marked_actual = mark_unexpected(actual, representation, format);
        let marked_expected = mark_missing(&S::additive_identity(), representation, format);
        format!(
            "expected {expression} to be {not}zero\n   but was: {marked_actual}\n  expected: {not}{marked_expected}"
        )
    }
}

impl Invertible for IsZero {}

impl<S, D> Expectation<S, D> for IsOne
where
    S: MultiplicativeIdentityProperty + PartialEq,
    D: Represent<S>,
{
    fn test(&mut self, subject: &S) -> bool {
        *subject == <S as MultiplicativeIdentityProperty>::multiplicative_identity()
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let not = if inverted { "not " } else { "" };
        let marked_actual = mark_unexpected(actual, representation, format);
        let marked_expected = mark_missing(&S::multiplicative_identity(), representation, format);
        format!(
            "expected {expression} to be {not}one\n   but was: {marked_actual}\n  expected: {not}{marked_expected}"
        )
    }
}

impl Invertible for IsOne {}

impl<S, D, R> AssertInfinity for Spec<'_, S, D, R>
where
    S: InfinityProperty,
    D: Represent<S>,
    R: FailingStrategy,
{
    fn is_infinite(self) -> Self {
        self.expecting(is_infinite())
    }

    fn is_finite(self) -> Self {
        self.expecting(is_finite())
    }
}

impl<S, D> Expectation<S, D> for IsFinite
where
    S: InfinityProperty,
    D: Represent<S>,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.is_finite_property()
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let (not, expected) = if inverted {
            ("not ", "a non-finite number")
        } else {
            ("", "a finite number")
        };
        let marked_actual = mark_unexpected(actual, representation, format);
        let marked_expected = mark_missing(expected, &DisplayRepresentation, format);
        format!(
            "expected {expression} to be {not}finite\n   but was: {marked_actual}\n  expected: {marked_expected}"
        )
    }
}

impl Invertible for IsFinite {}

impl<S, D> Expectation<S, D> for IsInfinite
where
    S: InfinityProperty,
    D: Represent<S>,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.is_infinite_property()
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let (not, expected) = if inverted {
            ("not ", "a non-infinite number")
        } else {
            ("", "an infinite number")
        };
        let marked_actual = mark_unexpected(actual, representation, format);
        let marked_expected = mark_missing(expected, &DisplayRepresentation, format);
        format!(
            "expected {expression} to be {not}infinite\n   but was: {marked_actual}\n  expected: {marked_expected}"
        )
    }
}

impl Invertible for IsInfinite {}

impl<S, D, R> AssertNotANumber for Spec<'_, S, D, R>
where
    S: IsNanProperty,
    D: Represent<S>,
    R: FailingStrategy,
{
    fn is_not_a_number(self) -> Self {
        self.expecting(not(is_a_number()))
    }

    fn is_a_number(self) -> Self {
        self.expecting(is_a_number())
    }
}

impl<S, D> Expectation<S, D> for IsANumber
where
    S: IsNanProperty,
    D: Represent<S>,
{
    fn test(&mut self, subject: &S) -> bool {
        !subject.is_nan_property()
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let (not, expected) = if inverted {
            ("not ", "NaN")
        } else {
            ("", "a number")
        };
        let marked_actual = mark_unexpected(actual, representation, format);
        let marked_expected = mark_missing(expected, &DisplayRepresentation, format);
        format!(
            "expected {expression} to be {not}a number\n   but was: {marked_actual}\n  expected: {marked_expected}"
        )
    }
}

impl Invertible for IsANumber {}

impl<S, D, R> AssertDecimalNumber for Spec<'_, S, D, R>
where
    S: DecimalProperties,
    D: Represent<S> + Represent<i64> + Represent<u64>,
    R: FailingStrategy,
{
    fn has_scale_of(self, expected_scale: i64) -> Self {
        self.expecting(has_scale_of(expected_scale))
    }

    fn has_precision_of(self, expected_precision: u64) -> Self {
        self.expecting(has_precision_of(expected_precision))
    }

    fn is_integer(self) -> Self {
        self.expecting(is_integer())
    }
}

impl<S, D> Expectation<S, D> for HasScaleOf
where
    S: DecimalProperties,
    D: Represent<i64>,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.scale_property() == self.expected_scale
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let not = if inverted { "not " } else { "" };
        let expected_scale = self.expected_scale;
        let marked_actual = mark_unexpected(&actual.scale_property(), representation, format);
        let marked_expected = mark_missing(&expected_scale, representation, format);
        let represented_expected_scale = Represented::from((&expected_scale, representation));
        format!(
            "expected {expression} to {not}have a scale of {represented_expected_scale}\n   but was: {marked_actual}\n  expected: {not}{marked_expected}"
        )
    }
}

impl Invertible for HasScaleOf {}

impl<S, D> Expectation<S, D> for HasPrecisionOf
where
    S: DecimalProperties,
    D: Represent<u64>,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.precision_property() == self.expected_precision
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let not = if inverted { "not " } else { "" };
        let expected_precision = self.expected_precision;
        let marked_actual = mark_unexpected(&actual.precision_property(), representation, format);
        let marked_expected = mark_missing(&expected_precision, representation, format);
        let represented_expected_precision =
            Represented::from((&expected_precision, representation));
        format!(
            "expected {expression} to {not}have a precision of {represented_expected_precision}\n   but was: {marked_actual}\n  expected: {not}{marked_expected}"
        )
    }
}

impl<S, D> Expectation<S, D> for IsInteger
where
    S: DecimalProperties,
    D: Represent<S>,
{
    fn test(&mut self, subject: &S) -> bool {
        subject.is_integer_property()
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &S,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let (not, expected) = if inverted {
            ("not ", "a decimal value with non-zero fraction")
        } else {
            ("", "an integer value")
        };
        let marked_actual = mark_unexpected(actual, representation, format);
        let marked_expected = mark_missing(expected, &DisplayRepresentation, format);
        format!(
            "expected {expression} to be {not}an integer value\n   but was: {marked_actual}\n  expected: {marked_expected}"
        )
    }
}
