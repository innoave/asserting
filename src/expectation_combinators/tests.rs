use crate::expectations::{
    IsBetween, IsGreaterThan, IsLessThan, IsNegative, IsOne, IsPositive, IsZero,
};
use crate::prelude::*;
use crate::spec::{Expectation, Expression};

#[test]
fn newly_created_rec_combinator_is_neither_success_nor_failure() {
    let rec = rec(IsZero);

    assert_that(rec.is_success()).is_false();
    assert_that(rec.is_failure()).is_false();
}

#[test]
fn rec_combinator_is_success_after_test_method_has_been_called() {
    let mut rec = rec(IsZero);

    rec.test(&0);

    assert_that(rec.is_success()).is_true();
}

#[test]
fn rec_combinator_is_failure_after_test_method_has_been_called() {
    let mut rec = rec(IsNegative);

    rec.test(&1);

    assert_that(rec.is_failure()).is_true();
}

#[test]
fn rec_combinator_returns_empty_message_if_test_is_successful() {
    let mut rec = rec(IsGreaterThan { expected: 10 });

    rec.test(&12);
    let message = rec.message(
        &Expression::from("foo"),
        &12,
        false,
        &DIFF_FORMAT_NO_HIGHLIGHT,
    );

    assert_that(message).is_empty();
}

#[test]
fn rec_combinator_returns_failure_message_if_test_is_failure() {
    let mut rec = rec(IsOne);

    rec.test(&12);
    let message = rec.message(
        &Expression::from("foo"),
        &12,
        false,
        &DIFF_FORMAT_NO_HIGHLIGHT,
    );

    assert_that(message).is_equal_to("expected foo to be one\n   but was: 12\n  expected: 1\n");
}

#[test]
fn all_combinator_asserts_2_expectations() {
    let subject = 42;

    assert_that(subject).expecting(all((IsPositive, Not(IsZero))));
}

#[test]
fn verify_all_combinator_asserts_2_expectations_fails() {
    let subject = 42;

    let failures = verify_that(subject)
        .expecting(all((IsNegative, IsZero)))
        .display_failures();

    assert_eq!(
        failures,
        &["assertion failed: expected subject to be negative\n   \
                but was: 42\n  \
               expected: < 0\n\
             expected subject to be zero\n   \
                but was: 42\n  \
               expected: 0\n\
           \n"]
    );
}

#[test]
fn all_combinator_asserts_3_expectations() {
    let subject = -42;

    assert_that(subject).expecting(all((
        IsNegative,
        Not(IsZero),
        IsBetween { min: -43, max: -42 },
    )));
}

#[test]
fn verify_all_combinator_asserts_3_expectations_fails() {
    let subject = -42;

    let failures = verify_that(subject)
        .expecting(all((
            IsPositive,
            Not(IsZero),
            IsBetween { min: 41, max: 43 },
        )))
        .display_failures();

    assert_eq!(
        failures,
        &["assertion failed: expected subject to be positive\n   \
                but was: -42\n  \
               expected: > 0\n\
             expected subject to be between 41 and 43\n   \
                but was: -42\n  \
               expected: 41 <= x <= 43\n\
           \n"]
    );
}

#[test]
fn all_combinator_asserts_4_expectations() {
    let subject = -42;

    assert_that(subject).expecting(all((
        IsNegative,
        Not(IsZero),
        IsLessThan { expected: 2 },
        IsBetween { min: -43, max: -42 },
    )));
}

#[test]
fn verify_all_combinator_asserts_4_expectations_fails() {
    let subject = -42;

    let failures = verify_that(subject)
        .expecting(all((
            IsPositive,
            Not(IsZero),
            IsGreaterThan { expected: 2 },
            IsBetween { min: 41, max: 43 },
        )))
        .display_failures();

    assert_eq!(
        failures,
        &["assertion failed: expected subject to be positive\n   \
                but was: -42\n  \
               expected: > 0\n\
             expected subject to be greater than 2\n   \
                but was: -42\n  \
               expected: > 2\n\
             expected subject to be between 41 and 43\n   \
                but was: -42\n  \
               expected: 41 <= x <= 43\n\
           \n"]
    );
}

#[test]
fn all_combinator_asserts_5_expectations() {
    let subject = 0;

    assert_that(subject).expecting(all((IsZero, IsZero, IsZero, IsZero, IsZero)));
}

#[test]
fn all_combinator_asserts_6_expectations() {
    let subject = 0;

    assert_that(subject).expecting(all((IsZero, IsZero, IsZero, IsZero, IsZero, IsZero)));
}

#[test]
fn all_combinator_asserts_7_expectations() {
    let subject = 0;

    assert_that(subject).expecting(all((
        IsZero, IsZero, IsZero, IsZero, IsZero, IsZero, IsZero,
    )));
}

#[test]
fn all_combinator_asserts_8_expectations() {
    let subject = 0;

    assert_that(subject).expecting(all((
        IsZero, IsZero, IsZero, IsZero, IsZero, IsZero, IsZero, IsZero,
    )));
}

#[test]
fn all_combinator_asserts_9_expectations() {
    let subject = 0;

    assert_that(subject).expecting(all((
        IsZero, IsZero, IsZero, IsZero, IsZero, IsZero, IsZero, IsZero, IsZero,
    )));
}

#[test]
fn all_combinator_asserts_10_expectations() {
    let subject = 0;

    assert_that(subject).expecting(all((
        IsZero, IsZero, IsZero, IsZero, IsZero, IsZero, IsZero, IsZero, IsZero, IsZero,
    )));
}

#[test]
fn all_combinator_asserts_11_expectations() {
    let subject = 0;

    assert_that(subject).expecting(all((
        IsZero, IsZero, IsZero, IsZero, IsZero, IsZero, IsZero, IsZero, IsZero, IsZero, IsZero,
    )));
}

#[test]
fn all_combinator_asserts_12_expectations() {
    let subject = 0;

    assert_that(subject).expecting(all((
        IsZero, IsZero, IsZero, IsZero, IsZero, IsZero, IsZero, IsZero, IsZero, IsZero, IsZero,
        IsZero,
    )));
}
