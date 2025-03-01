use crate::prelude::*;

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
