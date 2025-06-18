use crate::expectations::{
    IsBetween, IsEmpty, IsGreaterThan, IsLessThan, IsNegative, IsOne, IsPositive, IsZero,
    StringContains, StringContainsAnyOf,
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

    assert_that(subject).expecting(all((IsPositive, not(IsZero))));
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
        not(IsZero),
        IsBetween { min: -43, max: -42 },
    )));
}

#[test]
fn verify_all_combinator_asserts_3_expectations_fails() {
    let subject = -42;

    let failures = verify_that(subject)
        .expecting(all((
            IsPositive,
            not(IsZero),
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
        not(IsZero),
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
            not(IsZero),
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

#[test]
fn any_combinator_asserts_1_expectations() {
    let subject = "nulla elit fugiat reprehenderit";

    assert_that(subject).expecting(any((StringContainsAnyOf {
        expected: ['a', 'b', 'c'],
    },)));
}

#[test]
fn verify_any_combinator_asserts_1_expectations_fails() {
    let subject = "nulla elit fugiat reprehenderit";

    let failures = verify_that(subject)
        .expecting(any((StringContains {
            expected: "fugiaty",
        },)))
        .display_failures();

    assert_eq!(
        failures,
        &[
            "assertion failed: expected subject to contain \"fugiaty\"\n   \
                but was: \"nulla elit fugiat reprehenderit\"\n  \
               expected: \"fugiaty\"\n\
           \n"
        ]
    );
}

#[test]
fn any_combinator_asserts_2_expectations() {
    let subject = "nulla elit fugiat reprehenderit";

    assert_that(subject).expecting(any((
        IsEmpty,
        StringContainsAnyOf {
            expected: ['a', 'b', 'c'],
        },
    )));
}

#[test]
fn verify_any_combinator_asserts_2_expectations_fails() {
    let subject = "nulla elit fugiat reprehenderit";

    let failures = verify_that(subject)
        .expecting(any((
            StringContains {
                expected: "fugiaty",
            },
            StringContains { expected: "ellit" },
        )))
        .display_failures();

    assert_eq!(
        failures,
        &[
            "assertion failed: expected subject to contain \"fugiaty\"\n   \
                but was: \"nulla elit fugiat reprehenderit\"\n  \
               expected: \"fugiaty\"\n\
             expected subject to contain \"ellit\"\n   \
                but was: \"nulla elit fugiat reprehenderit\"\n  \
               expected: \"ellit\"\n\
           \n"
        ]
    );
}

#[test]
fn any_combinator_asserts_3_expectations() {
    let subject = "nulla elit fugiat reprehenderit";

    assert_that(subject).expecting(any((
        IsEmpty,
        StringContainsAnyOf {
            expected: ['a', 'b', 'c'],
        },
        StringContains {
            expected: "unfugiaty",
        },
    )));
}

#[test]
fn verify_any_combinator_asserts_3_expectations_fails() {
    let subject = "nulla elit fugiat reprehenderit";

    let failures = verify_that(subject)
        .expecting(any((
            StringContains {
                expected: "fugiaty",
            },
            StringContains { expected: "ellit" },
            StringContainsAnyOf {
                expected: ['x', 'y', 'z'],
            },
        )))
        .display_failures();

    assert_eq!(
        failures,
        &[
            "assertion failed: expected subject to contain \"fugiaty\"\n   \
                but was: \"nulla elit fugiat reprehenderit\"\n  \
               expected: \"fugiaty\"\n\
             expected subject to contain \"ellit\"\n   \
                but was: \"nulla elit fugiat reprehenderit\"\n  \
               expected: \"ellit\"\n\
             expected subject to contain any of ['x', 'y', 'z']\n   \
                but was: \"nulla elit fugiat reprehenderit\"\n  \
               expected: ['x', 'y', 'z']\n\
           \n"
        ]
    );
}

#[test]
fn any_combinator_asserts_4_expectations() {
    let subject = "nulla elit fugiat reprehenderit";

    assert_that(subject).expecting(any((
        not(IsEmpty),
        StringContainsAnyOf {
            expected: ['a', 'b', 'c'],
        },
        StringContains {
            expected: "unfugiaty",
        },
        StringContains { expected: "elit" },
    )));
}

#[test]
fn any_combinator_asserts_5_expectations() {
    let subject = "nulla elit fugiat reprehenderit";

    assert_that(subject).expecting(any((
        not(IsEmpty),
        StringContainsAnyOf {
            expected: ['a', 'b', 'c'],
        },
        StringContains {
            expected: "unfugiaty",
        },
        StringContains { expected: "elit" },
        StringContains { expected: 'd' },
    )));
}

#[test]
fn any_combinator_asserts_6_expectations() {
    let subject = "nulla elit fugiat reprehenderit";

    assert_that(subject).expecting(any((
        not(IsEmpty),
        StringContainsAnyOf {
            expected: ['a', 'b', 'c'],
        },
        StringContains {
            expected: "unfugiaty",
        },
        StringContains { expected: "elit" },
        StringContains { expected: 'd' },
        StringContains { expected: 'e' },
    )));
}

#[test]
fn any_combinator_asserts_7_expectations() {
    let subject = "nulla elit fugiat reprehenderit";

    assert_that(subject).expecting(any((
        not(IsEmpty),
        StringContainsAnyOf {
            expected: ['a', 'b', 'c'],
        },
        StringContains {
            expected: "unfugiaty",
        },
        StringContains { expected: "elit" },
        StringContains { expected: 'd' },
        StringContains { expected: 'e' },
        StringContains { expected: 'f' },
    )));
}

#[test]
fn any_combinator_asserts_8_expectations() {
    let subject = "nulla elit fugiat reprehenderit";

    assert_that(subject).expecting(any((
        not(IsEmpty),
        StringContainsAnyOf {
            expected: ['a', 'b', 'c'],
        },
        StringContains {
            expected: "unfugiaty",
        },
        StringContains { expected: "elit" },
        StringContains { expected: 'd' },
        StringContains { expected: 'e' },
        StringContains { expected: 'f' },
        StringContains { expected: 'g' },
    )));
}

#[test]
fn any_combinator_asserts_9_expectations() {
    let subject = "nulla elit fugiat reprehenderit";

    assert_that(subject).expecting(any((
        not(IsEmpty),
        StringContainsAnyOf {
            expected: ['a', 'b', 'c'],
        },
        StringContains {
            expected: "unfugiaty",
        },
        StringContains { expected: "elit" },
        StringContains { expected: 'd' },
        StringContains { expected: 'e' },
        StringContains { expected: 'f' },
        StringContains { expected: 'g' },
        StringContains { expected: 'h' },
    )));
}

#[test]
fn any_combinator_asserts_10_expectations() {
    let subject = "nulla elit fugiat reprehenderit";

    assert_that(subject).expecting(any((
        not(IsEmpty),
        StringContainsAnyOf {
            expected: ['a', 'b', 'c'],
        },
        StringContains {
            expected: "unfugiaty",
        },
        StringContains { expected: "elit" },
        StringContains { expected: 'd' },
        StringContains { expected: 'e' },
        StringContains { expected: 'f' },
        StringContains { expected: 'g' },
        StringContains { expected: 'h' },
        StringContains { expected: 'i' },
    )));
}

#[test]
fn any_combinator_asserts_11_expectations() {
    let subject = "nulla elit fugiat reprehenderit";

    assert_that(subject).expecting(any((
        not(IsEmpty),
        StringContainsAnyOf {
            expected: ['a', 'b', 'c'],
        },
        StringContains {
            expected: "unfugiaty",
        },
        StringContains { expected: "elit" },
        StringContains { expected: 'd' },
        StringContains { expected: 'e' },
        StringContains { expected: 'f' },
        StringContains { expected: 'g' },
        StringContains { expected: 'h' },
        StringContains { expected: 'i' },
        StringContains { expected: 'j' },
    )));
}

#[test]
fn any_combinator_asserts_12_expectations() {
    let subject = "nulla elit fugiat reprehenderit";

    assert_that(subject).expecting(any((
        not(IsEmpty),
        StringContainsAnyOf {
            expected: ['a', 'b', 'c'],
        },
        StringContains {
            expected: "unfugiaty",
        },
        StringContains { expected: "elit" },
        StringContains { expected: 'd' },
        StringContains { expected: 'e' },
        StringContains { expected: 'f' },
        StringContains { expected: 'g' },
        StringContains { expected: 'h' },
        StringContains { expected: 'i' },
        StringContains { expected: 'j' },
        StringContains { expected: 'k' },
    )));
}
