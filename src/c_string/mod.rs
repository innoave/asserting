//! Implementation of assertions for `CString` and `CStr` values.

use crate::properties::IsEmptyProperty;
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

#[cfg(test)]
mod tests;
