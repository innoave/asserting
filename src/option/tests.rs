use crate::prelude::*;
use crate::std::{
    string::{String, ToString},
    vec,
};

#[test]
fn option_of_i32_is_none() {
    let subject: Option<i32> = None;

    assert_that(subject).is_none();
}

#[test]
fn option_of_i32_is_some() {
    let subject: Option<i32> = Some(42);

    assert_that(subject).is_some();
}

#[test]
fn option_of_string_is_none() {
    let subject: Option<String> = None;

    assert_that(subject).is_none();
}

#[test]
fn option_of_string_is_some() {
    let subject: Option<String> = Some("te veniam dolore ut".to_string());

    assert_that(subject).is_some();
}

#[test]
fn option_of_custom_struct_is_none() {
    #[derive(Debug)]
    struct Foo;

    let subject: Option<Foo> = None;

    assert_that(subject).is_none();
}

#[test]
fn verify_option_of_custom_struct_is_none_fails() {
    #[derive(Debug)]
    struct Foo;

    let subject: Option<Foo> = Some(Foo);

    let failures = verify_that(subject)
        .named("my_thing")
        .is_none()
        .display_failures();

    assert_eq!(
        failures,
        &[r"expected my_thing to be None
   but was: Some(Foo)
  expected: None
"]
    );
}

#[test]
fn option_of_custom_struct_is_some() {
    #[derive(Debug)]
    struct Foo;

    let subject: Option<Foo> = Some(Foo);

    assert_that(subject).is_some();
}

#[test]
fn verify_option_of_custom_struct_is_some_fails() {
    #[derive(Debug)]
    struct MyStruct;

    let subject: Option<MyStruct> = None;

    let failures = verify_that(subject)
        .named("my_thing")
        .is_some()
        .display_failures();

    assert_eq!(
        failures,
        &[r"expected my_thing to be Some(_)
   but was: None
  expected: Some(_)
"]
    );
}

#[test]
fn borrowed_option_of_custom_struct_is_none() {
    #[derive(Debug)]
    struct Foo;

    let subject: Option<Foo> = None;

    assert_that(&subject).is_none();
}

#[test]
fn verify_borrowed_option_of_custom_struct_is_none_fails() {
    #[derive(Debug)]
    struct Foo;

    let subject: Option<Foo> = Some(Foo);

    let failures = verify_that(&subject)
        .named("my_thing")
        .is_none()
        .display_failures();

    assert_eq!(
        failures,
        &[r"expected my_thing to be None
   but was: Some(Foo)
  expected: None
"]
    );
}

#[test]
fn borrowed_option_of_custom_struct_is_some() {
    #[derive(Debug)]
    struct Foo;

    let subject: Option<Foo> = Some(Foo);

    assert_that(&subject).is_some();
}

#[test]
fn verify_borrowed_option_of_custom_struct_is_some_fails() {
    #[derive(Debug)]
    struct MyStruct;

    let subject: Option<MyStruct> = None;

    let failures = verify_that(&subject)
        .named("my_thing")
        .is_some()
        .display_failures();

    assert_eq!(
        failures,
        &[r"expected my_thing to be Some(_)
   but was: None
  expected: Some(_)
"]
    );
}

#[test]
fn option_of_borrowed_custom_struct_is_none() {
    #[derive(Debug)]
    struct Foo;

    let subject: Option<&Foo> = None;

    assert_that(&subject).is_none();
}

#[test]
fn option_of_borrowed_custom_struct_is_some() {
    #[derive(Debug)]
    struct Foo;

    let subject: Option<&Foo> = Some(&Foo);

    assert_that(&subject).is_some();
}

#[test]
fn option_of_string_has_some_str_value() {
    let subject = Some("non tempor ea delenit".to_string());

    assert_that(subject).has_value("non tempor ea delenit");
}

#[test]
fn verify_option_of_string_has_some_value_fails() {
    let subject = Some("labore dolore voluptate culpa".to_string());

    let failures = verify_that(subject)
        .named("my_thing")
        .has_value("labore dolores voluptate culpa")
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"expected my_thing to be some containing "labore dolores voluptate culpa"
   but was: Some("labore dolore voluptate culpa")
  expected: Some("labore dolores voluptate culpa")
"#
        ]
    );
}

#[test]
fn option_of_str_has_some_str_value() {
    let subject = Some("facilisi cupiditat elitr facilisis");

    assert_that(subject).has_value("facilisi cupiditat elitr facilisis");
}

#[test]
fn option_of_str_has_some_string_value() {
    let subject = Some("invidunt commodi eros vel");

    assert_that(subject).has_value("invidunt commodi eros vel".to_string());
}

#[test]
fn option_of_custom_struct_has_value() {
    #[derive(Debug, PartialEq)]
    struct Foo;

    let subject = Some(Foo);

    assert_that(subject).has_value(Foo);
}

#[test]
fn verify_option_of_custom_struct_has_value_fails() {
    #[derive(Debug, PartialEq)]
    struct Foo;

    let subject: Option<Foo> = None;

    let failures = verify_that(subject)
        .named("my_thing")
        .has_value(Foo)
        .display_failures();

    assert_eq!(
        failures,
        &[r"expected my_thing to be some containing Foo
   but was: None
  expected: Some(Foo)
"]
    );
}

#[test]
fn borrowed_option_of_custom_struct_has_value() {
    #[derive(Debug, PartialEq)]
    struct Foo;

    let subject = Some(Foo);

    assert_that(&subject).has_value(Foo);
}

#[test]
fn verify_borrowed_option_of_custom_struct_has_value_fails() {
    #[derive(Debug, PartialEq)]
    struct Foo;

    let subject: Option<Foo> = None;

    let failures = verify_that(&subject)
        .named("my_thing")
        .has_value(Foo)
        .display_failures();

    assert_eq!(
        failures,
        &[r"expected my_thing to be some containing Foo
   but was: None
  expected: Some(Foo)
"]
    );
}

#[test]
fn option_of_borrowed_custom_struct_has_value() {
    #[derive(Debug, PartialEq)]
    struct Foo;

    let subject = Some(&Foo);

    assert_that(subject).has_value(&Foo);
}

#[test]
fn map_option_with_some_value_to_its_value() {
    let subject = Some(vec![1, 2, 3]);

    assert_that(subject).some().is_not_empty();
}

#[cfg(feature = "panic")]
#[test]
fn map_option_with_none_to_its_value() {
    let subject: Option<Vec<usize>> = None;

    assert_that_code(|| {
        assert_that(subject).some().is_empty();
    })
    .panics_with_message("expected the subject to be `Some(_)`, but was `None`");
}

#[test]
fn map_borrowed_option_with_some_value_to_its_value() {
    let subject = Some(vec![1, 2, 3]);

    assert_that(&subject).some().is_not_empty();
}

#[cfg(feature = "panic")]
#[test]
fn map_borrowed_option_with_none_to_its_value() {
    let subject: Option<Vec<usize>> = None;

    assert_that_code(|| {
        assert_that(&subject).some().is_empty();
    })
    .panics_with_message("expected the subject to be `Some(_)`, but was `None`");
}

#[cfg(feature = "colored")]
mod colored {
    use crate::prelude::*;
    use crate::std::{vec, vec::Vec};

    #[test]
    fn highlight_diffs_option_of_i64_is_some() {
        let subject: Option<i64> = None;

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_YELLOW)
            .is_some()
            .display_failures();

        assert_eq!(
            failures,
            &["expected subject to be Some(_)\n   \
                but was: \u{1b}[31mNone\u{1b}[0m\n  \
               expected: \u{1b}[33mSome(_)\u{1b}[0m\n\
        "]
        );
    }

    #[test]
    fn highlight_diffs_option_of_i64_is_none() {
        let subject: Option<i64> = Some(3500);

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_YELLOW)
            .is_none()
            .display_failures();

        assert_eq!(
            failures,
            &["expected subject to be None\n   \
                but was: \u{1b}[31mSome(3500)\u{1b}[0m\n  \
               expected: \u{1b}[33mNone\u{1b}[0m\n\
        "]
        );
    }

    #[test]
    fn highlight_diffs_option_of_vec_of_i32_has_value_but_is_none() {
        let subject: Option<Vec<i32>> = None;

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_YELLOW)
            .has_value(vec![1, 2, 3, 5, 7])
            .display_failures();

        assert_eq!(
            failures,
            &[
                "expected subject to be some containing [1, 2, 3, 5, 7]\n   \
                but was: \u{1b}[31mNone\u{1b}[0m\n  \
               expected: \u{1b}[33mSome([1, 2, 3, 5, 7])\u{1b}[0m\n\
        "
            ]
        );
    }
}
