#[cfg(not(feature = "colored"))]
mod without_colored_feature {
    use super::DIFF_FORMAT_NO_HIGHLIGHT;
    use crate::spec::DiffFormat;
    use crate::std::{
        fmt::Debug,
        format,
        string::{String, ToString},
    };

    #[must_use]
    #[inline]
    pub const fn configured_diff_format_impl() -> DiffFormat {
        DIFF_FORMAT_NO_HIGHLIGHT
    }

    #[inline]
    pub fn mark_diff_impl<S, E>(actual: &S, expected: &E, _format: &DiffFormat) -> (String, String)
    where
        S: Debug,
        E: Debug,
    {
        (format!("{actual:?}"), format!("{expected:?}"))
    }

    #[inline]
    pub fn mark_unexpected_impl<T>(value: &T, _format: &DiffFormat) -> String
    where
        T: Debug,
    {
        format!("{value:?}")
    }

    #[inline]
    pub fn mark_missing_impl<T>(value: &T, _format: &DiffFormat) -> String
    where
        T: Debug,
    {
        format!("{value:?}")
    }

    #[inline]
    pub fn mark_unexpected_substr_impl(substr: &str, _format: &DiffFormat) -> String {
        substr.to_string()
    }

    #[inline]
    pub fn mark_missing_substr_impl(substr: &str, _format: &DiffFormat) -> String {
        substr.to_string()
    }

    #[inline]
    pub fn mark_unexpected_char_impl(character: char, _format: &DiffFormat) -> String {
        format!("{character}")
    }

    #[inline]
    pub fn mark_missing_char_impl(character: char, _format: &DiffFormat) -> String {
        format!("{character}")
    }
}

#[cfg(feature = "colored")]
mod with_colored_feature {
    use super::DIFF_FORMAT_NO_HIGHLIGHT;
    use crate::spec::{DiffFormat, Highlight};
    use crate::std::{fmt::Debug, format, string::String};

    /// Environment variable to set the highlight mode.
    #[cfg(feature = "std")]
    pub const ENV_VAR_HIGHLIGHT_DIFFS: &str = "ASSERTING_HIGHLIGHT_DIFFS";

    /// Highlight mode using the CVD-friendly colors red and blue.
    const HIGHLIGHT_MODE_RED_BLUE: &str = "red-blue";
    /// Highlight mode using the colors red and green.
    const HIGHLIGHT_MODE_RED_GREEN: &str = "red-green";
    /// Highlight mode using the colors red and yellow.
    const HIGHLIGHT_MODE_RED_YELLOW: &str = "red-yellow";
    /// Highlight mode using bold font.
    const HIGHLIGHT_MODE_BOLD: &str = "bold";
    /// Highlight mode for no highlight at all.
    const HIGHLIGHT_MODE_OFF: &str = "off";

    const TERM_FONT_BOLD: &str = "\u{1b}[1m";
    const TERM_COLOR_RED: &str = "\u{1b}[31m";
    const TERM_COLOR_GREEN: &str = "\u{1b}[32m";
    const TERM_COLOR_BLUE: &str = "\u{1b}[34m";
    const TERM_COLOR_YELLOW: &str = "\u{1b}[33m";
    const TERM_RESET: &str = "\u{1b}[0m";

    /// Default highlight mode.
    #[cfg(feature = "std")]
    const DEFAULT_HIGHLIGHT_MODE: &str = HIGHLIGHT_MODE_RED_GREEN;

    /// Default diff format.
    pub const DEFAULT_DIFF_FORMAT: DiffFormat = DIFF_FORMAT_RED_GREEN;

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
    const TERM_HIGHLIGHT_YELLOW: Highlight = Highlight {
        start: TERM_COLOR_YELLOW,
        end: TERM_RESET,
    };
    const TERM_NO_HIGHLIGHT: Highlight = Highlight { start: "", end: "" };

    /// A diff format that highlights differences in the colors red and blue.
    ///
    /// Differences in the actual value or not expected parts are colored in
    /// red. Differences in the expected value or parts that are missing in the
    /// actual value or colored in blue.
    pub const DIFF_FORMAT_RED_BLUE: DiffFormat = DiffFormat {
        unexpected: TERM_HIGHLIGHT_RED,
        missing: TERM_HIGHLIGHT_BLUE,
    };

    /// A diff format that highlights differences in the colors red and green.
    ///
    /// Differences in the actual value or not expected parts are colored in
    /// red. Differences in the expected value or parts that are missing in the
    /// actual value or colored in green.
    pub const DIFF_FORMAT_RED_GREEN: DiffFormat = DiffFormat {
        unexpected: TERM_HIGHLIGHT_RED,
        missing: TERM_HIGHLIGHT_GREEN,
    };

    /// A diff format that highlights differences in the colors red and yellow.
    ///
    /// Differences in the actual value or not expected parts are colored in
    /// red. Differences in the expected value or parts that are missing in the
    /// actual value or colored in yellow.
    pub const DIFF_FORMAT_RED_YELLOW: DiffFormat = DiffFormat {
        unexpected: TERM_HIGHLIGHT_RED,
        missing: TERM_HIGHLIGHT_YELLOW,
    };

    /// A diff format that highlights differences in the actual value in bold.
    pub const DIFF_FORMAT_BOLD: DiffFormat = DiffFormat {
        unexpected: TERM_HIGHLIGHT_BOLD,
        missing: TERM_NO_HIGHLIGHT,
    };

    #[must_use]
    pub fn diff_format_for_mode(mode: &str) -> Option<DiffFormat> {
        match mode.to_lowercase().as_str() {
            HIGHLIGHT_MODE_RED_BLUE => Some(DIFF_FORMAT_RED_BLUE),
            HIGHLIGHT_MODE_RED_GREEN => Some(DIFF_FORMAT_RED_GREEN),
            HIGHLIGHT_MODE_RED_YELLOW => Some(DIFF_FORMAT_RED_YELLOW),
            HIGHLIGHT_MODE_BOLD => Some(DIFF_FORMAT_BOLD),
            HIGHLIGHT_MODE_OFF => Some(DIFF_FORMAT_NO_HIGHLIGHT),
            _ => None,
        }
    }

    #[cfg(not(feature = "std"))]
    pub const fn configured_diff_format_impl() -> DiffFormat {
        DEFAULT_DIFF_FORMAT
    }

    #[cfg(feature = "std")]
    #[allow(clippy::print_stderr)]
    #[must_use]
    #[inline]
    pub fn configured_diff_format_impl() -> DiffFormat {
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

    #[inline]
    pub fn mark_diff_impl<S, E>(actual: &S, expected: &E, format: &DiffFormat) -> (String, String)
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
                    marked_actual.extend(format.unexpected.start.chars());
                    marked_actual.extend_from_slice(&actual[index..(index + length)]);
                    marked_actual.extend(format.unexpected.end.chars());
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
                    marked_expected.extend(format.missing.start.chars());
                    marked_expected.extend_from_slice(&expected[index..(index + length)]);
                    marked_expected.extend(format.missing.end.chars());
                },
            }
        }
        (
            String::from_iter(marked_actual),
            String::from_iter(marked_expected),
        )
    }

    #[inline]
    pub fn mark_unexpected_impl<T>(value: &T, format: &DiffFormat) -> String
    where
        T: Debug,
    {
        format!(
            "{}{value:?}{}",
            format.unexpected.start, format.unexpected.end
        )
    }

    #[inline]
    pub fn mark_missing_impl<T>(value: &T, format: &DiffFormat) -> String
    where
        T: Debug,
    {
        format!("{}{value:?}{}", format.missing.start, format.missing.end)
    }

    #[inline]
    pub fn mark_unexpected_substr_impl(substr: &str, format: &DiffFormat) -> String {
        format!(
            "{}{substr}{}",
            format.unexpected.start, format.unexpected.end
        )
    }

    #[inline]
    pub fn mark_missing_substr_impl(substr: &str, format: &DiffFormat) -> String {
        format!("{}{substr}{}", format.missing.start, format.missing.end)
    }

    #[inline]
    pub fn mark_unexpected_char_impl(character: char, format: &DiffFormat) -> String {
        format!(
            "{}{character}{}",
            format.unexpected.start, format.unexpected.end
        )
    }

    #[inline]
    pub fn mark_missing_char_impl(character: char, format: &DiffFormat) -> String {
        format!("{}{character}{}", format.missing.start, format.missing.end)
    }
}

#[cfg(feature = "colored")]
pub use with_colored_feature::{
    diff_format_for_mode, DIFF_FORMAT_BOLD, DIFF_FORMAT_RED_BLUE, DIFF_FORMAT_RED_GREEN,
    DIFF_FORMAT_RED_YELLOW,
};

use crate::spec::{DiffFormat, Highlight};
use crate::std::fmt::Debug;
use crate::std::string::String;
#[cfg(feature = "colored")]
use with_colored_feature::{
    configured_diff_format_impl, mark_diff_impl, mark_missing_char_impl, mark_missing_impl,
    mark_missing_substr_impl, mark_unexpected_char_impl, mark_unexpected_impl,
    mark_unexpected_substr_impl,
};
#[cfg(not(feature = "colored"))]
use without_colored_feature::{
    configured_diff_format_impl, mark_diff_impl, mark_missing_char_impl, mark_missing_impl,
    mark_missing_substr_impl, mark_unexpected_char_impl, mark_unexpected_impl,
    mark_unexpected_substr_impl,
};

const NO_HIGHLIGHT: Highlight = Highlight { start: "", end: "" };

/// Diff format that does not highlight anything.
///
/// Setting this format effectively switches off highlighting.
pub const DIFF_FORMAT_NO_HIGHLIGHT: DiffFormat = DiffFormat {
    unexpected: NO_HIGHLIGHT,
    missing: NO_HIGHLIGHT,
};

#[cfg(not(feature = "colored"))]
pub const DEFAULT_DIFF_FORMAT: DiffFormat = DIFF_FORMAT_NO_HIGHLIGHT;

#[cfg(feature = "colored")]
pub const DEFAULT_DIFF_FORMAT: DiffFormat = with_colored_feature::DEFAULT_DIFF_FORMAT;

#[allow(clippy::missing_const_for_fn)]
#[must_use]
pub fn configured_diff_format() -> DiffFormat {
    configured_diff_format_impl()
}

pub fn mark_diff<S, E>(actual: &S, expected: &E, format: &DiffFormat) -> (String, String)
where
    S: Debug,
    E: Debug,
{
    mark_diff_impl(actual, expected, format)
}

pub fn mark_unexpected<T>(value: &T, format: &DiffFormat) -> String
where
    T: Debug,
{
    mark_unexpected_impl(value, format)
}

pub fn mark_missing<T>(value: &T, format: &DiffFormat) -> String
where
    T: Debug,
{
    mark_missing_impl(value, format)
}

pub fn mark_unexpected_substr(substr: &str, format: &DiffFormat) -> String {
    mark_unexpected_substr_impl(substr, format)
}

pub fn mark_missing_substr(substr: &str, format: &DiffFormat) -> String {
    mark_missing_substr_impl(substr, format)
}

pub fn mark_unexpected_char(character: char, format: &DiffFormat) -> String {
    mark_unexpected_char_impl(character, format)
}

pub fn mark_missing_char(character: char, format: &DiffFormat) -> String {
    mark_missing_char_impl(character, format)
}

#[cfg(test)]
mod tests;
