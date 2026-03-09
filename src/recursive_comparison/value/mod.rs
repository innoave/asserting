//! Provides the [`Value`] type - the data structure used for recursive
//! comparison.

mod map;
mod number;
#[cfg(test)]
pub mod proptest_support;

use crate::recursive_comparison::path::Path;
use crate::std::borrow::Cow;
use crate::std::fmt::{self, Debug};
use crate::std::string::{String, ToString};
use crate::std::vec::Vec;
use crate::std::{format, vec};
pub use map::Map;
pub use number::{Number, F32, F64};

/// Represents a field in a struct, tuple, or enum.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Field {
    /// The name of the field.
    pub name: Cow<'static, str>,
    /// The value of the field.
    pub value: Value,
}

impl Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = &self.name;
        let value = &self.value;
        write!(f, "{name}: {value:?}",)
    }
}

impl<N> From<(N, Value)> for Field
where
    N: Into<Cow<'static, str>>,
{
    fn from((name, value): (N, Value)) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }
}

/// The [`Value`] is a data structure that can represent a value of any type in
/// Rust, including structs, tuples, and enums.
///
/// ## Note on `usize` and `isize`
///
/// The `serde` crate does not explicitly support `usize` and `isize`, because
/// the size of those types is platform-dependent and cannot be safely
/// serialized on one platform and deserialized on another.
///
/// For recursive comparison purposes we try to convert `usize` and `isize`
/// values to `u64` and `i64` values respectively. If the conversion fails, we
/// try to convert them to `u128` and `i128` values. If both conversions fail,
/// we panic.
///
/// ## Note on `f32` and `f64`
///
/// The [`Value`] type implements the `Eq`, `Ord` and `Hash` traits. Although
/// these traits cannot be implemented for `f32` and `f64` in a mathematically
/// correct way, for the purpose of asserting whether two values are equal,
/// the naive implementation we are using is perfectly fine.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Value {
    /// A boolean (`bool`)
    Bool(bool),
    /// A character (`char`)
    Char(char),
    /// A map of `Value` keys to `Value` values
    Map(Map),
    /// A number
    Number(Number),
    /// A sequence of `Value`s
    Seq(Vec<Self>),
    /// A string (`String` or `str`)
    String(String),
    /// A struct
    Struct {
        /// The type name of the struct
        type_name: Cow<'static, str>,
        /// The fields of the struct
        fields: Vec<Field>,
    },
    /// A struct variant of an enum
    StructVariant {
        /// The type name of the enum
        type_name: Cow<'static, str>,
        /// The name of the variant of the enum
        variant: Cow<'static, str>,
        /// The fields of the struct variant
        fields: Vec<Field>,
    },
    /// A tuple of any size
    ///
    /// The field name (`Field.name`) of a value in the tuple is the index into
    /// the tuple represented as string.
    Tuple(Vec<Field>),
    /// A tuple struct of any size
    TupleStruct {
        /// The type name of the tuple struct
        type_name: Cow<'static, str>,
        /// The values in the tuple struct
        ///
        /// The field name (`Field.name`) of a value in the tuple is the index
        /// into the tuple represented as string.
        values: Vec<Field>,
    },
    /// A tuple variant of an enum
    TupleVariant {
        /// The type name of the enum
        type_name: Cow<'static, str>,
        /// The name of the variant of the enum
        variant: Cow<'static, str>,
        /// The values in the tuple variant
        ///
        /// The field name (`Field.name`) of a value in the tuple is the index
        /// into the tuple represented as string.
        values: Vec<Field>,
    },
    /// A unit variant of an enum
    UnitVariant {
        /// The type name of the enum
        type_name: Cow<'static, str>,
        /// The name of the variant of the enum
        variant: Cow<'static, str>,
    },
    /// The unit value `()`
    Unit,
}

impl Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bool(value) => write!(f, "{value}"),
            Self::Char(value) => write!(f, "{value:?}"),
            Self::Map(value) => write!(f, "{value:?}"),
            Self::Number(value) => write!(f, "{value:?}"),
            Self::Seq(value) => write!(f, "{value:?}"),
            Self::String(value) => write!(f, "{value:?}"),
            Self::Struct { type_name, fields } => {
                if type_name.is_empty() {
                    if fields.is_empty() {
                        f.write_str("{}")
                    } else {
                        let mut prefix = "{ ";
                        for Field { name, value } in fields {
                            f.write_str(prefix)?;
                            f.write_str(name)?;
                            f.write_str(": ")?;
                            value.fmt(f)?;
                            prefix = ", ";
                        }
                        f.write_str(" }")
                    }
                } else {
                    let mut debug_struct = f.debug_struct(type_name);
                    for field in fields {
                        debug_struct.field(&field.name, &field.value);
                    }
                    debug_struct.finish()
                }
            },
            Self::StructVariant {
                type_name: _,
                variant,
                fields,
            } => {
                let mut debug_struct = f.debug_struct(variant);
                for field in fields {
                    debug_struct.field(&field.name, &field.value);
                }
                debug_struct.finish()
            },
            Self::Tuple(values) => {
                let mut debug_tuple = f.debug_tuple("");
                for value in values {
                    debug_tuple.field(&value.value);
                }
                debug_tuple.finish()
            },
            Self::TupleStruct { type_name, values } => {
                let mut debug_tuple = f.debug_tuple(type_name);
                for value in values {
                    debug_tuple.field(&value.value);
                }
                debug_tuple.finish()
            },
            Self::TupleVariant {
                type_name: _,
                variant,
                values,
            } => {
                let mut debug_tuple = f.debug_tuple(variant);
                for value in values {
                    debug_tuple.field(&value.value);
                }
                debug_tuple.finish()
            },
            Self::UnitVariant {
                type_name: _,
                variant,
            } => {
                write!(f, "{variant}")
            },
            Self::Unit => f.write_str("()"),
        }
    }
}

fn parse_index(token: &str) -> Option<usize> {
    if token.starts_with('+') || (token.starts_with('0') && token.len() != 1) {
        None
    } else {
        token.parse().ok()
    }
}

impl Value {
    /// Returns the type name of the actual value at runtime.
    pub fn type_name(&self) -> Cow<'static, str> {
        match self {
            Self::Bool(_) => Cow::Borrowed("bool"),
            Self::Char(_) => Cow::Borrowed("char"),
            Self::Map(map) => map.type_name(),
            Self::Number(number) => number.type_name(),
            Self::Seq(seq) => {
                if let Some(element) = seq.first() {
                    let element_type = element.type_name();
                    Cow::Owned(format!("Vec<{element_type}>"))
                } else {
                    Cow::Borrowed("Vec<Value>")
                }
            },
            Self::String(_) => Cow::Borrowed("String"),
            Self::Tuple(values) => {
                if values.is_empty() {
                    return Cow::Borrowed("()");
                }
                let mut type_name = String::from("(");
                for field in values {
                    type_name.push_str(&field.value.type_name());
                    type_name.push_str(", ");
                }
                if values.len() > 1 {
                    type_name.pop();
                }
                type_name.pop();
                type_name.push(')');
                Cow::Owned(type_name)
            },
            Self::Struct { type_name, .. }
            | Self::StructVariant { type_name, .. }
            | Self::TupleStruct { type_name, .. }
            | Self::TupleVariant { type_name, .. }
            | Self::UnitVariant { type_name, .. } => type_name.clone(),
            Self::Unit => Cow::Borrowed("()"),
        }
    }

    /// Returns a reference to the value at the given path in this `Value`.
    ///
    /// The [`Value`] type is a tree-like data structure. The value of a field
    /// at any level can be addressed by a [`Path`]. A path describes the
    /// field names that have to be traversed to reach the desired value.
    ///
    /// A path may look like "order.id" or "customer.address.city".
    ///
    /// The values of a tuple can be addressed by their index. E.g., the path
    /// "foo.0" returns the first value of the tuple in the foo field, the
    /// path "foo.1" returns the second value, and so on.
    ///
    /// A path may also be used to index into a sequence. E.g., the path
    /// "order.items.1.amount" returns the amount of the second item in an
    /// order.
    pub fn get_path(&self, path: &Path<'_>) -> Option<&Self> {
        path.segments()
            .iter()
            .try_fold(self, |target, token| match target {
                Self::Map(map) => map.get(&Self::String(token.to_string())),
                Self::Seq(seq) => parse_index(token).and_then(|index| seq.get(index)),
                Self::Struct { fields, .. } => fields.iter().find_map(|field| {
                    if field.name == *token {
                        Some(&field.value)
                    } else {
                        None
                    }
                }),
                Self::StructVariant { fields, .. } => fields.iter().find_map(|field| {
                    if field.name == *token {
                        Some(&field.value)
                    } else {
                        None
                    }
                }),
                Self::Tuple(values) => values.iter().find_map(|field| {
                    if field.name == *token {
                        Some(&field.value)
                    } else {
                        None
                    }
                }),
                Self::TupleStruct { values, .. } => values.iter().find_map(|field| {
                    if field.name == *token {
                        Some(&field.value)
                    } else {
                        None
                    }
                }),
                Self::TupleVariant { values, .. } => values.iter().find_map(|field| {
                    if field.name == *token {
                        Some(&field.value)
                    } else {
                        None
                    }
                }),
                _ => None,
            })
    }

    /// Returns an iterator over all values of this [`Value`]. The iterator
    /// traverses the values in depth-first order.
    ///
    /// The order of the returned field values is guaranteed to be the order
    /// in which the fields are defined in the source code of a struct.
    pub fn depth_first_iter(&self) -> DepthFirstIter<'_> {
        DepthFirstIter::new(self)
    }
}

/// An iterator over all values of a [`Value`] that yields the values in
/// depth-first order.
pub struct DepthFirstIter<'a> {
    stack: Vec<(Path<'a>, &'a Value)>,
}

impl<'a> DepthFirstIter<'a> {
    fn new(value: &'a Value) -> Self {
        Self {
            stack: vec![(Path::empty(), value)],
        }
    }
}

impl<'a> Iterator for DepthFirstIter<'a> {
    type Item = (Path<'a>, &'a Value);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some((path, value)) = self.stack.pop() {
                match value {
                    Value::Struct { fields, .. } => {
                        self.stack.extend(fields.iter().rev().map(|field| {
                            let sub_path = path.append(field.name.clone());
                            (sub_path, &field.value)
                        }));
                    },
                    Value::StructVariant { fields, .. } => {
                        self.stack.extend(fields.iter().rev().map(|field| {
                            let sub_path = path.append(field.name.clone());
                            (sub_path, &field.value)
                        }));
                    },
                    Value::Tuple(values) => {
                        self.stack.extend(values.iter().rev().map(|field| {
                            let sub_path = path.append(field.name.clone());
                            (sub_path, &field.value)
                        }));
                    },
                    Value::TupleStruct { values, .. } => {
                        self.stack.extend(values.iter().rev().map(|field| {
                            let sub_path = path.append(field.name.clone());
                            (sub_path, &field.value)
                        }));
                    },
                    Value::TupleVariant { values, .. } => {
                        self.stack.extend(values.iter().rev().map(|field| {
                            let sub_path = path.append(field.name.clone());
                            (sub_path, &field.value)
                        }));
                    },
                    value @ (Value::Bool(_)
                    | Value::Char(_)
                    | Value::Map(_)
                    | Value::Number(_)
                    | Value::Seq(_)
                    | Value::String(_)
                    | Value::UnitVariant { .. }
                    | Value::Unit) => return Some((path, value)),
                }
            } else {
                return None;
            }
        }
    }
}

/// Constructs a [`Field`] with the given name and value.
pub fn field(name: impl Into<Cow<'static, str>>, value: Value) -> Field {
    Field {
        name: name.into(),
        value,
    }
}

/// Constructs a [`Value`] representing an anonymous struct.
///
/// Although Rust does not have anonymous structs, [`Value`]s created by this
/// function can be used as the expected value in the [`is_equivalent_to`]
/// assertion.
///
/// [`is_equivalent_to`]: crate::assertions::AssertEquivalence::is_equivalent_to
pub fn struct_with_fields<T>(fields: impl IntoIterator<Item = T>) -> Value
where
    T: Into<Field>,
{
    struct_("", fields)
}

/// Constructs a [`Value`] representing a struct with the given type name and
/// fields.
///
/// The fields can be created either by calling the [`field`] function or by
/// specifying a tuple of field-name and value.
///
/// # Examples
///
/// Creating a [`Value`] for a struct with fields. The fields are created using
/// the [`field`] function:
///
/// ```
/// # use asserting::recursive_comparison::value::*;
/// let value = struct_("MyDto", [
///     field("name", string("Silvia")),
///     field("age", uint8(25)),
///     field("confirmed", bool(true)),
/// ]);
/// ```
///
/// Creating a [`Value`] for a struct with fields. The fields are specified as
/// tuples of field-name and value:
///
/// ```
/// # use asserting::recursive_comparison::value::*;
/// let value = struct_("MyDto", [
///     ("name", string("Silvia")),
///     ("age", uint8(25)),
///     ("confirmed", bool(true)),
/// ]);
/// ```
pub fn struct_<T>(
    type_name: impl Into<Cow<'static, str>>,
    fields: impl IntoIterator<Item = T>,
) -> Value
where
    T: Into<Field>,
{
    Value::Struct {
        type_name: type_name.into(),
        fields: fields.into_iter().map(Into::into).collect(),
    }
}

/// Constructs a [`Value`] representing a struct-variant of an enum.
///
/// # Examples
///
/// ```
/// # use asserting::recursive_comparison::value::*;
/// let value = struct_variant("Foo", "Bar", [
///     ("name", string("Silvia")),
///     ("visit_count", uint32(12)),
/// ]);
/// ```
pub fn struct_variant<T>(
    type_name: impl Into<Cow<'static, str>>,
    variant: impl Into<Cow<'static, str>>,
    fields: impl IntoIterator<Item = T>,
) -> Value
where
    T: Into<Field>,
{
    Value::StructVariant {
        type_name: type_name.into(),
        variant: variant.into(),
        fields: fields.into_iter().map(Into::into).collect(),
    }
}

/// Constructs a [`Value`] representing a tuple-struct.
///
/// # Examples
///
/// ```
/// # use asserting::recursive_comparison::value::*;
/// let value = tuple_struct("Velocity", [
///     float32(1.2)
/// ]);
/// ```
pub fn tuple_struct(
    type_name: impl Into<Cow<'static, str>>,
    values: impl IntoIterator<Item = Value>,
) -> Value {
    Value::TupleStruct {
        type_name: type_name.into(),
        values: values
            .into_iter()
            .enumerate()
            .map(|(index, value)| Field {
                name: index.to_string().into(),
                value,
            })
            .collect(),
    }
}

/// Constructs a [`Value`] representing a tuple.
///
/// # Examples
///
/// ```
/// # use asserting::recursive_comparison::value::*;
/// let value = tuple([
///     string("Dog"),
///     bool(false),
///     int16(144),
/// ]);
/// ```
pub fn tuple(values: impl IntoIterator<Item = Value>) -> Value {
    let fields = values
        .into_iter()
        .enumerate()
        .map(|(index, value)| Field {
            name: index.to_string().into(),
            value,
        })
        .collect::<Vec<_>>();
    if fields.is_empty() {
        return Value::Unit;
    }
    Value::Tuple(fields)
}

/// Constructs a [`Value`] representing a tuple-variant of an enum.
///
/// # Examples
///
/// ```
/// # use asserting::recursive_comparison::value::*;
/// use asserting::recursive_comparison::value::Value::TupleStruct;
/// let value = tuple_variant("Animal", "Cat", [
///     string("Mimi"),
///     uint8(7),
///     tuple_variant("Color", "Rgb", [uint8(200), uint8(180), uint8(26)]),
/// ]);
/// ```
pub fn tuple_variant(
    type_name: impl Into<Cow<'static, str>>,
    variant: impl Into<Cow<'static, str>>,
    values: impl IntoIterator<Item = Value>,
) -> Value {
    Value::TupleVariant {
        type_name: type_name.into(),
        variant: variant.into(),
        values: values
            .into_iter()
            .enumerate()
            .map(|(index, value)| Field {
                name: index.to_string().into(),
                value,
            })
            .collect(),
    }
}

/// Constructs a [`Value`] representing a unit struct value.
///
/// # Examples
///
/// ```
/// # use serde::Serialize;
/// # use asserting::recursive_comparison::serialize::to_recursive_value;
/// # use asserting::recursive_comparison::value::unit_struct;
/// #[derive(Serialize)]
/// struct Foo;
///
/// assert_eq!(to_recursive_value(&Foo), Ok(unit_struct("Foo")));
/// ```
pub fn unit_struct(type_name: impl Into<Cow<'static, str>>) -> Value {
    Value::Struct {
        type_name: type_name.into(),
        fields: vec![],
    }
}

/// Constructs a [`Value`] representing a unit variant of an enum.
///
/// # Examples
///
/// ```
/// # use asserting::recursive_comparison::value::unit_variant;
/// # use asserting::recursive_comparison::serialize::to_recursive_value;
/// let maybe: Option<u8> = None;
///
/// assert_eq!(to_recursive_value(&maybe), Ok(unit_variant("Option", "None")));
/// ```
pub fn unit_variant(
    type_name: impl Into<Cow<'static, str>>,
    variant: impl Into<Cow<'static, str>>,
) -> Value {
    Value::UnitVariant {
        type_name: type_name.into(),
        variant: variant.into(),
    }
}

/// Constructs a [`Value`] representing the unit type `()`.
pub fn unit() -> Value {
    Value::Unit
}

/// Constructs a [`Value`] representing an array of values.
///
/// Arrays are represented as tuples in a [`Value`]. This is because an array
/// has a fixed size known at compile time and `serde` serializes fixed-sized
/// types as tuples.
pub fn array(values: impl IntoIterator<Item = Value>) -> Value {
    tuple(values)
}

/// Constructs a [`Value`] representing the given `bool` value.
pub fn bool(value: bool) -> Value {
    Value::Bool(value)
}

/// Constructs a [`Value`] representing the given `i8` value.
pub fn int8(value: i8) -> Value {
    Value::Number(value.into())
}

/// Constructs a [`Value`] representing the given `i16` value.
pub fn int16(value: i16) -> Value {
    Value::Number(value.into())
}

/// Constructs a [`Value`] representing the given `i32` value.
pub fn int32(value: i32) -> Value {
    Value::Number(value.into())
}

/// Constructs a [`Value`] representing the given `i64` value.
pub fn int64(value: i64) -> Value {
    Value::Number(value.into())
}

/// Constructs a [`Value`] representing the given `i128` value.
pub fn int128(value: i128) -> Value {
    Value::Number(value.into())
}

/// Constructs a [`Value`] representing the given `isize` value.
///
/// As `serde` does not support `isize`, this method tries to convert the given
/// value to `i64`. If the conversion fails, it tries to convert it to `i128`.
/// If both conversions fail, it panics.
///
/// # Panics
///
/// This method panics if the given `isize` value cannot be converted to `i64`
/// or `i128`.
pub fn isize(value: isize) -> Value {
    Result::unwrap_or_else(i64::try_from(value).map(int64), |_| {
        i128::try_from(value).map_or_else(
            |err| panic!("can not convert isize to `Value`: {err}"),
            int128,
        )
    })
}

/// Constructs a [`Value`] representing the given `u8` value.
pub fn uint8(value: u8) -> Value {
    Value::Number(value.into())
}

/// Constructs a [`Value`] representing the given `u16` value.
pub fn uint16(value: u16) -> Value {
    Value::Number(value.into())
}

/// Constructs a [`Value`] representing the given `u32` value.
pub fn uint32(value: u32) -> Value {
    Value::Number(value.into())
}

/// Constructs a [`Value`] representing the given `u64` value.
pub fn uint64(value: u64) -> Value {
    Value::Number(value.into())
}

/// Constructs a [`Value`] representing the given `u128` value.
pub fn uint128(value: u128) -> Value {
    Value::Number(value.into())
}

/// Constructs a [`Value`] representing the given `usize` value.
///
/// As `serde` does not support `usize`, this method tries to convert the given
/// value to `u64`. If the conversion fails, it tries to convert it to `u128`.
/// If both conversions fail, it panics.
///
/// # Panics
///
/// This method panics if the given `usize` value cannot be converted to `u64`
/// or `u128`.
pub fn usize(value: usize) -> Value {
    Result::unwrap_or_else(u64::try_from(value).map(uint64), |_| {
        u128::try_from(value).map_or_else(
            |err| panic!("can not convert usize to `Value`: {err}"),
            uint128,
        )
    })
}

/// Constructs a [`Value`] representing the given `f32` value.
pub fn float32(value: f32) -> Value {
    Value::Number(value.into())
}

/// Constructs a [`Value`] representing the given `f64` value.
pub fn float64(value: f64) -> Value {
    Value::Number(value.into())
}

/// Constructs a [`Value`] representing the given `char` value.
pub fn char(value: char) -> Value {
    Value::Char(value)
}

/// Constructs a [`Value`] representing the given `String` or `&str` value.
pub fn string(value: impl Into<String>) -> Value {
    Value::String(value.into())
}

/// Constructs a [`Value`] representing the given sequence of `Value` values.
pub fn seq(values: impl IntoIterator<Item = Value>) -> Value {
    Value::Seq(Vec::from_iter(values))
}

/// Constructs a [`Value`] representing the given map of key-value pairs.
pub fn map(values: impl IntoIterator<Item = (Value, Value)>) -> Value {
    Value::Map(Map::from_iter(values))
}

/// Constructs a [`Value`] representing the given value as the `Some` variant of
/// an `Option<Value>`.
pub fn some(value: Value) -> Value {
    Value::TupleVariant {
        type_name: "Option".into(),
        variant: "Some".into(),
        values: vec![Field {
            name: "0".into(),
            value,
        }],
    }
}

/// Constructs a [`Value`] representing the `None` value of an `Option<Value>`.
pub fn none() -> Value {
    Value::UnitVariant {
        type_name: "Option".into(),
        variant: "None".into(),
    }
}

#[cfg(test)]
mod tests;
