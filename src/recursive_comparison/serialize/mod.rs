//! Serialization of any type to a [`Value`] using `serde`.

use crate::recursive_comparison::value::{Field, Number};
use crate::recursive_comparison::value::{Map, Value};
use crate::std::borrow::Cow;
use crate::std::error::Error as StdError;
use crate::std::fmt::{self, Display};
use serde_core::ser::Error as SerdeError;
use serde_core::{ser, Serialize, Serializer};

/// Serializes the given object of some type into a [`Value`]. The given type
/// must implement [`serde::Serialize`].
///
/// # Errors
///
/// This method returns a `Result` as of the API of `serde`. As the given object
/// is serialized into an in-memory representation, there are practically no
/// errors that can occur.
///
/// [`serde::Serialize`]: Serialize
pub fn to_recursive_values<T>(object: &T) -> Result<Value, Error>
where
    T: Serialize + ?Sized,
{
    object.serialize(SerializeValue)
}

/// The error type used when serializing objects to a [`Value`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// A custom error (as required by [`serde::Serialize`])
    ///
    /// [`serde::Serialize`]: Serialize
    Message(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Message(message) => write!(f, "{message}"),
        }
    }
}

impl StdError for Error {}

impl SerdeError for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::Message(msg.to_string())
    }
}

struct SerializeValue;

impl Serializer for SerializeValue {
    type Ok = Value;
    type Error = Error;
    type SerializeSeq = SerializeSeq;
    type SerializeTuple = SerializeTuple;
    type SerializeTupleStruct = SerializeTupleStruct;
    type SerializeTupleVariant = SerializeTupleVariant;
    type SerializeMap = SerializeMap;
    type SerializeStruct = SerializeStruct;
    type SerializeStructVariant = SerializeStructVariant;

    fn serialize_bool(self, value: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Bool(value))
    }

    fn serialize_i8(self, value: i8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(value.into()))
    }

    fn serialize_i16(self, value: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(value.into()))
    }

    fn serialize_i32(self, value: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(value.into()))
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(value.into()))
    }

    fn serialize_i128(self, value: i128) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(value.into()))
    }

    fn serialize_u8(self, value: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(value.into()))
    }

    fn serialize_u16(self, value: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(value.into()))
    }

    fn serialize_u32(self, value: u32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(value.into()))
    }

    fn serialize_u64(self, value: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(value.into()))
    }

    fn serialize_u128(self, value: u128) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(value.into()))
    }

    fn serialize_f32(self, value: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(value.into()))
    }

    fn serialize_f64(self, value: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Number(value.into()))
    }

    fn serialize_char(self, value: char) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Char(value))
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::String(value.into()))
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Seq(
            value
                .iter()
                .map(|v| Value::Number(Number::U8(*v)))
                .collect(),
        ))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::UnitVariant {
            type_name: "Option".into(),
            variant: "None".into(),
        })
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let value = value.serialize(Self)?;
        Ok(Value::TupleVariant {
            type_name: "Option".into(),
            variant: "Some".into(),
            values: vec![Field {
                name: "0".into(),
                value,
            }],
        })
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Unit)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Struct {
            type_name: name.into(),
            fields: Vec::new(),
        })
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(Value::UnitVariant {
            type_name: name.into(),
            variant: variant.into(),
        })
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Ok(Value::TupleStruct {
            type_name: name.into(),
            values: vec![Field {
                name: "0".into(),
                value: value.serialize(Self)?,
            }],
        })
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Ok(Value::TupleVariant {
            type_name: name.into(),
            variant: variant.into(),
            values: vec![Field {
                name: "0".into(),
                value: value.serialize(Self)?,
            }],
        })
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SerializeSeq {
            elements: len.map(Vec::with_capacity).unwrap_or_default(),
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(SerializeTuple {
            values: Vec::with_capacity(len),
            next_index: 0,
        })
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(SerializeTupleStruct {
            type_name: name.into(),
            values: Vec::with_capacity(len),
            next_index: 0,
        })
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(SerializeTupleVariant {
            type_name: name.into(),
            variant: variant.into(),
            values: Vec::with_capacity(len),
            next_index: 0,
        })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(SerializeMap {
            map: len.map(Map::with_capacity).unwrap_or_default(),
            next_key: None,
        })
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(SerializeStruct {
            struct_name: name.into(),
            fields: Vec::with_capacity(len),
        })
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(SerializeStructVariant {
            type_name: name.into(),
            variant: variant.into(),
            values: Vec::with_capacity(len),
        })
    }
}

struct SerializeSeq {
    elements: Vec<Value>,
}

impl ser::SerializeSeq for SerializeSeq {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let value = value.serialize(SerializeValue)?;
        self.elements.push(value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Seq(self.elements))
    }
}

struct SerializeMap {
    map: Map,
    next_key: Option<Value>,
}

impl ser::SerializeMap for SerializeMap {
    type Ok = Value;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let key = key.serialize(SerializeValue)?;
        self.next_key = Some(key);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let key = self.next_key.take();
        // Panic because this indicates a bug in the program rather than an expected failure.
        let key = key.unwrap_or_else(|| panic!("serialize_value called before serialize_key"));
        let value = value.serialize(SerializeValue)?;
        self.map.insert(key, value);
        Ok(())
    }

    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
    where
        K: ?Sized + Serialize,
        V: ?Sized + Serialize,
    {
        let key = key.serialize(SerializeValue)?;
        let value = value.serialize(SerializeValue)?;
        self.map.insert(key, value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Map(self.map))
    }
}

struct SerializeTuple {
    values: Vec<Field>,
    next_index: usize,
}

impl ser::SerializeTuple for SerializeTuple {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let name = self.next_index.to_string().into();
        self.next_index += 1;
        let value = value.serialize(SerializeValue)?;
        self.values.push(Field { name, value });
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Tuple(self.values))
    }
}

struct SerializeStruct {
    struct_name: Cow<'static, str>,
    fields: Vec<Field>,
}

impl ser::SerializeStruct for SerializeStruct {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let value = value.serialize(SerializeValue)?;
        self.fields.push(Field {
            name: key.into(),
            value,
        });
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Struct {
            type_name: self.struct_name,
            fields: self.fields,
        })
    }
}

struct SerializeTupleStruct {
    type_name: Cow<'static, str>,
    values: Vec<Field>,
    next_index: usize,
}

impl ser::SerializeTupleStruct for SerializeTupleStruct {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let name = self.next_index.to_string().into();
        self.next_index += 1;
        let value = value.serialize(SerializeValue)?;
        self.values.push(Field { name, value });
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::TupleStruct {
            type_name: self.type_name,
            values: self.values,
        })
    }
}

struct SerializeStructVariant {
    type_name: Cow<'static, str>,
    variant: Cow<'static, str>,
    values: Vec<Field>,
}

impl ser::SerializeStructVariant for SerializeStructVariant {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let name = key.into();
        let value = value.serialize(SerializeValue)?;
        self.values.push(Field { name, value });
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::StructVariant {
            type_name: self.type_name,
            variant: self.variant,
            fields: self.values,
        })
    }
}

struct SerializeTupleVariant {
    type_name: Cow<'static, str>,
    variant: Cow<'static, str>,
    values: Vec<Field>,
    next_index: usize,
}

impl ser::SerializeTupleVariant for SerializeTupleVariant {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let name = self.next_index.to_string().into();
        self.next_index += 1;
        let value = value.serialize(SerializeValue)?;
        self.values.push(Field { name, value });
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::TupleVariant {
            type_name: self.type_name,
            variant: self.variant,
            values: self.values,
        })
    }
}

#[cfg(test)]
mod tests;
