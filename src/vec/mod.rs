use crate::prelude::{IsEmptyProperty, LengthProperty};
#[cfg(not(any(feature = "std", test)))]
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
