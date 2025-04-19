//! Implementation of assertions for `CString` and `CStr` values.

use crate::properties::{IsEmptyProperty, LengthProperty};
use crate::std::ffi::{CStr, CString};

impl IsEmptyProperty for CString {
    fn is_empty_property(&self) -> bool {
        self.is_empty()
    }
}

impl IsEmptyProperty for &CStr {
    fn is_empty_property(&self) -> bool {
        self.is_empty()
    }
}

impl LengthProperty for CString {
    fn length_property(&self) -> usize {
        self.as_bytes().len()
    }
}

impl LengthProperty for &CStr {
    fn length_property(&self) -> usize {
        self.to_bytes().len()
    }
}

#[cfg(test)]
mod tests;
