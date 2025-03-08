use crate::prelude::*;
use crate::std::slice;

#[test]
fn slice_is_equal_to_a_vec() {
    let subject: &[i32] = &[1, 3, 5, 7];

    assert_that(subject).is_equal_to(vec![1, 3, 5, 7]);
}

#[test]
fn slice_is_equal_to_another_slice() {
    let subject: &[i32] = &[1, 3, 5, 7];

    assert_that(subject).is_equal_to(&[1, 3, 5, 7]);
}

#[test]
fn verify_slice_is_equal_to_another_slice_fails() {
    let subject: &[i32] = &[1, 3, 5, 7];

    let failures = verify_that(subject)
        .named("my_thing")
        .is_equal_to(&[2, 4, 6, 8])
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing is equal to [2, 4, 6, 8]
   but was: [1, 3, 5, 7]
  expected: [2, 4, 6, 8]
"
        ]
    );
}

#[test]
fn slice_is_empty() {
    let subject: &[i32] = &[];

    assert_that(subject).is_empty();
}

#[test]
fn slice_is_not_empty() {
    let subject: &[i32] = &[1, 3, 5, 7, 11];

    assert_that(subject).is_not_empty();
}

#[test]
fn slice_has_length() {
    let subject: &[i32] = &[1, 3, 5, 7, 11];

    assert_that(subject).has_length(5);
}

#[test]
fn slice_has_length_in_range() {
    let subject: &[i32] = &[1, 3, 5, 7, 11];

    assert_that(subject).has_length_in_range(4..=6);
}

#[test]
fn slice_contains() {
    let subject: &[i32] = &[1, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43];

    assert_that(subject)
        .contains(&19)
        .contains(&43)
        .contains(&1);
}

#[test]
fn verify_slice_contains_fails() {
    let subject: &[i32] = &[1, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43];

    let failures = verify_that(subject)
        .named("my_thing")
        .contains(&21)
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing to contain 21
   but was: [1, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43]
  expected: 21
"]
    );
}

#[test]
fn slice_iterator_contains() {
    let subject: slice::Iter<'_, i32> = [1, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43].iter();

    assert_that(subject)
        .contains(&19)
        .contains(&43)
        .contains(&1);
}

#[test]
fn verify_slice_iterator_contains_fails() {
    let subject: slice::Iter<'_, i32> = [1, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43].iter();

    let failures = verify_that(subject)
        .named("my_thing")
        .contains(&21)
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing to contain 21
   but was: [1, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43]
  expected: 21
"]
    );
}

#[test]
fn slice_contains_exactly_in_any_order() {
    let subject: &[i32] = &[5, 7, 11, 13, 1, 19, 11, 3, 17, 23, 23, 29, 31, 41, 37, 43];

    assert_that(subject).contains_exactly_in_any_order(&[
        1, 3, 5, 7, 11, 11, 13, 17, 19, 23, 23, 29, 31, 37, 41, 43,
    ]);
}

#[test]
fn verify_slice_contains_exactly_in_any_order_fails() {
    let subject: &[i32] = &[5, 7, 11, 13, 1, 11, 3, 17, 23, 23, 29, 31, 41, 37, 43];

    let failures = verify_that(subject)
        .named("my_thing")
        .contains_exactly_in_any_order(&[0, 1, 3, 5, 7, 11, 11, 13, 17, 19, 23, 29, 31, 37, 41])
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing to contain exactly in any order [0, 1, 3, 5, 7, 11, 11, 13, 17, 19, 23, 29, 31, 37, 41]
   but was: [5, 7, 11, 13, 1, 11, 3, 17, 23, 23, 29, 31, 41, 37, 43]
  expected: [0, 1, 3, 5, 7, 11, 11, 13, 17, 19, 23, 29, 31, 37, 41]
   missing: [0, 19]
     extra: [23, 43]
"
        ]
    );
}

#[test]
fn slice_contains_any_of() {
    let subject: &[i32] = &[5, 7, 11, 13, 1, 19, 11, 3, 17, 23, 23, 29, 31, 41, 37, 43];

    assert_that(subject).contains_any_of(&[2, 4, 6, 8, 19, 45]);
}

#[test]
fn verify_slice_contains_any_of_fails() {
    let subject: &[i32] = &[5, 7, 11, 13, 1, 11, 3, 17, 23, 23, 29, 31, 41, 37, 43];

    let failures = verify_that(subject)
        .named("my_thing")
        .contains_any_of(&[0, 2, 4, 8, 16, 32, 64])
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing to contain any of [0, 2, 4, 8, 16, 32, 64], but contained none of them
   but was: [5, 7, 11, 13, 1, 11, 3, 17, 23, 23, 29, 31, 41, 37, 43]
  expected: [0, 2, 4, 8, 16, 32, 64]
"
        ]
    );
}

#[test]
fn slice_contains_all_of() {
    let subject: &[i32] = &[5, 7, 11, 13, 1, 19, 11, 3, 17, 23, 23, 29, 31, 41, 37, 43];

    assert_that(subject).contains_all_of(&[5, 3, 37, 19, 19, 17, 43]);
}

#[test]
fn verify_slice_contains_all_of_fails() {
    let subject: &[i32] = &[5, 7, 11, 13, 1, 11, 3, 17, 23, 23, 29, 31, 41, 37, 43];

    let failures = verify_that(subject)
        .named("my_thing")
        .contains_all_of(&[5, 18, 3, 17, 45, 1, 1, 29, 0])
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing to contain all of [5, 18, 3, 17, 45, 1, 1, 29, 0]
   but was: [5, 7, 11, 13, 1, 11, 3, 17, 23, 23, 29, 31, 41, 37, 43]
  expected: [5, 18, 3, 17, 45, 1, 1, 29, 0]
   missing: [18, 45, 0]
"
        ]
    );
}

#[test]
fn slice_contains_only() {
    let subject: &[i32] = &[5, 11, 1, 3, 19, 11, 43];

    assert_that(subject).contains_only(&[1, 3, 5, 7, 11, 19, 43]);
}

#[test]
fn verify_slice_contains_only_fails() {
    let subject: &[i32] = &[5, 11, 1, 3, 19, 17, 11, 43];

    let failures = verify_that(subject)
        .named("my_thing")
        .contains_only(&[1, 3, 7, 11, 19])
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing to contain only [1, 3, 7, 11, 19]
   but was: [5, 11, 1, 3, 19, 17, 11, 43]
  expected: [1, 3, 7, 11, 19]
     extra: [5, 17, 43]
"
        ]
    );
}

#[test]
fn slice_contains_only_once() {
    let subject: &[i32] = &[5, 11, 1, 3, 19, 43];

    assert_that(subject).contains_only_once(&[1, 3, 5, 7, 11, 19, 23, 43]);
}

#[test]
fn verify_slice_contains_only_once_fails() {
    let subject: &[i32] = &[5, 11, 1, 3, 19, 17, 11, 43];

    let failures = verify_that(subject)
        .named("my_thing")
        .contains_only_once(&[1, 3, 7, 11, 19])
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing to contain only once [1, 3, 7, 11, 19]
     but was: [5, 11, 1, 3, 19, 17, 11, 43]
    expected: [1, 3, 7, 11, 19]
       extra: [5, 17, 43]
  duplicates: [11, 11]
"
        ]
    );
}
