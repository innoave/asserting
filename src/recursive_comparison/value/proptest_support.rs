//! Support for property-based testing with the [`proptest`] crate.
//!
//! This module mainly provides methods that provide [`Strategy`]s for
//! generating arbitrary values of type [`Value`] and [`Number`].

use crate::recursive_comparison::value::{F32, F64, Number, Value};
use crate::std::string::String;
use crate::std::vec;
use proptest::prelude::*;

/// Returns a [`Strategy`] for generating arbitrary values of type [`Value`].
pub fn any_value() -> impl Strategy<Value = Value> {
    prop_oneof![
        any::<bool>().prop_map(Value::Bool),
        any::<char>().prop_map(Value::Char),
        any_number().prop_map(Value::Number),
        any::<String>().prop_map(Value::String),
    ]
}

/// Returns a [`Strategy`] for generating arbitrary values of type [`Number`].
pub fn any_number() -> impl Strategy<Value = Number> {
    prop_oneof![
        any::<i8>().prop_map(Number::from),
        any::<i16>().prop_map(Number::from),
        any::<i32>().prop_map(Number::from),
        any::<i64>().prop_map(Number::from),
        any::<i128>().prop_map(Number::from),
        any::<u8>().prop_map(Number::from),
        any::<u16>().prop_map(Number::from),
        any::<u32>().prop_map(Number::from),
        any::<u64>().prop_map(Number::from),
        any::<u128>().prop_map(Number::from),
        any::<f32>().prop_map(Number::from),
        any::<f64>().prop_map(Number::from),
    ]
}

/// Returns a [`Strategy`] for generating arbitrary values of type [`F32`].
pub fn any_f32_newtype() -> impl Strategy<Value = F32> {
    any::<f32>().prop_map(F32)
}

/// Returns a [`Strategy`] for generating arbitrary values of type [`F64`].
pub fn any_f64_newtype() -> impl Strategy<Value = F64> {
    any::<f64>().prop_map(F64)
}
