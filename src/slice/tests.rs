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
