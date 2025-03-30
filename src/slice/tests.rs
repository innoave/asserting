use crate::prelude::*;
use crate::std::slice;
use crate::std::{
    string::{String, ToString},
    vec,
    vec::Vec,
};

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
            r"assertion failed: expected my_thing contains exactly in any order [0, 1, 3, 5, 7, 11, 11, 13, 17, 19, 23, 29, 31, 37, 41]
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
            r"assertion failed: expected my_thing contains any of [0, 2, 4, 8, 16, 32, 64], but contained none of them
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
            r"assertion failed: expected my_thing contains all of [5, 18, 3, 17, 45, 1, 1, 29, 0]
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
            r"assertion failed: expected my_thing contains only [1, 3, 7, 11, 19]
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
            r"assertion failed: expected my_thing contains only once [1, 3, 7, 11, 19]
     but was: [5, 11, 1, 3, 19, 17, 11, 43]
    expected: [1, 3, 7, 11, 19]
       extra: [5, 17, 43]
  duplicates: [11, 11]
"
        ]
    );
}

#[test]
fn slice_contains_exactly() {
    let subject: &[String] = &[
        "one".to_string(),
        "two".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "four".to_string(),
        "seven".to_string(),
        "eight".to_string(),
        "nine".to_string(),
    ];

    assert_that(subject).contains_exactly([
        "one", "two", "two", "three", "four", "five", "six", "four", "seven", "eight", "nine",
    ]);
}

#[test]
fn verify_slice_contains_exactly_fails_out_of_order() {
    let subject: &[String] = &[
        "one".to_string(),
        "two".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "four".to_string(),
        "seven".to_string(),
        "eight".to_string(),
        "nine".to_string(),
    ];

    let failures = verify_that(subject)
        .named("my_thing")
        .contains_exactly([
            "two", "two", "one", "three", "four", "five", "six", "seven", "four", "eight", "nine",
        ])
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"assertion failed: expected my_thing contains exactly in order ["two", "two", "one", "three", "four", "five", "six", "seven", "four", "eight", "nine"]
       but was: ["one", "two", "two", "three", "four", "five", "six", "four", "seven", "eight", "nine"]
      expected: ["two", "two", "one", "three", "four", "five", "six", "seven", "four", "eight", "nine"]
       missing: []
         extra: []
  out-of-order: ["one", "two", "four", "seven"]
"#
        ]
    );
}

#[test]
fn verify_slice_contains_exactly_fails_missing_and_extra() {
    let subject: &[String] = &[
        "one".to_string(),
        "two".to_string(),
        "six".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "six".to_string(),
        "seven".to_string(),
        "eleven".to_string(),
        "eight".to_string(),
        "nine".to_string(),
        "ten".to_string(),
    ];

    let failures = verify_that(subject)
        .named("my_thing")
        .contains_exactly([
            "one", "two", "two", "three", "four", "five", "six", "six", "seven", "four", "eight",
            "nine",
        ])
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"assertion failed: expected my_thing contains exactly in order ["one", "two", "two", "three", "four", "five", "six", "six", "seven", "four", "eight", "nine"]
       but was: ["one", "two", "six", "three", "four", "five", "six", "six", "seven", "eleven", "eight", "nine", "ten"]
      expected: ["one", "two", "two", "three", "four", "five", "six", "six", "seven", "four", "eight", "nine"]
       missing: ["two", "four"]
         extra: ["six", "eleven", "ten"]
  out-of-order: []
"#
        ]
    );
}

#[test]
fn verify_slice_contains_exactly_fails_expected_longer_than_vec() {
    let subject: &[String] = &[
        "one".to_string(),
        "two".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "four".to_string(),
        "seven".to_string(),
        "eight".to_string(),
        "nine".to_string(),
    ];

    let failures = verify_that(subject)
        .named("my_thing")
        .contains_exactly([
            "one", "two", "two", "three", "four", "five", "six", "four", "seven", "eight", "nine",
            "ten",
        ])
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"assertion failed: expected my_thing contains exactly in order ["one", "two", "two", "three", "four", "five", "six", "four", "seven", "eight", "nine", "ten"]
       but was: ["one", "two", "two", "three", "four", "five", "six", "four", "seven", "eight", "nine"]
      expected: ["one", "two", "two", "three", "four", "five", "six", "four", "seven", "eight", "nine", "ten"]
       missing: ["ten"]
         extra: []
  out-of-order: []
"#
        ]
    );
}

#[test]
fn slice_contains_sequence() {
    let subject: &[String] = &[
        "one".to_string(),
        "two".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "four".to_string(),
        "two".to_string(),
        "seven".to_string(),
        "eight".to_string(),
        "nine".to_string(),
    ];

    assert_that(subject).contains_sequence(["two", "three", "four", "five", "six", "four"]);
}

#[test]
fn verify_slice_contains_sequence_fails() {
    let subject: &[String] = &[
        "one".to_string(),
        "two".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "four".to_string(),
        "two".to_string(),
        "seven".to_string(),
        "two".to_string(),
        "three".to_string(),
        "five".to_string(),
        "four".to_string(),
        "six".to_string(),
        "four".to_string(),
        "eight".to_string(),
        "nine".to_string(),
        "ten".to_string(),
    ];

    let failures = verify_that(subject)
        .named("my_thing")
        .contains_sequence(["two", "three", "four", "five", "six", "six", "four"])
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"assertion failed: expected my_thing contains sequence ["two", "three", "four", "five", "six", "six", "four"]
       but was: ["one", "two", "two", "three", "four", "five", "six", "four", "two", "seven", "two", "three", "five", "four", "six", "four", "eight", "nine", "ten"]
      expected: ["two", "three", "four", "five", "six", "six", "four"]
       missing: ["six", "four"]
         extra: ["four", "two"]
"#
        ]
    );
}

#[test]
fn verify_slice_contains_sequence_fails_expected_longer_than_vec() {
    let subject: &[String] = &[
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
    ];

    let failures = verify_that(subject)
        .named("my_thing")
        .contains_sequence(["one", "two", "three", "four", "five", "six", "seven"])
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"assertion failed: expected my_thing contains sequence ["one", "two", "three", "four", "five", "six", "seven"]
       but was: ["one", "two", "three", "four", "five", "six"]
      expected: ["one", "two", "three", "four", "five", "six", "seven"]
       missing: ["seven"]
         extra: []
"#
        ]
    );
}

#[test]
fn slice_contains_all_in_order() {
    let subject: &[String] = &[
        "one".to_string(),
        "two".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "four".to_string(),
        "two".to_string(),
        "seven".to_string(),
        "eight".to_string(),
        "nine".to_string(),
    ];

    assert_that(subject)
        .contains_all_in_order(["one", "two", "two", "three", "four", "six", "four", "seven"]);
}

#[test]
fn verify_slice_contains_all_in_order_fails() {
    let subject: &[String] = &[
        "one".to_string(),
        "two".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "four".to_string(),
        "two".to_string(),
        "seven".to_string(),
        "two".to_string(),
        "three".to_string(),
        "five".to_string(),
        "four".to_string(),
        "six".to_string(),
        "four".to_string(),
        "eight".to_string(),
        "nine".to_string(),
        "ten".to_string(),
    ];

    let failures = verify_that(subject)
        .named("my_thing")
        .contains_all_in_order([
            "one", "two", "two", "seven", "two", "three", "six", "six", "ten",
        ])
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"assertion failed: expected my_thing contains all of ["one", "two", "two", "seven", "two", "three", "six", "six", "ten"] in order
       but was: ["one", "two", "two", "three", "four", "five", "six", "four", "two", "seven", "two", "three", "five", "four", "six", "four", "eight", "nine", "ten"]
      expected: ["one", "two", "two", "seven", "two", "three", "six", "six", "ten"]
       missing: ["six"]
"#
        ]
    );
}

#[test]
fn vec_starts_with() {
    let subject: Vec<i64> = vec![13, 5, 7, 19, 1, 3, 11, 29, 23, 31, 37];

    assert_that(subject).starts_with([13, 5, 7, 19, 1]);
}

#[test]
fn vec_starts_with_one_element() {
    let subject: Vec<i64> = vec![13, 5, 7, 19, 1, 3, 11, 29, 23, 31, 37];

    assert_that(subject).starts_with([13]);
}

#[test]
fn vec_starts_with_empty_sequence() {
    let subject: Vec<i64> = vec![13, 5, 7, 19, 1, 3, 11, 29, 23, 31, 37];

    assert_that(subject).starts_with([]);
}

#[test]
fn empty_vec_starts_with_empty_sequence() {
    let subject: Vec<i64> = vec![];

    assert_that(subject).starts_with([]);
}

#[test]
fn verify_vec_starts_with_fails() {
    let subject: Vec<i64> = vec![13, 5, 7, 19, 1, 3, 11, 29, 23, 31, 37];

    let failures = verify_that(subject)
        .named("my_thing")
        .starts_with([13, 5, 7, 1, 19])
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing starts with [13, 5, 7, 1, 19]
   but was: [13, 5, 7, 19, 1, 3, 11, 29, 23, 31, 37]
  expected: [13, 5, 7, 1, 19]
   missing: [1, 19]
     extra: [19, 1]
"
        ]
    );
}

#[test]
fn verify_empty_vec_starts_with_expected_sequence_longer_than_vec_fails() {
    let subject: Vec<i64> = vec![13, 5, 7, 19, 1, 3, 11, 29, 23, 31];

    let failures = verify_that(subject)
        .named("my_thing")
        .starts_with([13, 5, 7, 19, 1, 3, 11, 29, 23, 31, 37])
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing starts with [13, 5, 7, 19, 1, 3, 11, 29, 23, 31, 37]
   but was: [13, 5, 7, 19, 1, 3, 11, 29, 23, 31]
  expected: [13, 5, 7, 19, 1, 3, 11, 29, 23, 31, 37]
   missing: [37]
     extra: []
"
        ]
    );
}

#[test]
fn vec_ends_with() {
    let subject: Vec<i64> = vec![13, 5, 7, 19, 1, 3, 11, 29, 23, 31, 37];

    assert_that(subject).ends_with([11, 29, 23, 31, 37]);
}

#[test]
fn vec_ends_with_one_element() {
    let subject: Vec<i64> = vec![13, 5, 7, 19, 1, 3, 11, 29, 23, 31, 37];

    assert_that(subject).ends_with([37]);
}

#[test]
fn vec_ends_with_empty_sequence() {
    let subject: Vec<i64> = vec![13, 5, 7, 19, 1, 3, 11, 29, 23, 31, 37];

    assert_that(subject).ends_with([]);
}

#[test]
fn empty_vec_ends_with_empty_sequence() {
    let subject: Vec<i64> = vec![];

    assert_that(subject).ends_with([]);
}

#[test]
fn verify_vec_ends_with_fails() {
    let subject: Vec<i64> = vec![13, 5, 7, 19, 1, 3, 11, 29, 23, 31, 37];

    let failures = verify_that(subject)
        .named("my_thing")
        .ends_with([11, 23, 23, 31, 73])
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing ends with [11, 23, 23, 31, 73]
   but was: [13, 5, 7, 19, 1, 3, 11, 29, 23, 31, 37]
  expected: [11, 23, 23, 31, 73]
   missing: [23, 73]
     extra: [29, 37]
"
        ]
    );
}

#[test]
fn verify_empty_vec_ends_with_expected_sequence_longer_than_vec_fails() {
    let subject: Vec<i64> = vec![13, 5, 7, 19, 1, 3, 11, 29, 23, 31];

    let failures = verify_that(subject)
        .named("my_thing")
        .ends_with([41, 13, 5, 7, 19, 1, 3, 11, 29, 23, 31])
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing ends with [41, 13, 5, 7, 19, 1, 3, 11, 29, 23, 31]
   but was: [13, 5, 7, 19, 1, 3, 11, 29, 23, 31]
  expected: [41, 13, 5, 7, 19, 1, 3, 11, 29, 23, 31]
   missing: [41]
     extra: []
"
        ]
    );
}
