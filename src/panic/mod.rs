//! Handle panics in `std` and `no_std` environments.
//!
//! In assertion functions we use the [`trigger_panic`] function of this module
//! which supports a custom payload in panics when in `std` environment and
//! a string-formatted payload in `no_std` environments. The custom payload adds
//! the ability to verify the location of the test assertion by using the
//! [`assert_panic_location`] macro.
//!
//! The macros [`assert_panic_location!`] and [`assert_panic_message!`] provide
//! a convenient way for verifying the panic location and the panic message.
//! Note: These macros are intended for testing of assertion functions itself
//! but not for being used in end-user tests testing a project's code.
//!
//! [`assert_panic_location!`]: crate::assert_panic_location
//! [`assert_panic_message!`]: crate::assert_panic_message

use crate::std::string::String;

/// Panics with the given message and the location of the caller.
///
/// Using this function instead of the std `panic!` macro provides the
/// possibility to verify the panic location of assertions by using the macro
/// [`assert_panic_location!`].
///
/// In `std`-environments this function triggers a `std::panic::panic_any` with
/// a payload of type [`AssertionPanicPayload`]. In `no_std`-environments it
/// panics with a string-formatted payload.
///
/// # Panics
///
/// Calling this function **always** leads to a panic.
///
/// * If the `std` crate feature is active, the panic is triggered by calling
///   [`std::panic::panic_any`] with a payload of type [`AssertionPanicPayload`].
/// * In `no_std`-environments it panics with a string-formatted payload using
///   the [`std::panic!`] macro.
///
/// [`assert_panic_location!`]: crate::assert_panic_location
#[track_caller]
pub fn trigger_panic(message: impl Into<String>) -> ! {
    #[cfg(feature = "std")]
    {
        std::panic::panic_any(AssertionPanicPayload {
            message: message.into(),
            location: core::panic::Location::caller(),
        })
    }
    #[cfg(not(feature = "std"))]
    {
        let message = message.into();
        crate::std::panic!("{message}")
    }
}

/// A payload for panic calls that contains the panic message and the panic
/// location.
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[derive(Debug)]
pub struct AssertionPanicPayload {
    /// The panic message.
    pub message: String,
    /// The location where the panic occurred.
    pub location: &'static core::panic::Location<'static>,
}

#[cfg(feature = "std")]
mod std_impl {
    use super::AssertionPanicPayload;
    use crate::std::{fmt, fmt::Display};

    impl Display for AssertionPanicPayload {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.message)
        }
    }
}

/// Verifies that an assertion panics at the location in the tests.
///
/// This macro is intended for testing of assertion functions itself but not
/// for being used in end-user tests testing a project's code.
///
/// The expected location is taken from the line where the macro call starts.
/// This macro works only properly if the given assertion is located at the
/// same line as the macro call. For longer lines it might be necessary to
/// suppress automatic code formatting using the attribute `#[rustfmt::skip]`.
///
/// # Examples
///
/// ```
/// use asserting::prelude::*;
/// use asserting::assert_panic_location;
///
/// assert_panic_location!(assert_that!(41).is_equal_to(42));
///
/// #[rustfmt::skip]
/// assert_panic_location!(assert_that!("some longer assertion").starts_with("some").contains("much longer"));
/// ```
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[macro_export]
macro_rules! assert_panic_location {
    ($expression:expr) => {{
        let expected_line = line!();
        let expected_file = file!();

        let result = std::panic::catch_unwind(core::panic::AssertUnwindSafe(|| {
            $expression;
        }));

        let panic_error = result.expect_err("the assertion should have panicked!");
        let panic_payload = panic_error
            .downcast_ref::<$crate::panic::AssertionPanicPayload>()
            .expect("panic is not caused by an assertion!");

        // normalize file path
        let expected_file = expected_file.replace("\\", "/");
        let actual_file = panic_payload.location.file().replace("\\", "/");
        let actual_line = panic_payload.location.line();

        assert!(
            actual_file == expected_file && actual_line == expected_line,
            "wrong location in panic message!\n  expected location: {expected_file}:{expected_line}\n  actual location: {actual_file}:{actual_line}",
        );
    }};
}

/// Verifies that an assertion panics with the given message.
///
/// This macro is intended for testing of assertion functions itself but not
/// for being used in end-user tests testing a project's code.
///
/// # Examples
///
/// ```
/// use asserting::prelude::*;
/// use asserting::assert_panic_message;
///
/// assert_panic_message!(
///     assert_that!(41)
///         .with_diff_format(DIFF_FORMAT_RED_YELLOW)
///         .is_zero(),
///     "expected 41 to be zero\n   but was: \u{1b}[31m41\u{1b}[0m\n  expected: \u{1b}[33m0\u{1b}[0m\n"
/// );
///
/// assert_panic_message!(
///     assert_that!(-42)
///         .with_diff_format(DIFF_FORMAT_NO_HIGHLIGHT)
///         .is_positive(),
///     "expected -42 to be positive\n   but was: -42\n  expected: > 0\n"
/// );
/// ```
#[cfg(feature = "std")]
#[macro_export]
macro_rules! assert_panic_message {
    ($expression:expr, $expected_message:expr) => {{
        let result = std::panic::catch_unwind(core::panic::AssertUnwindSafe(|| {
            $expression;
        }));

        let panic_error = result.expect_err("the assertion should have panicked!");

        let actual_message = panic_error
            .downcast_ref::<$crate::panic::AssertionPanicPayload>()
            .map(|payload| payload.message.clone())
            .or_else(|| panic_error.downcast_ref::<String>().cloned())
            .or_else(|| panic_error.downcast_ref::<&str>().map(ToString::to_string))
            .expect("panic is not caused by an assertion!");

        let expected_message = $expected_message;

        assert_eq!(
            &actual_message, expected_message,
            "the panic message differs from the expected one!\n  expected message: {expected_message}\n  actual message: {actual_message}",
        );
    }};
}
