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
