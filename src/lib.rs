#![cfg_attr(not(any(feature = "std", test)), no_std)]

#[cfg(not(any(feature = "std", test)))]
extern crate alloc;

#[cfg(not(any(feature = "std", test)))]
mod std {
    pub use core::*;
}

#[cfg(any(feature = "std", test))]
mod std {
    pub use std::*;
}

pub mod spec;

pub mod prelude {
    pub use super::assert_that;
    pub use super::assertions::*;
    pub use super::spec::{assert_that, verify_that, CollectFailures, Location, PanicOnFail};
}

mod assertions;
mod boolean;
mod equality;
mod expectations;
mod option;
mod result;
mod string;
