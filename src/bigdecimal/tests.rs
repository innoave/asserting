use crate::prelude::*;
use bigdecimal::num_bigint::BigInt;
use bigdecimal::BigDecimal;

#[test]
fn bigdecimal_is_equal_to_other() {
    let subject = BigDecimal::new(BigInt::from(42_831), 3);

    assert_that(subject).is_equal_to(BigDecimal::new(BigInt::from(42_831), 3));

    assert_that(BigDecimal::new(BigInt::from(42_831), 3))
        .is_equal_to(BigDecimal::new(BigInt::from(428_310), 4));
    assert_that(BigDecimal::new(BigInt::from(0), 0))
        .is_equal_to(BigDecimal::new(BigInt::from(0), 2));
    assert_that(BigDecimal::new(BigInt::from(-0), 0))
        .is_equal_to(BigDecimal::new(BigInt::from(0), 0));
}

#[test]
fn verify_bigdecimal_is_equal_to_other_fails() {
    let subject = BigDecimal::new(BigInt::from(42_831), 3);

    let failures = verify_that(subject)
        .is_equal_to(BigDecimal::new(BigInt::from(-42_831), 3))
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected subject is equal to BigDecimal(sign=Minus, scale=3, digits=[42831])
   but was: BigDecimal(sign=Plus, scale=3, digits=[42831])
  expected: BigDecimal(sign=Minus, scale=3, digits=[42831])
"
        ]
    );
}

#[test]
fn bigdecimal_is_not_equal_to_other() {
    let subject = BigDecimal::new(BigInt::from(42_831), 3);

    assert_that(subject).is_not_equal_to(BigDecimal::new(BigInt::from(42_831), 2));
}

#[test]
fn borrowed_bigdecimal_is_equal_to_other() {
    let subject = BigDecimal::new(BigInt::from(-42_831), 3);

    assert_that(&subject).is_equal_to(&BigDecimal::new(BigInt::from(-42_831), 3));
}

#[test]
fn bigdecimal_is_less_than_other() {
    let subject = BigDecimal::new(BigInt::from(42_831), 3);

    assert_that(&subject).is_less_than(&BigDecimal::new(BigInt::from(1_592_834), 3));
    assert_that(subject).is_less_than(BigDecimal::new(BigInt::from(42_832), 3));
}

#[test]
fn bigdecimal_is_greater_than_other() {
    let subject = BigDecimal::new(BigInt::from(42_831), 3);

    assert_that(&subject).is_greater_than(&BigDecimal::new(BigInt::from(-232_199), 3));
    assert_that(subject).is_greater_than(BigDecimal::new(BigInt::from(42_830), 3));
}

#[test]
fn bigdecimal_is_at_least_other() {
    let subject = BigDecimal::new(BigInt::from(42_831), 3);

    assert_that(&subject).is_at_least(&BigDecimal::new(BigInt::from(42_831), 3));
    assert_that(&subject).is_at_least(&BigDecimal::new(BigInt::from(42_830), 3));
    assert_that(subject).is_at_least(BigDecimal::new(BigInt::from(-332), 3));
}

#[test]
fn bigdecimal_is_at_most_other() {
    let subject = BigDecimal::new(BigInt::from(42_831), 3);

    assert_that(&subject).is_at_most(&BigDecimal::new(BigInt::from(42_831), 3));
    assert_that(&subject).is_at_most(&BigDecimal::new(BigInt::from(42_832), 3));
    assert_that(subject).is_at_most(BigDecimal::new(BigInt::from(65_587_929), 3));
}

#[test]
fn bigdecimal_is_negative() {
    let subject = BigDecimal::new(BigInt::from(-42_831), 3);

    assert_that(&subject).is_negative();
}

#[test]
fn bigdecimal_is_not_negative() {
    assert_that(&BigDecimal::new(BigInt::from(42_831), 3)).is_not_negative();
    assert_that(BigDecimal::new(BigInt::from(0), 0)).is_not_negative();
}

#[test]
fn bigdecimal_is_positive() {
    let subject = BigDecimal::new(BigInt::from(42_831), 3);

    assert_that(&subject).is_positive();
}

#[test]
fn bigdecimal_is_not_positive() {
    assert_that(&BigDecimal::new(BigInt::from(-42_831), 3)).is_not_positive();
    assert_that(BigDecimal::new(BigInt::from(0), 0)).is_not_positive();
}

#[test]
fn bigdecimal_signum_of_zero() {
    assert_that(BigDecimal::new(BigInt::from(0), 0)).is_zero();
}

#[test]
fn borrowed_bigdecimal_is_negative() {
    assert_that(&BigDecimal::new(BigInt::from(-42_831), 3)).is_negative();
}

#[test]
fn borrowed_bigdecimal_is_positive() {
    assert_that(&BigDecimal::new(BigInt::from(42_831), 3)).is_positive();
}

#[test]
fn mutable_borrowed_bigdecimal_is_negative() {
    assert_that(&mut BigDecimal::new(BigInt::from(-42_831), 3)).is_negative();
}

#[test]
fn mutable_borrowed_bigdecimal_is_positive() {
    assert_that(&mut BigDecimal::new(BigInt::from(42_831), 3)).is_positive();
}

#[test]
fn bigdecimal_is_zero() {
    assert_that(BigDecimal::new(BigInt::from(0), 0)).is_zero();
    assert_that(BigDecimal::new(BigInt::from(-0), 0)).is_zero();
    assert_that(BigDecimal::new(BigInt::from(0), 2)).is_zero();
}

#[test]
fn bigdecimal_is_one() {
    assert_that(BigDecimal::new(BigInt::from(1), 0)).is_one();
}

#[test]
fn borrowed_bigdecimal_is_zero() {
    assert_that(&BigDecimal::new(BigInt::from(0), 0)).is_zero();
}

#[test]
fn borrowed_bigdecimal_is_one() {
    assert_that(&BigDecimal::new(BigInt::from(1), 0)).is_one();
}

#[test]
fn bigdecimalref_is_equal_to_other() {
    let subject = BigDecimal::new(BigInt::from(42_831), 3);

    assert_that(subject.to_ref()).is_equal_to(BigDecimal::new(BigInt::from(42_831), 3).to_ref());

    assert_that(BigDecimal::new(BigInt::from(42_831), 3).to_ref())
        .is_equal_to(BigDecimal::new(BigInt::from(428_310), 4).to_ref());
    assert_that(BigDecimal::new(BigInt::from(0), 0).to_ref())
        .is_equal_to(BigDecimal::new(BigInt::from(0), 2).to_ref());
    assert_that(BigDecimal::new(BigInt::from(-0), 0).to_ref())
        .is_equal_to(BigDecimal::new(BigInt::from(0), 0).to_ref());
}

#[test]
fn verify_bigdecimalref_is_equal_to_other_fails() {
    let subject = BigDecimal::new(BigInt::from(42_831), 3);

    let failures = verify_that(subject.to_ref())
        .is_equal_to(BigDecimal::new(BigInt::from(-42_831), 3).to_ref())
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected subject is equal to BigDecimalRef { sign: Minus, digits: 42831, scale: 3 }
   but was: BigDecimalRef { sign: Plus, digits: 42831, scale: 3 }
  expected: BigDecimalRef { sign: Minus, digits: 42831, scale: 3 }
"
        ]
    );
}

#[test]
fn bigdecimalref_is_not_equal_to_other() {
    let subject = BigDecimal::new(BigInt::from(42_831), 3);

    assert_that(subject.to_ref())
        .is_not_equal_to(BigDecimal::new(BigInt::from(42_831), 2).to_ref());
}

#[test]
fn bigdecimalref_is_negative() {
    let subject = BigDecimal::new(BigInt::from(-42_831), 3);

    assert_that(subject.to_ref()).is_negative();
}

#[test]
fn bigdecimalref_is_not_negative() {
    assert_that(BigDecimal::new(BigInt::from(42_831), 3).to_ref()).is_not_negative();
    assert_that(BigDecimal::new(BigInt::from(0), 0).to_ref()).is_not_negative();
}

#[test]
fn bigdecimalref_is_positive() {
    let subject = BigDecimal::new(BigInt::from(42_831), 3);

    assert_that(subject.to_ref()).is_positive();
}

#[test]
fn bigdecimalref_is_not_positive() {
    assert_that(BigDecimal::new(BigInt::from(-42_831), 3).to_ref()).is_not_positive();
    assert_that(BigDecimal::new(BigInt::from(0), 0).to_ref()).is_not_positive();
}

#[test]
fn bigdecimalref_signum_of_zero() {
    assert_that(BigDecimal::new(BigInt::from(0), 0).to_ref()).is_zero();
}

#[test]
fn bigdecimalref_is_zero() {
    assert_that(BigDecimal::new(BigInt::from(0), 0).to_ref()).is_zero();
    assert_that(BigDecimal::new(BigInt::from(-0), 0).to_ref()).is_zero();
    assert_that(BigDecimal::new(BigInt::from(0), 2).to_ref()).is_zero();
}

#[test]
fn bigdecimalref_is_one() {
    assert_that(BigDecimal::new(BigInt::from(1), 0).to_ref()).is_one();
}
