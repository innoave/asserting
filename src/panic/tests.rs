use crate::prelude::*;

#[test]
fn code_does_not_panic() {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    assert_that_code(|| {
        assert_that(add(2, 3)).is_equal_to(5);
    })
    .does_not_panic();
}

#[test]
fn verify_code_does_not_panic_fails() {
    let failures = verify_that_code(|| panic!("excepteur stet sadipscing eu"))
        .named("my_closure")
        .does_not_panic()
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"assertion failed: expected my_closure to not panic, but did panic
  with message: "excepteur stet sadipscing eu"
"#
        ]
    );
}

#[test]
fn code_does_panic() {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    assert_that_code(|| {
        assert_that(add(2, 3)).is_equal_to(4);
    })
    .panics();
}

#[test]
fn code_does_panic_with_message_from_assertion() {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    assert_that_code(|| {
        assert_that(add(2, 3))
            .with_diff_format(DIFF_FORMAT_NO_HIGHLIGHT)
            .is_equal_to(4);
    })
    .panics_with_message(
        "assertion failed: expected subject is equal to 4\n   but was: 5\n  expected: 4\n",
    );
}

#[test]
fn code_does_panic_with_message_from_panic_macro() {
    assert_that_code(|| {
        panic!("liber sea illum duis");
    })
    .panics_with_message("liber sea illum duis");
}

#[test]
fn verify_code_does_panic_fails() {
    let failures = verify_that_code(|| {
        assert_that(2 + 3).is_equal_to(5);
    })
    .named("my_closure")
    .panics()
    .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_closure to panic, but did not panic
"
        ]
    );
}

#[test]
fn verify_code_does_panic_with_message_fails_because_code_does_not_panic() {
    let failures = verify_that_code(|| {
        assert_that(2 + 3).is_equal_to(5);
    })
    .named("my_closure")
    .panics_with_message("nam veniam ut et")
    .display_failures();

    assert_eq!(
        failures,
        &[
            r#"assertion failed: expected my_closure to panic with message "nam veniam ut et",
  but did not panic
"#
        ]
    );
}

#[test]
fn verify_code_does_panic_with_message_fails_because_unexpected_panic_message() {
    let failures = verify_that_code(|| {
        assert_that(2 + 3)
            .with_diff_format(DIFF_FORMAT_NO_HIGHLIGHT)
            .is_equal_to(4);
    })
    .named("my_closure")
    .panics_with_message("lobortis lorem aliquam ex")
    .display_failures();

    assert_eq!(
        failures,
        &[
            "assertion failed: expected my_closure to panic with message \"lobortis lorem aliquam ex\"\n   \
   but was: \"assertion failed: expected subject is equal to 4\n   but was: 5\n  expected: 4\n\"\n  \
  expected: \"lobortis lorem aliquam ex\"\n\
"
        ]
    );
}

#[test]
fn verify_can_not_perform_two_does_not_panic_assertions_on_same_code_subject() {
    let failures = verify_that_code(|| {
        assert_that(2 + 3).is_equal_to(5);
    })
    .named("my_closure")
    .does_not_panic()
    .does_not_panic()
    .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: error in test assertion: only one expectation allowed when asserting closures!
"
        ]
    );
}

#[test]
fn verify_can_not_perform_two_panics_assertions_on_same_code_subject() {
    let failures = verify_that_code(|| {
        #[allow(unconditional_panic)]
        assert_that(2 / 0).is_equal_to(0);
    })
    .named("my_closure")
    .panics()
    .panics()
    .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: error in test assertion: only one expectation allowed when asserting closures!
"
        ]
    );
}

#[cfg(feature = "colored")]
mod colored {
    use crate::prelude::*;

    fn foo(message: Option<&str>) {
        if let Some(message) = message {
            panic!("{message}");
        }
    }

    #[test]
    fn highlight_diffs_code_does_not_panic() {
        let failures = verify_that_code(|| foo(Some("foo does not work with message")))
            .named("foo")
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .does_not_panic()
            .display_failures();

        assert_eq!(
            failures,
            &[
                "assertion failed: expected foo to not panic, but \u{1b}[31mdid panic\u{1b}[0m\n  \
                   with message: \"\u{1b}[31mfoo does not work with message\u{1b}[0m\"\n\
                "
            ]
        );
    }

    #[test]
    fn highlight_diffs_code_does_panic() {
        let failures = verify_that_code(|| foo(None))
            .named("foo")
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .panics()
            .display_failures();

        assert_eq!(
            failures,
            &["assertion failed: expected foo to panic, but \u{1b}[31mdid not panic\u{1b}[0m\n"]
        );
    }

    #[test]
    fn highlight_diffs_code_does_panic_with_message_but_does_not_panic() {
        let failures = verify_that_code(|| foo(None))
            .named("foo")
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .panics_with_message("hendrerit sint tempor ipsum")
            .display_failures();

        assert_eq!(
            failures,
            &["assertion failed: expected foo to panic with message \"hendrerit sint tempor ipsum\",\n  \
                 but \u{1b}[31mdid not panic\u{1b}[0m\n"]
        );
    }

    #[test]
    fn highlight_diffs_code_does_panic_with_message() {
        let failures = verify_that_code(|| foo(Some("foo does not work with message")))
            .named("foo")
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .panics_with_message("hendrerit sint tempor ipsum")
            .display_failures();

        assert_eq!(
            failures,
            &["assertion failed: expected foo to panic with message \"hendrerit sint tempor ipsum\"\n   \
                  but was: \"\u{1b}[31mfoo does not work with message\u{1b}[0m\"\n  \
                 expected: \"\u{1b}[32mhendrerit sint tempor ipsum\u{1b}[0m\"\n\
              "
            ]
        );
    }
}
