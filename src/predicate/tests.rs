use crate::prelude::*;

#[test]
fn assert_that_subject_satisfies_predicate() {
    let subject = 42;

    assert_that(subject)
        .named("my_thing")
        .satisfies(|actual| *actual == 42);
}

#[test]
fn verify_that_subject_satisfies_predicate_fails() {
    let subject = 51;

    let failures = verify_that(subject)
        .named("my_thing")
        .satisfies(|actual| *actual == 42)
        .display_failures();

    assert_eq!(
        failures,
        &["assertion failed: expected my_thing to satisfy the given predicate, but returned false\n"]
    );
}

#[test]
fn verify_that_subject_satisfies_predicate_fails_with_custom_message() {
    let subject = 51;

    let failures = verify_that(subject)
        .named("my_thing")
        .satisfies_with_message("the answer to all important questions is 42", |actual| {
            *actual == 42
        })
        .display_failures();

    assert_eq!(
        failures,
        &["assertion failed: the answer to all important questions is 42\n"]
    );
}
