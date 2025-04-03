#[cfg(not(feature = "color"))]
mod no_color {
    use super::DIFF_FORMAT_NO_HIGHLIGHT;
    use crate::spec::DiffFormat;

    #[must_use]
    pub const fn diff_format() -> DiffFormat {
        DIFF_FORMAT_NO_HIGHLIGHT
    }
}

#[cfg(feature = "color")]
mod with_color {
    use super::DIFF_FORMAT_NO_HIGHLIGHT;
    use crate::spec::{DiffFormat, Highlight};

    /// Environment variable to set the highlight mode.
    pub const ENV_VAR_HIGHLIGHT_DIFFS: &str = "ASSERTING_HIGHLIGHT_DIFFS";

    /// Highlight mode using CVD-friendly colors.
    const HIGHLIGHT_MODE_CVD_COLORED: &str = "cvd-colored";
    /// Highlight mode using colors.
    const HIGHLIGHT_MODE_COLORED: &str = "colored";
    /// Highlight mode using bold font.
    const HIGHLIGHT_MODE_BOLD: &str = "bold";
    /// Highlight mode for no highlight at all.
    const HIGHLIGHT_MODE_OFF: &str = "off";

    /// Default highlight mode.
    pub const DEFAULT_HIGHLIGHT_MODE: &str = HIGHLIGHT_MODE_CVD_COLORED;
    /// Default diff format.
    pub const DEFAULT_DIFF_FORMAT: DiffFormat = DIFF_FORMAT_CVD_COLORED;

    const TERM_FONT_BOLD: &str = "\u{1b}[1m";
    const TERM_COLOR_RED: &str = "\u{1b}[31m";
    const TERM_COLOR_GREEN: &str = "\u{1b}[32m";
    const TERM_COLOR_BLUE: &str = "\u{1b}[34m";
    const TERM_RESET: &str = "\u{1b}[0m";

    const TERM_HIGHLIGHT_BOLD: Highlight = Highlight {
        start: TERM_FONT_BOLD,
        end: TERM_RESET,
    };
    const TERM_HIGHLIGHT_RED: Highlight = Highlight {
        start: TERM_COLOR_RED,
        end: TERM_RESET,
    };
    const TERM_HIGHLIGHT_GREEN: Highlight = Highlight {
        start: TERM_COLOR_GREEN,
        end: TERM_RESET,
    };
    const TERM_HIGHLIGHT_BLUE: Highlight = Highlight {
        start: TERM_COLOR_BLUE,
        end: TERM_RESET,
    };
    const TERM_NO_HIGHLIGHT: Highlight = Highlight { start: "", end: "" };

    pub const DIFF_FORMAT_CVD_COLORED: DiffFormat = DiffFormat {
        actual: TERM_HIGHLIGHT_RED,
        expected: TERM_HIGHLIGHT_BLUE,
    };

    pub const DIFF_FORMAT_COLORED: DiffFormat = DiffFormat {
        actual: TERM_HIGHLIGHT_RED,
        expected: TERM_HIGHLIGHT_GREEN,
    };

    pub const DIFF_FORMAT_BOLD: DiffFormat = DiffFormat {
        actual: TERM_HIGHLIGHT_BOLD,
        expected: TERM_NO_HIGHLIGHT,
    };

    #[must_use]
    pub fn diff_format_for_mode(mode: &str) -> Option<DiffFormat> {
        match mode.to_lowercase().as_str() {
            HIGHLIGHT_MODE_CVD_COLORED => Some(DIFF_FORMAT_CVD_COLORED),
            HIGHLIGHT_MODE_COLORED => Some(DIFF_FORMAT_COLORED),
            HIGHLIGHT_MODE_BOLD => Some(DIFF_FORMAT_BOLD),
            HIGHLIGHT_MODE_OFF => Some(DIFF_FORMAT_NO_HIGHLIGHT),
            _ => None,
        }
    }

    #[allow(clippy::print_stderr)]
    #[must_use]
    pub fn diff_format() -> DiffFormat {
        use crate::std::env;

        match env::var(ENV_VAR_HIGHLIGHT_DIFFS) {
            Ok(value) => diff_format_for_mode(&value).unwrap_or_else(|| {
                #[cfg(feature = "std")]
                eprintln!(
                    "WARNING: the environment variable `{ENV_VAR_HIGHLIGHT_DIFFS}` is set to the unrecognized value {value:?}.\n\t=> Default highlight mode \"{DEFAULT_HIGHLIGHT_MODE}\" is used."
                );
                DEFAULT_DIFF_FORMAT
            }),
            Err(env::VarError::NotPresent) => DEFAULT_DIFF_FORMAT,
            Err(env::VarError::NotUnicode(value)) => {
                #[cfg(feature = "std")]
                eprintln!(
                    "WARNING: the environment variable `{ENV_VAR_HIGHLIGHT_DIFFS}` is set to the unrecognized value {value:?}.\n\t=> Default highlight mode \"{DEFAULT_HIGHLIGHT_MODE}\" is used."
                );
                DEFAULT_DIFF_FORMAT
            },
        }
    }
}

#[cfg(not(feature = "color"))]
pub use no_color::diff_format;

#[cfg(feature = "color")]
pub use with_color::{
    diff_format, diff_format_for_mode, DEFAULT_DIFF_FORMAT, DEFAULT_HIGHLIGHT_MODE,
    DIFF_FORMAT_BOLD, DIFF_FORMAT_COLORED, DIFF_FORMAT_CVD_COLORED,
};

use crate::spec::{DiffFormat, Highlight};
use crate::std::fmt::Debug;
use crate::std::format;
use crate::std::string::String;

const NO_HIGHLIGHT: Highlight = Highlight { start: "", end: "" };

/// Diff format that does not highlight anything.
///
/// Setting this format effectively switches off highlighting.
pub const DIFF_FORMAT_NO_HIGHLIGHT: DiffFormat = DiffFormat {
    actual: NO_HIGHLIGHT,
    expected: NO_HIGHLIGHT,
};

#[cfg(not(feature = "color"))]
pub fn mark_diff<S, E>(actual: &S, expected: &E, _format: &DiffFormat) -> (String, String)
where
    S: Debug,
    E: Debug,
{
    (format!("{actual:?}"), format!("{expected:?}"))
}

#[cfg(feature = "color")]
pub fn mark_diff<S, E>(actual: &S, expected: &E, format: &DiffFormat) -> (String, String)
where
    S: Debug,
    E: Debug,
{
    use crate::std::vec::Vec;
    use sdiff::Diff;

    let actual = format!("{actual:?}").chars().collect::<Vec<_>>();
    let expected = format!("{expected:?}").chars().collect::<Vec<_>>();
    let mut marked_actual = Vec::with_capacity(actual.len());
    let mut marked_expected = Vec::with_capacity(expected.len());
    let diffs = sdiff::diff(&actual, &expected);
    for diff in diffs {
        match diff {
            Diff::Left { index, length } => {
                marked_actual.extend(format.actual.start.chars());
                marked_actual.extend_from_slice(&actual[index..(index + length)]);
                marked_actual.extend(format.actual.end.chars());
            },
            Diff::Both {
                left_index,
                right_index,
                length,
            } => {
                marked_actual.extend_from_slice(&actual[left_index..left_index + length]);
                marked_expected.extend_from_slice(&expected[right_index..right_index + length]);
            },
            Diff::Right { index, length } => {
                marked_expected.extend(format.expected.start.chars());
                marked_expected.extend_from_slice(&expected[index..(index + length)]);
                marked_expected.extend(format.expected.end.chars());
            },
        }
    }
    (
        String::from_iter(marked_actual),
        String::from_iter(marked_expected),
    )
}

#[cfg(test)]
mod tests;
