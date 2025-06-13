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
            r"assertion failed: expected subject to be equal to BigDecimal(sign=Minus, scale=3, digits=[42831])
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
            r"assertion failed: expected subject to be equal to BigDecimalRef { sign: Minus, digits: 42831, scale: 3 }
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

#[test]
fn bigdecimal_has_precision_of() {
    let subject = BigDecimal::new(BigInt::from(420_831), 4);

    assert_that(subject).has_precision_of(6);
}

#[test]
fn bigdecimal_has_precision_of_trailing_zeros() {
    let subject = BigDecimal::new(BigInt::from(420_831_000), 7);

    assert_that(&subject).has_precision_of(9);

    assert_that(subject.normalized()).has_precision_of(6);
}

#[test]
fn verify_bigdecimal_has_precision_of_fails() {
    let subject = BigDecimal::new(BigInt::from(420_831_000), 7);

    let failures = verify_that(subject).has_precision_of(7).display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected subject to have a precision of 7
   but was: 9
  expected: 7
"
        ]
    );
}

#[test]
fn bigdecimal_has_scale_of() {
    let subject = BigDecimal::new(BigInt::from(420_831), 4);

    assert_that(subject).has_scale_of(4);
}

#[test]
fn bigdecimal_has_scale_of_with_zero_in_fraction() {
    let subject = BigDecimal::new(BigInt::from(420_830), 1);

    assert_that(&subject).has_scale_of(1);

    assert_that(subject.normalized()).has_scale_of(0);
}

#[test]
fn verify_bigdecimal_has_scale_of_fails() {
    let subject = BigDecimal::new(BigInt::from(420_831_000), 5);

    let failures = verify_that(subject.normalized())
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
fn bigdecimal_has_scale_of_trailing_zeros() {
    let subject = BigDecimal::new(BigInt::from(420_831_000), 4);

    assert_that(subject).has_scale_of(4);
}

#[test]
fn bigdecimal_is_integer() {
    let subject = BigDecimal::new(BigInt::from(420_830), 0);

    assert_that(subject).is_integer();
}

#[test]
fn bigdecimal_is_integer_zero_in_fraction() {
    let subject = BigDecimal::new(BigInt::from(420_830), 1);

    assert_that(subject).is_integer();
}

#[test]
fn verify_bigdecimal_is_integer_fails() {
    let subject = BigDecimal::new(BigInt::from(420_810), 2);

    let failures = verify_that(subject).is_integer().display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected subject to be an integer value
   but was: BigDecimal(sign=Plus, scale=2, digits=[420810])
  expected: an integer value
"]
    );
}

#[test]
fn borrowed_bigdecimal_has_precision_of() {
    let subject = BigDecimal::new(BigInt::from(420_831), 4);

    assert_that(&subject).has_precision_of(6);
}

#[test]
fn borrowed_bigdecimal_has_scale_of() {
    let subject = BigDecimal::new(BigInt::from(420_831), 4);

    assert_that(&subject).has_scale_of(4);
}

#[test]
fn borrowed_bigdecimal_is_integer() {
    let subject = BigDecimal::new(BigInt::from(420_830), 0);

    assert_that(&subject).is_integer();
}

#[test]
fn mutable_borrowed_bigdecimal_has_precision_of() {
    let mut subject = BigDecimal::new(BigInt::from(420_831), 4);

    assert_that(&mut subject).has_precision_of(6);
}

#[test]
fn mutable_borrowed_bigdecimal_has_scale_of() {
    let mut subject = BigDecimal::new(BigInt::from(420_831), 4);

    assert_that(&mut subject).has_scale_of(4);
}

#[test]
fn mutable_borrowed_bigdecimal_is_integer() {
    let mut subject = BigDecimal::new(BigInt::from(420_830), 0);

    assert_that(&mut subject).is_integer();
}
