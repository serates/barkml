use super::types::Metadata;
use super::types::ValueType;
use crate::error;
use base64::Engine;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use snafu::OptionExt;
use std::fmt;
use uuid::Uuid;

/// Stores the actual in-memory data for a value in BarkML
///
/// This enum represents all possible data types that can be stored in a BarkML value.
/// Each variant corresponds to a specific data type in the language.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Data {
    /// String value ('text')
    String(String),
    
    /// Generic signed integer
    Signed(i64),
    /// 8-bit signed integer
    I8(i8),
    /// 16-bit signed integer
    I16(i16),
    /// 32-bit signed integer
    I32(i32),
    /// 64-bit signed integer
    I64(i64),
    /// 128-bit signed integer
    I128(i128),
    
    /// Generic unsigned integer
    Unsigned(u64),
    /// 8-bit unsigned integer
    U8(u8),
    /// 16-bit unsigned integer
    U16(u16),
    /// 32-bit unsigned integer
    U32(u32),
    /// 64-bit unsigned integer
    U64(u64),
    /// 128-bit unsigned integer
    U128(u128),
    
    /// Generic floating point number
    Float(f64),
    /// 32-bit floating point number
    F32(f32),
    /// 64-bit floating point number
    F64(f64),
    
    /// Binary data (b'base64encoded')
    Bytes(Vec<u8>),
    /// Boolean value (true/false)
    Bool(bool),
    /// Semantic version (1.2.3)
    Version(semver::Version),
    /// Version requirement (^1.2.3, ~2.0)
    Require(semver::VersionReq),
    /// Macro reference (m'name' or m!name)
    Macro(String),
    /// Symbol identifier (:symbol)
    Symbol(String),
    /// Null value
    Null,
    /// Array of values
    Array(Vec<Value>),
    /// Table (key-value mapping)
    Table(IndexMap<String, Value>),
}

/// Represents an individual value in the BarkML language
///
/// A Value is the fundamental unit of data in BarkML. It contains the actual data,
/// a unique identifier, and metadata about the value's source location and annotations.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Value {
    /// Unique identifier for the value, used for reference tracking and macro resolution
    pub uid: Uuid,

    /// The actual data contained in this value
    pub data: Data,

    /// Metadata including source location, comments, and labels
    pub meta: Metadata,
}

impl Value {
    /// Creates a new Value with the given data and metadata
    pub fn new(data: Data, meta: Metadata) -> Self {
        Self {
            uid: Uuid::now_v7(),
            data,
            meta,
        }
    }
    
    /// Creates a deep clone of this value with a new UUID
    pub fn deep_clone(&self) -> Self {
        Self {
            uid: Uuid::now_v7(),
            data: self.data.clone(),
            meta: self.meta.clone(),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

macro_rules! variant {
    (fn $fn_new: ident, $fn_as: ident, $fn_mut: ident ($data_type: ty => $key: ident)) => {
        pub fn $fn_new(data: $data_type, meta: Metadata) -> Self {
            Self {
                uid: Uuid::now_v7(),
                data: Data::$key(data.into()),
                meta,
            }
        }

        pub fn $fn_as(&self) -> Option<&$data_type> {
            match &self.data {
                Data::$key(value) => Some(value),
                _ => None,
            }
        }

        pub fn $fn_mut(&mut self) -> Option<&mut $data_type> {
            match &mut self.data {
                Data::$key(value) => Some(value),
                _ => None,
            }
        }
    };
}

macro_rules! try_into {
    ($fn_name: ident, $ty_name: ty, $vty: expr) => {
        impl<'a> TryFrom<&'a Value> for $ty_name {
            type Error = error::Error;

            fn try_from(value: &'a Value) -> std::result::Result<$ty_name, Self::Error> {
                value
                    .$fn_name()
                    .context(error::ImplicitConvertSnafu {
                        left: $vty,
                        right: value.type_of(),
                    })
                    .cloned()
            }
        }
    };
}

impl<'a> TryFrom<&'a Value> for String {
    type Error = error::Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value.type_of() {
            ValueType::String => Ok(value.as_string().unwrap().clone()),
            ValueType::Symbol => Ok(value.as_symbol().unwrap().clone()),
            vtype => error::ImplicitConvertSnafu {
                left: ValueType::String,
                right: vtype,
            }
            .fail(),
        }
    }
}

try_into!(as_i8, i8, ValueType::I8);
try_into!(as_i16, i16, ValueType::I16);
try_into!(as_i32, i32, ValueType::I32);
try_into!(as_i128, i128, ValueType::I128);
try_into!(as_u8, u8, ValueType::U8);
try_into!(as_u16, u16, ValueType::U16);
try_into!(as_u32, u32, ValueType::U32);
try_into!(as_u128, u128, ValueType::U128);
try_into!(as_f32, f32, ValueType::F32);
try_into!(as_bytes, Vec<u8>, ValueType::Bytes);
try_into!(as_bool, bool, ValueType::Bool);
try_into!(as_version, semver::Version, ValueType::Version);
try_into!(as_require, semver::VersionReq, ValueType::Require);

impl<'a> TryFrom<&'a Value> for i64 {
    type Error = error::Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        value
            .as_i64()
            .or(value.as_int())
            .context(error::ImplicitConvertSnafu {
                left: ValueType::I64,
                right: value.type_of(),
            })
            .cloned()
    }
}

impl<'a> TryFrom<&'a Value> for u64 {
    type Error = error::Error;

    fn try_from(value: &'a Value) -> std::result::Result<u64, Self::Error> {
        value
            .as_u64()
            .or(value.as_uint())
            .context(error::ImplicitConvertSnafu {
                left: ValueType::U64,
                right: value.type_of(),
            })
            .cloned()
    }
}

impl<'a> TryFrom<&'a Value> for f64 {
    type Error = error::Error;

    fn try_from(value: &'a Value) -> std::result::Result<f64, Self::Error> {
        value
            .as_f64()
            .or(value.as_float())
            .context(error::ImplicitConvertSnafu {
                left: ValueType::F64,
                right: value.type_of(),
            })
            .cloned()
    }
}

impl<'a> TryFrom<&'a Value> for Vec<Value> {
    type Error = error::Error;

    fn try_from(value: &'a Value) -> std::result::Result<Vec<Value>, Self::Error> {
        let mut result = Vec::new();
        for value in value.as_array().context(error::ImplicitConvertSnafu {
            left: ValueType::Array(Vec::new()),
            right: value.type_of(),
        })? {
            result.push(value.clone());
        }
        Ok(result)
    }
}

impl<'a> TryFrom<&'a Value> for IndexMap<String, Value> {
    type Error = error::Error;

    fn try_from(value: &'a Value) -> std::result::Result<IndexMap<String, Value>, Self::Error> {
        let mut result = IndexMap::new();
        for (key, value) in value.as_table().context(error::ImplicitConvertSnafu {
            left: ValueType::Table(IndexMap::new()),
            right: value.type_of(),
        })? {
            result.insert(key.clone(), value.clone());
        }
        Ok(result)
    }
}

impl Value {
    variant!(fn new_string, as_string, as_string_mut (String => String));
    variant!(fn new_int, as_int, as_int_mut (i64 => Signed));
    variant!(fn new_i8, as_i8, as_i8_mut (i8 => I8));
    variant!(fn new_i16, as_i16, as_i16_mut (i16 => I16));
    variant!(fn new_i32, as_i32, as_i32_mut (i32 => I32));
    variant!(fn new_i64, as_i64, as_i64_mut (i64 => I64));
    variant!(fn new_i128, as_i128, as_i128_mut (i128 => I128));
    variant!(fn new_uint, as_uint, as_uint_mut (u64 => Unsigned));
    variant!(fn new_u8, as_u8, as_u8_mut (u8 => U8));
    variant!(fn new_u16, as_u16, as_u16_mut (u16 => U16));
    variant!(fn new_u32, as_u32, as_u32_mut (u32 => U32));
    variant!(fn new_u64, as_u64, as_u64_mut (u64 => U64));
    variant!(fn new_u128, as_u128, as_u128_mut (u128 => U128));
    variant!(fn new_float, as_float, as_float_mut (f64 => Float));
    variant!(fn new_f32, as_f32, as_f32_mut (f32 => F32));
    variant!(fn new_f64, as_f64, as_f64_mut (f64 => F64));
    variant!(fn new_bytes, as_bytes, as_bytes_mut (Vec<u8> => Bytes));
    variant!(fn new_bool, as_bool, as_bool_mut (bool => Bool));
    variant!(fn new_version, as_version, as_version_mut (semver::Version => Version));
    variant!(fn new_require, as_require, as_require_mut (semver::VersionReq => Require));
    variant!(fn new_macro, as_macro, as_macro_mut (String => Macro));
    variant!(fn new_symbol, as_symbol, as_symbol_mut (String => Symbol));
    variant!(fn new_array, as_array, as_array_mut (Vec<Value> => Array));
    variant!(fn new_table, as_table, as_table_mut (IndexMap<String, Value> => Table));

    pub fn new_null(meta: Metadata) -> Self {
        Self {
            uid: Uuid::now_v7(),
            data: Data::Null,
            meta,
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self.data, Data::Null)
    }

    pub(crate) fn to_macro_string(&self) -> String {
        match &self.data {
            Data::Macro(value) | Data::Symbol(value) | Data::String(value) => value.clone(),
            Data::Array(array) => array
                .iter()
                .map(|x| x.to_macro_string())
                .collect::<Vec<_>>()
                .join(","),
            Data::Table(children) => children
                .iter()
                .map(|x| format!("{}:{}", x.0, x.1.to_macro_string()))
                .collect::<Vec<_>>()
                .join(","),
            Data::Unsigned(value) => value.to_string(),
            Data::U8(value) => value.to_string(),
            Data::U16(value) => value.to_string(),
            Data::U32(value) => value.to_string(),
            Data::U64(value) => value.to_string(),
            Data::U128(value) => value.to_string(),
            Data::Signed(value) => value.to_string(),
            Data::I8(value) => value.to_string(),
            Data::I16(value) => value.to_string(),
            Data::I32(value) => value.to_string(),
            Data::I64(value) => value.to_string(),
            Data::I128(value) => value.to_string(),
            Data::Float(value) => value.to_string(),
            Data::F32(value) => value.to_string(),
            Data::F64(value) => value.to_string(),
            Data::Bool(value) => if *value { "true" } else { "false" }.to_string(),
            Data::Bytes(value) => {
                base64::engine::general_purpose::STANDARD.encode(value.as_slice())
            }
            Data::Null => "null".to_string(),
            Data::Version(value) => value.to_string(),
            Data::Require(value) => value.to_string(),
        }
    }

    pub fn type_of(&self) -> ValueType {
        match &self.data {
            Data::String(_) => ValueType::String,
            Data::Unsigned(_) => ValueType::Unsigned,
            Data::U8(_) => ValueType::U8,
            Data::U16(_) => ValueType::U16,
            Data::U32(_) => ValueType::U32,
            Data::U64(_) => ValueType::U64,
            Data::U128(_) => ValueType::U128,
            Data::Signed(_) => ValueType::Signed,
            Data::I8(_) => ValueType::I8,
            Data::I16(_) => ValueType::I16,
            Data::I32(_) => ValueType::I32,
            Data::I64(_) => ValueType::I64,
            Data::I128(_) => ValueType::I128,
            Data::Float(_) => ValueType::Float,
            Data::F32(_) => ValueType::F32,
            Data::F64(_) => ValueType::F64,
            Data::Bytes(_) => ValueType::Bytes,
            Data::Bool(_) => ValueType::Bool,
            Data::Macro(_) => ValueType::Macro,
            Data::Symbol(_) => ValueType::Symbol,
            Data::Version(_) => ValueType::Version,
            Data::Require(_) => ValueType::Require,
            Data::Array(values) => ValueType::Array(values.iter().map(|x| x.type_of()).collect()),
            Data::Table(values) => ValueType::Table(
                values
                    .iter()
                    .map(|(k, v)| (k.clone(), v.type_of()))
                    .collect(),
            ),
            Data::Null => ValueType::Null,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(comment) = self.meta.comment.as_ref() {
            f.write_fmt(format_args!("/* {comment} /*"))?;
        }
        if let Some(label) = self.meta.label.as_ref() {
            f.write_fmt(format_args!("!{label} "))?;
        }
        match self.data.clone() {
            Data::String(value) => f.write_fmt(format_args!("'{value}'")),
            Data::Signed(value) => f.write_fmt(format_args!("{value}")),
            Data::I8(value) => f.write_fmt(format_args!("{value}i8")),
            Data::I16(value) => f.write_fmt(format_args!("{value}i16")),
            Data::I32(value) => f.write_fmt(format_args!("{value}i32")),
            Data::I64(value) => f.write_fmt(format_args!("{value}i64")),
            Data::I128(value) => f.write_fmt(format_args!("{value}i128")),
            Data::Unsigned(value) => f.write_fmt(format_args!("{value}")),
            Data::U8(value) => f.write_fmt(format_args!("{value}u8")),
            Data::U16(value) => f.write_fmt(format_args!("{value}u16")),
            Data::U32(value) => f.write_fmt(format_args!("{value}u32")),
            Data::U64(value) => f.write_fmt(format_args!("{value}u64")),
            Data::U128(value) => f.write_fmt(format_args!("{value}u128")),
            Data::Float(value) => f.write_fmt(format_args!("{value}")),
            Data::F32(value) => f.write_fmt(format_args!("{value}f32")),
            Data::F64(value) => f.write_fmt(format_args!("{value}f64")),
            Data::Bool(value) => {
                if value {
                    f.write_fmt(format_args!("true"))
                } else {
                    f.write_fmt(format_args!("false"))
                }
            }
            Data::Bytes(value) => f.write_fmt(format_args!(
                "b'{}'",
                base64::engine::general_purpose::STANDARD.encode(value.as_slice())
            )),
            Data::Macro(value) => f.write_fmt(format_args!("m!'{value}'")),
            Data::Symbol(value) => f.write_fmt(format_args!(":{value}")),
            Data::Null => f.write_fmt(format_args!("null")),
            Data::Version(value) => f.write_fmt(format_args!("{value}")),
            Data::Require(value) => f.write_fmt(format_args!("{value}")),
            Data::Array(value) => f.write_fmt(format_args!(
                "[{}]",
                value
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )),
            Data::Table(value) => f.write_fmt(format_args!(
                "{{\n{}}}",
                value
                    .iter()
                    .map(|(k, v)| format!("'{k}' = {v}"))
                    .collect::<Vec<_>>()
                    .join(",\n")
            )),
        }
    }
}
