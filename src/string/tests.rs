use crate::prelude::*;

#[test]
fn string_has_length() {
    let subject: String = "aute lobortis voluptua pariatur".to_string();

    assert_that(subject).has_length(31);
}

#[test]
fn str_has_length() {
    let subject: &str = "ad fugiat duo erat";

    assert_that(subject).has_length(18);
}

#[test]
fn verify_str_has_length_fails() {
    let subject: &str = "officia volutpat duis iriure";

    let failures = verify_that(subject)
        .named("my_thing")
        .has_length(29)
        .display_failures();
    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing has length 29
   but was: 28
  expected: 29
"]
    );
}

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
fn str_contains_a_char() {
    let subject: &str = "praesent doming liber accusam";

    assert_that(subject).contains(' ');
}

#[test]
fn verify_string_contains_other_str_fails() {
    let subject: String = "invidunt eos hendrerit commodo".to_string();

    assert_eq!(
        verify_that(subject)
            .named("my_thing")
            .contains("not a substring")
            .display_failures(),
        &[
            r#"assertion failed: expected my_thing to contain "not a substring"
   but was: "invidunt eos hendrerit commodo"
  expected: "not a substring"
"#
        ]
    );
}

#[test]
fn verify_string_contains_other_string_fails() {
    let subject: String = "invidunt eos hendrerit commodo".to_string();

    assert_eq!(
        verify_that(subject)
            .named("my_thing")
            .contains("not a substring".to_string())
            .display_failures(),
        &[
            r#"assertion failed: expected my_thing to contain "not a substring"
   but was: "invidunt eos hendrerit commodo"
  expected: "not a substring"
"#
        ]
    );
}

#[test]
fn string_contains_any_char_of_a_slice_of_chars() {
    let subject: String = "dolore reprehenderit erat duis".to_string();

    assert_that(subject).contains_any_of(&['o', 'e', 'r', 't'][..]);
}

#[test]
fn str_contains_any_char_of_an_array_of_chars() {
    let subject: &str = "duo excepteur invidunt nonumy";

    assert_that(subject).contains_any_of(['x', 'v', 'y']);
}

#[test]
fn string_contains_any_char_of_a_borrowed_array_of_chars() {
    let subject: String = "sadipscing nibh nisi voluptua".to_string();

    assert_that(subject).contains_any_of(&['a', 'e', 'i', 'o', 'u']);
}

#[test]
fn verify_str_contains_any_char_of_a_slice_of_chars_fails() {
    let subject: &str = "luptatum in nihil laoreet";

    assert_eq!(
        verify_that(subject)
            .named("my_thing")
            .contains_any_of(&['x', 'y', 'z'][..])
            .display_failures(),
        &[
            r#"assertion failed: expected my_thing to contain any of ['x', 'y', 'z']
   but was: "luptatum in nihil laoreet"
  expected: ['x', 'y', 'z']
"#
        ]
    );
}

#[test]
fn verify_string_contains_any_of_char_of_an_array_of_chars_fails() {
    let subject: String = "luptatum in nihil laoreet".to_string();

    assert_eq!(
        verify_that(subject)
            .named("my_thing")
            .contains_any_of(['x', 'y', 'z'])
            .display_failures(),
        &[
            r#"assertion failed: expected my_thing to contain any of ['x', 'y', 'z']
   but was: "luptatum in nihil laoreet"
  expected: ['x', 'y', 'z']
"#
        ]
    );
}

#[test]
fn verify_str_contains_any_of_char_of_a_borrowed_array_of_chars_fails() {
    let subject: &str = "luptatum in nihil laoreet";

    assert_eq!(
        verify_that(subject)
            .named("my_thing")
            .contains_any_of(&['x', 'y', 'z'])
            .display_failures(),
        &[
            r#"assertion failed: expected my_thing to contain any of ['x', 'y', 'z']
   but was: "luptatum in nihil laoreet"
  expected: ['x', 'y', 'z']
"#
        ]
    );
}
