use crate::prelude::*;
use rust_decimal::Decimal;

#[test]
fn decimal_is_equal_to_other() {
    let subject = Decimal::new(42_831, 3);

    assert_that(subject).is_equal_to(Decimal::new(42_831, 3));

    assert_that(Decimal::new(42_831, 3)).is_equal_to(Decimal::new(428_310, 4));
    assert_that(Decimal::new(0, 0)).is_equal_to(Decimal::new(0, 2));
    assert_that(Decimal::new(-0, 0)).is_equal_to(Decimal::new(0, 0));
}

#[test]
fn verify_decimal_is_equal_to_other_fails() {
    let subject = Decimal::new(42_831, 3);

    let failures = verify_that(subject)
        .is_equal_to(Decimal::new(-42_831, 3))
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected subject is equal to -42.831
   but was: 42.831
  expected: -42.831
"]
    );
}

#[test]
fn decimal_is_not_equal_to_other() {
    let subject = Decimal::new(42_831, 3);

    assert_that(subject).is_not_equal_to(Decimal::new(42_831, 2));
}

#[test]
fn borrowed_decimal_is_equal_to_other() {
    let subject = Decimal::new(-42_831, 3);

    assert_that(&subject).is_equal_to(&Decimal::new(-42_831, 3));
}

#[test]
fn decimal_is_less_than_other() {
    let subject = Decimal::new(42_831, 3);

    assert_that(&subject).is_less_than(&Decimal::new(1_592_834, 3));
    assert_that(subject).is_less_than(Decimal::new(42_832, 3));
}

#[test]
fn decimal_is_greater_than_other() {
    let subject = Decimal::new(42_831, 3);

    assert_that(&subject).is_greater_than(&Decimal::new(-232_199, 3));
    assert_that(subject).is_greater_than(Decimal::new(42_830, 3));
}

#[test]
fn decimal_is_at_least_other() {
    let subject = Decimal::new(42_831, 3);

    assert_that(&subject).is_at_least(&Decimal::new(42_831, 3));
    assert_that(&subject).is_at_least(&Decimal::new(42_830, 3));
    assert_that(subject).is_at_least(Decimal::new(-332, 3));
}

#[test]
fn decimal_is_at_most_other() {
    let subject = Decimal::new(42_831, 3);

    assert_that(&subject).is_at_most(&Decimal::new(42_831, 3));
    assert_that(&subject).is_at_most(&Decimal::new(42_832, 3));
    assert_that(subject).is_at_most(Decimal::new(65_587_929, 3));
}

#[test]
fn decimal_is_negative() {
    let subject = Decimal::new(-42_831, 3);

    assert_that(&subject).is_negative();
}

#[test]
fn decimal_is_not_negative() {
    assert_that(&Decimal::new(42_831, 3)).is_not_negative();
    assert_that(Decimal::new(0, 0)).is_not_negative();
}

#[test]
fn decimal_is_positive() {
    let subject = Decimal::new(42_831, 3);

    assert_that(&subject).is_positive();
}

#[test]
fn decimal_is_not_positive() {
    assert_that(&Decimal::new(-42_831, 3)).is_not_positive();
    assert_that(Decimal::new(0, 0)).is_not_positive();
}

#[test]
fn decimal_signum_of_zero() {
    assert_that(Decimal::new(0, 0)).is_zero();
    assert_that(Decimal::new(0, 0).is_sign_positive()).is_true();
    assert_that(Decimal::new(0, 0).is_sign_negative()).is_false();
    assert_that(Decimal::new(-0, 0).is_sign_negative()).is_false();
}

#[test]
fn borrowed_decimal_is_negative() {
    assert_that(&Decimal::new(-42_831, 3)).is_negative();
}

#[test]
fn borrowed_decimal_is_positive() {
    assert_that(&Decimal::new(42_831, 3)).is_positive();
}

#[test]
fn mutable_borrowed_decimal_is_negative() {
    assert_that(&mut Decimal::new(-42_831, 3)).is_negative();
}

#[test]
fn mutable_borrowed_decimal_is_positive() {
    assert_that(&mut Decimal::new(42_831, 3)).is_positive();
}

#[test]
fn decimal_is_zero() {
    assert_that(Decimal::new(0, 0)).is_zero();
    assert_that(Decimal::new(-0, 0)).is_zero();
    assert_that(Decimal::new(0, 2)).is_zero();
}

#[test]
fn decimal_is_one() {
    assert_that(Decimal::new(1, 0)).is_one();
}

#[test]
fn borrowed_decimal_is_zero() {
    assert_that(&Decimal::new(0, 0)).is_zero();
}

#[test]
fn borrowed_decimal_is_one() {
    assert_that(&Decimal::new(1, 0)).is_one();
}

#[test]
fn decimal_has_precision_of() {
    let subject = Decimal::new(420_831, 4);

    assert_that(subject).has_precision_of(29);
}

#[test]
fn verify_decimal_has_precision_of_fails() {
    let subject = Decimal::new(420_831_000, 7);

    let failures = verify_that(subject).has_precision_of(7).display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected subject to have a precision of 7
   but was: 29
  expected: 7
"
        ]
    );
}

#[test]
fn decimal_has_scale_of() {
    let subject = Decimal::new(420_831, 4);

    assert_that(subject).has_scale_of(4);
}

#[test]
fn decimal_has_scale_of_with_zero_in_fraction() {
    let subject = Decimal::new(420_830, 1);

    assert_that(subject).has_scale_of(1);

    assert_that(subject.normalize()).has_scale_of(0);
}

#[test]
fn decimal_has_scale_of_trailing_zeros() {
    let subject = Decimal::new(420_831_000, 4);

    assert_that(subject).has_scale_of(4);
}

#[test]
fn verify_decimal_has_scale_of_fails() {
    let subject = Decimal::new(420_831_000, 5);

    let failures = verify_that(subject.normalize())
        .has_scale_of(5)
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected subject to have a scale of 5
   but was: 2
  expected: 5
"]
    );
}

#[test]
fn decimal_is_integer() {
    let subject = Decimal::new(420_830, 0);

    assert_that(subject).is_integer();
}

#[test]
fn decimal_is_integer_zero_in_fraction() {
    let subject = Decimal::new(420_830, 1);

    assert_that(subject).is_integer();
}

#[test]
fn verify_decimal_is_integer_fails() {
    let subject = Decimal::new(420_810, 2);

    let failures = verify_that(subject).is_integer().display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected subject to be an integer value
   but was: 4208.10
  expected: an integer value
"]
    );
}
