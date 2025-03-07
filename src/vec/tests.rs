use crate::prelude::*;

#[test]
fn vec_is_equal_to_another_vec() {
    let subject: Vec<i32> = vec![1, 3, 5, 7];

    assert_that(subject).is_equal_to(vec![1, 3, 5, 7]);
}

#[test]
fn vec_is_equal_to_a_slice() {
    let subject: Vec<i32> = vec![1, 3, 5, 7];

    assert_that(subject).is_equal_to(&[1, 3, 5, 7]);
}

#[test]
fn verify_vec_is_equal_to_another_vec_fails() {
    let subject: Vec<i32> = vec![2, 4, 6, 8];

    let failures = verify_that(subject)
        .named("my_thing")
        .is_equal_to(vec![1, 3, 5, 7])
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing is equal to [1, 3, 5, 7]
   but was: [2, 4, 6, 8]
  expected: [1, 3, 5, 7]
"
        ]
    );
}

#[test]
fn vec_is_empty() {
    let subject: Vec<i32> = vec![];

    assert_that(subject).is_empty();
}

#[test]
fn vec_is_not_empty() {
    let subject: Vec<i32> = vec![1, 3, 5, 7, 11, 13, 17];

    assert_that(subject).is_not_empty();
}

#[test]
fn vec_has_length() {
    let subject: Vec<i32> = vec![1, 3, 5, 7, 11, 13];

    assert_that(subject).has_length(6);
}

#[test]
fn vec_has_length_in_range() {
    let subject: Vec<i32> = vec![1, 3, 5, 7, 11, 13, 17, 19];

    assert_that(subject).has_length_in_range(7..=8);
}

#[test]
fn vec_contains() {
    let subject: Vec<i32> = vec![1, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43];

    assert_that(subject).contains(19).contains(43).contains(1);
}

#[test]
fn verify_vec_contains_fails() {
    let subject: Vec<i32> = vec![1, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43];

    let failures = verify_that(subject)
        .named("my_thing")
        .contains(42)
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing to contain 42
   but was: [1, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43]
  expected: 42
"]
    );
}
