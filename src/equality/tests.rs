use crate::equality::IsEqualTo;
use crate::specification::assert_that;

#[test]
fn string_is_equal_to_string() {
    let subject: String = "a string result".to_string();

    assert_that(subject).is_equal_to("a string result".to_string());
}

#[test]
fn string_is_equal_to_str() {
    let subject: String = "a string result".to_string();

    assert_that(subject).is_equal_to("a string result");
}

#[test]
fn string_ref_is_equal_to_str() {
    let subject: String = "a string result".to_string();

    assert_that::<&String>(&subject).is_equal_to("a string result");
}

#[test]
fn str_is_equal_to_str() {
    let subject: &str = "a string result";

    assert_that(subject).is_equal_to("a string result");
}

#[test]
fn str_is_equal_to_string() {
    let subject: &str = "a string result";

    assert_that(subject).is_equal_to("a string result".to_string());
}

#[test]
fn usize_is_equal_to_usize() {
    let subject: usize = 42;

    assert_that(subject).is_equal_to(42);
}

#[test]
fn bool_is_equal_to_bool() {
    let subject: bool = true;

    assert_that(subject).is_equal_to(true);
}
