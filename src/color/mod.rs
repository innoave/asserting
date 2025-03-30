#[cfg(not(feature = "color"))]
mod no_color {
    use super::DIFF_FORMAT_NO_HIGHLIGHT;
    use crate::spec::{DiffFormat, Highlight};
    use crate::std::{format, string::String};
    use core::fmt::Debug;

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

    #[allow(clippy::print_stderr)]
    pub fn diff_format() -> DiffFormat {
        use crate::std::env;

        match env::var("ASSERTING_HIGHLIGHT_DIFFS").map(|var| var.to_lowercase()) {
            Ok(value) => match &value[..] {
                "cvd-colored" => DIFF_FORMAT_CVD_COLORED,
                "colored" => DIFF_FORMAT_COLORED,
                "bold" => DIFF_FORMAT_BOLD,
                "off" => DIFF_FORMAT_OFF,
                _ => {
                    #[cfg(feature = "std")]
                    eprintln!(
                        "WARNING: the environment variable `ASSERTING_HIGHLIGHT_DIFFS` is set to the unrecognized value {value:?}.\n\t=> Default highlight mode \"cvd-colored\" is used.",
                    );
                    DIFF_FORMAT_CVD_COLORED
                },
            },
            Err(env::VarError::NotPresent) => DIFF_FORMAT_CVD_COLORED,
            Err(env::VarError::NotUnicode(value)) => {
                #[cfg(feature = "std")]
                eprintln!(
                    "WARNING: the environment variable `ASSERTING_HIGHLIGHT_DIFFS` is set to the unrecognized value {value:?}.\n\t=> Default highlight mode \"cvd-colored\" is used."
                );
                DIFF_FORMAT_CVD_COLORED
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
pub use with_color::diff_format;

#[cfg(not(feature = "color"))]
use no_color::mark;

#[cfg(feature = "color")]
use with_color::mark;

use crate::spec::{DiffFormat, Highlight};
use crate::std::fmt::Debug;
use crate::std::string::String;

const NO_HIGHLIGHT: Highlight = Highlight { start: "", end: "" };

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
