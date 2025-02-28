use crate::prelude::*;

//
// String and str
//

#[test]
fn string_is_equal_to_string() {
    let subject: String = "stet invidunt gubergren iusto".to_string();

    assert_that(subject).is_equal_to("stet invidunt gubergren iusto".to_string());
}

#[test]
fn string_is_equal_to_str() {
    let subject: String = "adipisici mollit hendrerit nostrud".to_string();

    assert_that(subject).is_equal_to("adipisici mollit hendrerit nostrud");
}

#[test]
fn string_ref_is_equal_to_str() {
    let subject: &String = &"duo exerci laborum doming".to_string();

    assert_that(subject).is_equal_to("duo exerci laborum doming");
}

#[test]
fn str_is_equal_to_str() {
    let subject: &str = "id elit vero praesent";

    assert_that(subject).is_equal_to("id elit vero praesent");
}

#[test]
fn str_is_equal_to_string() {
    let subject: &str = "ex tincidunt nam cupiditat";

    assert_that(subject).is_equal_to("ex tincidunt nam cupiditat");
}

#[test]
fn string_is_not_equal_to_string() {
    let subject: String = "volutpat voluptate nibh volutpat".to_string();

    assert_that(subject).is_not_equal_to("wisi nihil commodi ex".to_string());
}

#[test]
fn string_is_not_equal_to_str() {
    let subject: String = "consectetuer qui tincidunt adipiscing".to_string();

    assert_that(subject).is_not_equal_to("takimata wisi dolor vulputate");
}

#[test]
fn string_ref_is_not_equal_to_str() {
    let subject: String = "sunt facer clita delenit".to_string();

    assert_that(&subject).is_not_equal_to("tation zzril proident suscipit");
}

#[test]
fn str_is_not_equal_to_str() {
    let subject: &str = "cum consectetur sadipscing vulputate";

    assert_that(subject).is_not_equal_to("quod accumsan veniam doming");
}

#[test]
fn str_is_not_equal_to_string() {
    let subject: &str = "veniam mollit incidunt tincidunt";

    assert_that(subject).is_not_equal_to("est commodo eleifend imperdiet".to_string());
}

#[test]
fn check_string_is_equal_to_str_fails() {
    assert_eq!(
        check_that(String::new())
            .named("my_thing")
            .is_equal_to("aute duis eleifend molestie")
            .to_string(),
        r#"assertion failed: expected my_thing is equal to "aute duis eleifend molestie"
   but was: ""
  expected: "aute duis eleifend molestie"
"#
    );
}

//
// Integer
//

#[test]
fn usize_is_equal_to_usize() {
    let subject: usize = 42;

    assert_that(subject).is_equal_to(42);
}

#[test]
fn usize_is_not_equal_to_usize() {
    let subject: usize = 42;

    assert_that(subject).is_not_equal_to(51);
}

#[test]
fn i32_is_equal_to_i32() {
    let subject: i32 = -42;

    assert_that(subject).is_equal_to(-42);
}

#[test]
fn i32_is_not_equal_to_i32() {
    let subject: i32 = 42;

    assert_that(subject).is_not_equal_to(-42);
}

#[test]
fn check_i32_is_equal_to_i32_fails() {
    assert_eq!(
        check_that(42)
            .named("my_thing")
            .is_equal_to(-42)
            .to_string(),
        r"assertion failed: expected my_thing is equal to -42
   but was: 42
  expected: -42
"
    );
}

//
// bool
//

#[test]
fn bool_is_equal_to_bool() {
    let subject: bool = true;

    assert_that(subject).is_equal_to(true);
}

#[test]
fn bool_is_not_equal_to_bool() {
    let subject: bool = true;

    assert_that(subject).is_not_equal_to(false);
}

#[test]
fn check_bool_is_not_equal_to_false_fails() {
    assert_eq!(
        check_that(false)
            .named("my_thing")
            .is_not_equal_to(false)
            .to_string(),
        r"assertion failed: expected my_thing is not equal to false
   but was: false
  expected: false
"
    );
}
