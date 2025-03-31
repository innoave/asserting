#[cfg(not(feature = "color"))]
mod no_color {
    use super::DIFF_FORMAT_NO_HIGHLIGHT;
    use crate::spec::{DiffFormat, Highlight};
    use crate::std::{format, string::String};
    use core::fmt::Debug;

    #[must_use]
    pub const fn diff_format() -> DiffFormat {
        DIFF_FORMAT_NO_HIGHLIGHT
    }

    pub fn mark<T>(value: &T, _highlight: &Highlight) -> String
    where
        T: Debug,
    {
        format!("{value:?}")
    }
}

#[cfg(feature = "color")]
mod with_color {
    use crate::spec::{DiffFormat, Highlight};
    use core::fmt::Debug;

    /// Environment variable to set the highlight mode.
    pub const ENV_VAR_HIGHLIGHT_DIFFS: &str = "ASSERTING_HIGHLIGHT_DIFFS";

    /// Highlight mode using CVD-friendly colors.
    pub const HIGHLIGHT_MODE_CVD_COLORED: &str = "cvd-colored";
    /// Highlight mode using colors.
    pub const HIGHLIGHT_MODE_COLORED: &str = "colored";
    /// Highlight mode using bold font.
    pub const HIGHLIGHT_MODE_BOLD: &str = "bold";
    /// Highlight mode for no highlight at all.
    pub const HIGHLIGHT_MODE_OFF: &str = "off";

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

    const DIFF_FORMAT_CVD_COLORED: DiffFormat = DiffFormat {
        actual: TERM_HIGHLIGHT_RED,
        expected: TERM_HIGHLIGHT_BLUE,
    };

    const DIFF_FORMAT_COLORED: DiffFormat = DiffFormat {
        actual: TERM_HIGHLIGHT_RED,
        expected: TERM_HIGHLIGHT_GREEN,
    };

    const DIFF_FORMAT_BOLD: DiffFormat = DiffFormat {
        actual: TERM_HIGHLIGHT_BOLD,
        expected: TERM_NO_HIGHLIGHT,
    };

    const DIFF_FORMAT_OFF: DiffFormat = DiffFormat {
        actual: TERM_NO_HIGHLIGHT,
        expected: TERM_NO_HIGHLIGHT,
    };

    #[must_use]
    pub fn diff_format_for_mode(mode: &str) -> Option<DiffFormat> {
        match mode.to_lowercase().as_str() {
            HIGHLIGHT_MODE_CVD_COLORED => Some(DIFF_FORMAT_CVD_COLORED),
            HIGHLIGHT_MODE_COLORED => Some(DIFF_FORMAT_COLORED),
            HIGHLIGHT_MODE_BOLD => Some(DIFF_FORMAT_BOLD),
            HIGHLIGHT_MODE_OFF => Some(DIFF_FORMAT_OFF),
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

    pub fn mark<T>(value: &T, highlight: &Highlight) -> String
    where
        T: Debug,
    {
        format!("{}{value:?}{}", highlight.start, highlight.end)
    }
}

#[cfg(not(feature = "color"))]
pub use no_color::diff_format;

#[cfg(feature = "color")]
pub use with_color::{
    diff_format, diff_format_for_mode, DEFAULT_DIFF_FORMAT, DEFAULT_HIGHLIGHT_MODE,
    HIGHLIGHT_MODE_BOLD, HIGHLIGHT_MODE_COLORED, HIGHLIGHT_MODE_CVD_COLORED, HIGHLIGHT_MODE_OFF,
};

#[cfg(not(feature = "color"))]
use no_color::mark;

#[cfg(feature = "color")]
use with_color::mark;

use crate::spec::{DiffFormat, Highlight};
use crate::std::fmt::Debug;
use crate::std::string::String;

const NO_HIGHLIGHT: Highlight = Highlight { start: "", end: "" };

/// Diff format that does not highlight anything.
///
/// Setting this format effectively switches off highlighting.
pub const DIFF_FORMAT_NO_HIGHLIGHT: DiffFormat = DiffFormat {
    actual: NO_HIGHLIGHT,
    expected: NO_HIGHLIGHT,
};

pub fn mark_diff<S, E>(actual: &S, expected: &E, format: &DiffFormat) -> (String, String)
where
    S: Debug,
    E: Debug,
{
    (
        mark(actual, &format.actual),
        mark(expected, &format.expected),
    )
}

#[cfg(test)]
mod tests;
