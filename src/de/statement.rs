//! Statement deserializer implementation.

// External crates
use serde::Deserializer;
use serde::de::{self, DeserializeSeed, IntoDeserializer, MapAccess, SeqAccess, Visitor};

// Parent module
use super::ValueDeserializer;
use super::error::{self, Result};

// Local crate
use crate::{Statement, StatementData};

/// Deserializer for BarkML statements.
pub struct StatementDeserializer<'a> {
    statement: &'a Statement,
}

impl<'a> StatementDeserializer<'a> {
    /// Create a new statement deserializer.
    pub fn new(statement: &'a Statement) -> Self {
        Self { statement }
    }
}

impl<'de, 'a> Deserializer<'de> for StatementDeserializer<'a> {
    type Error = error::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self.statement.data {
            StatementData::Single(value) => {
                let value_deserializer = ValueDeserializer::new(value);
                value_deserializer.deserialize_any(visitor)
            }
            StatementData::Group(children) | StatementData::Labeled(_, children) => {
                let map = StatementMapAccess::new(children);
                visitor.visit_map(map)
            }
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self.statement.data {
            StatementData::Single(value) => {
                let value_deserializer = ValueDeserializer::new(value);
                value_deserializer.deserialize_bool(visitor)
            }
            _ => error::TypeMismatchSnafu {
                expected: "bool",
                found: "statement group".to_string(),
            }
            .fail(),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self.statement.data {
            StatementData::Single(value) => {
                let value_deserializer = ValueDeserializer::new(value);
                value_deserializer.deserialize_i8(visitor)
            }
            _ => error::TypeMismatchSnafu {
                expected: "i8",
                found: "statement group".to_string(),
            }
            .fail(),
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self.statement.data {
            StatementData::Single(value) => {
                let value_deserializer = ValueDeserializer::new(value);
                value_deserializer.deserialize_i16(visitor)
            }
            _ => error::TypeMismatchSnafu {
                expected: "i16",
                found: "statement group".to_string(),
            }
            .fail(),
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self.statement.data {
            StatementData::Single(value) => {
                let value_deserializer = ValueDeserializer::new(value);
                value_deserializer.deserialize_i32(visitor)
            }
            _ => error::TypeMismatchSnafu {
                expected: "i32",
                found: "statement group".to_string(),
            }
            .fail(),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self.statement.data {
            StatementData::Single(value) => {
                let value_deserializer = ValueDeserializer::new(value);
                value_deserializer.deserialize_i64(visitor)
            }
            _ => error::TypeMismatchSnafu {
                expected: "i64",
                found: "statement group".to_string(),
            }
            .fail(),
        }
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self.statement.data {
            StatementData::Single(value) => {
                let value_deserializer = ValueDeserializer::new(value);
                value_deserializer.deserialize_i128(visitor)
            }
            _ => error::TypeMismatchSnafu {
                expected: "i128",
                found: "statement group".to_string(),
            }
            .fail(),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self.statement.data {
            StatementData::Single(value) => {
                let value_deserializer = ValueDeserializer::new(value);
                value_deserializer.deserialize_u8(visitor)
            }
            _ => error::TypeMismatchSnafu {
                expected: "u8",
                found: "statement group".to_string(),
            }
            .fail(),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self.statement.data {
            StatementData::Single(value) => {
                let value_deserializer = ValueDeserializer::new(value);
                value_deserializer.deserialize_u16(visitor)
            }
            _ => error::TypeMismatchSnafu {
                expected: "u16",
                found: "statement group".to_string(),
            }
            .fail(),
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self.statement.data {
            StatementData::Single(value) => {
                let value_deserializer = ValueDeserializer::new(value);
                value_deserializer.deserialize_u32(visitor)
            }
            _ => error::TypeMismatchSnafu {
                expected: "u32",
                found: "statement group".to_string(),
            }
            .fail(),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self.statement.data {
            StatementData::Single(value) => {
                let value_deserializer = ValueDeserializer::new(value);
                value_deserializer.deserialize_u64(visitor)
            }
            _ => error::TypeMismatchSnafu {
                expected: "u64",
                found: "statement group".to_string(),
            }
            .fail(),
        }
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self.statement.data {
            StatementData::Single(value) => {
                let value_deserializer = ValueDeserializer::new(value);
                value_deserializer.deserialize_u128(visitor)
            }
            _ => error::TypeMismatchSnafu {
                expected: "u128",
                found: "statement group".to_string(),
            }
            .fail(),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self.statement.data {
            StatementData::Single(value) => {
                let value_deserializer = ValueDeserializer::new(value);
                value_deserializer.deserialize_f32(visitor)
            }
            _ => error::TypeMismatchSnafu {
                expected: "f32",
                found: "statement group".to_string(),
            }
            .fail(),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self.statement.data {
            StatementData::Single(value) => {
                let value_deserializer = ValueDeserializer::new(value);
                value_deserializer.deserialize_f64(visitor)
            }
            _ => error::TypeMismatchSnafu {
                expected: "f64",
                found: "statement group".to_string(),
            }
            .fail(),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self.statement.data {
            StatementData::Single(value) => {
                let value_deserializer = ValueDeserializer::new(value);
                value_deserializer.deserialize_char(visitor)
            }
            _ => error::TypeMismatchSnafu {
                expected: "char",
                found: "statement group".to_string(),
            }
            .fail(),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self.statement.data {
            StatementData::Single(value) => {
                let value_deserializer = ValueDeserializer::new(value);
                value_deserializer.deserialize_str(visitor)
            }
            _ => error::TypeMismatchSnafu {
                expected: "str",
                found: "statement group".to_string(),
            }
            .fail(),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self.statement.data {
            StatementData::Single(value) => {
                let value_deserializer = ValueDeserializer::new(value);
                value_deserializer.deserialize_bytes(visitor)
            }
            _ => error::TypeMismatchSnafu {
                expected: "bytes",
                found: "statement group".to_string(),
            }
            .fail(),
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self.statement.data {
            StatementData::Single(value) => {
                let value_deserializer = ValueDeserializer::new(value);
                value_deserializer.deserialize_option(visitor)
            }
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self.statement.data {
            StatementData::Single(value) => {
                let value_deserializer = ValueDeserializer::new(value);
                value_deserializer.deserialize_unit(visitor)
            }
            _ => error::TypeMismatchSnafu {
                expected: "unit",
                found: "statement group".to_string(),
            }
            .fail(),
        }
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self.statement.data {
            StatementData::Single(value) => {
                let value_deserializer = ValueDeserializer::new(value);
                value_deserializer.deserialize_seq(visitor)
            }
            StatementData::Group(children) | StatementData::Labeled(_, children) => {
                let seq = StatementSeqAccess::new(children);
                visitor.visit_seq(seq)
            }
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self.statement.data {
            StatementData::Single(value) => {
                let value_deserializer = ValueDeserializer::new(value);
                value_deserializer.deserialize_map(visitor)
            }
            StatementData::Group(children) | StatementData::Labeled(_, children) => {
                let map = StatementMapAccess::new(children);
                visitor.visit_map(map)
            }
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match &self.statement.data {
            StatementData::Single(value) => {
                let value_deserializer = ValueDeserializer::new(value);
                value_deserializer.deserialize_enum(name, variants, visitor)
            }
            StatementData::Group(children) | StatementData::Labeled(_, children) => {
                if children.len() == 1 {
                    let (key, stmt) = children.iter().next().unwrap();
                    visitor.visit_enum(StatementEnumAccess::new(key, stmt))
                } else {
                    error::InvalidValueSnafu {
                        value: "statement group with multiple children".to_string(),
                        expected_type: "enum",
                    }
                    .fail()
                }
            }
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(&self.statement.id)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }
}

/// Map access for statement groups
struct StatementMapAccess<'a> {
    iter: indexmap::map::Iter<'a, String, Statement>,
    value: Option<&'a Statement>,
}

impl<'a> StatementMapAccess<'a> {
    fn new(children: &'a indexmap::IndexMap<String, Statement>) -> Self {
        Self {
            iter: children.iter(),
            value: None,
        }
    }
}

impl<'de, 'a> MapAccess<'de> for StatementMapAccess<'a> {
    type Error = error::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((key, statement)) => {
                self.value = Some(statement);
                seed.deserialize(key.as_str().into_deserializer()).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        match self.value.take() {
            Some(statement) => {
                let deserializer = StatementDeserializer::new(statement);
                seed.deserialize(deserializer)
            }
            None => error::MessageSnafu {
                message: "value is missing".to_string(),
            }
            .fail(),
        }
    }
}

/// Sequence access for statement groups (treating them as ordered collections)
struct StatementSeqAccess<'a> {
    iter: indexmap::map::Values<'a, String, Statement>,
}

impl<'a> StatementSeqAccess<'a> {
    fn new(children: &'a indexmap::IndexMap<String, Statement>) -> Self {
        Self {
            iter: children.values(),
        }
    }
}

impl<'de, 'a> SeqAccess<'de> for StatementSeqAccess<'a> {
    type Error = error::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(statement) => {
                let deserializer = StatementDeserializer::new(statement);
                seed.deserialize(deserializer).map(Some)
            }
            None => Ok(None),
        }
    }
}

/// Enum access for statement-based enums
struct StatementEnumAccess<'a> {
    variant: &'a str,
    statement: &'a Statement,
}

impl<'a> StatementEnumAccess<'a> {
    fn new(variant: &'a str, statement: &'a Statement) -> Self {
        Self { variant, statement }
    }
}

impl<'de, 'a> de::EnumAccess<'de> for StatementEnumAccess<'a> {
    type Error = error::Error;
    type Variant = StatementDeserializer<'a>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        let variant = seed.deserialize(self.variant.into_deserializer())?;
        let deserializer = StatementDeserializer::new(self.statement);
        Ok((variant, deserializer))
    }
}

impl<'de, 'a> de::VariantAccess<'de> for StatementDeserializer<'a> {
    type Error = error::Error;

    fn unit_variant(self) -> Result<()> {
        match &self.statement.data {
            StatementData::Single(value) => {
                let value_deserializer = ValueDeserializer::new(value);
                value_deserializer.unit_variant()
            }
            StatementData::Group(children) | StatementData::Labeled(_, children) => {
                if children.is_empty() {
                    Ok(())
                } else {
                    error::TypeMismatchSnafu {
                        expected: "unit variant",
                        found: "non-empty statement group",
                    }
                    .fail()
                }
            }
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(self)
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }
}
