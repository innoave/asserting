use crate::prelude::*;
use crate::spec::{AssertFailure, Expression, OwnedLocation};
use crate::std::{
    format,
    string::{String, ToString},
};

#[test]
fn default_of_expression_is_subject() {
    assert_that!(&*Expression::default()).is_equal_to("subject");
}

#[test]
fn location_display_format() {
    let location = Location::new("src/my_module/my_test.rs", 54, 13);

    assert_that!(format!("{}", location)).is_equal_to("src/my_module/my_test.rs:54:13");
}

#[test]
fn owned_location_display_format() {
    let location = OwnedLocation::new("src/my_module/my_test.rs", 54, 13);

    assert_that!(format!("{}", location)).is_equal_to("src/my_module/my_test.rs:54:13");
}

#[test]
fn construct_owned_location_from_location() {
    let location = Location::new("src/my_module/my_test.rs", 54, 13);
    let owned_location = OwnedLocation::from(location);

    assert_that!(owned_location).is_equal_to(OwnedLocation {
        file: "src/my_module/my_test.rs".to_string(),
        line: 54,
        column: 13,
    });
}

#[test]
fn owned_location_can_be_referenced_as_location() {
    let owned_location = OwnedLocation::new("src/my_module/my_test.rs", 54, 13);

    let location = owned_location.as_location();

    assert_that!(location).is_equal_to(Location {
        file: "src/my_module/my_test.rs",
        line: 54,
        column: 13,
    });
}

#[test]
fn assert_failure_display_format() {
    let failure = AssertFailure {
        description: Some("this thing is the best".to_string()),
        message: "but this thing is the worst\ninstead it should be the best".to_string(),
        location: Some(OwnedLocation::new("src/thing_module/thing_test.rs", 54, 13)),
    };

    assert_that!(format!("{}", failure))
        .is_equal_to("assertion failed: this thing is the best\nbut this thing is the worst\ninstead it should be the best\n");
}

#[test]
fn mapping_subject_in_spec() {
    struct Point {
        x: i64,
        y: i64,
    }

    let target = Point { x: 12, y: -64 };

    assert_that(target)
        .mapping(|s| (s.x, s.y))
        .is_equal_to((12, -64));
}

#[cfg(feature = "float-cmp")]
#[test]
fn extracting_from_subject_in_spec() {
    struct Foo {
        lorem: String,
        ipsum: f64,
    }

    let foo = Foo {
        lorem: "clita aute consequat dolor".into(),
        ipsum: 0.4519,
    };

    assert_that(&foo)
        .extracting(|s| &s.lorem)
        .is_equal_to("clita aute consequat dolor");

    assert_that(&foo)
        .extracting(|s| s.ipsum)
        .is_close_to(0.4519);
}

#[test]
fn assert_that_macro_with_owned_string_subject() {
    let input_string = String::from("erat esse sit aliqua");

    assert_that!(input_string).is_equal_to("erat esse sit aliqua");
}

#[test]
fn assert_that_macro_with_borrowed_string_subject() {
    let input_string = String::from("erat esse sit aliqua");

    assert_that!(&input_string).is_equal_to("erat esse sit aliqua");
}

#[test]
fn assert_that_macro_with_borrowed_str_subject() {
    let input_string = "adipiscing rebum amet iusto";

    assert_that!(input_string).is_equal_to("adipiscing rebum amet iusto");
}

#[test]
#[should_panic(
    expected = "assertion failed: expected ultimate_answer to be equal to 42\n   but was: 51\n  expected: 42\n"
)]
fn assert_that_macro_is_equal_to_with_integers_fails() {
    let ultimate_answer = 51;

    assert_that!(ultimate_answer)
        .with_diff_format(DIFF_FORMAT_NO_HIGHLIGHT)
        .is_equal_to(42);
}

#[test]
fn assert_that_option_is_some_chained_with_has_value() {
    let subject = Some(42);

    assert_that!(subject).is_some().has_value(42);
}

#[test]
fn verify_that_option_is_some_chained_with_has_value_fails_as_none() {
    let my_variable = None::<i32>;

    let failures = verify_that!(my_variable)
        .is_some()
        .has_value(42)
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_variable to be Some(_)
   but was: None
  expected: Some(_)
",
            r"assertion failed: expected my_variable to be some containing 42
   but was: None
  expected: Some(42)
",
        ]
    );
}

#[test]
fn verify_that_a_subject_with_custom_description_is_equal_to_fails() {
    let an_answer = 51;

    let failures = verify_that(an_answer)
        .described_as("the answer to all important questions is 42")
        .is_equal_to(42)
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: the answer to all important questions is 42
expected subject to be equal to 42
   but was: 51
  expected: 42
"
        ]
    );
}

#[test]
fn verify_that_a_subject_with_custom_name_and_custom_description_is_equal_to_fails() {
    let subject = 51;

    let failures = verify_that(subject)
        .named("answer")
        .described_as("the answer to all important questions is 42")
        .is_equal_to(42)
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: the answer to all important questions is 42
expected answer to be equal to 42
   but was: 51
  expected: 42
"
        ]
    );
}

#[test]
fn soft_assertions_with_chained_assertion_methods() {
    let subject = "the answer to all important questions is 42".to_string();

    verify_that(subject)
        .contains("important")
        .has_at_most_length(43)
        .soft_panic();
}

#[test]
#[should_panic = "assertion failed: expected subject to contain \"unimportant\"\n   \
       but was: \"the answer to all important questions is 42\"\n  \
      expected: \"unimportant\"\n\
    \n\
    assertion failed: expected subject to have at most a length of 41\n   \
       but was: 43\n  \
      expected: <= 41\n\
"]
fn soft_assertions_panic_once_with_multiple_failure_messages() {
    let subject = "the answer to all important questions is 42".to_string();

    verify_that(subject)
        .contains("unimportant")
        .has_at_most_length(41)
        .soft_panic();
}

#[test]
fn assert_each_item_of_an_iterator() {
    let subject = [2, 4, 6, 8, 10];

    assert_that(subject)
        .is_not_empty()
        .each_item(|e| e.is_positive().is_at_most(20));
}

#[test]
fn assert_each_item_of_a_borrowed_iterator() {
    let subject = [2, 4, 6, 8, 10];

    assert_that(&subject)
        .is_not_empty()
        .each_item(|e| e.is_positive().is_at_most(&20));
}

#[test]
#[should_panic = "assertion failed: expected numbers 2. item to be not equal to 4\n   but was: 4\n  expected: not 4\n"]
fn assert_each_item_of_an_iterator_panics_if_one_assertion_fails() {
    let subject = [2, 4, 6, 8, 10];

    assert_that(subject)
        .named("numbers")
        .is_not_empty()
        .each_item(|e| e.is_not_equal_to(4));
}

#[test]
fn verify_assert_each_item_of_an_iterator_fails() {
    let subject = [2, 4, 6, 8, 10];

    let failures = verify_that(&subject)
        .named("numbers")
        .each_item(|e| e.is_greater_than(&2).is_at_most(&7))
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected numbers 1. item to be greater than 2
   but was: 2
  expected: > 2
",
            r"assertion failed: expected numbers 4. item to be at most 7
   but was: 8
  expected: <= 7
",
            r"assertion failed: expected numbers 5. item to be at most 7
   but was: 10
  expected: <= 7
",
        ]
    );
}

#[cfg(feature = "colored")]
mod colored {
    use crate::prelude::*;

    #[test]
    #[should_panic = "assertion failed: expected subject to contain \"unimportant\"\n   \
       but was: \"\u{1b}[31mthe answer to all important questions is 42\u{1b}[0m\"\n  \
      expected: \"\u{1b}[32munimportant\u{1b}[0m\"\n\
    \n\
    assertion failed: expected subject to have at most a length of 41\n   \
       but was: \u{1b}[31m43\u{1b}[0m\n  \
      expected: <= \u{1b}[32m41\u{1b}[0m\n\
"]
    fn soft_assertions_panic_message_contains_highlighted_diffs() {
        let subject = "the answer to all important questions is 42";

        verify_that(subject)
            .with_configured_diff_format()
            .contains("unimportant")
            .has_at_most_length(41)
            .soft_panic();
    }
}
