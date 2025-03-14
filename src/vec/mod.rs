//! Implementation of assertions for `Vec` values.

use crate::prelude::{IsEmptyProperty, LengthProperty};
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

impl<T> IsEmptyProperty for Vec<T> {
    fn is_empty_property(&self) -> bool {
        self.is_empty()
    }
}

impl<T> LengthProperty for Vec<T> {
    fn length_property(&self) -> usize {
        self.len()
    }
}

#[cfg(test)]
mod tests;
