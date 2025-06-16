//! Error types for BarkML serialization.

// Standard library
use std::fmt;

// External crates
use serde::ser;
use snafu::Snafu;

/// Result type for serialization operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Serialization error types.
#[derive(Debug, Snafu, Clone, PartialEq)]
pub enum Error {
    /// Custom error message
    #[snafu(display("{message}"))]
    Message { message: String },

    /// Unsupported type error
    #[snafu(display("unsupported type: {type_name}"))]
    UnsupportedType { type_name: String },

    /// Invalid key type error
    #[snafu(display("invalid key type: expected string, found {found}"))]
    InvalidKeyType { found: String },

    /// Sequence length mismatch error
    #[snafu(display("sequence length mismatch: expected {expected}, found {found}"))]
    SequenceLengthMismatch { expected: usize, found: usize },

    /// Map key serialization error
    #[snafu(display("map key must be a string"))]
    MapKeyMustBeString,

    /// Enum variant serialization error
    #[snafu(display("enum variant '{variant}' serialization failed"))]
    EnumVariantFailed { variant: String },

    /// Nested serialization error
    #[snafu(display("nested serialization error: {source}"))]
    NestedSerialization { source: Box<Error> },
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Message {
            message: msg.to_string(),
        }
    }
}
