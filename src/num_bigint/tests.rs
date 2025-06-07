use crate::prelude::*;
use num_bigint::BigInt;

#[test]
fn bigint_is_equal_to_other() {
    let subject = BigInt::from(42);

    assert_that(subject).is_equal_to(BigInt::from(42));
}

#[test]
fn verify_bigint_is_equal_to_other_fails() {
    let subject = BigInt::from(42);

    let failures = verify_that(subject)
        .is_equal_to(BigInt::from(-42))
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected subject is equal to -42
   but was: 42
  expected: -42
"]
    );
}

#[test]
fn bigint_is_not_equal_to_other() {
    let subject = BigInt::from(42);

    assert_that(subject).is_not_equal_to(BigInt::from(0));
}

#[test]
fn borrowed_bigint_is_equal_to_bigint() {
    let subject = BigInt::from(42);

    assert_that(&subject).is_equal_to(&BigInt::from(42));
}

#[test]
fn bigint_is_less_than_other() {
    let subject = BigInt::from(42);

    assert_that(&subject).is_less_than(&BigInt::from(92_834));
    assert_that(subject).is_less_than(BigInt::from(43));
}

#[test]
fn bigint_is_greater_than_other() {
    let subject = BigInt::from(42);

    assert_that(&subject).is_greater_than(&BigInt::from(-232_199));
    assert_that(subject).is_greater_than(BigInt::from(41));
}

#[test]
fn bigint_is_at_least_other() {
    let subject = BigInt::from(42);

    assert_that(&subject).is_at_least(&BigInt::from(42));
    assert_that(&subject).is_at_least(&BigInt::from(41));
    assert_that(subject).is_at_least(BigInt::from(-33));
}

#[test]
fn bigint_is_at_most_other() {
    let subject = BigInt::from(42);

    assert_that(&subject).is_at_most(&BigInt::from(42));
    assert_that(&subject).is_at_most(&BigInt::from(43));
    assert_that(subject).is_at_most(BigInt::from(1_587_929));
}

#[test]
fn bigint_is_negative() {
    let subject = BigInt::from(-42);

    assert_that(&subject).is_negative();
}

#[test]
fn bigint_is_not_negative() {
    assert_that(&BigInt::from(42)).is_not_negative();
    assert_that(BigInt::from(0)).is_not_negative();
}

#[test]
fn bigint_is_positive() {
    let subject = BigInt::from(42);

    assert_that(&subject).is_positive();
}

#[test]
fn bigint_is_not_positive() {
    assert_that(&BigInt::from(-42)).is_not_positive();
    assert_that(BigInt::from(0)).is_not_positive();
}

#[test]
fn borrowed_bigint_is_negative() {
    assert_that(&BigInt::from(-42)).is_negative();
}

#[test]
fn borrowed_bigint_is_positive() {
    assert_that(&BigInt::from(42)).is_positive();
}

#[test]
fn mutable_borrowed_bigint_is_negative() {
    assert_that(&mut BigInt::from(-42)).is_negative();
}

#[test]
fn mutable_borrowed_bigint_is_positive() {
    assert_that(&mut BigInt::from(42)).is_positive();
}

#[test]
fn bigint_is_zero() {
    assert_that(BigInt::from(0)).is_zero();
}

#[test]
fn bigint_is_one() {
    assert_that(BigInt::from(1)).is_one();
}

#[test]
fn borrowed_bigint_is_zero() {
    assert_that(&BigInt::from(0)).is_zero();
}

#[test]
fn borrowed_bigint_is_one() {
    assert_that(&BigInt::from(1)).is_one();
}
