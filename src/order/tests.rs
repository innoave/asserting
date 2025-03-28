use crate::prelude::*;

//
// comparing `i32`
//

#[test]
fn i32_is_less_than_other_i32() {
    let subject = 42;

    assert_that(subject).is_less_than(43);
}

#[test]
fn verify_i32_is_less_than_other_i32_fails() {
    let subject = 42;

    let failures = verify_that(subject)
        .named("my_thing")
        .is_less_than(42)
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing is less than 42
   but was: 42
  expected: < 42
"]
    );
}

#[test]
fn i32_is_greater_than_other_i32() {
    let subject = 42;

    assert_that(subject).is_greater_than(41);
}

#[test]
fn verify_i32_is_greater_than_other_i32_fails() {
    let subject = 42;

    let failures = verify_that(subject)
        .named("my_thing")
        .is_greater_than(42)
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing is greater than 42
   but was: 42
  expected: > 42
"]
    );
}

#[test]
fn i32_is_at_most_other_i32() {
    let subject = 42;

    assert_that(subject).is_at_most(42);
}

#[test]
fn verify_i32_is_at_most_other_i32_fails() {
    let subject = 42;

    let failures = verify_that(subject)
        .named("my_thing")
        .is_at_most(41)
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing is at most 41
   but was: 42
  expected: <= 41
"]
    );
}

#[test]
fn i32_is_at_least_other_i32() {
    let subject = 42;

    assert_that(subject).is_at_least(42);
}

#[test]
fn verify_i32_is_at_least_other_i32_fails() {
    let subject = 42;

    let failures = verify_that(subject)
        .named("my_thing")
        .is_at_least(43)
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing is at least 43
   but was: 42
  expected: >= 43
"]
    );
}

//
// comparing `char`
//

#[test]
fn char_is_less_than_other_char() {
    let subject = 'C';

    assert_that(subject).is_less_than('D');
}

#[test]
fn verify_char_is_less_than_other_char_fails() {
    let subject = 'C';

    let failures = verify_that(subject)
        .named("my_thing")
        .is_less_than('C')
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing is less than 'C'
   but was: 'C'
  expected: < 'C'
"]
    );
}

#[test]
fn char_is_greater_than_other_char() {
    let subject = 'D';

    assert_that(subject).is_greater_than('C');
}

#[test]
fn verify_char_is_greater_than_other_char_fails() {
    let subject = 'D';

    let failures = verify_that(subject)
        .named("my_thing")
        .is_greater_than('D')
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing is greater than 'D'
   but was: 'D'
  expected: > 'D'
"]
    );
}

#[test]
fn char_is_at_most_other_char() {
    let subject = 'C';

    assert_that(subject).is_at_most('C');
}

#[test]
fn verify_char_is_at_most_other_char_fails() {
    let subject = 'C';

    let failures = verify_that(subject)
        .named("my_thing")
        .is_at_most('B')
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing is at most 'B'
   but was: 'C'
  expected: <= 'B'
"]
    );
}

#[test]
fn char_is_at_least_other_char() {
    let subject = 'D';

    assert_that(subject).is_at_least('D');
}

#[test]
fn verify_char_is_at_least_other_char_fails() {
    let subject = 'D';

    let failures = verify_that(subject)
        .named("my_thing")
        .is_at_least('E')
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing is at least 'E'
   but was: 'D'
  expected: >= 'E'
"]
    );
}
