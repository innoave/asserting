//! Functions for highlighting differences between expected and actual values
//! for failed assertions.
//!
//! The highlighting differences functionality of `asserting` is gated behind
//! the crate feature `colored`.
//!
//! Highlighting differences between expected and actual value helps with
//! spotting the differences between the two values and makes finding the
//! reason for a failed test much easier. The concept is to compare the expected
//! and the actual values and highlight missing and unexpected parts when
//! printing the actual and expected values with the assertion failure.
//!
//! When printing the expected value parts or the whole value is highlighted as
//! "missing" if there is no related part in the actual value. On the other
//! hand, when printing the actual value parts or the whole value is highlighted
//! as "unexpected" if there is no related part in the expected value. The
//! "missing" parts and the "unexpected" parts are highlighted in two different
//! colors (or in bold glyphs).
//!
//! Which colors are used or whether bold glyphs should be used instead of
//! colors can be configured via the environment variable
//! `ASSERTING_HIGHLIGHT_DIFFS`. See the documentation of the function
//! [`diff_format_for_mode`] for a list of the supported highlight modes. Using
//! an environment variable to configure the highlight mode (colors) has the
//! advantage that each developer working on the same code base can set the
//! colors to his/her liking.
//!
//! The intended way to configure the environment variable is to add the setting
//! to the Cargo config.toml in the users home directory. For example, to set
//! the colors to red and blue set the environment variable to `red-blue` within
//! the `[env]` section of the `~/.cargo/config.toml` file, like so:
//!
//! ```toml
//! [env]
//! ASSERTING_HIGHLIGHT_DIFFS = "red-blue"
//! ```
//!
//! This feature respects the [`NO_COLOR`] environment variable. If `NO_COLOR`
//! is set to a non-empty string, no colors are used, regardless of the mode set
//! with the `ASSERTING_HIGHLIGHT_DIFFS` environment variable.
//!
//! The functions provided by this module help with highlighting missing and
//! unexpected parts when composing the failure message for an assertion.
//!
//! [`NO_COLOR`]: https://no-color.org/

#[cfg(feature = "colored")]
#[cfg_attr(docsrs, doc(cfg(feature = "colored")))]
pub use with_colored_feature::{
    diff_format_for_mode, DIFF_FORMAT_BOLD, DIFF_FORMAT_RED_BLUE, DIFF_FORMAT_RED_GREEN,
    DIFF_FORMAT_RED_YELLOW,
};

use crate::spec::{DiffFormat, Highlight};
use crate::std::fmt::Debug;
use crate::std::format;
use crate::std::string::{String, ToString};
use crate::std::vec::Vec;
use hashbrown::HashSet;
#[cfg(feature = "colored")]
use with_colored_feature::{
    configured_diff_format_impl, mark_diff_impl, mark_missing_char_impl, mark_missing_impl,
    mark_missing_string_impl, mark_unexpected_char_impl, mark_unexpected_impl,
    mark_unexpected_string_impl,
};
#[cfg(not(feature = "colored"))]
use without_colored_feature::{
    configured_diff_format_impl, mark_diff_impl, mark_missing_char_impl, mark_missing_impl,
    mark_missing_string_impl, mark_unexpected_char_impl, mark_unexpected_impl,
    mark_unexpected_string_impl,
};

const NO_HIGHLIGHT: Highlight = Highlight { start: "", end: "" };

/// Diff format that does not highlight anything.
///
/// Setting this format effectively switches off highlighting.
pub const DIFF_FORMAT_NO_HIGHLIGHT: DiffFormat = DiffFormat {
    unexpected: NO_HIGHLIGHT,
    missing: NO_HIGHLIGHT,
};

/// Default diff format.
///
/// When the crate feature `colored` is enabled, the default diff format
/// highlights unexpected values in <span style="color: red;">red</span> and
/// missing values in <span style="color: green;">green</span>.
///
/// Without the crate feature `colored` enabled, the default diff format does not
/// highlight any differences in the messages for failed assertions.
pub const DEFAULT_DIFF_FORMAT: DiffFormat = {
    #[cfg(not(feature = "colored"))]
    {
        without_colored_feature::DEFAULT_DIFF_FORMAT
    }
    #[cfg(feature = "colored")]
    {
        with_colored_feature::DEFAULT_DIFF_FORMAT
    }
};

/// Reads the configured [`DiffFormat`] and returns it.
///
/// The behavior of this function is dependent on whether the crate features
/// `colored` and `std` are enabled or not.
///
/// When both features `colored` and `std` are enabled, the highlight mode is
/// read from the environment variable `ASSERTING_HIGHLIGHT_DIFFS`. If the
/// environment variable is set to a supported highlight mode, the
/// [`DiffFormat`] related to this mode is returned. Otherwise, the default diff
/// format [`DEFAULT_DIFF_FORMAT`] is returned. See the documentation of
/// [`diff_format_for_mode`] for a list of supported highlight modes.
///
/// If the environment variable [`NO_COLOR`] is set to a non-empty string, no
/// and `ASSERTING_HIGHLIGHT_DIFFS` is set to a color-mode, then
/// [`DIFF_FORMAT_NO_HIGHLIGHT`] is returned, which switches off colors.
///
/// When in a no-std environment with the feature `std` not enabled and the
/// `colored` feature is enabled. The default diff format
/// [`DEFAULT_DIFF_FORMAT`] is returned.
///
/// When the crate feature `colored` is not enabled, the diff format
/// [`DIFF_FORMAT_NO_HIGHLIGHT`] is returned, which means that highlighting is
/// switched off.
///
/// [`NO_COLOR`]: https://no-color.org/
#[allow(clippy::missing_const_for_fn)]
#[must_use]
pub fn configured_diff_format() -> DiffFormat {
    configured_diff_format_impl()
}

/// Highlights differences between the expected and the actual value and returns
/// the debug formatted values with marked differences.
///
/// The style for marking differences is determined by the provided
/// [`DiffFormat`].
///
/// It first converts the actual and the expected value into their debug
/// formatted string representation. Then a diff algorithm is applied to
/// determine the differences between the expected and the actual value.
/// Finally, the differences are marked according to the provided
/// [`DiffFormat`].
///
/// It returns a tuple of two `String`s. The first string contains the actual
/// value, and the second one contains the expected value. Both strings
/// represent their according value as debug formatted string with differences
/// highlighted.
///
/// # Examples
///
/// ```
/// # #[cfg(not(feature = "colored"))]
/// # fn main() {}
/// # #[cfg(feature = "colored")]
/// # fn main() {
/// use asserting::colored::{mark_diff, DIFF_FORMAT_RED_GREEN};
///
/// let actual = "Hello Welt!";
/// let expected = "Hello World!";
///
/// let (marked_actual, marked_expected) = mark_diff(&actual, &expected, &DIFF_FORMAT_RED_GREEN);
///
/// assert_eq!(marked_actual, "\"Hello W\u{1b}[31me\u{1b}[0ml\u{1b}[31mt\u{1b}[0m!\"");
/// assert_eq!(marked_expected, "\"Hello W\u{1b}[32mor\u{1b}[0ml\u{1b}[32md\u{1b}[0m!\"");
/// # }
/// ```
///
/// ```
/// # #[cfg(not(feature = "colored"))]
/// # fn main() {}
/// # #[cfg(feature = "colored")]
/// # fn main() {
/// use asserting::colored::{mark_diff, DIFF_FORMAT_RED_BLUE};
///
/// #[derive(Debug)]
/// struct Pos {
///     x: i32,
///     y: i32,
/// }
///
/// let actual = Pos { x: 45, y: -21 };
/// let expected = Pos { x: -45, y: -33 };
///
/// let (marked_actual, marked_expected) = mark_diff(&actual, &expected, &DIFF_FORMAT_RED_BLUE);
///
/// assert_eq!(marked_actual, "Pos { x: 45, y: -\u{1b}[31m21\u{1b}[0m }");
/// assert_eq!(marked_expected, "Pos { x: \u{1b}[34m-\u{1b}[0m45, y: -\u{1b}[34m33\u{1b}[0m }");
/// # }
/// ```
pub fn mark_diff<S, E>(actual: &S, expected: &E, format: &DiffFormat) -> (String, String)
where
    S: Debug + ?Sized,
    E: Debug + ?Sized,
{
    let actual = format!("{actual:?}");
    let expected = format!("{expected:?}");
    mark_diff_impl(&actual, &expected, format)
}

/// Highlights differences between the expected and the actual string and
/// returns new strings with marked differences.
///
/// The style for marking differences is determined by the provided
/// [`DiffFormat`].
///
/// A diff algorithm is applied to determine the differences between the
/// expected and the actual string. The differences are marked according to the
/// provided [`DiffFormat`].
///
/// It returns a tuple of two `String`s. The first string contains the actual
/// value, and the second one contains the expected value. Both strings
/// are a copy of the actual respectively expected string but with differences
/// highlighted.
pub fn mark_diff_str(actual: &str, expected: &str, format: &DiffFormat) -> (String, String) {
    mark_diff_impl(actual, expected, format)
}

/// Highlights the given value as "unexpected value" using the color for
/// unexpected values or bold as specified by the given [`DiffFormat`].
pub fn mark_unexpected<T>(value: &T, format: &DiffFormat) -> String
where
    T: Debug + ?Sized,
{
    mark_unexpected_impl(value, format)
}

/// Highlights the given value as "missing value" using the color for
/// "missing values" as specified by the given [`DiffFormat`].
pub fn mark_missing<T>(value: &T, format: &DiffFormat) -> String
where
    T: Debug + ?Sized,
{
    mark_missing_impl(value, format)
}

/// Highlights the given string as "unexpected value" using the color for
/// unexpected values or bold as specified by the given [`DiffFormat`].
///
/// When using this function in comparison to [`mark_unexpected`], the returned
/// string does not contain quotes at the start and end of the string as they
/// appear in the debug formatted string returned by [`mark_unexpected`].
pub fn mark_unexpected_string(string: &str, format: &DiffFormat) -> String {
    mark_unexpected_string_impl(string, format)
}

/// Highlights the given string as "missing value" using the color for
/// missing values as specified by the given [`DiffFormat`].
///
/// When using this function in comparison to [`mark_missing`], the returned
/// string does not contain quotes at the start and end of the string as they
/// appear in the debug formatted string returned by [`mark_missing`].
pub fn mark_missing_string(string: &str, format: &DiffFormat) -> String {
    mark_missing_string_impl(string, format)
}

/// Highlights the given character as "unexpected value" using the color for
/// unexpected values or bold as specified by the given [`DiffFormat`].
///
/// When using this function in comparison to [`mark_unexpected`], the returned
/// string does not contain single quotes around the character as they
/// appear in the debug formatted string returned by [`mark_unexpected`].
pub fn mark_unexpected_char(character: char, format: &DiffFormat) -> String {
    mark_unexpected_char_impl(character, format)
}

/// Highlights the given character as "missing value" using the color for
/// missing values as specified by the given [`DiffFormat`].
///
/// When using this function in comparison to [`mark_missing`], the returned
/// string does not contain single quotes around the character as they
/// appear in the debug formatted string returned by [`mark_missing`].
pub fn mark_missing_char(character: char, format: &DiffFormat) -> String {
    mark_missing_char_impl(character, format)
}

/// Highlights a substring within a string using the color for unexpected values
/// or bold as specified by the given [`DiffFormat`].
///
/// If the string does not contain the substring, a copy of the string is
/// returned without anything highlighted.
///
/// # Examples
///
/// ```
/// # #[cfg(not(feature = "colored"))]
/// # fn main() {}
/// # #[cfg(feature = "colored")]
/// # fn main() {
/// use asserting::colored::{mark_unexpected_substring_in_string, DIFF_FORMAT_RED_YELLOW};
///
/// let marked_string = mark_unexpected_substring_in_string(
///     "mollit est eu amet",
///     "est eu",
///     &DIFF_FORMAT_RED_YELLOW,
/// );
///
/// assert_eq!(marked_string, "mollit \u{1b}[31mest eu\u{1b}[0m amet");
/// # }
/// ```
pub fn mark_unexpected_substring_in_string(
    string: &str,
    substring: &str,
    format: &DiffFormat,
) -> String {
    mark_substring_in_string(string, substring, format, mark_unexpected_string)
}

/// Highlights a substring within a string using the color for missing values
/// as specified by the given [`DiffFormat`].
///
/// If the string does not contain the substring, a copy of the string is
/// returned without anything highlighted.
///
/// # Examples
///
/// ```
/// # #[cfg(not(feature = "colored"))]
/// # fn main() {}
/// # #[cfg(feature = "colored")]
/// # fn main() {
/// use asserting::colored::{mark_missing_substring_in_string, DIFF_FORMAT_RED_YELLOW};
///
/// let marked_string = mark_missing_substring_in_string(
///     "mollit est eu amet",
///     "est eu",
///     &DIFF_FORMAT_RED_YELLOW,
/// );
///
/// assert_eq!(marked_string, "mollit \u{1b}[33mest eu\u{1b}[0m amet");
/// # }
/// ```
pub fn mark_missing_substring_in_string(
    string: &str,
    substring: &str,
    format: &DiffFormat,
) -> String {
    mark_substring_in_string(string, substring, format, mark_missing_string)
}

fn mark_substring_in_string<F>(
    string: &str,
    substring: &str,
    format: &DiffFormat,
    mark: F,
) -> String
where
    F: Fn(&str, &DiffFormat) -> String,
{
    if let Some(position) = string.find(substring) {
        let length = substring.len();
        let begin = &string[..position];
        let end = &string[position + length..];
        let marked_substr = mark(substring, format);
        format!("{begin}{marked_substr}{end}")
    } else {
        string.to_string()
    }
}

/// Highlights all occurences of a character within a string using the color for
/// unexpected values or bold as specified by the given [`DiffFormat`].
///
/// If the string does not contain the character, a copy of the string is
/// returned without anything highlighted.
///
/// # Examples
///
/// ```
/// # #[cfg(not(feature = "colored"))]
/// # fn main() {}
/// # #[cfg(feature = "colored")]
/// # fn main() {
/// use asserting::colored::{mark_unexpected_char_in_string, DIFF_FORMAT_RED_YELLOW};
///
/// let marked_string = mark_unexpected_char_in_string(
///     "zzril mazim sint",
///     'z',
///     &DIFF_FORMAT_RED_YELLOW,
/// );
///
/// assert_eq!(marked_string, "\u{1b}[31mzz\u{1b}[0mril ma\u{1b}[31mz\u{1b}[0mim sint");
/// # }
/// ```
pub fn mark_unexpected_char_in_string(
    string: &str,
    character: char,
    format: &DiffFormat,
) -> String {
    mark_char_in_string(string, character, format, mark_unexpected_string)
}

/// Highlights all occurences of a character within a string using the color for
/// missing values as specified by the given [`DiffFormat`].
///
/// If the string does not contain the character, a copy of the string is
/// returned without anything highlighted.
///
/// # Examples
///
/// ```
/// # #[cfg(not(feature = "colored"))]
/// # fn main() {}
/// # #[cfg(feature = "colored")]
/// # fn main() {
/// use asserting::colored::{mark_missing_char_in_string, DIFF_FORMAT_RED_YELLOW};
///
/// let marked_string = mark_missing_char_in_string(
///     "zzril mazim sint",
///     'z',
///     &DIFF_FORMAT_RED_YELLOW,
/// );
///
/// assert_eq!(marked_string, "\u{1b}[33mzz\u{1b}[0mril ma\u{1b}[33mz\u{1b}[0mim sint");
/// # }
/// ```
pub fn mark_missing_char_in_string(string: &str, character: char, format: &DiffFormat) -> String {
    mark_char_in_string(string, character, format, mark_missing_string)
}

fn mark_char_in_string<F>(string: &str, character: char, format: &DiffFormat, mark: F) -> String
where
    F: Fn(&str, &DiffFormat) -> String,
{
    let mut marked_string = String::with_capacity(string.len());
    let mut parts = string.split(character);
    let mut chars_to_mark = String::new();
    parts
        .next()
        .iter()
        .for_each(|part| marked_string.push_str(part));
    for part in parts {
        chars_to_mark.push(character);
        if !part.is_empty() {
            marked_string.push_str(&mark(&chars_to_mark, format));
            chars_to_mark.clear();
            marked_string.push_str(part);
        }
    }
    marked_string
}

/// Highlights selected characters within a string using the color for
/// unexpected values or bold as specified by the given [`DiffFormat`].
///
/// # Examples
///
/// ```
/// # #[cfg(not(feature = "colored"))]
/// # fn main() {}
/// # #[cfg(feature = "colored")]
/// # fn main() {
/// use asserting::colored::{mark_selected_chars_in_string_as_unexpected, DIFF_FORMAT_RED_YELLOW};
///
/// let marked_string = mark_selected_chars_in_string_as_unexpected(
///     "rebum placerat consetetur",
///     &[0, 8, 9, 10, 24].into(),
///     &DIFF_FORMAT_RED_YELLOW,
/// );
///
/// assert_eq!(marked_string, "\u{1b}[31mr\u{1b}[0mebum pl\u{1b}[31mace\u{1b}[0mrat consetetu\u{1b}[31mr\u{1b}[0m");
/// # }
/// ```
pub fn mark_selected_chars_in_string_as_unexpected(
    string: &str,
    selected: &HashSet<usize>,
    format: &DiffFormat,
) -> String {
    mark_selected_chars_in_string(string, selected, &format.unexpected)
}

/// Highlights selected characters within a string using the color for
/// missing values as specified by the given [`DiffFormat`].
///
/// # Examples
///
/// ```
/// # #[cfg(not(feature = "colored"))]
/// # fn main() {}
/// # #[cfg(feature = "colored")]
/// # fn main() {
/// use asserting::colored::{mark_selected_chars_in_string_as_missing, DIFF_FORMAT_RED_YELLOW};
///
/// let marked_string = mark_selected_chars_in_string_as_missing(
///     "rebum placerat consetetur",
///     &[0, 20, 21].into(),
///     &DIFF_FORMAT_RED_YELLOW,
/// );
///
/// assert_eq!(marked_string, "\u{1b}[33mr\u{1b}[0mebum placerat conse\u{1b}[33mte\u{1b}[0mtur");
/// # }
/// ```
pub fn mark_selected_chars_in_string_as_missing(
    string: &str,
    selected: &HashSet<usize>,
    format: &DiffFormat,
) -> String {
    mark_selected_chars_in_string(string, selected, &format.missing)
}

fn mark_selected_chars_in_string(
    string: &str,
    selected: &HashSet<usize>,
    highlight: &Highlight,
) -> String {
    let mut marked_string = String::with_capacity(string.len());
    let mut to_mark = selected.iter().copied().collect::<Vec<_>>();
    to_mark.sort_unstable();
    let mut to_mark = to_mark.into_iter();
    let mut last_sel_idx = to_mark.next().unwrap_or(usize::MAX);
    let mut start_idx = last_sel_idx;
    let mut end_idx = last_sel_idx;
    for sel_idx in to_mark.by_ref() {
        let last_plus_one = last_sel_idx + 1;
        last_sel_idx = sel_idx;
        if sel_idx == last_plus_one {
            end_idx = sel_idx;
        } else {
            break;
        }
    }
    for (chr_idx, chr) in string.chars().enumerate() {
        if chr_idx == start_idx {
            marked_string.push_str(highlight.start);
            if last_sel_idx == end_idx {
                start_idx = usize::MAX;
            } else {
                start_idx = last_sel_idx;
            }
        }
        marked_string.push(chr);
        if chr_idx == end_idx {
            marked_string.push_str(highlight.end);
            end_idx = last_sel_idx;
            for sel_idx in to_mark.by_ref() {
                let last_plus_one = last_sel_idx + 1;
                last_sel_idx = sel_idx;
                if sel_idx == last_plus_one {
                    end_idx = sel_idx;
                } else {
                    break;
                }
            }
        }
    }
    marked_string
}

/// Highlights selected items of a collection using the given [`DiffFormat`].
///
/// This function formats the given collection for debug and highlights those
/// items of the collection where the index is present in the `selected_indices`
/// parameter.
///
/// Whether the items are highlighted as "unexpected" or "missing" depends on
/// the function specified in the `mark` parameter.
///
/// # Example
///
/// ```
/// # #[cfg(not(feature = "colored"))]
/// # fn main() {}
/// # #[cfg(feature = "colored")]
/// # fn main() {
/// use asserting::colored::{mark_missing, mark_selected_items_in_collection, DIFF_FORMAT_RED_BLUE};
/// use hashbrown::HashSet;
///
/// let collection = [1, 2, 3, 4, 5];
/// let selected_items: HashSet<_> = [1, 2, 4].into();
///
/// let marked_collection = mark_selected_items_in_collection(
///     &collection,
///     &selected_items,
///     &DIFF_FORMAT_RED_BLUE,
///     mark_missing
/// );
///
/// assert_eq!(marked_collection, "[1, \u{1b}[34m2\u{1b}[0m, \u{1b}[34m3\u{1b}[0m, 4, \u{1b}[34m5\u{1b}[0m]");
/// # }
/// ```
pub fn mark_selected_items_in_collection<T, F>(
    collection: &[T],
    selected_indices: &HashSet<usize>,
    format: &DiffFormat,
    mark: F,
) -> String
where
    T: Debug,
    F: Fn(&T, &DiffFormat) -> String,
{
    let mut marked_collection = String::with_capacity(collection.len() + 2);
    marked_collection.push('[');
    collection
        .iter()
        .enumerate()
        .map(|(index, item)| {
            if selected_indices.contains(&index) {
                mark(item, format)
            } else {
                format!("{item:?}")
            }
        })
        .for_each(|item| {
            marked_collection.push_str(&item);
            marked_collection.push_str(", ");
        });
    if marked_collection.len() >= 3 {
        marked_collection.pop();
        marked_collection.pop();
    }
    marked_collection.push(']');
    marked_collection
}

/// Highlights all items of a collection using the given [`DiffFormat`].
///
/// This function formats the given collection for debug and highlights all
/// items of the collection individually (instead of the whole debug string).
///
/// Whether the items are highlighted as "unexpected" or "missing" depends on
/// the function specified in the `mark` parameter.
///
/// # Example
///
/// ```
/// # #[cfg(not(feature = "colored"))]
/// # fn main() {}
/// # #[cfg(feature = "colored")]
/// # fn main() {
/// use asserting::colored::{mark_all_items_in_collection, mark_unexpected, DIFF_FORMAT_RED_BLUE};
/// use hashbrown::HashSet;
///
/// let collection = [1, 2, 3, 4, 5];
///
/// let marked_collection = mark_all_items_in_collection(
///     &collection,
///     &DIFF_FORMAT_RED_BLUE,
///     mark_unexpected
/// );
///
/// assert_eq!(marked_collection, "[\u{1b}[31m1\u{1b}[0m, \u{1b}[31m2\u{1b}[0m, \u{1b}[31m3\u{1b}[0m, \u{1b}[31m4\u{1b}[0m, \u{1b}[31m5\u{1b}[0m]");
/// # }
/// ```
pub fn mark_all_items_in_collection<T, F>(collection: &[T], format: &DiffFormat, mark: F) -> String
where
    T: Debug,
    F: Fn(&T, &DiffFormat) -> String,
{
    let mut marked_collection = String::with_capacity(collection.len() + 2);
    marked_collection.push('[');
    collection
        .iter()
        .map(|item| mark(item, format))
        .for_each(|item| {
            marked_collection.push_str(&item);
            marked_collection.push_str(", ");
        });
    if marked_collection.len() >= 3 {
        marked_collection.pop();
        marked_collection.pop();
    }
    marked_collection.push(']');
    marked_collection
}

/// Highlights selected entries in a map using the given [`DiffFormat`].
///
/// This function formats the given map for debug and highlights those entries
/// in the map where the index is present in the `selected_indices` parameter.
///
/// Whether the entries are highlighted as "unexpected" or "missing" depends on
/// the function specified in the `mark` parameter.
///
/// # Example
///
/// ```
/// # #[cfg(not(all(feature = "colored", feature = "std")))]
/// # fn main() {}
/// # #[cfg(all(feature = "colored", feature = "std"))]
/// # fn main() {
/// use asserting::colored::{mark_missing_string, mark_selected_entries_in_map, DIFF_FORMAT_RED_BLUE};
/// use hashbrown::HashSet;
/// use std::collections::BTreeMap;
///
/// let map: BTreeMap<_, _> = [(1, "one"), (2, "two"), (3, "three"), (4, "four")].into();
/// let selected_entries: HashSet<_> = [0, 2].into();
///
/// let map_entries: Vec<_> = map.iter().collect();
/// let marked_map = mark_selected_entries_in_map(
///     &map_entries,
///     &selected_entries,
///     &DIFF_FORMAT_RED_BLUE,
///     mark_missing_string
/// );
///
/// assert_eq!(marked_map, "{\u{1b}[34m1: \"one\"\u{1b}[0m, 2: \"two\", \u{1b}[34m3: \"three\"\u{1b}[0m, 4: \"four\"}");
/// # }
/// ```
pub fn mark_selected_entries_in_map<K, V, F>(
    map_entries: &[(K, V)],
    selected_indices: &HashSet<usize>,
    format: &DiffFormat,
    mark: F,
) -> String
where
    K: Debug,
    V: Debug,
    F: Fn(&str, &DiffFormat) -> String,
{
    let mut marked_map_entries = String::with_capacity(map_entries.len() + 2);
    marked_map_entries.push('{');
    map_entries
        .iter()
        .enumerate()
        .map(|(index, entry)| {
            let key_value_pair = format!("{:?}: {:?}", entry.0, entry.1);
            if selected_indices.contains(&index) {
                mark(&key_value_pair, format)
            } else {
                key_value_pair
            }
        })
        .for_each(|entry| {
            marked_map_entries.push_str(&entry);
            marked_map_entries.push_str(", ");
        });
    if marked_map_entries.len() >= 3 {
        marked_map_entries.pop();
        marked_map_entries.pop();
    }
    marked_map_entries.push('}');
    marked_map_entries
}

/// Highlights all entries in a map using the given [`DiffFormat`].
///
/// This function formats the given map for debug and highlights all entries in
/// the map individually (instead of the whole debug string).
///
/// Whether the entries are highlighted as "unexpected" or "missing" depends on
/// the function specified in the `mark` parameter.
///
/// # Example
///
/// ```
/// # #[cfg(not(all(feature = "colored", feature = "std")))]
/// # fn main() {}
/// # #[cfg(all(feature = "colored", feature = "std"))]
/// # fn main() {
/// use asserting::colored::{mark_all_entries_in_map, mark_unexpected_string, DIFF_FORMAT_RED_BLUE};
/// use std::collections::BTreeMap;
///
/// let map: BTreeMap<_, _> = [(1, "one"), (2, "two"), (3, "three"), (4, "four")].into();
///
/// let map_entries: Vec<_> = map.iter().collect();
/// let marked_map = mark_all_entries_in_map(
///     &map_entries,
///     &DIFF_FORMAT_RED_BLUE,
///     mark_unexpected_string
/// );
///
/// assert_eq!(marked_map, "{\u{1b}[31m1: \"one\"\u{1b}[0m, \u{1b}[31m2: \"two\"\u{1b}[0m, \u{1b}[31m3: \"three\"\u{1b}[0m, \u{1b}[31m4: \"four\"\u{1b}[0m}");
/// # }
/// ```
pub fn mark_all_entries_in_map<K, V, F>(
    map_entries: &[(K, V)],
    format: &DiffFormat,
    mark: F,
) -> String
where
    K: Debug,
    V: Debug,
    F: Fn(&str, &DiffFormat) -> String,
{
    let mut marked_map_entries = String::with_capacity(map_entries.len() + 2);
    marked_map_entries.push('{');
    map_entries
        .iter()
        .map(|entry| {
            let key_value_pair = format!("{:?}: {:?}", entry.0, entry.1);
            mark(&key_value_pair, format)
        })
        .for_each(|entry| {
            marked_map_entries.push_str(&entry);
            marked_map_entries.push_str(", ");
        });
    if marked_map_entries.len() >= 3 {
        marked_map_entries.pop();
        marked_map_entries.pop();
    }
    marked_map_entries.push('}');
    marked_map_entries
}

#[cfg(not(feature = "colored"))]
mod without_colored_feature {
    use super::DIFF_FORMAT_NO_HIGHLIGHT;
    use crate::spec::DiffFormat;
    use crate::std::{
        fmt::Debug,
        format,
        string::{String, ToString},
    };

    /// Default diff format.
    pub const DEFAULT_DIFF_FORMAT: DiffFormat = DIFF_FORMAT_NO_HIGHLIGHT;

    #[must_use]
    #[inline]
    pub const fn configured_diff_format_impl() -> DiffFormat {
        DEFAULT_DIFF_FORMAT
    }

    #[inline]
    pub fn mark_diff_impl(actual: &str, expected: &str, _format: &DiffFormat) -> (String, String) {
        (actual.to_string(), expected.to_string())
    }

    #[inline]
    pub fn mark_unexpected_impl<T>(value: &T, _format: &DiffFormat) -> String
    where
        T: Debug + ?Sized,
    {
        format!("{value:?}")
    }

    #[inline]
    pub fn mark_missing_impl<T>(value: &T, _format: &DiffFormat) -> String
    where
        T: Debug + ?Sized,
    {
        format!("{value:?}")
    }

    #[inline]
    pub fn mark_unexpected_string_impl(string: &str, _format: &DiffFormat) -> String {
        string.to_string()
    }

    #[inline]
    pub fn mark_missing_string_impl(string: &str, _format: &DiffFormat) -> String {
        string.to_string()
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

    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    const ENV_VAR_NO_COLOR: &str = "NO_COLOR";

    /// Environment variable to set the highlight mode.
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
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

    /// Returns a [`DiffFormat`] for the given highlight mode.
    ///
    /// Supported highlight modes are:
    ///
    /// | mode           | diff format                  |
    /// |----------------|------------------------------|
    /// | `"red-green"`  | [`DIFF_FORMAT_RED_GREEN`]    |
    /// | `"red-blue"`   | [`DIFF_FORMAT_RED_BLUE`]     |
    /// | `"red-yellow"` | [`DIFF_FORMAT_RED_YELLOW`]   |
    /// | `"bold"`       | [`DIFF_FORMAT_BOLD`]         |
    /// | `"off"`        | [`DIFF_FORMAT_NO_HIGHLIGHT`] |
    ///
    /// The mode string is case-insensitive.
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

    /// Returns true if the mode is a color mode and not "bold" or "off".
    #[cfg(feature = "std")]
    fn is_color_mode(mode: &str) -> bool {
        !matches!(
            mode.to_lowercase().as_str(),
            HIGHLIGHT_MODE_BOLD | HIGHLIGHT_MODE_OFF
        )
    }

    /// Returns true if the environment variable `NO_COLOR` is set.
    #[cfg(feature = "std")]
    fn is_no_color_env_var_set() -> bool {
        use crate::env;

        match env::var(ENV_VAR_NO_COLOR) {
            Ok(value) => !value.is_empty(),
            Err(env::VarError::NotPresent) => false,
            Err(env::VarError::NotUnicode(value)) => !value.is_empty(),
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
        use crate::env;

        match env::var(ENV_VAR_HIGHLIGHT_DIFFS) {
            Ok(value) => {
                if is_color_mode(&value) && is_no_color_env_var_set() {
                    DIFF_FORMAT_NO_HIGHLIGHT
                } else {
                    diff_format_for_mode(&value).unwrap_or_else(|| {
                        #[cfg(feature = "std")]
                        eprintln!(
                            "WARNING: the environment variable `{ENV_VAR_HIGHLIGHT_DIFFS}` is set to the unrecognized value {value:?}.\n\t=> Default highlight mode \"{DEFAULT_HIGHLIGHT_MODE}\" is used."
                        );
                        DEFAULT_DIFF_FORMAT
                    })
                }
            },
            Err(env::VarError::NotPresent) => {
                if is_no_color_env_var_set() {
                    DIFF_FORMAT_NO_HIGHLIGHT
                } else {
                    DEFAULT_DIFF_FORMAT
                }
            },
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
    pub fn mark_diff_impl(actual: &str, expected: &str, format: &DiffFormat) -> (String, String) {
        use crate::std::vec::Vec;
        use sdiff::Diff;

        let actual = actual.chars().collect::<Vec<_>>();
        let expected = expected.chars().collect::<Vec<_>>();
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
        T: Debug + ?Sized,
    {
        format!(
            "{}{value:?}{}",
            format.unexpected.start, format.unexpected.end
        )
    }

    #[inline]
    pub fn mark_missing_impl<T>(value: &T, format: &DiffFormat) -> String
    where
        T: Debug + ?Sized,
    {
        format!("{}{value:?}{}", format.missing.start, format.missing.end)
    }

    #[inline]
    pub fn mark_unexpected_string_impl(string: &str, format: &DiffFormat) -> String {
        format!(
            "{}{string}{}",
            format.unexpected.start, format.unexpected.end
        )
    }

    #[inline]
    pub fn mark_missing_string_impl(string: &str, format: &DiffFormat) -> String {
        format!("{}{string}{}", format.missing.start, format.missing.end)
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

#[cfg(test)]
mod tests;
