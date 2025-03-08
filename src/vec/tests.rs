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

#[test]
fn vec_contains_exactly_in_any_order() {
    let subject: Vec<i32> = vec![5, 7, 11, 13, 1, 19, 11, 3, 17, 23, 23, 29, 31, 41, 37, 43];

    assert_that(subject).contains_exactly_in_any_order([
        1, 3, 5, 7, 11, 11, 13, 17, 19, 23, 23, 29, 31, 37, 41, 43,
    ]);
}

#[test]
fn verify_vec_contains_exactly_in_any_order_fails() {
    let subject: Vec<i32> = vec![5, 7, 11, 13, 1, 11, 3, 17, 23, 23, 29, 31, 41, 37, 43];

    let failures = verify_that(subject)
        .named("my_thing")
        .contains_exactly_in_any_order([0, 1, 3, 5, 7, 11, 11, 13, 17, 19, 23, 29, 31, 37, 41])
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
fn vec_contains_any_of() {
    let subject: Vec<i32> = vec![5, 7, 11, 13, 1, 19, 11, 3, 17, 23, 23, 29, 31, 41, 37, 43];

    assert_that(subject).contains_any_of([2, 4, 6, 8, 19, 45]);
}

#[test]
fn verify_vec_contains_any_of_fails() {
    let subject: Vec<i32> = vec![5, 7, 11, 13, 1, 11, 3, 17, 23, 23, 29, 31, 41, 37, 43];

    let failures = verify_that(subject)
        .named("my_thing")
        .contains_any_of([0, 2, 4, 8, 16, 32, 64])
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
fn vec_contains_all_of() {
    let subject: Vec<i32> = vec![5, 7, 11, 13, 1, 19, 11, 3, 17, 23, 23, 29, 31, 41, 37, 43];

    assert_that(subject).contains_all_of([5, 3, 37, 19, 19, 17, 43]);
}

#[test]
fn verify_vec_contains_all_of_fails() {
    let subject: Vec<i32> = vec![5, 7, 11, 13, 1, 11, 3, 17, 23, 23, 29, 31, 41, 37, 43];

    let failures = verify_that(subject)
        .named("my_thing")
        .contains_all_of([5, 18, 3, 17, 45, 1, 1, 29, 0])
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
fn vec_contains_only() {
    let subject: Vec<i32> = vec![5, 11, 1, 3, 19, 11, 43];

    assert_that(subject).contains_only([1, 3, 5, 7, 11, 19, 43]);
}

#[test]
fn verify_vec_contains_only_fails() {
    let subject: Vec<i32> = vec![5, 11, 1, 3, 19, 17, 11, 43];

    let failures = verify_that(subject)
        .named("my_thing")
        .contains_only([1, 3, 7, 11, 19])
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
fn vec_contains_only_once() {
    let subject: Vec<i32> = vec![5, 11, 1, 3, 19, 43];

    assert_that(subject).contains_only_once([1, 3, 5, 7, 11, 19, 23, 43]);
}

#[test]
fn verify_vec_contains_only_once_fails() {
    let subject: Vec<i32> = vec![5, 11, 1, 3, 19, 17, 11, 43];

    let failures = verify_that(subject)
        .named("my_thing")
        .contains_only_once([1, 3, 7, 11, 19])
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
