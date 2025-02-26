#![cfg_attr(not(any(feature = "std", test)), no_std)]

#[cfg(not(any(feature = "std", test)))]
mod std {
    pub use core::*;
}

#[cfg(any(feature = "std", test))]
mod std {
    pub use std::*;
}

pub mod assertions;
pub mod spec;

pub mod prelude {
    pub use super::assert_that;
    pub use super::assertions::*;
    pub use super::spec::*;
}

mod boolean;
mod equality;
mod option;
