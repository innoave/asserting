use crate::prelude::*;
use crate::std::fmt::{self, Display};
use crate::std::{
    string::{String, ToString},
    vec,
    vec::Vec,
};
use anyhow::anyhow;

#[test]
fn result_of_i32_is_ok() {
    let subject: Result<i32, String> = Ok(42);

    assert_that(subject).is_ok();
}

#[test]
fn result_of_custom_types_is_ok() {
    #[derive(Debug)]
    struct MyValue;

    #[derive(Debug)]
    struct MyError;

    let subject: Result<MyValue, MyError> = Ok(MyValue);

    assert_that(subject).is_ok();
}

#[test]
fn result_of_custom_types_is_err() {
    #[derive(Debug)]
    struct MyValue;

    #[derive(Debug)]
    struct MyError;

    let subject: Result<MyValue, MyError> = Err(MyError);

    assert_that(subject).is_err();
}

#[test]
fn verify_result_of_custom_types_is_ok_fails() {
    #[derive(Debug)]
    struct MyValue;

    #[allow(dead_code)]
    #[derive(Debug)]
    struct MyError(String);

    let subject: Result<MyValue, MyError> = Err(MyError("aute nam ad amet".to_string()));

    let failures = verify_that(subject)
        .named("my_thing")
        .is_ok()
        .display_failures();

    assert_eq!(
        failures,
        &[r#"assertion failed: expected my_thing to be Ok(_)
   but was: Err(MyError("aute nam ad amet"))
  expected: Ok(_)
"#]
    );
}

#[test]
fn verify_result_of_custom_types_is_err_fails() {
    #[allow(dead_code)]
    #[derive(Debug)]
    struct MyValue(i32);

    #[derive(Debug)]
    struct MyError;

    let subject: Result<MyValue, MyError> = Ok(MyValue(42));

    let failures = verify_that(subject)
        .named("my_thing")
        .is_err()
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing to be Err(_)
   but was: Ok(MyValue(42))
  expected: Err(_)
"]
    );
}

#[test]
fn result_of_custom_types_has_value() {
    #[derive(Debug, PartialEq)]
    struct MyValue(i32);

    #[derive(Debug)]
    struct MyError;

    let subject: Result<MyValue, MyError> = Ok(MyValue(42));

    assert_that(subject).has_value(MyValue(42));
}

#[test]
fn result_of_custom_types_has_error() {
    #[derive(Debug)]
    struct MyValue;

    #[derive(Debug, PartialEq)]
    struct MyError(String);

    let subject: Result<MyValue, MyError> = Err(MyError("to complicated!".to_string()));

    assert_that(subject).has_error(MyError("to complicated!".to_string()));
}

#[test]
fn verify_result_of_custom_types_has_value_fails() {
    #[derive(Debug, PartialEq)]
    struct MyValue(String);

    #[allow(dead_code)]
    #[derive(Debug)]
    struct MyError(String);

    let subject: Result<MyValue, MyError> = Err(MyError("amet esse rebum feugait".to_string()));

    let failures = verify_that(subject)
        .named("my_thing")
        .has_value(MyValue("sea non obcaecat nostrud".to_string()))
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"assertion failed: expected my_thing to be ok containing MyValue("sea non obcaecat nostrud")
   but was: Err(MyError("amet esse rebum feugait"))
  expected: Ok(MyValue("sea non obcaecat nostrud"))
"#
        ]
    );
}

#[test]
fn verify_result_of_custom_types_has_error_fails() {
    #[allow(dead_code)]
    #[derive(Debug)]
    struct MyValue(u32);

    #[derive(Debug, PartialEq)]
    struct MyError(i32);

    let subject: Result<MyValue, MyError> = Ok(MyValue(42));

    let failures = verify_that(subject)
        .named("my_thing")
        .has_error(MyError(-1))
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing to be an error containing MyError(-1)
   but was: Ok(MyValue(42))
  expected: Err(MyError(-1))
"
        ]
    );
}

#[test]
fn borrowed_result_of_custom_types_is_ok() {
    #[derive(Debug)]
    struct MyValue;

    #[derive(Debug)]
    struct MyError;

    let subject: Result<MyValue, MyError> = Ok(MyValue);

    assert_that(&subject).is_ok();
}

#[test]
fn borrowed_result_of_custom_types_is_err() {
    #[derive(Debug)]
    struct MyValue;

    #[derive(Debug)]
    struct MyError;

    let subject: Result<MyValue, MyError> = Err(MyError);

    assert_that(&subject).is_err();
}

#[test]
fn verify_borrowed_result_of_custom_types_is_ok_fails() {
    #[derive(Debug)]
    struct MyValue;

    #[allow(dead_code)]
    #[derive(Debug)]
    struct MyError(String);

    let subject: Result<MyValue, MyError> = Err(MyError("aute nam ad amet".to_string()));

    let failures = verify_that(&subject)
        .named("my_thing")
        .is_ok()
        .display_failures();

    assert_eq!(
        failures,
        &[r#"assertion failed: expected my_thing to be Ok(_)
   but was: Err(MyError("aute nam ad amet"))
  expected: Ok(_)
"#]
    );
}

#[test]
fn verify_borrowed_result_of_custom_types_is_err_fails() {
    #[allow(dead_code)]
    #[derive(Debug)]
    struct MyValue(i32);

    #[derive(Debug)]
    struct MyError;

    let subject: Result<MyValue, MyError> = Ok(MyValue(42));

    let failures = verify_that(&subject)
        .named("my_thing")
        .is_err()
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing to be Err(_)
   but was: Ok(MyValue(42))
  expected: Err(_)
"]
    );
}

#[test]
fn borrowed_result_of_custom_types_has_value() {
    #[derive(Debug, PartialEq)]
    struct MyValue(i32);

    #[derive(Debug)]
    struct MyError;

    let subject: Result<MyValue, MyError> = Ok(MyValue(42));

    assert_that(&subject).has_value(MyValue(42));
}

#[test]
fn borrowed_result_of_custom_types_has_error() {
    #[derive(Debug)]
    struct MyValue;

    #[derive(Debug, PartialEq)]
    struct MyError(String);

    let subject: Result<MyValue, MyError> = Err(MyError("to complicated!".to_string()));

    assert_that(&subject).has_error(MyError("to complicated!".to_string()));
}

#[test]
fn verify_borrowed_result_of_custom_types_has_value_fails() {
    #[derive(Debug, PartialEq)]
    struct MyValue(String);

    #[allow(dead_code)]
    #[derive(Debug)]
    struct MyError(String);

    let subject: Result<MyValue, MyError> = Err(MyError("amet esse rebum feugait".to_string()));

    let failures = verify_that(&subject)
        .named("my_thing")
        .has_value(MyValue("sea non obcaecat nostrud".to_string()))
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"assertion failed: expected my_thing to be ok containing MyValue("sea non obcaecat nostrud")
   but was: Err(MyError("amet esse rebum feugait"))
  expected: Ok(MyValue("sea non obcaecat nostrud"))
"#
        ]
    );
}

#[test]
fn verify_borrowed_result_of_custom_types_has_error_fails() {
    #[allow(dead_code)]
    #[derive(Debug)]
    struct MyValue(u32);

    #[derive(Debug, PartialEq)]
    struct MyError(i32);

    let subject: Result<MyValue, MyError> = Ok(MyValue(42));

    let failures = verify_that(&subject)
        .named("my_thing")
        .has_error(MyError(-1))
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing to be an error containing MyError(-1)
   but was: Ok(MyValue(42))
  expected: Err(MyError(-1))
"
        ]
    );
}

#[test]
fn map_result_with_ok_value_to_its_ok_value() {
    let subject: Result<Vec<u64>, String> = Ok(vec![]);

    assert_that(subject).ok().is_empty();
}

#[cfg(feature = "panic")]
#[test]
fn map_result_with_err_value_to_its_ok_value() {
    let subject: Result<Vec<usize>, String> = Err("nam nihil iure liber".to_string());

    assert_that_code(|| {
        assert_that(subject).ok().is_not_empty();
    })
        .panics_with_message("assertion failed: expected the subject to be `Ok(_)`, but was `Err(\"nam nihil iure liber\")`");
}

#[test]
fn map_result_with_err_value_to_its_err_value() {
    let subject: Result<(), String> = Err("tempor aliquip amet exerci".to_string());

    assert_that(subject).err().is_not_empty();
}

#[cfg(feature = "panic")]
#[test]
fn map_result_with_ok_value_to_its_err_value() {
    let subject: Result<Vec<usize>, String> = Ok(vec![1, 2, 3]);

    assert_that_code(|| {
        assert_that(subject).err().is_not_empty();
    })
    .panics_with_message(
        "assertion failed: expected the subject to be `Err(_)`, but was `Ok([1, 2, 3])`",
    );
}

#[test]
fn map_borrowed_result_with_ok_value_to_its_ok_value() {
    let subject: Result<Vec<u64>, String> = Ok(vec![]);

    assert_that(&subject).ok().is_empty();
}

#[cfg(feature = "panic")]
#[test]
fn map_borrowed_result_with_err_value_to_its_ok_value() {
    let subject: Result<Vec<usize>, String> = Err("nam nihil iure liber".to_string());

    assert_that_code(|| {
        assert_that(&subject).ok().is_not_empty();
    })
        .panics_with_message("assertion failed: expected the subject to be `Ok(_)`, but was `Err(\"nam nihil iure liber\")`");
}

#[test]
fn map_borrowed_result_with_err_value_to_its_err_value() {
    let subject: Result<(), String> = Err("tempor aliquip amet exerci".to_string());

    assert_that(&subject).err().is_not_empty();
}

#[cfg(feature = "panic")]
#[test]
fn map_borrowed_result_with_ok_value_to_its_err_value() {
    let subject: Result<Vec<usize>, String> = Ok(vec![1, 2, 3]);

    assert_that_code(|| {
        assert_that(&subject).err().is_not_empty();
    })
    .panics_with_message(
        "assertion failed: expected the subject to be `Err(_)`, but was `Ok([1, 2, 3])`",
    );
}

#[test]
fn result_error_has_message_for_an_anyhow_error() {
    let subject: Result<(), anyhow::Error> = Err(anyhow!("id hendrerit clita kasd"));

    assert_that(subject).has_error_message("id hendrerit clita kasd");
}

#[test]
fn result_error_has_message_for_custom_error_type() {
    #[derive(Debug)]
    struct OpaqueError(String);

    impl Display for OpaqueError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(&self.0)
        }
    }

    let subject: Result<(), OpaqueError> =
        Err(OpaqueError("soluta dolor vero takimata".to_string()));

    assert_that(subject).has_error_message("soluta dolor vero takimata");
}

#[cfg(feature = "panic")]
#[test]
fn verify_result_error_has_message_for_ok_value() {
    let subject: Result<(), anyhow::Error> = Ok(());

    assert_that_code(|| {
        assert_that(subject).has_error_message("vulputate voluptate sanctus quod");
    }).panics_with_message(
        r#"assertion failed: expected the subject to be `Err(_)` with message "vulputate voluptate sanctus quod", but was `Ok(())`"#,
    );
}

#[test]
fn borrowed_result_error_has_message_for_an_anyhow_error() {
    let subject: Result<(), anyhow::Error> = Err(anyhow!("id hendrerit clita kasd"));

    assert_that(&subject).has_error_message("id hendrerit clita kasd");
}

#[test]
fn borrowed_result_error_has_message_for_custom_error_type() {
    #[derive(Debug)]
    struct OpaqueError(String);

    impl Display for OpaqueError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(&self.0)
        }
    }

    let subject: Result<(), OpaqueError> =
        Err(OpaqueError("soluta dolor vero takimata".to_string()));

    assert_that(&subject).has_error_message("soluta dolor vero takimata");
}

#[cfg(feature = "panic")]
#[test]
fn verify_borrowed_result_error_has_message_for_ok_value() {
    let subject: Result<(), anyhow::Error> = Ok(());

    assert_that_code(|| {
        assert_that(&subject).has_error_message("vulputate voluptate sanctus quod");
    }).panics_with_message(
        r#"assertion failed: expected the subject to be `Err(_)` with message "vulputate voluptate sanctus quod", but was `Ok(())`"#,
    );
}

#[cfg(feature = "colored")]
mod colored {
    use crate::prelude::*;
    use crate::std::{
        string::{String, ToString},
        vec,
        vec::Vec,
    };

    #[test]
    fn highlight_diffs_result_of_i64_is_ok() {
        let subject: Result<i64, String> = Err("esse augue id esse".to_string());

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_BLUE)
            .is_ok()
            .display_failures();

        assert_eq!(
            failures,
            &["assertion failed: expected subject to be Ok(_)\n   \
                but was: \u{1b}[31mErr(\"esse augue id esse\")\u{1b}[0m\n  \
               expected: \u{1b}[34mOk(_)\u{1b}[0m\n\
            "]
        );
    }

    #[test]
    fn highlight_diffs_result_of_i64_is_err() {
        let subject: Result<i64, String> = Ok(3500);

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .is_err()
            .display_failures();

        assert_eq!(
            failures,
            &["assertion failed: expected subject to be Err(_)\n   \
                but was: \u{1b}[31mOk(3500)\u{1b}[0m\n  \
               expected: \u{1b}[32mErr(_)\u{1b}[0m\n\
            "]
        );
    }

    #[test]
    fn highlight_diffs_option_of_vec_of_i32_has_value_but_is_err() {
        let subject: Result<Vec<i32>, String> = Err("minim facer liber kasd".to_string());

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_YELLOW)
            .has_value(vec![1, 2, 3, 5, 7])
            .display_failures();

        assert_eq!(
            failures,
            &[
                "assertion failed: expected subject to be ok containing [1, 2, 3, 5, 7]\n   \
                but was: \u{1b}[31mErr(\"minim facer liber kasd\")\u{1b}[0m\n  \
               expected: \u{1b}[33mOk([1, 2, 3, 5, 7])\u{1b}[0m\n\
            "
            ]
        );
    }

    #[test]
    fn highlight_diffs_option_of_vec_of_i32_has_error_but_is_ok() {
        let subject: Result<Vec<i32>, String> = Ok(vec![1, 2, 3, 5, 7]);

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_YELLOW)
            .has_error("at feugait nihil qui")
            .display_failures();

        assert_eq!(
            failures,
            &[
                "assertion failed: expected subject to be an error containing \"at feugait nihil qui\"\n   \
                but was: \u{1b}[31mOk([1, 2, 3, 5, 7])\u{1b}[0m\n  \
               expected: \u{1b}[33mErr(\"at feugait nihil qui\")\u{1b}[0m\n\
            "
            ]
        );
    }
}
