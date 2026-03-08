//! Defines the [`Path`] type that addresses a field in a [`Value`].
//!
//! [`Value`]: crate::recursive_comparison::value::Value

use crate::std::borrow::Cow;
use crate::std::borrow::ToOwned;
use crate::std::fmt;
use crate::std::fmt::{Debug, Display};
use crate::std::string::String;
use crate::std::vec;
use crate::std::vec::Vec;

/// Defines a path to a field in a struct, tuple, or enum variant.
///
/// In the text representation the path to a field contains the navigation from
/// field to field. The fields are separated by a dot (`'.'`). The last field in
/// the path is the target field. A leading or trailing dot has no meaning and
/// is ignored.
///
/// Examples for paths addressing a field:
///
/// * `"name"` - addresses the field `name` on the first level of a struct
/// * `"address.zip"` - addresses the field `zip` of the embedded struct behind field `address`
///
/// The values of a tuple can be addressed by specifying the index of a value
/// inside the tuple. The index is zero-based, with an index 0 addressing the
/// first value in the tuple.
///
/// Examples for paths addressing single values in a tuple:
///
/// * `"path.to.tuple.0"` - for the first value in the tuple
/// * `"path.to.tuple.1"` - for the second value in the tuple
///
/// A path can also address an item in a sequence by specifying the index into
/// the sequence. The index is zero-based, with an index 0 addressing the first
/// item in a sequence.
///
/// Examples for paths address items in a sequence:
///
/// * `"order.items.0"` - for one item in the sequence
/// * `"order.items.1.product_id"` - for a field of the second item in the sequence
///
/// # Usage
///
/// A [`Path`] can be contructed from a string by using the [`Path::new`]
/// method, or by converting a string into a [`Path`].
///
/// Examples:
///
/// ```
/// # use asserting::recursive_comparison::path::Path;
/// let path1 = Path::new("path.to.field");
/// let path2 = Path::from("path.to.field");
///
/// assert_eq!(path1, path2);
/// ```
///
/// Another way to construct a [`Path`] is to start with an empty path and then
/// append segments as needed using the [`Path::append`] method. The segments
/// given to the `append` method should not contain any dot (`'.'`) as every given
/// string is treated as one segment. The given segments are not parsed for
/// dots.
///
/// Example:
///
/// ```
/// # use asserting::recursive_comparison::path::Path;
/// let path = Path::empty()
///         .append("order")
///         .append("items")
///         .append("0")
///         .append("product_id");
///
/// assert_eq!(path, Path::from("order.items.0.product_id"))
/// ```
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Path<'a>(Vec<Cow<'a, str>>);

impl<'a> Path<'a> {
    /// The separator used to separate segments in a path.
    pub const SEPARATOR: char = '.';

    /// Creates a new [`Path`] from a string.
    ///
    /// The string is parsed for dots (`'.'`) to get the separate segments of
    /// the path. A leading or trailing dot has no meaning and is ignored.
    pub fn new(field_path: &'a str) -> Self {
        Self::from(field_path)
    }

    /// Creates an empty [`Path`].
    pub fn empty() -> Self {
        Self(vec![])
    }

    /// Returns a slice of the segments of this path.
    pub fn segments(&self) -> &[Cow<'a, str>] {
        &self.0
    }

    /// Appends a new segment to this path and returns the resulting path as a
    /// new [`Path`].
    ///
    /// The [`Path`] is an immutable data type.
    #[must_use = "Path is immutable, so append returns a new Path with the given segment appended"]
    pub fn append(&self, segment: impl Into<Cow<'a, str>>) -> Self {
        let mut path = self.0.clone();
        path.push(segment.into());
        Self(path)
    }

    /// Returns `true` if this path starts with the given path.
    ///
    /// Returns `false` if this path does not start with the given path and if
    /// the given path is longer than this path.
    ///
    /// If the given path is empty, it returns `true` if this path is also
    /// empty, and `false` otherwise.
    pub fn starts_with(&self, other: &Self) -> bool {
        let other_len = other.0.len();
        let self_len = self.0.len();
        if other_len == 0 {
            return self_len == 0;
        }
        self_len >= other_len && self.0[..other_len] == other.0
    }
}

impl Debug for Path<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

impl Display for Path<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for field_name in &self.0 {
            if first {
                first = false;
                write!(f, "{field_name}")?;
            } else {
                write!(f, "{}{field_name}", Self::SEPARATOR)?;
            }
        }
        Ok(())
    }
}

impl<'a> From<&'a str> for Path<'a> {
    fn from(value: &'a str) -> Self {
        let field_names = value
            .split(Path::SEPARATOR)
            .filter_map(|field_name| {
                if field_name.is_empty() {
                    None
                } else {
                    Some(Cow::Borrowed(field_name))
                }
            })
            .collect::<Vec<_>>();
        Self(field_names)
    }
}

impl From<String> for Path<'_> {
    fn from(field_path: String) -> Self {
        let field_names = field_path
            .split(Path::SEPARATOR)
            .filter_map(|field_name| {
                if field_name.is_empty() {
                    None
                } else {
                    Some(Cow::Owned(field_name.to_owned()))
                }
            })
            .collect::<Vec<_>>();
        Path(field_names)
    }
}

#[cfg(test)]
mod tests;
