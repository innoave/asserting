//! Implementation of assertions for `slice` values.

use crate::prelude::{IsEmptyProperty, LengthProperty};

impl<T> IsEmptyProperty for &[T] {
    fn is_empty_property(&self) -> bool {
        self.is_empty()
    }
}

impl<T> LengthProperty for &[T] {
    fn length_property(&self) -> usize {
        self.len()
    }
}

#[cfg(test)]
mod tests;
