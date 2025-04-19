//! Implementation of assertions for `OsString` and `OsStr` values.
//!
//! `OsString` and `OsStr` are only available in std environments. Thus,
//! assertions for those types are only available with crate feature `std`
//! enabled.

use crate::properties::{IsEmptyProperty, LengthProperty};
use crate::std::ffi::{OsStr, OsString};

impl IsEmptyProperty for OsString {
    fn is_empty_property(&self) -> bool {
        self.is_empty()
    }
}

impl LengthProperty for OsString {
    fn length_property(&self) -> usize {
        self.len()
    }
}

impl IsEmptyProperty for &OsStr {
    fn is_empty_property(&self) -> bool {
        self.is_empty()
    }
}

impl LengthProperty for &OsStr {
    fn length_property(&self) -> usize {
        self.len()
    }
}

#[cfg(test)]
mod tests;
