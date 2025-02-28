use crate::prelude::*;

#[test]
fn string_contains_other_str() {
    let subject: String = "illum kasd nostrud possim".to_string();

    assert_that(subject).contains("nostrud");
}

#[test]
fn str_contains_other_str() {
    let subject: &str = "consectetuer duis quis veniam";

    assert_that(subject).contains("quis veniam");
}

#[test]
fn str_contains_other_string() {
    let subject: &str = "voluptua liber assum facilisis";

    assert_that(subject).contains("voluptua liber assum facilisis".to_string());
}

#[test]
fn string_contains_other_slice_of_chars() {
    let subject: String = "dolore reprehenderit erat duis".to_string();

    assert_that(subject).contains(&['o', 'e', 'r', 't'][..]);
}

#[test]
fn str_contains_other_array_of_chars() {
    let subject: &str = "duo excepteur invidunt nonumy";

    assert_that(subject).contains(['x', 'v', 'y']);
}

#[test]
fn string_contains_other_borrowed_array_of_chars() {
    let subject: String = "sadipscing nibh nisi voluptua".to_string();

    assert_that(subject).contains(&['a', 'e', 'i', 'o', 'u']);
}

#[test]
fn check_string_contains_other_str_fails() {
    let subject: String = "invidunt eos hendrerit commodo".to_string();

    assert_eq!(
        check_that(subject)
            .named("my_thing")
            .contains("not a substring")
            .to_string(),
        r#"assertion failed: expected my_thing contains "not a substring"
   but was: "invidunt eos hendrerit commodo"
  expected: "not a substring"
"#
    );
}

#[test]
fn check_str_contains_other_array_of_chars_fails() {
    let subject: String = "luptatum in nihil laoreet".to_string();

    assert_eq!(
        check_that(subject)
            .named("my_thing")
            .contains(['x', 'y', 'z'])
            .to_string(),
        r#"assertion failed: expected my_thing contains ['x', 'y', 'z']
   but was: "luptatum in nihil laoreet"
  expected: ['x', 'y', 'z']
"#
    );
}
