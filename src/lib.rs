#![cfg_attr(not(any(feature = "std", test)), no_std)]

#[cfg(not(any(feature = "std", test)))]
mod std {
    pub use core::*;
}

#[cfg(any(feature = "std", test))]
mod std {
    pub use std::*;
}

pub mod assertion;
pub mod specification;

mod equality;

pub mod prelude {
    pub use super::assert_that;
    pub use super::assertion::*;
    pub use super::specification::*;
}
