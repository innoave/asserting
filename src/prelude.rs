//! Re-export of all types, traits, functions and macros that are needed to
//! write assertions in tests.
//!
//! When writing assertions in tests importing this prelude is all that should
//! be needed.
//!
//! # Example
//!
//! ```
//! use asserting::prelude::*;
//!
//! assert_that!(3 + 4).is_equal_to(7);
//! assert_that!(&[1, 3, 5, 7, 11]).contains_exactly(&[1, 3, 5, 7, 11]);
//! assert_that!("iusto obcaecat stet eos").starts_with("iusto").ends_with(" eos");
//! ```

pub use super::{
    assert_that,
    assertions::*,
    colored::{DEFAULT_DIFF_FORMAT, DIFF_FORMAT_NO_HIGHLIGHT},
    expectations::{all, rec, All, Not, Rec},
    properties::*,
    spec::{assert_that, verify_that, CollectFailures, Location, PanicOnFail},
    verify_that,
};

#[cfg(feature = "colored")]
#[cfg_attr(docsrs, doc(cfg(feature = "colored")))]
pub use super::colored::{
    DIFF_FORMAT_BOLD, DIFF_FORMAT_RED_BLUE, DIFF_FORMAT_RED_GREEN, DIFF_FORMAT_RED_YELLOW,
};

#[cfg(feature = "panic")]
#[cfg_attr(docsrs, doc(cfg(feature = "panic")))]
pub use super::{
    assert_that_code,
    spec::{assert_that_code, verify_that_code},
    verify_that_code,
};
