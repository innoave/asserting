mod map;
mod number;
#[cfg(test)]
pub mod proptest_support;

pub use map::Map;
pub use number::{Number, F32, F64};

use crate::recursive_comparison::path::Path;
use crate::std::borrow::Cow;
use crate::std::fmt::{self, Debug};
use crate::std::string::String;

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Field {
    pub name: Cow<'static, str>,
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

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Value {
    Bool(bool),
    Char(char),
    Map(Map),
    Number(Number),
    Seq(Vec<Self>),
    String(String),
    Struct {
        type_name: Cow<'static, str>,
        fields: Vec<Field>,
    },
    StructVariant {
        type_name: Cow<'static, str>,
        variant: Cow<'static, str>,
        fields: Vec<Field>,
    },
    Tuple(Vec<Field>),
    TupleStruct {
        type_name: Cow<'static, str>,
        values: Vec<Field>,
    },
    TupleVariant {
        type_name: Cow<'static, str>,
        variant: Cow<'static, str>,
        values: Vec<Field>,
    },
    UnitVariant {
        type_name: Cow<'static, str>,
        variant: Cow<'static, str>,
    },
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
                    let mut prefix = "{ ";
                    for Field { name, value } in fields {
                        f.write_str(prefix)?;
                        f.write_str(name)?;
                        f.write_str(": ")?;
                        value.fmt(f)?;
                        prefix = ", ";
                    }
                    f.write_str(" }")
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

    pub fn depth_first_iter(&self) -> DepthFirstIter<'_> {
        DepthFirstIter::new(self)
    }
}

pub struct DepthFirstIter<'a> {
    stack: Vec<(Path<'a>, &'a Value)>,
}

impl<'a> DepthFirstIter<'a> {
    pub fn new(value: &'a Value) -> Self {
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

pub fn field(name: impl Into<Cow<'static, str>>, value: Value) -> Field {
    Field {
        name: name.into(),
        value,
    }
}

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

pub fn tuple(values: impl IntoIterator<Item = Value>) -> Value {
    Value::Tuple(
        values
            .into_iter()
            .enumerate()
            .map(|(index, value)| Field {
                name: index.to_string().into(),
                value,
            })
            .collect(),
    )
}

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

pub fn unit_struct(type_name: impl Into<Cow<'static, str>>) -> Value {
    Value::Struct {
        type_name: type_name.into(),
        fields: vec![],
    }
}

pub fn unit_variant(
    type_name: impl Into<Cow<'static, str>>,
    variant: impl Into<Cow<'static, str>>,
) -> Value {
    Value::UnitVariant {
        type_name: type_name.into(),
        variant: variant.into(),
    }
}

pub fn unit() -> Value {
    Value::Unit
}

pub fn array(values: impl IntoIterator<Item = Value>) -> Value {
    tuple(values)
}

pub fn bool(value: bool) -> Value {
    Value::Bool(value)
}

pub fn int8(value: i8) -> Value {
    Value::Number(value.into())
}

pub fn int16(value: i16) -> Value {
    Value::Number(value.into())
}

pub fn int32(value: i32) -> Value {
    Value::Number(value.into())
}

pub fn int64(value: i64) -> Value {
    Value::Number(value.into())
}

pub fn int128(value: i128) -> Value {
    Value::Number(value.into())
}

pub fn isize(value: isize) -> Value {
    Result::unwrap_or_else(i64::try_from(value).map(int64), |_| {
        i128::try_from(value).map_or_else(
            |err| panic!("can not convert isize to `Value`: {err}"),
            int128,
        )
    })
}

pub fn uint8(value: u8) -> Value {
    Value::Number(value.into())
}

pub fn uint16(value: u16) -> Value {
    Value::Number(value.into())
}

pub fn uint32(value: u32) -> Value {
    Value::Number(value.into())
}

pub fn uint64(value: u64) -> Value {
    Value::Number(value.into())
}

pub fn uint128(value: u128) -> Value {
    Value::Number(value.into())
}

pub fn usize(value: usize) -> Value {
    Result::unwrap_or_else(u64::try_from(value).map(uint64), |_| {
        u128::try_from(value).map_or_else(
            |err| panic!("can not convert usize to `Value`: {err}"),
            uint128,
        )
    })
}

pub fn float32(value: f32) -> Value {
    Value::Number(value.into())
}

pub fn float64(value: f64) -> Value {
    Value::Number(value.into())
}

pub fn char(value: char) -> Value {
    Value::Char(value)
}

pub fn string(value: impl Into<String>) -> Value {
    Value::String(value.into())
}

pub fn seq(values: impl IntoIterator<Item = Value>) -> Value {
    Value::Seq(Vec::from_iter(values))
}

pub fn map(values: impl IntoIterator<Item = (Value, Value)>) -> Value {
    Value::Map(Map::from_iter(values))
}

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

pub fn none() -> Value {
    Value::UnitVariant {
        type_name: "Option".into(),
        variant: "None".into(),
    }
}

#[cfg(test)]
mod tests;
