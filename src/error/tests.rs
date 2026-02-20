use crate::prelude::*;
use crate::std::error::Error;
use crate::std::fmt::{self, Display};
use crate::std::vec::Vec;

#[derive(Debug)]
struct SuperError {
    source: SourceError,
}

impl Display for SuperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "super-error caused by {}", self.source)
    }
}

impl Error for SuperError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

#[derive(Debug, PartialEq)]
enum SourceError {
    Foo,
    Bar,
}

impl Display for SourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Foo => f.write_str("foo error"),
            Self::Bar => f.write_str("bar error"),
        }
    }
}

impl Error for SourceError {}

#[test]
fn error_has_debug_string() {
    let error = SuperError {
        source: SourceError::Bar,
    };

    assert_that(error).has_debug_string("SuperError { source: Bar }");
}

#[test]
fn verify_error_has_debug_string_fails() {
    let error = SuperError {
        source: SourceError::Foo,
    };

    let failures = verify_that(error)
        .has_debug_string("SuperError { source: Bar }")
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"expected subject to have a debug string equal to "SuperError { source: Bar }"
   but was: SuperError { source: Foo }
  expected: SuperError { source: Bar }
"#
        ]
    );
}

#[test]
fn error_does_not_have_debug_string() {
    let error = SuperError {
        source: SourceError::Bar,
    };

    assert_that(error).does_not_have_debug_string("SuperError { source: Foo }");
}

#[test]
fn verify_error_does_not_have_debug_string_fails() {
    let error = SuperError {
        source: SourceError::Bar,
    };

    let failures = verify_that(error)
        .does_not_have_debug_string("SuperError { source: Bar }")
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"expected subject to not have a debug string equal to "SuperError { source: Bar }"
   but was: SuperError { source: Bar }
  expected: not SuperError { source: Bar }
"#
        ]
    );
}

#[test]
fn error_has_display_string() {
    let error = SuperError {
        source: SourceError::Bar,
    };

    assert_that(error).has_display_string("super-error caused by bar error");
}

#[test]
fn verify_error_has_display_string_fails() {
    let error = SuperError {
        source: SourceError::Foo,
    };

    let failures = verify_that(error)
        .has_display_string("super-error caused by bar error")
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"expected subject to have a display string equal to "super-error caused by bar error"
   but was: "super-error caused by foo error"
  expected: "super-error caused by bar error"
"#
        ]
    );
}

#[test]
fn error_does_not_have_display_string() {
    let error = SuperError {
        source: SourceError::Bar,
    };

    assert_that(error).does_not_have_display_string("super-error caused by foo error");
}

#[test]
fn verify_error_does_not_have_display_string_fails() {
    let error = SuperError {
        source: SourceError::Foo,
    };

    let failures = verify_that(error)
        .does_not_have_display_string("super-error caused by foo error")
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"expected subject to not have a display string equal to "super-error caused by foo error"
   but was: "super-error caused by foo error"
  expected: not "super-error caused by foo error"
"#
        ]
    );
}

#[test]
fn source_error_has_no_source() {
    let error = SourceError::Foo;

    assert_that(error).has_no_source();
}

#[test]
fn verify_error_has_no_source_fails() {
    let error = SuperError {
        source: SourceError::Foo,
    };

    let failures = verify_that(error)
        .named("my error")
        .has_no_source()
        .display_failures();

    assert_eq!(
        failures,
        &[r"expected my error to have no source
   but was: SuperError { source: Foo }
  expected: <error with no source>
"]
    );
}

#[test]
fn super_error_has_source() {
    let error = SuperError {
        source: SourceError::Foo,
    };

    assert_that(error).has_source();
}

#[test]
fn verify_error_has_source_fails() {
    let error = SourceError::Bar;

    let failures = verify_that(error)
        .named("my error")
        .has_source()
        .display_failures();

    assert_eq!(
        failures,
        &[r"expected my error to have a source
   but was: Bar
  expected: <error with some source>
"]
    );
}

#[test]
fn super_error_has_source_message() {
    let error = SuperError {
        source: SourceError::Foo,
    };

    assert_that(error).has_source_message("foo error");
}

#[test]
fn verify_error_has_source_message_fails_wrong_source() {
    let error = SuperError {
        source: SourceError::Bar,
    };

    let failures = verify_that(error)
        .named("my error")
        .has_source_message("foo error")
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"expected my error to have a source message equal to "foo error"
   but was: "bar error"
  expected: "foo error"
"#
        ]
    );
}

#[test]
fn verify_error_has_source_message_fails_error_without_source() {
    let error = SourceError::Foo;

    let failures = verify_that(error)
        .named("my error")
        .has_source_message("foo error")
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"expected my error to have a source message equal to "foo error"
   but was: Foo - which has no source
  expected: "foo error"
"#
        ]
    );
}

#[test]
fn result_is_error_which_has_source() {
    let result: Result<Vec<i32>, SuperError> = Err(SuperError {
        source: SourceError::Bar,
    });

    assert_that(&result)
        .err()
        .has_source()
        .has_source_message("bar error");
}

#[cfg(feature = "colored")]
mod colored {
    use crate::error::tests::{SourceError, SuperError};
    use crate::prelude::*;

    #[test]
    fn highlight_diffs_error_has_no_source() {
        let error = SuperError {
            source: SourceError::Foo,
        };

        let failures = verify_that(error)
            .with_diff_format(DIFF_FORMAT_RED_YELLOW)
            .has_no_source()
            .display_failures();

        assert_eq!(
            failures,
            &["expected subject to have no source\n   \
                but was: \u{1b}[31mSuperError { source: Foo }\u{1b}[0m\n  \
               expected: \u{1b}[33m<error with no source>\u{1b}[0m\n\
            "]
        );
    }

    #[test]
    fn highlight_diffs_error_has_source() {
        let error = SourceError::Foo;

        let failures = verify_that(error)
            .with_diff_format(DIFF_FORMAT_RED_YELLOW)
            .has_source()
            .display_failures();

        assert_eq!(
            failures,
            &["expected subject to have a source\n   \
                but was: \u{1b}[31mFoo\u{1b}[0m\n  \
               expected: \u{1b}[33m<error with some source>\u{1b}[0m\n\
            "]
        );
    }

    #[test]
    fn highlight_diffs_error_has_source_message_fails_wrong_source() {
        let error = SuperError {
            source: SourceError::Bar,
        };

        let failures = verify_that(error)
            .with_diff_format(DIFF_FORMAT_RED_YELLOW)
            .has_source_message("foo error")
            .display_failures();

        assert_eq!(
            failures,
            &[
                "expected subject to have a source message equal to \"foo error\"\n   \
                    but was: \"\u{1b}[31mbar error\u{1b}[0m\"\n  \
                   expected: \"\u{1b}[33mfoo error\u{1b}[0m\"\n\
            "
            ]
        );
    }

    #[test]
    fn highlight_diffs_error_has_source_message_fails_error_without_source() {
        let error = SourceError::Foo;

        let failures = verify_that(error)
            .with_diff_format(DIFF_FORMAT_RED_YELLOW)
            .has_source_message("foo error")
            .display_failures();

        assert_eq!(
            failures,
            &[
                "expected subject to have a source message equal to \"foo error\"\n   \
                    but was: \u{1b}[31mFoo\u{1b}[0m - which has no source\n  \
                   expected: \u{1b}[33m\"foo error\"\u{1b}[0m\n\
            "
            ]
        );
    }
}

#[cfg(feature = "regex")]
mod with_regex {
    use super::*;

    #[test]
    fn error_has_debug_string_matching_regex() {
        let error = SuperError {
            source: SourceError::Bar,
        };

        assert_that(error).debug_string().matches(r"SuperError.*");
    }

    #[test]
    fn verify_error_has_debug_string_matching_regex_fails() {
        let error = SuperError {
            source: SourceError::Bar,
        };

        let failures = verify_that(error)
            .debug_string()
            .matches(r"SuperError \{ (source|target): Foo \}")
            .display_failures();

        assert_eq!(
            failures,
            &[
                r"expected subject's debug string to match the regex SuperError \{ (source|target): Foo \}
               but was: SuperError { source: Bar }
  does not match regex: SuperError \{ (source|target): Foo \}
"
            ]
        );
    }

    #[test]
    fn error_has_display_string_matching_regex() {
        let error = SuperError {
            source: SourceError::Bar,
        };

        assert_that(error).display_string().matches(r".*-error .*");
    }

    #[test]
    fn verify_error_has_display_string_matching_regex_fails() {
        let error = SuperError {
            source: SourceError::Bar,
        };

        let failures = verify_that(error)
            .display_string()
            .matches(r".*-error-caused.*")
            .display_failures();

        assert_eq!(
            failures,
            &[
                r"expected subject's display string to match the regex .*-error-caused.*
               but was: super-error caused by bar error
  does not match regex: .*-error-caused.*
"
            ]
        );
    }
}
