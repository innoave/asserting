//! Implementation of assertions for `Vec` values.

use crate::properties::{IsEmptyProperty, LengthProperty};
use crate::std::vec::Vec;

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
