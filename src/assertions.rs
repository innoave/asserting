//! Definitions of the assertions that are provided by this crate.
//!
//! Assertions define the methods that are used to assert that the actual test
//! result is as expected. Assertions are defined by traits that are implemented
//! for one or several types. An assertion can be applied to all types that
//! implement this assertion.
//!
//! All assertions provided by this crate are defined in this module. Browse
//! over the traits in this module to get information about all provided
//! assertions.
#![allow(clippy::wrong_self_convention, clippy::return_self_not_must_use)]

use crate::spec::Spec;
use crate::std::fmt::Debug;
use crate::std::ops::RangeBounds;
use crate::std::string::String;

/// Assert whether two values are equal or not.
///
/// These assertions are implemented for all types that implement `PartialEq<E>`
/// with `E` being the type of the expected value.
///
/// # Examples
///
/// ```
/// use asserting::prelude::*;
///
/// let subject = "ea rebum dignissim suscipit".to_string();
/// assert_that!(subject).is_equal_to("ea rebum dignissim suscipit");
///
/// let the_answer = 42;
/// assert_that!(the_answer).is_equal_to(42);
/// ```
pub trait AssertEquality<E> {
    /// Verifies that the subject is equal to some other value.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// let the_answer = 42;
    /// assert_that!(the_answer).is_equal_to(42);
    ///
    /// assert_that!(6 * 7).is_equal_to(42);
    ///
    /// let subject = "volutpat sunt te tincidunt".to_string();
    /// assert_that!(subject).is_equal_to("volutpat sunt te tincidunt");
    /// ```
    #[track_caller]
    fn is_equal_to(self, expected: E) -> Self;

    /// Verifies that subject is not equal to some other value.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// assert_that!(5 * 8).is_not_equal_to(42);
    ///
    /// let subject = "volutpat sunt te tincidunt".to_string();
    /// assert_that!(subject).is_not_equal_to("Hello, World!");
    /// ```
    #[track_caller]
    fn is_not_equal_to(self, expected: E) -> Self;
}

/// Assert approximate equality for floating point numbers.
///
/// # Examples
///
/// ```
/// use asserting::prelude::*;
///
/// assert_that!(10.0_f32 / 3.0).is_close_to_with_margin(3.333, (0.001, 5));
/// assert_that!(10.0_f64 / 3.0).is_close_to_with_margin(3.333, (0.001, 5));
///
/// assert_that!(10.0_f32 / 3.0).is_not_close_to_with_margin(3.333, (0.0001, 5));
/// assert_that!(10.0_f64 / 3.0).is_not_close_to_with_margin(3.333, (0.0001, 5));
/// ```
#[cfg(feature = "float-cmp")]
#[cfg_attr(docsrs, doc(cfg(feature = "float-cmp")))]
pub trait AssertIsCloseToWithinMargin<E, M> {
    /// Verifies that the actual value is approximately equal to the expected
    /// value.
    ///
    /// For comparison, the epsilon and ULPS values of the given margin are
    /// used.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// assert_that!(10.0_f32 / 3.0).is_close_to_with_margin(3.333, (0.001, 5));
    /// assert_that!(10.0_f64 / 3.0).is_close_to_with_margin(3.333, (0.001, 5));
    /// ```
    ///
    /// The following articles describe the challenges with comparing floating
    /// point numbers and the meaning of the epsilon and ULPS values:
    ///
    /// * [https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/)
    /// * [https://floating-point-gui.de/errors/comparison/](https://floating-point-gui.de/errors/comparison/)
    #[track_caller]
    fn is_close_to_with_margin(self, expected: E, margin: impl Into<M>) -> Self;

    /// Verifies that the actual value not approximately equals to the expected
    /// value.
    ///
    /// For comparison, the epsilon and ULPS values of the given margin are
    /// used.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// assert_that!(10.0_f32 / 3.0).is_not_close_to_with_margin(3.333, (0.0001, 5));
    /// assert_that!(10.0_f64 / 3.0).is_not_close_to_with_margin(3.333, (0.0001, 5));
    /// ```
    ///
    /// The following articles describe the challenges with comparing floating
    /// point numbers and the meaning of the epsilon and ULPS values:
    ///
    /// * [https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/)
    /// * [https://floating-point-gui.de/errors/comparison/](https://floating-point-gui.de/errors/comparison/)
    #[track_caller]
    fn is_not_close_to_with_margin(self, expected: E, margin: impl Into<M>) -> Self;
}

/// Assert approximate equality for floating point numbers.
///
/// # Examples
///
/// ```
/// use asserting::prelude::*;
///
/// assert_that!(5.0_f32 / 2.0).is_close_to(2.5);
/// assert_that!(10.0_f64 / 8.0).is_close_to(1.25);
///
/// assert_that!(5.0_f32 / 2.5).is_not_close_to(2.01);
/// assert_that!(10.0_f64 / 8.0).is_not_close_to(1.255);
/// ```
#[cfg(feature = "float-cmp")]
#[cfg_attr(docsrs, doc(cfg(feature = "float-cmp")))]
pub trait AssertIsCloseToWithDefaultMargin<E> {
    /// Verifies that the actual value is approximately equal to the expected
    /// value.
    ///
    /// For the approximation, a default margin with 4 * epsilon and 4 * ULPS is
    /// used.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// assert_that!(5.0_f32 / 2.0).is_close_to(2.5);
    /// assert_that!(10.0_f64 / 8.0).is_close_to(1.25);
    /// ```
    ///
    /// The following articles describe the challenges with comparing floating
    /// point numbers and the meaning of the epsilon and ULPS values:
    ///
    /// * [https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/)
    /// * [https://floating-point-gui.de/errors/comparison/](https://floating-point-gui.de/errors/comparison/)
    #[track_caller]
    fn is_close_to(self, expected: E) -> Self;

    /// Verifies that the actual value is not approximately equal to the expected
    /// value.
    ///
    /// For the approximation, a default margin with 4 * epsilon and 4 * ULPS is
    /// used.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// assert_that!(10.0_f32 / 2.0).is_not_close_to(5.01);
    /// assert_that!(10.0_f64 / 2.0).is_not_close_to(5.01);
    /// ```
    ///
    /// The following articles describe the challenges with comparing floating
    /// point numbers and the meaning of the epsilon and ULPS values:
    ///
    /// * [https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/)
    /// * [https://floating-point-gui.de/errors/comparison/](https://floating-point-gui.de/errors/comparison/)
    #[track_caller]
    fn is_not_close_to(self, expected: E) -> Self;
}

/// Assert whether a value is greater than or less than another value, as well
/// as at most as big or at least as big as another value.
///
/// These assertions are implemented for all types that implement
/// `PartialOrd<E>` with `E` being the type of the expected value the subject
/// is being compared to.
///
/// # Examples
///
/// ```
/// use time::macros::date;
/// use asserting::prelude::*;
///
/// let some_result: u16 = 42;
///
/// assert_that!(some_result).is_at_most(43);
/// assert_that!(some_result).is_at_most(42);
/// assert_that!(some_result).is_at_least(42);
/// assert_that!(some_result).is_at_least(41);
/// assert_that!(some_result).is_greater_than(41);
/// assert_that!(some_result).is_less_than(43);
///
/// let some_letter: char = 'M';
///
/// assert_that!(some_letter).is_before('P');
/// assert_that!(some_letter).is_after('K');
/// assert_that!(some_letter).is_between('A', 'Z');
///
/// let some_date = date!(2025-04-20);
///
/// assert_that!(some_date).is_before(date!(2025-04-21));
/// assert_that!(some_date).is_after(date!(2025-04-19));
/// assert_that!(some_date).is_between(date!(2025-04-19), date!(2025-04-21));
///```
pub trait AssertOrder<E> {
    /// Verifies that the subject is less than some expected value.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// assert_that!(4).is_less_than(5);
    /// assert_that!(-1).is_less_than(1);
    /// assert_that!(-2).is_less_than(-1);
    /// assert_that!(0.5).is_less_than(1.0);
    /// ```
    #[track_caller]
    fn is_less_than(self, expected: E) -> Self;

    /// Verifies that the subject is greater than some expected value.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// assert_that!(5).is_greater_than(4);
    /// assert_that!(1).is_greater_than(-1);
    /// assert_that!(-1).is_greater_than(-2);
    /// assert_that!(0.5).is_greater_than(0.1);
    /// ```
    #[track_caller]
    fn is_greater_than(self, expected: E) -> Self;

    /// Verifies that the subject is less than or equal to some expected value.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// assert_that!(4).is_at_most(5);
    /// assert_that!(5).is_at_most(5);
    /// assert_that!(-2).is_at_most(-1);
    /// assert_that!(-2).is_at_most(-2);
    /// assert_that!(0.9).is_at_most(1.0);
    /// ```
    #[track_caller]
    fn is_at_most(self, expected: E) -> Self;

    /// Verifies that the subject is greater than or equal to some expected
    /// value.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// assert_that!(5).is_at_least(4);
    /// assert_that!(5).is_at_least(5);
    /// assert_that!(-1).is_at_least(-2);
    /// assert_that!(-2).is_at_least(-2);
    /// assert_that!(1.4).is_at_least(1.0);
    /// ```
    #[track_caller]
    fn is_at_least(self, expected: E) -> Self;

    /// Verifies that the subject is before some expected value.
    ///
    /// This is equivalent to asserting a subject to be less than the expected
    /// value.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// assert_that!('M').is_before('N');
    /// assert_that!(4).is_before(5);
    /// assert_that!(0.8).is_before(1.0);
    ///
    /// use time::macros::date;
    ///
    /// assert_that!(date!(2025-05-30)).is_before(date!(2025-06-01));
    /// ```
    #[track_caller]
    fn is_before(self, expected: E) -> Self;

    /// Verifies that the subject is after some expected value.
    ///
    /// This is equivalent to asserting a subject to be greater than the
    /// expected value.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// assert_that!('N').is_after('M');
    /// assert_that!(5).is_after(4);
    /// assert_that!(1.2).is_after(1.0);
    ///
    /// use time::macros::date;
    ///
    /// assert_that!(date!(2025-06-01)).is_after(date!(2025-05-30));
    /// ```
    #[track_caller]
    fn is_after(self, expected: E) -> Self;

    /// Verifies that the subject is between a min value and a max value.
    ///
    /// Min and max values are included. This is equivalent to asserting a
    /// subject to be greater than or equal to the min value and to be less than
    /// or equal to the max value.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// assert_that!('B').is_between('A', 'C');
    /// assert_that!(5).is_between(4, 6);
    /// assert_that!(1.5).is_between(0.9, 1.8);
    ///
    /// use time::macros::date;
    ///
    /// assert_that!(date!(2025-06-01)).is_between(date!(2025-05-30), date!(2025-06-02));
    /// ```
    #[track_caller]
    fn is_between(self, min: E, max: E) -> Self;
}

/// Assert whether a value is within an expected range.
///
/// The expected range can be any of range.
///
/// These assertions are implemented for all types `T` that implement
/// `PartialOrd<E>` with `E` being the type of the expected value. And `E` must
/// implement `PartialOrd<T>`.
///
/// # Examples
///
/// ```
/// use asserting::prelude::*;
///
/// assert_that!(7).is_in_range(6..8);
/// assert_that!(8).is_not_in_range(6..8);
/// assert_that!(1234).is_in_range(6..);
/// assert_that!(5).is_not_in_range(6..);
/// assert_that!(-33).is_in_range(..-1);
///
/// assert_that!('M').is_in_range('A'..='Z');
/// assert_that!('M').is_not_in_range('a'..='z');
/// assert_that!('k').is_in_range('h'..'n');
/// assert_that!('r').is_in_range('H'..);
/// assert_that!('N').is_in_range(..'n');
/// ```
pub trait AssertInRange<E> {
    /// Verifies that the subject is within the expected range.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// assert_that!(7).is_in_range(6..8);
    /// assert_that!(7).is_in_range(7..8);
    /// assert_that!(7).is_in_range(6..=7);
    /// assert_that!(123).is_in_range(10..);
    /// assert_that!(-33).is_in_range(..-10);
    ///
    /// assert_that!(0.5).is_in_range(0.0..1.0);
    ///
    /// assert_that!('K').is_in_range('A'..'M');
    /// assert_that!('g').is_in_range('a'..='z');
    /// assert_that!('!').is_in_range(..'A');
    /// assert_that!('~').is_in_range('z'..);
    /// ```
    #[track_caller]
    fn is_in_range<R>(self, range: R) -> Self
    where
        R: RangeBounds<E> + Debug;

    /// Verifies that the subject is not within the expected range.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// assert_that!(10).is_not_in_range(1..10);
    /// assert_that!(10).is_not_in_range(1..=9);
    ///
    /// assert_that!(1.0).is_not_in_range(0.0..1.0);
    ///
    /// assert_that!('C').is_not_in_range('A'..'C');
    /// assert_that!('D').is_not_in_range('A'..='C');
    /// assert_that!('b').is_not_in_range('A'..='Z');
    /// ```
    #[track_caller]
    fn is_not_in_range<R>(self, range: R) -> Self
    where
        R: RangeBounds<E> + Debug;
}

/// Assert whether a numeric value is negative or positive.
///
/// # Examples
///
/// ```
/// use asserting::prelude::*;
///
/// assert_that!(-42).is_negative();
/// assert_that!(42).is_positive();
/// assert_that!(0).is_not_negative();
/// assert_that!(1).is_not_negative();
/// assert_that!(0).is_not_positive();
/// assert_that!(-1).is_not_positive();
///
/// assert_that!(-0.1).is_negative();
/// assert_that!(0.1).is_positive();
/// assert_that!(0.0).is_not_negative();
/// assert_that!(0.1).is_not_negative();
/// assert_that!(0.0).is_not_positive();
/// assert_that!(-0.1).is_not_positive();
/// ```
pub trait AssertSignum {
    /// Verifies that the subject is a negative number.
    ///
    /// This is equivalent to asserting that a number is less than 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// assert_that!(-5).is_negative();
    /// assert_that!(-0.9).is_negative();
    /// ```
    ///
    /// ```should_panic
    /// use asserting::prelude::*;
    ///
    /// assert_that!(0).is_negative();
    /// ```
    #[track_caller]
    fn is_negative(self) -> Self;

    /// Verifies that the subject is a non-negative number.
    ///
    /// This is equivalent to asserting that a number is greater than or equal
    /// to 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// assert_that!(5).is_not_negative();
    /// assert_that!(1.3).is_not_negative();
    /// assert_that!(0).is_not_negative();
    /// ```
    #[track_caller]
    fn is_not_negative(self) -> Self;

    /// Verifies that the subject is a positive number.
    ///
    /// This is equivalent to asserting that a number is greater than 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// assert_that!(5).is_positive();
    /// assert_that!(2.7).is_positive();
    /// ```
    ///
    /// ```should_panic
    /// use asserting::prelude::*;
    ///
    /// assert_that!(0).is_positive();
    /// ```
    #[track_caller]
    fn is_positive(self) -> Self;

    /// Verifies that the subject is a non-positive number.
    ///
    /// This is equivalent to asserting that a number is less than or equal to
    /// 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// assert_that!(-5).is_not_positive();
    /// assert_that!(-0.9).is_not_positive();
    /// assert_that!(0).is_not_positive();
    /// ```
    #[track_caller]
    fn is_not_positive(self) -> Self;
}

/// Assert the additive and multiplicative identity of a number.
///
/// # Examples
///
/// ```
/// use asserting::prelude::*;
///
/// assert_that!(0).is_zero();
/// assert_that!(1).is_one();
/// assert_that!(0.0).is_zero();
/// assert_that!(1.0).is_one();
/// ```
pub trait AssertNumericIdentity {
    /// Verifies whether the subject is the additive identity (zero).
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// assert_that!(0).is_zero();
    /// assert_that!(0.0).is_zero();
    /// ```
    #[track_caller]
    fn is_zero(self) -> Self;

    /// Verifies whether the subject is the multiplicative identity (one).
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// assert_that!(1).is_one();
    /// assert_that!(1.0).is_one();
    #[track_caller]
    fn is_one(self) -> Self;
}

/// Assert whether a numeric value is infinite or finite.
///
/// # Examples
///
/// ```
/// use asserting::prelude::*;
///
/// assert_that!(0.1).is_finite();
/// assert_that!(0.0).is_finite();
/// assert_that!(f32::INFINITY).is_infinite();
/// assert_that!(f32::NEG_INFINITY).is_infinite();
/// assert_that!(f64::INFINITY).is_infinite();
/// assert_that!(f64::NEG_INFINITY).is_infinite();
/// ```
///
/// Assert negative and positive infinity:
///
/// ```
/// use asserting::prelude::*;
///
/// assert_that!(f64::INFINITY).is_positive().is_infinite();
/// assert_that!(f64::NEG_INFINITY).is_negative().is_infinite();
/// ```
pub trait AssertInfinity {
    /// Verifies that the subject is an infinite number.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// assert_that!(f32::INFINITY).is_infinite();
    /// assert_that!(f64::INFINITY).is_infinite();
    ///
    /// assert_that!(f32::INFINITY).is_positive().is_infinite();
    /// assert_that!(f32::NEG_INFINITY).is_negative().is_infinite();
    ///
    /// assert_that!(f64::INFINITY).is_positive().is_infinite();
    /// assert_that!(f64::NEG_INFINITY).is_negative().is_infinite();
    /// ```
    #[track_caller]
    fn is_infinite(self) -> Self;

    /// Verifies that the subject is a finite number.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// assert_that!(2.96).is_finite();
    /// assert_that!(0.0).is_finite();
    /// assert_that!(-123.45).is_finite();
    /// ```
    #[track_caller]
    fn is_finite(self) -> Self;
}

/// Assert whether a numeric value is not a number.
///
/// # Examples
///
/// ```
/// use asserting::prelude::*;
///
/// assert_that!(0.1).is_a_number();
/// assert_that!(0.0).is_a_number();
/// assert_that!(f32::NAN).is_not_a_number();
/// assert_that!(f64::NAN).is_not_a_number();
/// ```
pub trait AssertNotANumber {
    /// Verifies that the subject is not a number.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// assert_that!(f32::NAN).is_not_a_number();
    /// assert_that!(f64::NAN).is_not_a_number();
    /// ```
    #[track_caller]
    fn is_not_a_number(self) -> Self;

    /// Verifies that the subject is a number.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// assert_that!(0.1).is_a_number();
    /// assert_that!(0.0).is_a_number();
    /// assert_that!(-0.1).is_a_number();
    /// ```
    #[track_caller]
    fn is_a_number(self) -> Self;
}

/// Assert decimal number specific properties.
pub trait AssertDecimalNumber {
    /// Verifies the scale of a decimal number.
    ///
    /// It compares the scale, the total number of digits to the right of the
    /// decimal point (including insignificant leading zeros), to the expected
    /// scale.
    ///
    /// # Examples
    ///
    /// For `bigdecimal::BigDecimal` (requires crate feature `bigdecimal`):
    ///
    /// ```
    /// # #[cfg(not(feature = "bigdecimal"))]
    /// # fn main() {}
    /// # #[cfg(feature = "bigdecimal")]
    /// # fn main() {
    /// use asserting::prelude::*;
    /// use bigdecimal::BigDecimal;
    ///
    /// let subject: BigDecimal = "42.0839".parse().unwrap();
    /// assert_that!(subject).has_scale_of(4);
    ///
    /// let subject: BigDecimal = "1.053700".parse().unwrap();
    /// assert_that!(&subject).has_scale_of(6);
    /// assert_that!(subject.normalized()).has_scale_of(4);
    /// # }
    /// ```
    ///
    /// For `rust_decimal::Decimal` (requires crate feature `rust-decimal`):
    ///
    /// ```
    /// # #[cfg(not(feature = "rust-decimal"))]
    /// # fn main() {}
    /// # #[cfg(feature = "rust-decimal")]
    /// # fn main() {
    /// use asserting::prelude::*;
    /// use rust_decimal::Decimal;
    ///
    /// let subject: Decimal = "42.0839".parse().unwrap();
    /// assert_that!(subject).has_scale_of(4);
    ///
    /// let subject: Decimal = "1.053700".parse().unwrap();
    /// assert_that!(subject).has_scale_of(6);
    /// assert_that!(subject.normalize()).has_scale_of(4);
    /// # }
    /// ```
    #[track_caller]
    fn has_scale_of(self, expected_scale: i64) -> Self;

    /// Verifies the precision of a decimal number.
    ///
    /// It compares the precision, the total number of digits in the non-scaled
    /// integer representation, to the expected precision.
    ///
    /// # Examples
    ///
    /// For `bigdecimal::BigDecimal` (requires crate feature `bigdecimal`):
    ///
    /// ```
    /// # #[cfg(not(feature = "bigdecimal"))]
    /// # fn main() {}
    /// # #[cfg(feature = "bigdecimal")]
    /// # fn main() {
    /// use asserting::prelude::*;
    /// use bigdecimal::BigDecimal;
    ///
    /// let subject: BigDecimal = "42.0839".parse().unwrap();
    ///
    /// assert_that!(subject).has_precision_of(6);
    /// # }
    /// ```
    ///
    /// For `rust_decimal::Decimal` (requires crate feature `rust-decimal`):
    ///
    /// ```
    /// # #[cfg(not(feature = "rust-decimal"))]
    /// # fn main() {}
    /// # #[cfg(feature = "rust-decimal")]
    /// # fn main() {
    /// use asserting::prelude::*;
    /// use rust_decimal::Decimal;
    ///
    /// let subject: Decimal = "42.083916".parse().unwrap();
    /// assert_that!(subject).has_precision_of(29);
    ///
    /// let subject: Decimal = "1.05".parse().unwrap();
    /// assert_that!(subject).has_precision_of(29);
    /// # }
    /// ```
    ///
    /// Note: `rust_decimal::Decimal` is fixed precision decimal number. The
    /// actual precision is always 29.
    #[track_caller]
    fn has_precision_of(self, expected_precision: u64) -> Self;

    /// Verifies that a decimal number has zero fractional digits (is equivalent
    /// to an integer).
    ///
    /// # Examples
    ///
    /// For `bigdecimal::BigDecimal` (requires crate feature `bigdecimal`):
    ///
    /// ```
    /// # #[cfg(not(feature = "bigdecimal"))]
    /// # fn main() {}
    /// # #[cfg(feature = "bigdecimal")]
    /// # fn main() {
    /// use asserting::prelude::*;
    /// use bigdecimal::BigDecimal;
    ///
    /// let subject: BigDecimal = "14_752.0".parse().unwrap();
    ///
    /// assert_that!(subject).is_integer();
    /// # }
    /// ```
    ///
    /// For `rust_decimal::Decimal` (requires crate feature `rust-decimal`):
    ///
    /// ```
    /// # #[cfg(not(feature = "rust-decimal"))]
    /// # fn main() {}
    /// # #[cfg(feature = "rust-decimal")]
    /// # fn main() {
    /// use asserting::prelude::*;
    /// use rust_decimal::Decimal;
    ///
    /// let subject: Decimal = "14_752.0".parse().unwrap();
    ///
    /// assert_that!(subject).is_integer();
    /// # }
    /// ```
    #[track_caller]
    fn is_integer(self) -> Self;
}

/// Assert whether some value or expression is true or false.
///
/// # Examples
///
/// ```
/// use asserting::prelude::*;
///
/// let subject = 42 > 41;
/// assert_that!(subject).is_true();
///
/// assert_that!(12 == 12).is_true();
///
/// assert_that!(42 < 42).is_false();
/// ```
pub trait AssertBoolean {
    /// Verifies that the subject is true.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// let subject = 42 > 41;
    /// assert_that!(subject).is_true();
    ///
    /// assert_that!(12 == 12).is_true();
    ///
    /// assert_that!(41 < 42).is_true();
    /// ```
    #[track_caller]
    fn is_true(self) -> Self;

    /// Verifies that the subject is false.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// let subject = 42 > 43;
    /// assert_that!(subject).is_false();
    ///
    /// assert_that!(12 == 13).is_false();
    ///
    /// assert_that!(42 < 42).is_false();
    /// ```
    #[track_caller]
    fn is_false(self) -> Self;
}

/// Assert properties or classifications of a character.
///
/// # Examples
///
/// ```
/// use asserting::prelude::*;
///
/// assert_that!('b').is_lowercase();
/// assert_that!('G').is_uppercase();
/// assert_that!('H').is_alphabetic();
/// assert_that!('z').is_alphanumeric();
/// assert_that!('9').is_alphanumeric();
/// assert_that!('7').is_digit(8);
/// assert_that!('9').is_digit(10);
/// assert_that!('F').is_digit(16);
/// assert_that!('\t').is_whitespace();
/// assert_that!('\u{1b}').is_control_char();
/// assert_that!('@').is_ascii();
/// ```
///
/// To assert ASCII uppercase or lowercase, we combine the assertions `is_ascii`
/// and `is_uppercase` or `is_lowercase` in chained assertions.
///
/// ```
/// use asserting::prelude::*;
///
/// assert_that('c').is_ascii().is_lowercase();
/// assert_that('M').is_ascii().is_uppercase();
/// assert_that('a').is_ascii().is_alphabetic();
/// assert_that('\t').is_ascii().is_control_char();
/// ```
pub trait AssertChar {
    /// Verify that a character is lowercase.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// assert_that!('b').is_lowercase();
    /// assert_that!('n').is_lowercase();
    /// ```
    #[track_caller]
    fn is_lowercase(self) -> Self;

    /// Verify that a character is uppercase.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// assert_that!('G').is_uppercase();
    /// assert_that!('N').is_uppercase();
    /// ```
    #[track_caller]
    fn is_uppercase(self) -> Self;

    /// Verify that a character is an ASCII character.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// assert_that!('a').is_ascii();
    /// assert_that!('5').is_ascii();
    /// assert_that!('#').is_ascii();
    /// assert_that!('@').is_ascii();
    /// ```
    #[track_caller]
    fn is_ascii(self) -> Self;

    /// Verify that a character is an alphabetic character.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// assert_that!('b').is_alphabetic();
    /// assert_that!('Z').is_alphabetic();
    /// ```
    #[track_caller]
    fn is_alphabetic(self) -> Self;

    /// Verify that a character is an alphabetic character or a digit.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// assert_that!('z').is_alphanumeric();
    /// assert_that!('9').is_alphanumeric();
    /// ```
    #[track_caller]
    fn is_alphanumeric(self) -> Self;

    /// Verify that a character is a control character.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// assert_that!('\t').is_control_char();
    /// assert_that!('\u{1b}').is_control_char();
    /// ```
    #[track_caller]
    fn is_control_char(self) -> Self;

    /// Verify that a character is a digit within the given radix.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// assert_that!('7').is_digit(8);
    /// assert_that!('9').is_digit(10);
    /// assert_that!('F').is_digit(16);
    /// ```
    #[track_caller]
    fn is_digit(self, radix: u32) -> Self;

    /// Verify that a character is whitespace.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// assert_that!(' ').is_whitespace();
    /// assert_that!('\t').is_whitespace();
    /// ```
    #[track_caller]
    fn is_whitespace(self) -> Self;
}

/// Assert whether a string, collection or iterator is empty or not.
///
/// These assertions are implemented for all types `T` that implement the
/// trait [`IsEmptyProperty`](crate::properties::IsEmptyProperty). This
/// property trait is implemented for string like types and collection like
/// types of the `std` lib. For example:
///
/// * `String`, `&str`, `OsString`, `CString`, etc.
/// * `Vec`, array, slice, `VecDeque`, `LinkedList`, etc.
/// * `HashMap`, `HashSet`, `BTreeSet`, etc.
///
/// # Examples
///
/// ```
/// use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
/// use asserting::prelude::*;
///
/// let some_string = String::new();
/// assert_that!(some_string).is_empty();
///
/// let some_str = "ad praesent aliqua qui";
/// assert_that!(some_str).is_not_empty();
///
/// let some_vec: Vec<String> = vec![];
/// assert_that!(some_vec).is_empty();
///
/// let some_array = [12, 24, 36, 48];
/// assert_that!(some_array).is_not_empty();
///
/// let some_slice: &[_] = &['a', 'b', 'c'][..];
/// assert_that!(some_slice).is_not_empty();
///
/// let some_btree_set = BTreeSet::<i64>::new();
/// assert_that!(&some_btree_set).is_empty();
///
/// let some_dequeue = VecDeque::<String>::new();
/// assert_that!(some_dequeue).is_empty();
/// ```
///
/// with crate feature `std` enabled:
///
/// ```
/// # #[cfg(not(feature = "std"))]
/// # fn main() {
/// # }
/// # #[cfg(feature = "std")]
/// # fn main() {
/// use std::collections::{HashMap, HashSet};
/// use asserting::prelude::*;
///
/// let some_set: HashSet<_> = [1, 3, 5, 7, 11, 13, 17, 19].into_iter().collect();
/// assert_that!(&some_set).is_not_empty();
///
/// let some_map: HashMap<String, usize> = HashMap::new();
/// assert_that!(some_map).is_empty();
/// # }
/// ```
pub trait AssertEmptiness {
    /// Verifies that the subject is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
    /// use asserting::prelude::*;
    ///
    /// let some_string = String::new();
    /// assert_that!(some_string).is_empty();
    ///
    /// let some_vec: Vec<String> = vec![];
    /// assert_that!(some_vec).is_empty();
    ///
    /// let some_array: [char; 0] = [];
    /// assert_that!(some_array).is_empty();
    ///
    /// let some_slice: &[char] = &[][..];
    /// assert_that!(some_slice).is_empty();
    ///
    /// let some_btree_set = BTreeSet::<i64>::new();
    /// assert_that!(&some_btree_set).is_empty();
    ///
    /// let some_dequeue = VecDeque::<String>::new();
    /// assert_that!(some_dequeue).is_empty();
    /// ```
    #[track_caller]
    fn is_empty(self) -> Self;

    /// Verifies that the subject is not empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
    /// use asserting::prelude::*;
    ///
    /// let some_str = "ad praesent aliqua qui";
    /// assert_that!(some_str).is_not_empty();
    ///
    /// let some_vec = vec![1, 2, 3];
    /// assert_that!(some_vec).is_not_empty();
    ///
    /// let some_array = [12, 24, 36, 48];
    /// assert_that!(some_array).is_not_empty();
    ///
    /// let some_slice: &[_] = &['a', 'b', 'c'][..];
    /// assert_that!(some_slice).is_not_empty();
    /// ```
    #[track_caller]
    fn is_not_empty(self) -> Self;
}

/// Assert the length of a subject.
///
/// These assertions are implemented for all types `T` that implement the
/// trait [`LengthProperty`](crate::properties::LengthProperty). This
/// property trait is implemented for string like types and collection like
/// types of the `std` lib. For example:
///
/// * `String`, `&str`, `OsString`, `OsStr`
/// * `Vec`, array, slice, `VecDeque`, `LinkedList`, etc.
/// * `HashMap`, `HashSet`, `BTreeSet`, etc.
///
/// # Examples
///
/// ```
/// use std::collections::{BTreeSet, VecDeque};
/// use asserting::prelude::*;
///
/// let some_str = "takimata te iriure nonummy";
/// assert_that!(some_str).has_length(26);
/// assert_that!(some_str).has_length_in_range(12..32);
/// assert_that!(some_str).has_length_in_range(12..=32);
/// assert_that!(some_str).has_length_in_range(12..);
/// assert_that!(some_str).has_length_in_range(..32);
/// assert_that!(some_str).has_length_in_range(..=32);
/// assert_that!(some_str).has_length_less_than(27);
/// assert_that!(some_str).has_length_greater_than(25);
/// assert_that!(some_str).has_at_most_length(26);
/// assert_that!(some_str).has_at_most_length(30);
/// assert_that!(some_str).has_at_least_length(26);
/// assert_that!(some_str).has_at_least_length(20);
///
/// let some_vec = vec!['m', 'Q', 'k', 'b'];
/// assert_that!(&some_vec).has_length(4);
/// assert_that!(&some_vec).has_length_in_range(2..6);
/// assert_that!(&some_vec).has_length_in_range(2..=6);
/// assert_that!(&some_vec).has_length_in_range(2..);
/// assert_that!(&some_vec).has_length_in_range(..6);
/// assert_that!(&some_vec).has_length_in_range(..=6);
/// assert_that!(&some_vec).has_length_less_than(5);
/// assert_that!(&some_vec).has_length_greater_than(3);
/// assert_that!(&some_vec).has_at_most_length(4);
/// assert_that!(&some_vec).has_at_most_length(10);
/// assert_that!(&some_vec).has_at_least_length(4);
/// assert_that!(&some_vec).has_at_least_length(1);
///
/// let some_btree_set = BTreeSet::from_iter([1, 3, 5, 7, 11, 13, 17, 19]);
/// assert_that!(some_btree_set).has_length(8);
///
/// let some_dequeue = VecDeque::from_iter(["one", "two", "three"]);
/// assert_that!(&some_dequeue).has_length(3);
/// ```
///
/// with crate feature `std` enabled:
///
/// ```
/// # #[cfg(not(feature = "std"))]
/// # fn main() {
/// # }
/// # #[cfg(feature = "std")]
/// # fn main() {
/// use std::collections::{HashMap, HashSet};
/// use asserting::prelude::*;
///
/// let some_set: HashSet<u8> = [1, 3, 5, 7, 11, 13, 17, 19].into_iter().collect();
/// assert_that!(some_set).has_length(8);
///
/// let some_map: HashMap<char, usize> = [('A', 25), ('B', 2), ('C', 12), ('D', 18)].into_iter().collect();
/// assert_that!(&some_map).has_length(4);
/// # }
/// ```
pub trait AssertHasLength<E> {
    /// Verifies that the subject has the expected length.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::{BTreeSet, VecDeque};
    /// use asserting::prelude::*;
    ///
    /// let some_str = "takimata te iriure nonummy";
    /// assert_that!(some_str).has_length(26);
    ///
    /// let some_vec = vec!['m', 'Q', 'k', 'b'];
    /// assert_that!(&some_vec).has_length(4);
    ///
    /// let some_btree_set = BTreeSet::from_iter([1, 3, 5, 7, 11, 13, 17, 19]);
    /// assert_that!(some_btree_set).has_length(8);
    ///
    /// let some_dequeue = VecDeque::from_iter(["one", "two", "three"]);
    /// assert_that!(&some_dequeue).has_length(3);
    /// ```
    ///
    /// with crate feature `std` enabled:
    ///
    /// ```
    /// # #[cfg(not(feature = "std"))]
    /// # fn main() {
    /// # }
    /// # #[cfg(feature = "std")]
    /// # fn main() {
    /// use std::collections::{HashMap, HashSet};
    /// use asserting::prelude::*;
    ///
    /// let some_set: HashSet<u8> = [1, 3, 5, 7, 11, 13, 17, 19].into_iter().collect();
    /// assert_that!(some_set).has_length(8);
    ///
    /// let some_map: HashMap<char, usize> = [('A', 25), ('B', 2), ('C', 12), ('D', 18)].into_iter().collect();
    /// assert_that!(&some_map).has_length(4);
    /// # }
    /// ```
    #[track_caller]
    fn has_length(self, expected_length: E) -> Self;

    /// Verifies that the subject has a length in the expected range.
    ///
    /// The expected range can be any type of range.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::{BTreeSet, VecDeque};
    /// use asserting::prelude::*;
    ///
    /// let some_str = "takimata te iriure nonummy";
    /// assert_that!(some_str).has_length_in_range(12..32);
    /// assert_that!(some_str).has_length_in_range(12..=32);
    /// assert_that!(some_str).has_length_in_range(12..);
    /// assert_that!(some_str).has_length_in_range(..32);
    /// assert_that!(some_str).has_length_in_range(..=32);
    ///
    /// let some_vec = vec!['m', 'Q', 'k', 'b'];
    /// assert_that!(&some_vec).has_length_in_range(2..6);
    /// assert_that!(&some_vec).has_length_in_range(2..=6);
    /// assert_that!(&some_vec).has_length_in_range(2..);
    /// assert_that!(&some_vec).has_length_in_range(..6);
    /// assert_that!(&some_vec).has_length_in_range(..=6);
    ///
    /// let some_btree_set = BTreeSet::from_iter([1, 3, 5, 7, 11, 13, 17, 19]);
    /// assert_that!(&some_btree_set).has_length_in_range(6..10);
    /// assert_that!(&some_btree_set).has_length_in_range(6..=10);
    /// assert_that!(&some_btree_set).has_length_in_range(8..);
    /// assert_that!(&some_btree_set).has_length_in_range(..9);
    /// assert_that!(&some_btree_set).has_length_in_range(..=8);
    ///
    /// let some_dequeue = VecDeque::from_iter(["one", "two", "three"]);
    /// assert_that!(&some_dequeue).has_length_in_range(2..5);
    /// assert_that!(&some_dequeue).has_length_in_range(2..=5);
    /// assert_that!(&some_dequeue).has_length_in_range(3..);
    /// assert_that!(&some_dequeue).has_length_in_range(..4);
    /// assert_that!(&some_dequeue).has_length_in_range(..=3);
    /// ```
    ///
    /// with crate feature `std` enabled:
    ///
    /// ```
    /// # #[cfg(not(feature = "std"))]
    /// # fn main() {
    /// # }
    /// # #[cfg(feature = "std")]
    /// # fn main() {
    /// use std::collections::{HashMap, HashSet};
    /// use asserting::prelude::*;
    ///
    /// let some_set: HashSet<u8> = [1, 3, 5, 7, 11, 13, 17, 19].into_iter().collect();
    /// assert_that!(&some_set).has_length_in_range(4..12);
    /// assert_that!(&some_set).has_length_in_range(4..=12);
    /// assert_that!(&some_set).has_length_in_range(8..);
    /// assert_that!(&some_set).has_length_in_range(..9);
    /// assert_that!(&some_set).has_length_in_range(..=8);
    ///
    /// let some_map: HashMap<char, usize> = [('A', 25), ('B', 2), ('C', 12), ('D', 18)].into_iter().collect();
    /// assert_that!(&some_map).has_length_in_range(2..8);
    /// assert_that!(&some_map).has_length_in_range(2..=8);
    /// assert_that!(&some_map).has_length_in_range(4..);
    /// assert_that!(&some_map).has_length_in_range(..5);
    /// assert_that!(&some_map).has_length_in_range(..=4);
    /// # }
    /// ```
    #[track_caller]
    fn has_length_in_range<U>(self, expected_range: U) -> Self
    where
        U: RangeBounds<usize> + Debug;

    /// Verifies that the subject has a length that is less than the expected
    /// length.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::{BTreeSet, VecDeque};
    /// use asserting::prelude::*;
    ///
    /// let some_str = "takimata te iriure nonummy";
    /// assert_that!(some_str).has_length_less_than(40);
    /// assert_that!(some_str).has_length_less_than(27);
    ///
    /// let some_vec = vec!['m', 'Q', 'k', 'b'];
    /// assert_that!(&some_vec).has_length_less_than(10);
    /// assert_that!(&some_vec).has_length_less_than(5);
    ///
    /// let some_btree_set = BTreeSet::from_iter([1, 3, 5, 7, 11, 13, 17, 19]);
    /// assert_that!(&some_btree_set).has_length_less_than(20);
    /// assert_that!(&some_btree_set).has_length_less_than(9);
    ///
    /// let some_dequeue = VecDeque::from_iter(["one", "two", "three"]);
    /// assert_that!(&some_dequeue).has_length_less_than(10);
    /// assert_that!(&some_dequeue).has_length_less_than(4);
    /// ```
    ///
    /// with crate feature `std` enabled:
    ///
    /// ```
    /// # #[cfg(not(feature = "std"))]
    /// # fn main() {
    /// # }
    /// # #[cfg(feature = "std")]
    /// # fn main() {
    /// use std::collections::{HashMap, HashSet};
    /// use asserting::prelude::*;
    ///
    /// let some_set: HashSet<u8> = [1, 3, 5, 7, 11, 13, 17, 19].into_iter().collect();
    /// assert_that!(&some_set).has_length_less_than(15);
    /// assert_that!(&some_set).has_length_less_than(9);
    ///
    /// let some_map: HashMap<char, usize> = [('A', 25), ('B', 2), ('C', 12), ('D', 18)].into_iter().collect();
    /// assert_that!(&some_map).has_length_less_than(10);
    /// assert_that!(&some_map).has_length_less_than(5);
    /// # }
    /// ```
    #[track_caller]
    fn has_length_less_than(self, expected_length: E) -> Self;

    /// Verifies that the subject has a length that is greater than the expected
    /// length.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::{BTreeSet, VecDeque};
    /// use asserting::prelude::*;
    ///
    /// let some_str = "takimata te iriure nonummy";
    /// assert_that!(some_str).has_length_greater_than(20);
    /// assert_that!(some_str).has_length_greater_than(25);
    ///
    /// let some_vec = vec!['m', 'Q', 'k', 'b'];
    /// assert_that!(&some_vec).has_length_greater_than(1);
    /// assert_that!(&some_vec).has_length_greater_than(3);
    ///
    /// let some_btree_set = BTreeSet::from_iter([1, 3, 5, 7, 11, 13, 17, 19]);
    /// assert_that!(&some_btree_set).has_length_greater_than(4);
    /// assert_that!(&some_btree_set).has_length_greater_than(7);
    ///
    /// let some_dequeue = VecDeque::from_iter(["one", "two", "three"]);
    /// assert_that!(&some_dequeue).has_length_greater_than(1);
    /// assert_that!(&some_dequeue).has_length_greater_than(2);
    /// ```
    ///
    /// with crate feature `std` enabled:
    ///
    /// ```
    /// # #[cfg(not(feature = "std"))]
    /// # fn main() {
    /// # }
    /// # #[cfg(feature = "std")]
    /// # fn main() {
    /// use std::collections::{HashMap, HashSet};
    /// use asserting::prelude::*;
    ///
    /// let some_set: HashSet<u8> = [1, 3, 5, 7, 11, 13, 17, 19].into_iter().collect();
    /// assert_that!(&some_set).has_length_greater_than(4);
    /// assert_that!(&some_set).has_length_greater_than(7);
    ///
    /// let some_map: HashMap<char, usize> = [('A', 25), ('B', 2), ('C', 12), ('D', 18)].into_iter().collect();
    /// assert_that!(&some_map).has_length_greater_than(2);
    /// assert_that!(&some_map).has_length_greater_than(3);
    /// # }
    /// ```
    #[track_caller]
    fn has_length_greater_than(self, expected_length: E) -> Self;

    /// Verifies that the subject has a length that is at most the expected
    /// length.
    ///
    /// In other words, the length shall be less than or equal to the expected
    /// length.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::{BTreeSet, VecDeque};
    /// use asserting::prelude::*;
    ///
    /// let some_str = "takimata te iriure nonummy";
    /// assert_that!(some_str).has_at_most_length(30);
    /// assert_that!(some_str).has_at_most_length(26);
    ///
    /// let some_vec = vec!['m', 'Q', 'k', 'b'];
    /// assert_that!(&some_vec).has_at_most_length(10);
    /// assert_that!(&some_vec).has_at_most_length(4);
    ///
    /// let some_btree_set = BTreeSet::from_iter([1, 3, 5, 7, 11, 13, 17, 19]);
    /// assert_that!(&some_btree_set).has_at_most_length(12);
    /// assert_that!(&some_btree_set).has_at_most_length(8);
    ///
    /// let some_dequeue = VecDeque::from_iter(["one", "two", "three"]);
    /// assert_that!(&some_dequeue).has_at_most_length(10);
    /// assert_that!(&some_dequeue).has_at_most_length(3);
    /// ```
    ///
    /// with crate feature `std` enabled:
    ///
    /// ```
    /// # #[cfg(not(feature = "std"))]
    /// # fn main() {
    /// # }
    /// # #[cfg(feature = "std")]
    /// # fn main() {
    /// use std::collections::{HashMap, HashSet};
    /// use asserting::prelude::*;
    ///
    /// let some_set: HashSet<u8> = [1, 3, 5, 7, 11, 13, 17, 19].into_iter().collect();
    /// assert_that!(&some_set).has_at_most_length(15);
    /// assert_that!(&some_set).has_at_most_length(8);
    ///
    /// let some_map: HashMap<char, usize> = [('A', 25), ('B', 2), ('C', 12), ('D', 18)].into_iter().collect();
    /// assert_that!(&some_map).has_at_most_length(10);
    /// assert_that!(&some_map).has_at_most_length(4);
    /// # }
    /// ```
    #[track_caller]
    fn has_at_most_length(self, expected_length: E) -> Self;

    /// Verifies that the subject has a length that is at least the expected
    /// length.
    ///
    /// In other words, the length shall be greater than or equal to the
    /// expected length.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::{BTreeSet, VecDeque};
    /// use asserting::prelude::*;
    ///
    /// let some_str = "takimata te iriure nonummy";
    /// assert_that!(some_str).has_at_least_length(20);
    /// assert_that!(some_str).has_at_least_length(26);
    ///
    /// let some_vec = vec!['m', 'Q', 'k', 'b'];
    /// assert_that!(&some_vec).has_at_least_length(1);
    /// assert_that!(&some_vec).has_at_least_length(4);
    ///
    /// let some_btree_set = BTreeSet::from_iter([1, 3, 5, 7, 11, 13, 17, 19]);
    /// assert_that!(&some_btree_set).has_at_least_length(4);
    /// assert_that!(&some_btree_set).has_at_least_length(8);
    ///
    /// let some_dequeue = VecDeque::from_iter(["one", "two", "three"]);
    /// assert_that!(&some_dequeue).has_at_least_length(1);
    /// assert_that!(&some_dequeue).has_at_least_length(3);
    /// ```
    ///
    /// with crate feature `std` enabled:
    ///
    /// ```
    /// # #[cfg(not(feature = "std"))]
    /// # fn main() {
    /// # }
    /// # #[cfg(feature = "std")]
    /// # fn main() {
    /// use std::collections::{HashMap, HashSet};
    /// use asserting::prelude::*;
    ///
    /// let some_set: HashSet<u8> = [1, 3, 5, 7, 11, 13, 17, 19].into_iter().collect();
    /// assert_that!(&some_set).has_at_least_length(4);
    /// assert_that!(&some_set).has_at_least_length(8);
    ///
    /// let some_map: HashMap<char, usize> = [('A', 25), ('B', 2), ('C', 12), ('D', 18)].into_iter().collect();
    /// assert_that!(&some_map).has_at_least_length(2);
    /// assert_that!(&some_map).has_at_least_length(4);
    /// # }
    /// ```
    #[track_caller]
    fn has_at_least_length(self, expected_length: E) -> Self;
}

/// Assert the number of characters contained in a string or similar container.
///
/// These assertions are implemented for all types `T` that implement the trait
/// [`CharCountProperty`](crate::properties::CharCountProperty). This property
/// is implemented for `String` and `&str`.
///
/// # Examples
///
/// ```
/// use asserting::prelude::*;
///
/// let subject = "imper \u{0180} diet al \u{02AA} \u{01AF} zzril";
/// assert_that!(subject).has_length(28);
/// assert_that!(subject).has_char_count(25);
///
/// let subject = "imper diet al zzril";
/// assert_that!(subject).has_length(19);
/// assert_that!(subject).has_char_count(19);
///
/// let subject = "imper \u{0180} diet al \u{02AA} \u{01AF} zzril";
/// assert_that!(subject).has_char_count_in_range(12..=36);
/// assert_that!(subject).has_char_count_less_than(26);
/// assert_that!(subject).has_char_count_greater_than(24);
/// assert_that!(subject).has_at_most_char_count(30);
/// assert_that!(subject).has_at_most_char_count(25);
/// assert_that!(subject).has_at_least_char_count(20);
/// assert_that!(subject).has_at_least_char_count(25);
/// ```
pub trait AssertHasCharCount<E> {
    /// Verifies that the subject contains the expected number of characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// let subject = "imper \u{0180} diet al \u{02AA} \u{01AF} zzril";
    /// assert_that!(subject).has_length(28);
    /// assert_that!(subject).has_char_count(25);
    ///
    /// let subject = "imper diet al zzril";
    /// assert_that!(subject).has_length(19);
    /// assert_that!(subject).has_char_count(19);
    /// ```
    #[track_caller]
    fn has_char_count(self, expected: E) -> Self;

    /// Verifies that the subject contains a number of characters that is in the
    /// expected range.
    ///
    /// The expected range must be a closed range with both ends inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// let subject = "imper \u{0180} diet al \u{02AA} \u{01AF} zzril";
    /// assert_that!(subject).has_char_count_in_range(12..36);
    /// assert_that!(subject).has_char_count_in_range(12..=36);
    /// ```
    #[track_caller]
    fn has_char_count_in_range<U>(self, range: U) -> Self
    where
        U: RangeBounds<usize> + Debug;

    /// Verifies that the subject contains less than the expected number of
    /// characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// let subject = "imper \u{0180} diet al \u{02AA} \u{01AF} zzril";
    /// assert_that!(subject).has_char_count_less_than(26);
    /// ```
    #[track_caller]
    fn has_char_count_less_than(self, expected: E) -> Self;

    /// Verifies that the subject contains more than the expected number of
    /// characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// let subject = "imper \u{0180} diet al \u{02AA} \u{01AF} zzril";
    /// assert_that!(subject).has_char_count_greater_than(24);
    /// ```
    #[track_caller]
    fn has_char_count_greater_than(self, expected: E) -> Self;

    /// Verifies that the subject contains at least the expected number of
    /// characters.
    ///
    /// In other words, the number of characters shall be less than or equal
    /// to the expected number.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// let subject = "imper \u{0180} diet al \u{02AA} \u{01AF} zzril";
    /// assert_that!(subject).has_at_most_char_count(30);
    /// assert_that!(subject).has_at_most_char_count(25);
    /// ```
    #[track_caller]
    fn has_at_most_char_count(self, expected: E) -> Self;

    /// Verifies that the subject contains at least the expected number of
    /// characters.
    ///
    /// In other words, the number of characters shall be greater than or equal
    /// to the expected number.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// let subject = "imper \u{0180} diet al \u{02AA} \u{01AF} zzril";
    /// assert_that!(subject).has_at_least_char_count(20);
    /// assert_that!(subject).has_at_least_char_count(25);
    /// ```
    #[track_caller]
    fn has_at_least_char_count(self, expected: E) -> Self;
}

/// Assert whether a subject of the `Option` type holds some value or has none.
///
/// # Examples
///
/// ```
/// use asserting::prelude::*;
///
/// let subject = Some("nisl possim nobis non".to_string());
/// assert_that!(subject).is_some();
///
/// #[derive(Debug)]
/// struct MyType;
///
/// let subject: Option<MyType> = None;
/// assert_that!(subject).is_none();
/// ```
pub trait AssertOption {
    /// Verifies that the subject has some value.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// let subject = Some("nisl possim nobis non".to_string());
    /// assert_that!(subject).is_some();
    ///
    /// #[derive(Debug)]
    /// struct MyType;
    ///
    /// let subject = Some(MyType);
    /// assert_that!(subject).is_some();
    /// ```
    #[track_caller]
    fn is_some(self) -> Self;

    /// Verifies that the subject has no value.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// let subject: Option<String> = None;
    /// assert_that!(subject).is_none();
    ///
    /// #[derive(Debug)]
    /// struct MyType;
    ///
    /// let subject: Option<MyType> = None;
    /// assert_that!(subject).is_none();
    /// ```
    #[track_caller]
    fn is_none(self) -> Self;
}

/// Assert the value of an option by mapping the subject.
///
/// If the option is none, the assertion fails.
///
/// # Examples
///
/// ```
/// use asserting::prelude::*;
///
/// let subject: Option<Vec<usize>> = Some(vec![1, 2, 3]);
/// assert_that!(subject).some().contains_exactly([1, 2, 3]);
///
/// let subject: Option<&str> = Some("ullamco cupiditat diam hendrerit");
/// assert_that!(subject).some().is_not_empty();
/// ```
pub trait AssertOptionValue<'a, T, R> {
    /// Maps the subject to the option's value if it has some. Otherwise, this
    /// assertion fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// let subject: Option<Vec<usize>> = Some(vec![1, 2, 3]);
    /// assert_that!(subject).some().contains_exactly([1, 2, 3]);
    ///
    /// let subject: Option<&str> = Some("ullamco cupiditat diam hendrerit");
    /// assert_that!(subject).some().is_not_empty();
    /// ```
    #[track_caller]
    fn some(self) -> Spec<'a, T, R>;
}

/// Assert the value of a borrowed option by mapping the subject.
///
/// If the option is none, the assertion fails.
///
/// # Examples
///
/// ```
/// use asserting::prelude::*;
///
/// let subject: Option<Vec<usize>> = Some(vec![1, 2, 3]);
/// assert_that!(&subject).some().contains_exactly(&[1, 2, 3]);
///
/// let subject: Option<&str> = Some("ullamco cupiditat diam hendrerit");
/// assert_that!(&subject).some().is_not_empty();
/// ```
pub trait AssertBorrowedOptionValue<'a, T, R> {
    /// Maps the subject to the option's value if it has some. Otherwise, this
    /// assertion fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// let subject: Option<Vec<usize>> = Some(vec![1, 2, 3]);
    /// assert_that!(&subject).some().contains_exactly(&[1, 2, 3]);
    ///
    /// let subject: Option<&str> = Some("ullamco cupiditat diam hendrerit");
    /// assert_that!(&subject).some().is_not_empty();
    /// ```
    #[track_caller]
    fn some(self) -> Spec<'a, &'a T, R>;
}

/// Assert whether a subject of the `Result` type holds some value or an error.
///
/// # Examples
///
/// ```
/// use asserting::prelude::*;
///
/// let subject: Result<f64, String> = Ok(-3.14);
/// assert_that!(subject).is_ok();
///
/// let subject: Result<(), String> = Err("consequat sanctus ea exercitation".to_string());
/// assert_that!(subject).is_err();
/// ```
pub trait AssertResult {
    /// Verifies that the subject has an ok value.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// let subject: Result<f64, String> = Ok(-3.14);
    /// assert_that!(subject).is_ok();
    /// ```
    #[track_caller]
    fn is_ok(self) -> Self;

    /// Verifies that the subject has an err value.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// let subject: Result<(), String> = Err("consequat sanctus ea exercitation".to_string());
    /// assert_that!(subject).is_err();
    /// ```
    #[track_caller]
    fn is_err(self) -> Self;
}

/// Assert the ok-value or error of a result by mapping the subject.
///
/// # Examples
///
/// ```
/// use asserting::prelude::*;
///
/// let subject: Result<Vec<usize>, String> = Ok(vec![1, 2, 3]);
/// assert_that!(subject).ok().is_not_empty();
///
/// let subject: Result<u64, String> = Err("te anim adipisici mollit".to_string());
/// assert_that!(subject).err().is_equal_to("te anim adipisici mollit");
/// ```
pub trait AssertResultValue<'a, T, E, R> {
    /// Maps the subject to the result's ok value.
    ///
    /// If the result is an error, this method panics.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// let subject: Result<Vec<usize>, String> = Ok(vec![1, 2, 3]);
    /// assert_that!(subject).ok().is_not_empty();
    /// ```
    #[track_caller]
    fn ok(self) -> Spec<'a, T, R>;

    /// Maps the subject to the result's err value.
    ///
    /// If the result is an ok value, this method panics.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// let subject: Result<u64, String> = Err("te anim adipisici mollit".to_string());
    /// assert_that!(subject).err().is_equal_to("te anim adipisici mollit");
    /// ```
    #[track_caller]
    fn err(self) -> Spec<'a, E, R>;
}

/// Assert the ok-value or error of a borrowed result by mapping the subject.
///
/// # Examples
///
/// ```
/// use asserting::prelude::*;
///
/// let subject: Result<Vec<usize>, String> = Ok(vec![1, 2, 3]);
/// assert_that!(&subject).ok().is_not_empty();
///
/// let subject: Result<u64, String> = Err("te anim adipisici mollit".to_string());
/// assert_that!(&subject).err().is_equal_to("te anim adipisici mollit");
/// ```
pub trait AssertBorrowedResultValue<'a, T, E, R> {
    /// Maps the subject to the result's ok value.
    ///
    /// If the result is an error, this method panics.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// let subject: Result<Vec<usize>, String> = Ok(vec![1, 2, 3]);
    /// assert_that!(&subject).ok().contains_exactly(&[1, 2, 3]);
    /// ```
    #[track_caller]
    fn ok(self) -> Spec<'a, &'a T, R>;

    /// Maps the subject to the result's err value.
    ///
    /// If the result is an ok value, this method panics.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// let subject: Result<u64, String> = Err("te anim adipisici mollit".to_string());
    /// assert_that!(&subject).err().is_equal_to("te anim adipisici mollit");
    /// ```
    #[track_caller]
    fn err(self) -> Spec<'a, &'a E, R>;
}

/// Assert that a subject of some container type holds a value that is equal to
/// the expected one.
///
/// This assertion is implemented for the `Option` type and the `Result` type.
/// For `Option` it compares the value to the expected one if it has some or
/// fails if it holds none. For `Result` it compares the ok value to the
/// expected one if it is an ok or fails if it holds an error.
///
/// The value type of the `Option` or `Result` must implement `PartialEq<E>`
/// where `E` is the type of the expected value.
///
/// To assert the error value of a `Result` use [`AssertHasError::has_error`].
///
/// # Examples
///
/// ```
/// use asserting::prelude::*;
///
/// let subject = Some(-3.14);
/// assert_that!(subject).has_value(-3.14);
///
/// let subject: Result<f64, String> = Ok(6.28);
/// assert_that!(subject).has_value(6.28);
/// ```
pub trait AssertHasValue<E> {
    /// Verifies that the subject holds a value that is equal to the expected
    /// one.
    ///
    /// For `Option` it compares the value in `Some(value)` and for `Result`
    /// it compares the value in `Ok(value)`. If an `Option` is `None` or a
    /// `Result` is `Err(error)` then the assertion fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// let subject = Some(-3.14);
    /// assert_that!(subject).has_value(-3.14);
    ///
    /// let subject: Result<f64, String> = Ok(6.28);
    /// assert_that!(subject).has_value(6.28);
    /// ```
    #[track_caller]
    fn has_value(self, expected: E) -> Self;
}

/// Assert that a subject of some container type holds an error value that is
/// equal to the expected one.
///
/// This assertion is implemented for the `Result` type. It compares the value
/// in `Err(value)` with the expected one. The error type in the `Result` must
/// implement `PartialEq<E>` where `E` is the type of the expected error value.
///
/// To assert the ok value of a `Result` use [`AssertHasValue::has_value`].
///
/// # Examples
///
/// ```
/// use asserting::prelude::*;
///
/// let subject: Result<(), String> = Err("labore gubergren ut ipsum".to_string());
/// assert_that!(subject).has_error("labore gubergren ut ipsum");
/// ```
pub trait AssertHasError<E> {
    /// Verifies that the subject holds an error value that is equal to the
    /// expected one.
    ///
    /// For `Result` it compares the value in `Err(value)`. If the `Result`
    /// holds an `Ok(value)`, the assertion fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// let subject: Result<(), String> = Err("labore gubergren ut ipsum".to_string());
    /// assert_that!(subject).has_error("labore gubergren ut ipsum");
    /// ```
    #[track_caller]
    fn has_error(self, expected: E) -> Self;
}

/// Assert that a subject of some container type holds an error value that has
/// a message equal to the expected message.
///
/// This is useful for opaque error types that do not implement
/// `PartialEq`. Since the `std::error::Error` trait requires that error
/// types implement `Display`, the string representation of the error value
/// is compared to an expected message string.
///
/// This assertion is implemented for the `Result` type. It compares the string
/// representation of the error value with the expected message.
///
/// To assert the ok value of a `Result` use [`AssertHasValue::has_value`].
///
/// # Examples
///
/// ```
/// use anyhow::anyhow;
/// use asserting::prelude::*;
///
/// let subject: Result<(), anyhow::Error> = Err(anyhow!("mollit in ullamcorper no".to_string()));
/// assert_that!(subject).has_error_message("mollit in ullamcorper no");
/// ```
pub trait AssertHasErrorMessage<'a, E, R> {
    /// Verifies that the subject is an error value with the expected message.
    ///
    /// This is useful for opaque error types that do not implement
    /// `PartialEq`. Since the `std::error::Error` trait requires that error
    /// types implement `Display`, the string representation of the error value
    /// is compared to an expected message string.
    ///
    /// This method panics if the actual subject is not an error value.
    ///
    /// # Examples
    ///
    /// ```
    /// use anyhow::anyhow;
    /// use asserting::prelude::*;
    ///
    /// let subject: Result<(), anyhow::Error> = Err(anyhow!("mollit in ullamcorper no".to_string()));
    /// assert_that!(subject).has_error_message("mollit in ullamcorper no");
    /// ```
    #[track_caller]
    fn has_error_message(self, expected_message: E) -> Spec<'a, String, R>;
}

/// Assert the source of any type that implements `std::error::Error`.
///
/// # Examples
///
/// ```
/// use asserting::prelude::*;
/// use std::error::Error;
/// use std::fmt::{self, Display};
///
/// #[derive(Debug)]
/// struct SuperError {
///     source: SourceError,
/// }
///
/// impl Display for SuperError {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         write!(f, "super-error caused by {}", self.source)
///     }
/// }
///
/// impl Error for SuperError {
///     fn source(&self) -> Option<&(dyn Error + 'static)> {
///         Some(&self.source)
///     }
/// }
///
/// #[derive(Debug, PartialEq)]
/// enum SourceError {
///     Foo,
///     Bar,
/// }
///
/// impl Display for SourceError {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         match self {
///             Self::Foo => f.write_str("foo error"),
///             Self::Bar => f.write_str("bar error"),
///         }
///     }
/// }
///
/// impl Error for SourceError {}
///
/// assert_that!(&SourceError::Foo).has_no_source();
///
/// let error = SuperError {
///     source: SourceError::Foo,
/// };
///
/// assert_that!(&error).has_source();
/// assert_that!(&error).has_source_message("foo error");
/// ```
pub trait AssertErrorHasSource<'a, R> {
    /// Verifies that an error has no source.
    ///
    /// # Example
    ///
    /// ```
    /// use asserting::prelude::*;
    /// use std::error::Error;
    /// use std::fmt::{self, Display};
    ///
    /// #[derive(Debug, PartialEq)]
    /// enum SimpleError {
    ///     Foo,
    ///     Bar,
    /// }
    ///
    /// impl Display for SimpleError {
    ///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         match self {
    ///             Self::Foo => f.write_str("foo error"),
    ///             Self::Bar => f.write_str("bar error"),
    ///         }
    ///     }
    /// }
    ///
    /// impl Error for SimpleError {}
    ///
    ///
    /// let error = SimpleError::Bar;
    ///
    /// assert_that!(&error).has_no_source();
    ///
    /// // error in result
    /// let result: Result<Vec<i32>, SimpleError> = Err(SimpleError::Foo);
    ///
    /// assert_that!(&result).err().has_no_source();
    /// ```
    #[track_caller]
    fn has_no_source(self) -> Self;

    /// Verifies that an error has some source.
    ///
    /// # Example
    ///
    /// ```
    /// use asserting::prelude::*;
    /// use std::error::Error;
    /// use std::fmt::{self, Display};
    ///
    /// #[derive(Debug)]
    /// struct SuperError {
    ///     source: SourceError,
    /// }
    ///
    /// impl Display for SuperError {
    ///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         write!(f, "super-error caused by {}", self.source)
    ///     }
    /// }
    ///
    /// impl Error for SuperError {
    ///     fn source(&self) -> Option<&(dyn Error + 'static)> {
    ///         Some(&self.source)
    ///     }
    /// }
    ///
    /// #[derive(Debug, PartialEq)]
    /// enum SourceError {
    ///     Foo,
    ///     Bar,
    /// }
    ///
    /// impl Display for SourceError {
    ///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         match self {
    ///             Self::Foo => f.write_str("foo error"),
    ///             Self::Bar => f.write_str("bar error"),
    ///         }
    ///     }
    /// }
    ///
    /// impl Error for SourceError {}
    ///
    ///
    /// let error = SuperError {
    ///     source: SourceError::Foo,
    /// };
    ///
    /// assert_that!(&error).has_source();
    ///
    /// // error in result
    /// let result: Result<Vec<i32>, SuperError> = Err(SuperError {
    ///     source: SourceError::Bar,
    /// });
    ///
    /// assert_that!(result).err().has_source();
    /// ```
    #[track_caller]
    fn has_source(self) -> Self;

    /// Verifies that an error has some source which converted to a string
    /// equals the expected message.
    ///
    /// # Example
    ///
    /// ```
    /// use asserting::prelude::*;
    /// use std::error::Error;
    /// use std::fmt::{self, Display};
    ///
    /// #[derive(Debug)]
    /// struct SuperError {
    ///     source: SourceError,
    /// }
    ///
    /// impl Display for SuperError {
    ///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         write!(f, "super-error caused by {}", self.source)
    ///     }
    /// }
    ///
    /// impl Error for SuperError {
    ///     fn source(&self) -> Option<&(dyn Error + 'static)> {
    ///         Some(&self.source)
    ///     }
    /// }
    ///
    /// #[derive(Debug, PartialEq)]
    /// enum SourceError {
    ///     Foo,
    ///     Bar,
    /// }
    ///
    /// impl Display for SourceError {
    ///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         match self {
    ///             Self::Foo => f.write_str("foo error"),
    ///             Self::Bar => f.write_str("bar error"),
    ///         }
    ///     }
    /// }
    ///
    /// impl Error for SourceError {}
    ///
    ///
    /// let error = SuperError {
    ///     source: SourceError::Bar,
    /// };
    ///
    /// assert_that!(&error).has_source_message("bar error");
    ///
    /// // error in result
    /// let result: Result<Vec<i32>, SuperError> = Err(SuperError {
    ///     source: SourceError::Foo,
    /// });
    ///
    /// assert_that!(result).err().has_source_message("foo error");
    /// ```
    #[track_caller]
    fn has_source_message(
        self,
        expected_source_message: impl Into<String>,
    ) -> Spec<'a, Option<String>, R>;
}

/// Assert a type formatted into a debug string.
///
/// The subject's type must implement `Debug` and the expected type must
/// implement `AsRef<str>`.
///
/// # Examples
///
/// ```
/// use asserting::prelude::*;
///
/// #[derive(Debug)]
/// struct Foo {
///     hello: String,
/// }
///
/// let subject = Foo { hello: "World".into() };
///
/// assert_that!(&subject).has_debug_message("Foo { hello: \"World\" }");
/// assert_that!(&subject).does_not_have_debug_message("Bar { hello: \"World\" }");
/// ```
pub trait AssertHasDebugMessage<E> {
    /// Verifies that a subject formatted for debugging results in the expected
    /// string.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// #[derive(Debug)]
    /// struct Foo {
    ///     hello: String,
    /// }
    ///
    /// let subject = Foo { hello: "World".into() };
    ///
    /// assert_that!(subject).has_debug_message("Foo { hello: \"World\" }");
    /// ```
    #[track_caller]
    fn has_debug_message(self, expected: E) -> Self;

    /// Verifies that a subject formatted for debugging does not result in the
    /// expected string.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// #[derive(Debug)]
    /// struct Foo {
    ///     hello: String,
    /// }
    ///
    /// let subject = Foo { hello: "World".into() };
    ///
    /// assert_that!(subject).does_not_have_debug_message("Hello World");
    /// ```
    #[track_caller]
    fn does_not_have_debug_message(self, expected: E) -> Self;
}

/// Assert a type formatted into a display string.
///
/// The subject's type must implement `Display` and the expected type must
/// implement `AsRef<str>`.
///
/// # Examples
///
/// ```
/// use core::fmt::{self, Display};
/// use asserting::prelude::*;
///
/// struct Foo {
///     hello: String,
/// }
///
/// impl Display for Foo {fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         write!(f, "Hello {}", self.hello)
///     }
/// }
///
/// let subject = Foo { hello: "World".into() };
///
/// assert_that!(&subject).has_display_message("Hello World");
/// assert_that!(&subject).does_not_have_display_message("Foo { hello: \"World\" }");
/// ```
pub trait AssertHasDisplayMessage<E> {
    /// Verifies that a subject formatted for display results in the expected
    /// string.
    ///
    /// # Examples
    ///
    /// ```
    /// use core::fmt::{self, Display};
    /// use asserting::prelude::*;
    ///
    /// struct Foo {
    ///     hello: String,
    /// }
    ///
    /// impl Display for Foo {fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         write!(f, "Hello {}", self.hello)
    ///     }
    /// }
    ///
    /// let subject = Foo { hello: "World".into() };
    ///
    /// assert_that!(&subject).has_display_message("Hello World");
    /// ```
    #[track_caller]
    fn has_display_message(self, expected: E) -> Self;

    /// Verifies that a subject formatted for display does not result in the
    /// expected string.
    ///
    /// # Examples
    ///
    /// ```
    /// use core::fmt::{self, Display};
    /// use asserting::prelude::*;
    ///
    /// struct Foo {
    ///     hello: String,
    /// }
    ///
    /// impl Display for Foo {fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         write!(f, "Hello {}", self.hello)
    ///     }
    /// }
    ///
    /// let subject = Foo { hello: "World".into() };
    ///
    /// assert_that!(&subject).does_not_have_display_message("Foo { hello: \"World\" }");
    /// ```
    #[track_caller]
    fn does_not_have_display_message(self, expected: E) -> Self;
}

/// Assert that a string contains a substring or character.
///
/// # Examples
///
/// ```
/// use asserting::prelude::*;
///
/// let subject = "doming vulputate proident eum";
///
/// assert_that!(subject).contains("tate p");
/// assert_that!(subject).contains('u');
/// assert_that!(subject).starts_with("dom");
/// assert_that!(subject).starts_with('d');
/// assert_that!(subject).ends_with("t eum");
/// assert_that!(subject).ends_with('m');
///
/// assert_that!(subject).does_not_contain("pat");
/// assert_that!(subject).does_not_contain('k');
/// assert_that!(subject).does_not_start_with("omi");
/// assert_that!(subject).does_not_start_with('o');
/// assert_that!(subject).does_not_end_with("meum");
/// assert_that!(subject).does_not_end_with('u');
/// ```
pub trait AssertStringPattern<E> {
    /// Verifies that a string contains a substring or character.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// let subject = "doming vulputate proident eum";
    ///
    /// assert_that!(subject).contains("tate p");
    /// assert_that!(subject).contains('u');
    /// ```
    #[track_caller]
    fn contains(self, pattern: E) -> Self;

    /// Verifies that a string does not contain a substring or character.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// let subject = "consequat nihil sanctus commodo";
    ///
    /// assert_that!(subject).does_not_contain("nixil");
    /// assert_that!(subject).does_not_contain('v');
    /// ```
    #[track_caller]
    fn does_not_contain(self, pattern: E) -> Self;

    /// Verifies that a string starts with a substring or character.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// let subject = "doming vulputate proident eum";
    ///
    /// assert_that!(subject).starts_with("dom");
    /// assert_that!(subject).starts_with('d');
    /// ```
    #[track_caller]
    fn starts_with(self, pattern: E) -> Self;

    /// Verifies that a string does not start with a substring or character.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// let subject = "ex nulla nostrud proident";
    ///
    /// assert_that!(subject).does_not_start_with("nulla");
    /// assert_that!(subject).does_not_start_with('v');
    /// ```
    #[track_caller]
    fn does_not_start_with(self, pattern: E) -> Self;

    /// Verifies that a string ends with a substring or character.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// let subject = "doming vulputate proident eum";
    ///
    /// assert_that!(subject).ends_with("t eum");
    /// assert_that!(subject).ends_with('m');
    /// ```
    #[track_caller]
    fn ends_with(self, pattern: E) -> Self;

    /// Verifies that a string does not end with a substring or character.
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// let subject = "sunt lorem at duo";
    ///
    /// assert_that!(subject).does_not_end_with("duos");
    /// assert_that!(subject).does_not_end_with('v');
    /// ```
    #[track_caller]
    fn does_not_end_with(self, pattern: E) -> Self;
}

/// Assert that a string contains any char from a collection of chars.
///
/// # Examples
///
/// ```
/// use asserting::prelude::*;
///
/// let subject = "vel aliquip doming eros";
///
/// assert_that!(subject).contains_any_of(['a', 'b', 'm', 'z']);
/// assert_that!(subject).contains_any_of(&['a', 'b', 'm', 'z']);
/// assert_that!(subject).contains_any_of(&['a', 'b', 'm', 'z'][..]);
///
/// assert_that!(subject).does_not_contain_any_of(['x', 'y', 'z']);
/// assert_that!(subject).does_not_contain_any_of(&['x', 'y', 'z']);
/// assert_that!(subject).does_not_contain_any_of(&['x', 'y', 'z'][..]);
/// ```
pub trait AssertStringContainsAnyOf<E> {
    /// Verifies that a string contains any char from a collection of
    /// characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// let subject = "vel aliquip doming eros";
    ///
    /// assert_that!(subject).contains_any_of(['a', 'b', 'm', 'z']);
    /// assert_that!(subject).contains_any_of(&['a', 'b', 'm', 'z']);
    /// assert_that!(subject).contains_any_of(&['a', 'b',  'm', 'z'][..]);
    /// ```
    #[track_caller]
    fn contains_any_of(self, expected: E) -> Self;

    /// Verifies that a string does not contain any char from a collection of
    /// characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// let subject = "sunt lorem at duo";
    ///
    /// assert_that!(subject).does_not_contain_any_of(['v', 'w', 'x']);
    /// assert_that!(subject).does_not_contain_any_of(&['v', 'w', 'x']);
    /// assert_that!(subject).does_not_contain_any_of(&['v', 'w', 'x'][..]);
    /// ```
    #[track_caller]
    fn does_not_contain_any_of(self, expected: E) -> Self;
}

/// Assert that a string matches a regex pattern.
///
/// # Example
///
/// ```
/// # #[cfg(not(feature = "regex"))]
/// # fn main() {}
/// # #[cfg(feature = "regex")]
/// # fn main() {
/// use asserting::prelude::*;
///
/// assert_that("tation odio placerat in").matches(r"\b\w{8}\b");
/// assert_that("tation odio placerat in").does_not_match(r"^[A-Z0-9 ]+$");
/// # }
/// ```
#[cfg(feature = "regex")]
#[cfg_attr(docsrs, doc(cfg(feature = "regex")))]
pub trait AssertStringMatches {
    /// Verifies that a string matches a regex pattern.
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(not(feature = "regex"))]
    /// # fn main() {}
    /// # #[cfg(feature = "regex")]
    /// # fn main() {
    /// use asserting::prelude::*;
    ///
    /// assert_that("tation odio placerat in").matches(r"\b\w{8}\b");
    /// # }
    /// ```
    ///
    /// # Panics
    ///
    /// This method panics if the given regex pattern is invalid or exceeds the
    /// size limit.
    #[track_caller]
    fn matches(self, regex_pattern: &str) -> Self;

    /// Verifies that a string does not match a regex pattern.
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(not(feature = "regex"))]
    /// # fn main() {}
    /// # #[cfg(feature = "regex")]
    /// # fn main() {
    /// use asserting::prelude::*;
    ///
    /// assert_that("tation odio placerat in").does_not_match(r"^[A-Z0-9 ]+$");
    /// # }
    /// ```
    ///
    /// # Panics
    ///
    /// This method panics if the given regex pattern is invalid or exceeds the
    /// size limit.
    #[track_caller]
    fn does_not_match(self, regex_pattern: &str) -> Self;
}

/// Assert that an iterator or collection contains the expected value.
///
/// This assertion is implemented for any collection or iterator of items that
/// implement `PartialEq<E>` with `E` being the type of the expected value.
///
/// # Examples
///
/// ```
/// use asserting::prelude::*;
/// use std::collections::BTreeMap;
///
/// let some_array = [1, 3, 5, 7];
/// assert_that!(some_array).contains(5);
///
/// let some_slice = &['b', 'X', 'k', 'G'][..];
/// assert_that!(some_slice).contains(&'X');
///
/// let some_vec = vec![12, 4, 6, 10, 8];
/// assert_that!(some_vec).contains(12);
///
/// let some_btree_map = BTreeMap::from_iter([('a', 3), ('b', 0), ('c', 8)]);
/// assert_that!(some_btree_map).contains(('b', 0));
/// ```
pub trait AssertIteratorContains<'a, U, E, R> {
    /// Verifies that the actual collection/iterator contains at least one
    /// element that is equal to the expected value.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    /// use std::collections::BTreeMap;
    ///
    /// let some_array = [1, 3, 5, 7];
    /// assert_that!(some_array).contains(5);
    ///
    /// let some_slice = &['b', 'X', 'k', 'G'][..];
    /// assert_that!(some_slice).contains(&'X');
    ///
    /// let some_vec = vec![12, 4, 6, 10, 8];
    /// assert_that!(some_vec).contains(12);
    ///
    /// let some_btree_map = BTreeMap::from_iter([('a', 3), ('b', 0), ('c', 8)]);
    /// assert_that!(some_btree_map).contains(('b', 0));
    /// ```
    #[track_caller]
    fn contains(self, element: E) -> Spec<'a, U, R>;

    /// Verifies that the actual collection/iterator does not contain an element
    /// that is equal to the expected value.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    /// use std::collections::BTreeMap;
    ///
    /// let some_array = [1, 3, 5, 7];
    /// assert_that!(some_array).does_not_contain(2);
    ///
    /// let some_slice = &['b', 'X', 'k', 'G'][..];
    /// assert_that!(some_slice).does_not_contain(&'Y');
    ///
    /// let some_vec = vec![12, 4, 6, 10, 8];
    /// assert_that!(some_vec).does_not_contain(5);
    ///
    /// let some_btree_map = BTreeMap::from_iter([('a', 3), ('b', 0), ('c', 8)]);
    /// assert_that!(some_btree_map).does_not_contain(('d', 5));
    /// ```
    #[track_caller]
    fn does_not_contain(self, element: E) -> Spec<'a, U, R>;
}

/// Assert values in a collection.
///
/// These assertions do not rely on the order in which the collection iterates
/// over its values. They are implemented for any iterator over items that
/// implement `PartialEq<E>` with `E` being the type of the items in the
/// expected collection or iterator.
pub trait AssertIteratorContainsInAnyOrder<'a, S, E, R> {
    /// Verifies that the actual collection/iterator contains exactly the given
    /// values and nothing else in any order.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    /// use std::collections::BTreeMap;
    ///
    /// let some_array = [1, 3, 5, 7];
    /// assert_that!(some_array).contains_exactly_in_any_order([3, 1, 5, 7]);
    ///
    /// let some_slice = &['b', 'X', 'k', 'G'][..];
    /// assert_that!(some_slice).contains_exactly_in_any_order(&['X', 'k', 'b', 'G']);
    ///
    /// let some_vec = vec![12, 4, 6, 10, 8];
    /// assert_that!(some_vec).contains_exactly_in_any_order([8, 10, 6, 4, 12]);
    ///
    /// let some_btree_map = BTreeMap::from_iter([('a', 3), ('b', 0), ('c', 8)]);
    /// assert_that!(some_btree_map).contains_exactly_in_any_order([('b', 0), ('a', 3), ('c', 8)]);
    /// ```
    #[track_caller]
    fn contains_exactly_in_any_order(self, expected: E) -> Spec<'a, S, R>;

    /// Verifies that the actual collection/iterator contains at least one of
    /// the specified values.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    /// use std::collections::BTreeMap;
    ///
    /// let some_array = [1, 3, 5, 7];
    /// assert_that!(some_array).contains_any_of([2, 3, 4]);
    ///
    /// let some_slice = &['b', 'X', 'k', 'G'][..];
    /// assert_that!(some_slice).contains_any_of(&['a', 'b', 'c', 'd']);
    ///
    /// let some_vec = vec![12, 4, 6, 10, 8];
    /// assert_that!(some_vec).contains_any_of([1, 2, 3, 4, 5]);
    ///
    /// let some_btree_map = BTreeMap::from_iter([('a', 3), ('b', 0), ('c', 8)]);
    /// assert_that!(some_btree_map).contains_any_of([('x', 2), ('a', 3), ('y', 7)]);
    /// ```
    #[track_caller]
    fn contains_any_of(self, expected: E) -> Spec<'a, S, R>;

    /// Verifies that the actual collection/iterator does not contain any of
    /// the specified values.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    /// use std::collections::BTreeMap;
    ///
    /// let some_array = [1, 3, 5, 7];
    /// assert_that!(some_array).does_not_contain_any_of([2, 4, 6]);
    ///
    /// let some_slice = &['b', 'X', 'k', 'G'][..];
    /// assert_that!(some_slice).does_not_contain_any_of(&['a', 'A', 'c', 'd']);
    ///
    /// let some_vec = vec![12, 4, 6, 10, 8];
    /// assert_that!(some_vec).does_not_contain_any_of([1, 2, 3, 5, 7]);
    ///
    /// let some_btree_map = BTreeMap::from_iter([('a', 3), ('b', 0), ('c', 8)]);
    /// assert_that!(some_btree_map).does_not_contain_any_of([('x', 2), ('z', 3), ('y', 7)]);
    /// ```
    #[track_caller]
    fn does_not_contain_any_of(self, expected: E) -> Spec<'a, S, R>;

    /// Verifies that the actual collection/iterator contains the given values
    /// in any order.
    ///
    /// The collection/iterator may contain more values than the given ones, but
    /// at least all the specified ones.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    /// use std::collections::BTreeMap;
    ///
    /// let some_array = [1, 3, 5, 7];
    /// assert_that!(some_array).contains_all_of([3, 1, 5]);
    ///
    /// let some_slice = &['b', 'X', 'k', 'G'][..];
    /// assert_that!(some_slice).contains_all_of(&['k', 'b']);
    ///
    /// let some_vec = vec![12, 4, 6, 10, 8];
    /// assert_that!(some_vec).contains_all_of([4, 6, 10, 12]);
    ///
    /// let some_btree_map = BTreeMap::from_iter([('a', 3), ('b', 0), ('c', 8)]);
    /// assert_that!(some_btree_map).contains_all_of([('a', 3), ('b', 0)]);
    /// ```
    #[track_caller]
    fn contains_all_of(self, expected: E) -> Spec<'a, S, R>;

    /// Verifies that the actual collection/iterator contains only the given
    /// values and nothing else in any order and ignoring duplicates.
    ///
    /// The collection may contain fewer values than the expected ones.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    /// use std::collections::BTreeMap;
    ///
    /// let some_array = [1, 3, 5, 7];
    /// assert_that!(some_array).contains_only([0, 5, 3, 1, 7, 9]);
    ///
    /// let some_slice = &['b', 'X', 'k', 'G'][..];
    /// assert_that!(some_slice).contains_only(&['X', 'a', 'k', 'b', 'G', 'A']);
    ///
    /// let some_vec = vec![12, 4, 6, 10, 8];
    /// assert_that!(some_vec).contains_only([2, 4, 6, 8, 10, 12, 14, 0]);
    ///
    /// let some_btree_map = BTreeMap::from_iter([('a', 3), ('b', 0), ('c', 8)]);
    /// assert_that!(some_btree_map).contains_only([('a', 3), ('b', 0), ('c', 8), ('d', 4)]);
    /// ```
    #[track_caller]
    fn contains_only(self, expected: E) -> Spec<'a, S, R>;

    /// Verifies that the actual collection/iterator contains only the given
    /// values in any order and each of them only once.
    ///
    /// The collection may contain fewer values than the expected ones.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    /// use std::collections::BTreeMap;
    ///
    /// let some_array = [1, 3, 5, 7];
    /// assert_that!(some_array).contains_only_once([0, 1, 3, 5, 7, 9]);
    ///
    /// let some_slice = &['b', 'X', 'k', 'G'][..];
    /// assert_that!(some_slice).contains_only_once(&['a', 'b', 'X', 'k', 'G']);
    ///
    /// let some_vec = vec![12, 4, 6, 10, 8];
    /// assert_that!(some_vec).contains_only_once([4, 6, 8, 10, 12, 15, 20]);
    /// ```
    #[track_caller]
    fn contains_only_once(self, expected: E) -> Spec<'a, S, R>;
}

/// Assert values in an ordered collection.
///
/// These assertions are applicable to collections which iterate over their
/// values in a defined order.
pub trait AssertIteratorContainsInOrder<'a, S, E, R> {
    /// Verifies that the actual collection/iterator contains exactly the given
    /// values and nothing else in the given order.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    /// use std::collections::BTreeMap;
    ///
    /// let some_array = [1, 3, 5, 7];
    /// assert_that!(some_array).contains_exactly([1, 3, 5, 7]);
    ///
    /// let some_slice = &['b', 'X', 'k', 'G'][..];
    /// assert_that!(some_slice).contains_exactly(&['b', 'X', 'k', 'G']);
    ///
    /// let some_vec = vec![12, 4, 6, 10, 8];
    /// assert_that!(some_vec).contains_exactly([12, 4, 6, 10, 8]);
    ///
    /// let some_btree_map = BTreeMap::from_iter([('a', 3), ('b', 0), ('c', 8)]);
    /// assert_that!(some_btree_map).contains_exactly([('a', 3), ('b', 0), ('c', 8)]);
    /// ```
    #[track_caller]
    fn contains_exactly(self, expected: E) -> Spec<'a, S, R>;

    /// Verifies that the actual collection/iterator contains the given sequence
    /// of values in the given order and without extra values between the
    /// sequence values.
    ///
    /// May contain more values as in the given sequence before and after the
    /// sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    /// use std::collections::BTreeMap;
    ///
    /// let some_array = [1, 3, 5, 7, 9];
    /// assert_that!(some_array).contains_sequence([3, 5, 7]);
    ///
    /// let some_slice = &['b', 'X', 'k', 'G'][..];
    /// assert_that!(some_slice).contains_sequence(&['b', 'X']);
    ///
    /// let some_vec = vec![12, 4, 6, 10, 8];
    /// assert_that!(some_vec).contains_sequence([6, 10, 8]);
    ///
    /// let some_btree_map = BTreeMap::from_iter([('a', 3), ('b', 0), ('c', 8)]);
    /// assert_that!(some_btree_map).contains_sequence([('a', 3), ('b', 0), ('c', 8)]);
    /// ```
    #[track_caller]
    fn contains_sequence(self, expected: E) -> Spec<'a, S, R>;

    /// Verifies that the actual collection/iterator contains all the given
    /// values and in the given order, possibly with other values between them.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    /// use std::collections::BTreeMap;
    ///
    /// let some_array = [1, 3, 5, 7, 9];
    /// assert_that!(some_array).contains_all_in_order([3, 5, 9]);
    ///
    /// let some_slice = &['b', 'X', 'k', 'G'][..];
    /// assert_that!(some_slice).contains_all_in_order(&['b', 'G']);
    ///
    /// let some_vec = vec![12, 4, 6, 10, 8];
    /// assert_that!(some_vec).contains_all_in_order([12, 4, 10, 8]);
    ///
    /// let some_btree_map = BTreeMap::from_iter([('a', 3), ('b', 0), ('c', 8)]);
    /// assert_that!(some_btree_map).contains_all_in_order([('a', 3), ('c', 8)]);
    /// ```
    #[track_caller]
    fn contains_all_in_order(self, expected: E) -> Spec<'a, S, R>;

    /// Verifies that the actual collection/iterator contains the given values
    /// as the first elements in order.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    /// use std::collections::BTreeMap;
    ///
    /// let some_array = [1, 3, 5, 7];
    /// assert_that!(some_array).starts_with([1, 3, 5]);
    ///
    /// let some_slice = &['b', 'X', 'k', 'G'][..];
    /// assert_that!(some_slice).starts_with(&['b', 'X']);
    ///
    /// let some_vec = vec![12, 4, 6, 10, 8];
    /// assert_that!(some_vec).starts_with([12, 4, 6]);
    ///
    /// let some_btree_map = BTreeMap::from_iter([('a', 3), ('b', 0), ('c', 8)]);
    /// assert_that!(some_btree_map).starts_with([('a', 3), ('b', 0)]);
    /// ```
    #[track_caller]
    fn starts_with(self, expected: E) -> Spec<'a, S, R>;

    /// Verifies that the actual collection/iterator contains the given values
    /// as the last elements in order.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    /// use std::collections::BTreeMap;
    ///
    /// let some_array = [1, 3, 5, 7];
    /// assert_that!(some_array).ends_with([3, 5, 7]);
    ///
    /// let some_slice = &['b', 'X', 'k', 'G'][..];
    /// assert_that!(some_slice).ends_with(&['k', 'G']);
    ///
    /// let some_vec = vec![12, 4, 6, 10, 8];
    /// assert_that!(some_vec).ends_with([10, 8]);
    ///
    /// let some_btree_map = BTreeMap::from_iter([('a', 3), ('b', 0), ('c', 8)]);
    /// assert_that!(some_btree_map).ends_with([('b', 0), ('c', 8)]);
    /// ```
    #[track_caller]
    fn ends_with(self, expected: E) -> Spec<'a, S, R>;
}

/// Assert the order of the values within a collection.
///
/// These assertions are applicable to ordered collections only.
pub trait AssertIsSorted {
    /// Verifies that the actual collection is sorted in ascending order.
    #[track_caller]
    fn is_sorted_ascending(self) -> Self;

    /// Verifies that the actual collection is sorted in descending order.
    #[track_caller]
    fn is_sorted_descending(self) -> Self;
}

/// Assert that the code under test panics, panics with a certain message or
/// does not panic.
///
/// # Examples
///
/// ```
/// use asserting::prelude::*;
///
/// fn do_something(input: &str) {
///     if input.is_empty() {
///         panic!("input is empty");
///     }
/// }
///
/// assert_that_code!(|| {
///     do_something("correct input");
/// }).does_not_panic();
///
/// assert_that_code!(|| {
///     do_something("");
/// }).panics();
///
/// assert_that_code!(|| {
///     do_something("");
/// }).panics_with_message("input is empty");
/// ```
#[cfg(feature = "panic")]
#[cfg_attr(docsrs, doc(cfg(feature = "panic")))]
pub trait AssertCodePanics<'a, R> {
    /// Verifies that the actual code under test does not panic.
    ///
    /// # Example
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// fn do_something(input: &str) {
    ///     if input.is_empty() {
    ///         panic!("input is empty");
    ///     }
    /// }
    ///
    /// assert_that_code!(|| {
    ///     do_something("correct input");
    /// }).does_not_panic();
    /// ```
    #[track_caller]
    fn does_not_panic(self) -> Spec<'a, (), R>;

    /// Verifies that the actual code under test panics with any message.
    ///
    /// # Example
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// fn do_something(input: &str) {
    ///     if input.is_empty() {
    ///         panic!("input is empty");
    ///     }
    /// }
    ///
    /// assert_that_code!(|| {
    ///     do_something("");
    /// }).panics();
    /// ```
    #[track_caller]
    fn panics(self) -> Spec<'a, (), R>;

    /// Verifies that the actual code under test panics with the given
    /// message.
    ///
    /// # Example
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// fn do_something(input: &str) {
    ///     if input.is_empty() {
    ///         panic!("input is empty");
    ///     }
    /// }
    ///
    /// assert_that_code!(|| {
    ///     do_something("");
    /// }).panics_with_message("input is empty");
    /// ```
    #[track_caller]
    fn panics_with_message(self, message: impl Into<String>) -> Spec<'a, (), R>;
}

/// Assertions for the keys of a map.
///
/// # Examples
///
/// ```
/// # #[cfg(not(feature = "std"))]
/// # fn main() {}
/// # #[cfg(feature = "std")]
/// # fn main() {
/// use asserting::prelude::*;
/// use std::collections::HashMap;
///
/// let subject: HashMap<_, _> = [(4, "four"), (1, "one"), (5, "five"), (8, "eight")].into();
///
/// assert_that!(&subject).contains_key(5);
/// assert_that!(&subject).does_not_contain_key(3);
/// assert_that!(&subject).contains_keys([4, 8]);
/// assert_that!(&subject).does_not_contain_keys([3, 2, 7]);
/// assert_that!(&subject).contains_exactly_keys([4, 1, 5, 8]);
/// # }
/// ```
///
/// ```
/// use asserting::prelude::*;
/// use hashbrown::HashMap;
///
/// let subject: HashMap<_, _> = [(4, "four"), (1, "one"), (5, "five"), (8, "eight")].into();
///
/// assert_that!(&subject).contains_key(4);
/// assert_that!(&subject).does_not_contain_key(7);
/// assert_that!(&subject).contains_keys([1, 5]);
/// assert_that!(&subject).does_not_contain_keys([2, 7, 6]);
/// assert_that!(&subject).contains_exactly_keys([1, 4, 5, 8]);
/// ```
///
/// ```
/// # #[cfg(not(feature = "std"))]
/// # fn main() {}
/// # #[cfg(feature = "std")]
/// # fn main() {
/// use asserting::prelude::*;
/// use std::collections::BTreeMap;
///
/// let subject: BTreeMap<_, _> = [(4, "four"), (1, "one"), (5, "five"), (8, "eight")].into();
///
/// assert_that!(&subject).contains_key(4);
/// assert_that!(&subject).does_not_contain_key(2);
/// assert_that!(&subject).contains_keys([1, 4, 8]);
/// assert_that!(&subject).does_not_contain_keys([2, 3, 6]);
/// assert_that!(&subject).contains_exactly_keys([4, 5, 8, 1]);
/// # }
/// ```
pub trait AssertMapContainsKey<E> {
    /// Verify that the actual map contains a mapping for the given key.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(not(feature = "std"))]
    /// # fn main() {}
    /// # #[cfg(feature = "std")]
    /// # fn main() {
    /// use asserting::prelude::*;
    /// use std::collections::HashMap;
    ///
    /// let subject: HashMap<_, _> = [(4, "four"), (1, "one"), (5, "five"), (8, "eight")].into();
    ///
    /// assert_that!(&subject).contains_key(5);
    /// assert_that!(subject).contains_key(1);
    /// # }
    /// ```
    ///
    /// ```
    /// use asserting::prelude::*;
    /// use hashbrown::HashMap;
    ///
    /// let subject: HashMap<_, _> = [(4, "four"), (1, "one"), (5, "five"), (8, "eight")].into();
    ///
    /// assert_that!(&subject).contains_key(4);
    /// assert_that!(subject).contains_key(8);
    /// ```
    ///
    /// ```
    /// # #[cfg(not(feature = "std"))]
    /// # fn main() {}
    /// # #[cfg(feature = "std")]
    /// # fn main() {
    /// use asserting::prelude::*;
    /// use std::collections::BTreeMap;
    ///
    /// let subject: BTreeMap<_, _> = [(4, "four"), (1, "one"), (5, "five"), (8, "eight")].into();
    ///
    /// assert_that!(&subject).contains_key(4);
    /// assert_that!(subject).contains_key(5);
    /// # }
    /// ```
    #[track_caller]
    fn contains_key(self, expected_key: E) -> Self;

    /// Verify that the actual map does not contain any mapping for the given
    /// key.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(not(feature = "std"))]
    /// # fn main() {}
    /// # #[cfg(feature = "std")]
    /// # fn main() {
    /// use asserting::prelude::*;
    /// use std::collections::HashMap;
    ///
    /// let subject: HashMap<_, _> = [(4, "four"), (1, "one"), (5, "five"), (8, "eight")].into();
    ///
    /// assert_that!(&subject).does_not_contain_key(2);
    /// assert_that!(subject).does_not_contain_key(3);
    /// # }
    /// ```
    ///
    /// ```
    /// use asserting::prelude::*;
    /// use hashbrown::HashMap;
    ///
    /// let subject: HashMap<_, _> = [(4, "four"), (1, "one"), (5, "five"), (8, "eight")].into();
    ///
    /// assert_that!(&subject).does_not_contain_key(6);
    /// assert_that!(subject).does_not_contain_key(7);
    /// ```
    ///
    /// ```
    /// # #[cfg(not(feature = "std"))]
    /// # fn main() {}
    /// # #[cfg(feature = "std")]
    /// # fn main() {
    /// use asserting::prelude::*;
    /// use std::collections::BTreeMap;
    ///
    /// let subject: BTreeMap<_, _> = [(4, "four"), (1, "one"), (5, "five"), (8, "eight")].into();
    ///
    /// assert_that!(&subject).does_not_contain_key(3);
    /// assert_that!(subject).does_not_contain_key(9);
    /// # }
    /// ```
    #[track_caller]
    fn does_not_contain_key(self, expected_key: E) -> Self;

    /// Verify that the actual map contains a mapping for each of the given
    /// keys.
    ///
    /// The order of the keys is not relevant and duplicates are ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(not(feature = "std"))]
    /// # fn main() {}
    /// # #[cfg(feature = "std")]
    /// # fn main() {
    /// use asserting::prelude::*;
    /// use std::collections::HashMap;
    ///
    /// let subject: HashMap<_, _> = [(4, "four"), (1, "one"), (5, "five"), (8, "eight")].into();
    ///
    /// assert_that!(&subject).contains_keys([4, 5]);
    /// assert_that!(&subject).contains_keys([8, 1, 5]);
    /// assert_that!(&subject).contains_keys([8, 1, 1]);
    /// # }
    /// ```
    ///
    /// ```
    /// use asserting::prelude::*;
    /// use hashbrown::HashMap;
    ///
    /// let subject: HashMap<_, _> = [(4, "four"), (1, "one"), (5, "five"), (8, "eight")].into();
    ///
    /// assert_that!(&subject).contains_keys([1, 5]);
    /// assert_that!(&subject).contains_keys([8, 1, 4]);
    /// assert_that!(&subject).contains_keys([8, 4, 4]);
    /// ```
    ///
    /// ```
    /// # #[cfg(not(feature = "std"))]
    /// # fn main() {}
    /// # #[cfg(feature = "std")]
    /// # fn main() {
    /// use asserting::prelude::*;
    /// use std::collections::BTreeMap;
    ///
    /// let subject: BTreeMap<_, _> = [(4, "four"), (1, "one"), (5, "five"), (8, "eight")].into();
    ///
    /// assert_that!(&subject).contains_keys([1, 4, 5, 8]);
    /// assert_that!(&subject).contains_keys([5, 4, 8]);
    /// assert_that!(&subject).contains_keys([5, 5, 8]);
    /// # }
    /// ```
    #[track_caller]
    fn contains_keys(self, expected_keys: impl IntoIterator<Item = E>) -> Self;

    /// Verify that the actual map does not contain any mapping for one of the
    /// given keys.
    ///
    /// The order of the keys is not relevant and duplicates are ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(not(feature = "std"))]
    /// # fn main() {}
    /// # #[cfg(feature = "std")]
    /// # fn main() {
    /// use asserting::prelude::*;
    /// use std::collections::HashMap;
    ///
    /// let subject: HashMap<_, _> = [(4, "four"), (1, "one"), (5, "five"), (8, "eight")].into();
    ///
    /// assert_that!(&subject).does_not_contain_keys([2, 3]);
    /// assert_that!(&subject).does_not_contain_keys([6, 3, 7]);
    /// assert_that!(&subject).does_not_contain_keys([3, 6, 3]);
    /// # }
    /// ```
    ///
    /// ```
    /// use asserting::prelude::*;
    /// use hashbrown::HashMap;
    ///
    /// let subject: HashMap<_, _> = [(4, "four"), (1, "one"), (5, "five"), (8, "eight")].into();
    ///
    /// assert_that!(&subject).does_not_contain_keys([6, 7]);
    /// assert_that!(&subject).does_not_contain_keys([3, 2, 6]);
    /// assert_that!(&subject).does_not_contain_keys([7, 2, 7]);
    /// ```
    ///
    /// ```
    /// # #[cfg(not(feature = "std"))]
    /// # fn main() {}
    /// # #[cfg(feature = "std")]
    /// # fn main() {
    /// use asserting::prelude::*;
    /// use std::collections::BTreeMap;
    ///
    /// let subject: BTreeMap<_, _> = [(4, "four"), (1, "one"), (5, "five"), (8, "eight")].into();
    ///
    /// assert_that!(&subject).does_not_contain_keys([2, 3, 6, 7]);
    /// assert_that!(&subject).does_not_contain_keys([7, 3, 6]);
    /// assert_that!(&subject).does_not_contain_keys([2, 2, 9]);
    /// # }
    /// ```
    #[track_caller]
    fn does_not_contain_keys(self, expected_keys: impl IntoIterator<Item = E>) -> Self;

    /// Verifies that the actual map contains a mapping for each of the expected
    /// keys but no more.
    ///
    /// The order of the keys is not relevant and duplicates are ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(not(feature = "std"))]
    /// # fn main() {}
    /// # #[cfg(feature = "std")]
    /// # fn main() {
    /// use asserting::prelude::*;
    /// use std::collections::HashMap;
    ///
    /// let subject: HashMap<_, _> = [(4, "four"), (1, "one"), (5, "five"), (8, "eight")].into();
    ///
    /// assert_that!(&subject).contains_exactly_keys([4, 1, 5, 8]);
    /// assert_that!(&subject).contains_exactly_keys([1, 4, 5, 8]);
    /// assert_that!(&subject).contains_exactly_keys([1, 4, 5, 8, 8]);
    /// # }
    /// ```
    ///
    /// ```
    /// use asserting::prelude::*;
    /// use hashbrown::HashMap;
    ///
    /// let subject: HashMap<_, _> = [(4, "four"), (1, "one"), (5, "five"), (8, "eight")].into();
    ///
    /// assert_that!(&subject).contains_exactly_keys([4, 1, 5, 8]);
    /// assert_that!(&subject).contains_exactly_keys([1, 4, 5, 8]);
    /// assert_that!(&subject).contains_exactly_keys([1, 1, 4, 5, 8]);
    /// ```
    ///
    /// ```
    /// # #[cfg(not(feature = "std"))]
    /// # fn main() {}
    /// # #[cfg(feature = "std")]
    /// # fn main() {
    /// use asserting::prelude::*;
    /// use std::collections::BTreeMap;
    ///
    /// let subject: BTreeMap<_, _> = [(4, "four"), (1, "one"), (5, "five"), (8, "eight")].into();
    ///
    /// assert_that!(&subject).contains_exactly_keys([1, 4, 5, 8]);
    /// assert_that!(&subject).contains_exactly_keys([5, 4, 1, 8]);
    /// assert_that!(&subject).contains_exactly_keys([5, 4, 4, 1, 8]);
    /// # }
    /// ```
    #[track_caller]
    fn contains_exactly_keys(self, expected_keys: impl IntoIterator<Item = E>) -> Self;
}

/// Assertions for the values of a map.
///
/// # Examples
///
/// ```
/// # #[cfg(not(feature = "std"))]
/// # fn main() {}
/// # #[cfg(feature = "std")]
/// # fn main() {
/// use asserting::prelude::*;
/// use std::collections::HashMap;
///
/// let subject: HashMap<_, _> = [(4, "four"), (1, "one"), (5, "five"), (8, "eight")].into();
///
/// assert_that!(&subject).contains_value("five");
/// assert_that!(&subject).does_not_contain_value("three");
/// assert_that!(&subject).contains_values(["four", "eight"]);
/// assert_that!(&subject).does_not_contain_values(["three", "two", "seven"]);
/// # }
/// ```
///
/// ```
/// use asserting::prelude::*;
/// use hashbrown::HashMap;
///
/// let subject: HashMap<_, _> = [(4, "four"), (1, "one"), (5, "five"), (8, "eight")].into();
///
/// assert_that!(&subject).contains_value("four");
/// assert_that!(&subject).does_not_contain_value("seven");
/// assert_that!(&subject).contains_values(["one", "five"]);
/// assert_that!(&subject).does_not_contain_values(["two", "seven", "six"]);
/// ```
///
/// ```
/// # #[cfg(not(feature = "std"))]
/// # fn main() {}
/// # #[cfg(feature = "std")]
/// # fn main() {
/// use asserting::prelude::*;
/// use std::collections::BTreeMap;
///
/// let subject: BTreeMap<_, _> = [(4, "four"), (1, "one"), (5, "five"), (8, "eight")].into();
///
/// assert_that!(&subject).contains_value("four");
/// assert_that!(&subject).does_not_contain_value("two");
/// assert_that!(&subject).contains_values(["four", "one", "eight"]);
/// assert_that!(&subject).does_not_contain_values(["two", "three", "six"]);
/// # }
/// ```
pub trait AssertMapContainsValue<E> {
    /// Verify that the actual map contains at least one mapping where the value
    /// is equal to the expected one.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(not(feature = "std"))]
    /// # fn main() {}
    /// # #[cfg(feature = "std")]
    /// # fn main() {
    /// use asserting::prelude::*;
    /// use std::collections::HashMap;
    ///
    /// let subject: HashMap<_, _> = [(4, "four"), (1, "one"), (5, "five"), (8, "eight")].into();
    ///
    /// assert_that!(&subject).contains_value("five");
    /// assert_that!(subject).contains_value("one");
    /// # }
    /// ```
    ///
    /// ```
    /// use asserting::prelude::*;
    /// use hashbrown::HashMap;
    ///
    /// let subject: HashMap<_, _> = [(4, "four"), (1, "one"), (5, "five"), (8, "eight")].into();
    ///
    /// assert_that!(&subject).contains_value("one");
    /// assert_that!(subject).contains_value("eight");
    /// ```
    ///
    /// ```
    /// # #[cfg(not(feature = "std"))]
    /// # fn main() {}
    /// # #[cfg(feature = "std")]
    /// # fn main() {
    /// use asserting::prelude::*;
    /// use std::collections::BTreeMap;
    ///
    /// let subject: BTreeMap<_, _> = [(4, "four"), (1, "one"), (5, "five"), (8, "eight")].into();
    ///
    /// assert_that!(&subject).contains_value("four");
    /// assert_that!(subject).contains_value("five");
    /// # }
    /// ```
    #[track_caller]
    fn contains_value(self, expected_value: E) -> Self;

    /// Verify that the actual map does not contain any mapping where the value
    /// is equal to the expected one.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(not(feature = "std"))]
    /// # fn main() {}
    /// # #[cfg(feature = "std")]
    /// # fn main() {
    /// use asserting::prelude::*;
    /// use std::collections::HashMap;
    ///
    /// let subject: HashMap<_, _> = [(4, "four"), (1, "one"), (5, "five"), (8, "eight")].into();
    ///
    /// assert_that!(&subject).does_not_contain_value("two");
    /// assert_that!(subject).does_not_contain_value("three");
    /// # }
    /// ```
    ///
    /// ```
    /// use asserting::prelude::*;
    /// use hashbrown::HashMap;
    ///
    /// let subject: HashMap<_, _> = [(4, "four"), (1, "one"), (5, "five"), (8, "eight")].into();
    ///
    /// assert_that!(&subject).does_not_contain_value("six");
    /// assert_that!(subject).does_not_contain_value("seven");
    /// ```
    ///
    /// ```
    /// # #[cfg(not(feature = "std"))]
    /// # fn main() {}
    /// # #[cfg(feature = "std")]
    /// # fn main() {
    /// use asserting::prelude::*;
    /// use std::collections::BTreeMap;
    ///
    /// let subject: BTreeMap<_, _> = [(4, "four"), (1, "one"), (5, "five"), (8, "eight")].into();
    ///
    /// assert_that!(&subject).does_not_contain_value("three");
    /// assert_that!(subject).does_not_contain_value("nine");
    /// # }
    /// ```
    #[track_caller]
    fn does_not_contain_value(self, expected_value: E) -> Self;

    /// Verify that the actual map contains at least one mapping for each of the
    /// given values, where the mapping contains one of the expected values.
    ///
    /// The order of the values is not relevant and duplicates are ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(not(feature = "std"))]
    /// # fn main() {}
    /// # #[cfg(feature = "std")]
    /// # fn main() {
    /// use asserting::prelude::*;
    /// use std::collections::HashMap;
    ///
    /// let subject: HashMap<_, _> = [(4, "four"), (1, "one"), (5, "five"), (8, "eight")].into();
    ///
    /// assert_that!(&subject).contains_values(["four", "five"]);
    /// assert_that!(&subject).contains_values(["eight", "one", "five"]);
    /// assert_that!(&subject).contains_values(["eight", "one", "one"]);
    /// # }
    /// ```
    ///
    /// ```
    /// use asserting::prelude::*;
    /// use hashbrown::HashMap;
    ///
    /// let subject: HashMap<_, _> = [(4, "four"), (1, "one"), (5, "five"), (8, "eight")].into();
    ///
    /// assert_that!(&subject).contains_values(["one", "five"]);
    /// assert_that!(&subject).contains_values(["eight", "one", "four"]);
    /// assert_that!(&subject).contains_values(["eight", "four", "four"]);
    /// ```
    ///
    /// ```
    /// # #[cfg(not(feature = "std"))]
    /// # fn main() {}
    /// # #[cfg(feature = "std")]
    /// # fn main() {
    /// use asserting::prelude::*;
    /// use std::collections::BTreeMap;
    ///
    /// let subject: BTreeMap<_, _> = [(4, "four"), (1, "one"), (5, "five"), (8, "eight")].into();
    ///
    /// assert_that!(&subject).contains_values(["one", "four", "five", "eight"]);
    /// assert_that!(&subject).contains_values(["five", "four", "eight"]);
    /// assert_that!(&subject).contains_values(["five", "five", "eight"]);
    /// # }
    /// ```
    #[track_caller]
    fn contains_values(self, expected_values: impl IntoIterator<Item = E>) -> Self;

    /// Verify that the actual map does not contain any mapping where the value
    /// is one of the given values.
    ///
    /// The order of the values is not relevant and duplicates are ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(not(feature = "std"))]
    /// # fn main() {}
    /// # #[cfg(feature = "std")]
    /// # fn main() {
    /// use asserting::prelude::*;
    /// use std::collections::HashMap;
    ///
    /// let subject: HashMap<_, _> = [(4, "four"), (1, "one"), (5, "five"), (8, "eight")].into();
    ///
    /// assert_that!(&subject).does_not_contain_values(["two", "three"]);
    /// assert_that!(&subject).does_not_contain_values(["six", "three", "seven"]);
    /// assert_that!(&subject).does_not_contain_values(["three", "six", "three"]);
    /// # }
    /// ```
    ///
    /// ```
    /// use asserting::prelude::*;
    /// use hashbrown::HashMap;
    ///
    /// let subject: HashMap<_, _> = [(4, "four"), (1, "one"), (5, "five"), (8, "eight")].into();
    ///
    /// assert_that!(&subject).does_not_contain_values(["six", "seven"]);
    /// assert_that!(&subject).does_not_contain_values(["three", "two", "six"]);
    /// assert_that!(&subject).does_not_contain_values(["seven", "two", "seven"]);
    /// ```
    ///
    /// ```
    /// # #[cfg(not(feature = "std"))]
    /// # fn main() {}
    /// # #[cfg(feature = "std")]
    /// # fn main() {
    /// use asserting::prelude::*;
    /// use std::collections::BTreeMap;
    ///
    /// let subject: BTreeMap<_, _> = [(4, "four"), (1, "one"), (5, "five"), (8, "eight")].into();
    ///
    /// assert_that!(&subject).does_not_contain_values(["two", "three", "six", "seven"]);
    /// assert_that!(&subject).does_not_contain_values(["seven", "three", "six"]);
    /// assert_that!(&subject).does_not_contain_values(["two", "two", "nine"]);
    /// # }
    /// ```
    #[track_caller]
    fn does_not_contain_values(self, expected_values: impl IntoIterator<Item = E>) -> Self;
}
