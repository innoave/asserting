use crate::prelude::*;

//
// Is in range for `i32`
//

#[test]
fn i32_is_in_range() {
    let subject = 42;

    assert_that(subject).is_in_range(41..=43);
}

#[test]
fn verify_i32_is_in_range_fails() {
    let subject = 42;

    let failures = verify_that(subject)
        .named("my_thing")
        .is_in_range(43..=51)
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing is within range of 43..=51
   but was: 42
  expected: 43 <= x <= 51
"
        ]
    );
}

#[test]
fn i32_is_not_in_range() {
    let subject = 42;

    assert_that(subject).is_not_in_range(39..=41);
}

#[test]
fn verify_i32_is_not_in_range_fails() {
    let subject = 42;

    let failures = verify_that(subject)
        .named("my_thing")
        .is_not_in_range(41..=42)
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing is not within range of 41..=42
   but was: 42
  expected: x < 41 || x > 42
"
        ]
    );
}

//
// Is in range for `char`
//

#[test]
fn char_is_in_range() {
    let subject = 'K';

    assert_that(subject).is_in_range('J'..='L');
}

#[test]
fn verify_char_is_in_range_fails() {
    let subject = 'K';

    let failures = verify_that(subject)
        .named("my_thing")
        .is_in_range('L'..='Z')
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing is within range of 'L'..='Z'
   but was: 'K'
  expected: 'L' <= x <= 'Z'
"
        ]
    );
}

#[test]
fn char_is_not_in_range() {
    let subject = 'K';

    assert_that(subject).is_not_in_range('A'..='J');
}

#[test]
fn verify_char_is_not_in_range_fails() {
    let subject = 'K';

    let failures = verify_that(subject)
        .named("my_thing")
        .is_not_in_range('J'..='K')
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing is not within range of 'J'..='K'
   but was: 'K'
  expected: x < 'J' || x > 'K'
"
        ]
    );
}
