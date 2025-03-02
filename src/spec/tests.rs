use crate::prelude::*;
use crate::spec::{AssertFailure, Expression};

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
fn assert_failure_display_format() {
    let failure = AssertFailure {
        description: Some("this thing is the best"),
        message: "but this thing is the worst\ninstead it should be the best".to_string(),
        location: Some(Location::new("src/thing_module/thing_test.rs", 54, 13)),
    };

    assert_that!(format!("{}", failure))
        .is_equal_to("assertion failed: this thing is the best\nbut this thing is the worst\ninstead it should be the best\n");
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
    expected = "assertion failed: expected ultimate_answer is equal to 42\n   but was: 51\n  expected: 42\n"
)]
fn assert_that_macro_is_equal_to_with_integers_fails() {
    let ultimate_answer = 51;

    assert_that!(ultimate_answer).is_equal_to(42);
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
            r"assertion failed: expected my_variable is Some(_)
   but was: None
  expected: Some(_)
",
            r"assertion failed: expected my_variable is some containing 42
   but was: None
  expected: Some(42)
",
        ]
    );
}

#[test]
fn verify_that_a_subject_with_custom_description_is_equal_to_fails() {
    let an_anwser = 51;

    let failures = verify_that(an_anwser)
        .described_as("the answer to all important questions is 42")
        .is_equal_to(42)
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: the answer to all important questions is 42
expected subject is equal to 42
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
expected answer is equal to 42
   but was: 51
  expected: 42
"
        ]
    );
}

#[test]
fn assert_that_subject_satisfies_predicate() {
    let subject = 42;

    assert_that(subject)
        .named("my_thing")
        .satisfies(|actual| *actual == 42);
}

#[test]
fn verify_that_subject_satisfies_predicate_fails() {
    let subject = 51;

    let failures = verify_that(subject)
        .named("my_thing")
        .satisfies(|actual| *actual == 42)
        .display_failures();

    assert_eq!(
        failures,
        &["assertion failed: expected my_thing to satisfy predicate, but returned false\n"]
    );
}

#[test]
fn verify_that_subject_satisfies_predicate_fails_with_custom_message() {
    let subject = 51;

    let failures = verify_that(subject)
        .named("my_thing")
        .satisfies_with_message("the answer to all important questions is 42", |actual| {
            *actual == 42
        })
        .display_failures();

    assert_eq!(
        failures,
        &["assertion failed: the answer to all important questions is 42\n"]
    );
}
