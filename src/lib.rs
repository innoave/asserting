#![cfg_attr(not(any(feature = "std", test)), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
mod std {
    pub use core::*;
}

#[cfg(feature = "std")]
mod std {
    pub use std::*;
}

pub mod assertions;
pub mod expectations;
pub mod prelude;
pub mod properties;
pub mod spec;

mod boolean;
mod collection;
mod equality;
mod integer;
mod iterator;
mod length;
mod option;
mod order;
mod predicate;
mod range;
mod result;
mod slice;
mod string;
mod vec;

#[cfg(feature = "panic")]
mod panic;
