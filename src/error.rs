//! Error types and handling for BarkML parsing and processing.
//!
//! This module defines the comprehensive error types used throughout the BarkML library.
//! All errors implement the `snafu::Snafu` trait for rich error context and formatting.

// Standard library imports
use std::{
    num::{ParseFloatError, ParseIntError},
    path::PathBuf,
};

// External crate imports
use snafu::Snafu;

// Local crate imports
use crate::{
    ast::{Location, ValueType},
    Token,
};

/// Comprehensive error type for all BarkML operations.
///
/// This enum covers all possible error conditions that can occur during
/// BarkML parsing, loading, and processing. Each variant includes contextual
/// information to help with debugging and error reporting.
#[derive(Debug, Snafu, Clone, Default, PartialEq)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display(
        "{location} - type error: cannot assign a value of type '{right}' to a field with type '{left}'"
    ))]
    Assign {
        location: Location,
        left: ValueType,
        right: ValueType,
    },
    #[snafu(display("failed to resolve basename of path"))]
    Basename,
    #[snafu(display("{location} - invalid base64 encoding: {source}"))]
    Base64 {
        location: Location,
        source: base64::DecodeError,
    },
    #[snafu(display(
        "name collision between {left_id} ({left_location}) and {right_id} ({right_location})"
    ))]
    Collision {
        left_id: String,
        left_location: Location,
        right_id: String,
        right_location: Location,
    },
    #[snafu(display("{location} - unexpected end of file"))]
    Eof { location: Location },
    #[snafu(display("{location} - syntax error: expected {expected}, found {got}\n{context}"))]
    Expected {
        location: Location,
        expected: String,
        got: Token,
        context: String,
    },
    #[snafu(display("{location} - invalid floating point number: {source}"))]
    Float {
        location: Location,
        source: ParseFloatError,
    },
    #[snafu(display("type error: implicit conversion from '{left}' to '{right}' is not allowed"))]
    ImplicitConvert { left: ValueType, right: ValueType },
    #[snafu(display("{location} - invalid integer: {source}"))]
    Integer {
        location: Location,
        source: ParseIntError,
    },
    #[snafu(display("i/o error occurred during loading: {reason}"))]
    Io { reason: String },
    #[snafu(display("{location} - infinite loop detected during macro resolution"))]
    Loop { location: Location },
    #[snafu(display("{location} - array index out of bounds: no element at index {index}"))]
    NoElement { location: Location, index: usize },
    #[snafu(display("{location} - field not found: '{field}'"))]
    NoField { location: Location, field: String },
    #[snafu(display("{location} - field '{field}' is not a value"))]
    NoValue { location: Location, field: String },
    #[snafu(display(
        "{location} - macro resolution failed: could not locate value at path '{path}'"
    ))]
    NoMacro { location: Location, path: String },
    #[snafu(display(
        "missing main module: the standard loader requires at least one main module to load"
    ))]
    NoMain,
    #[snafu(display("file not found: '{}'", path.display()))]
    NotFound { path: PathBuf },
    #[snafu(display("{location} - not a scope with fields"))]
    NotScope { location: Location },
    #[snafu(display("{location} - recursion limit exceeded: maximum depth of {limit} reached"))]
    RecursionLimit { location: Location, limit: usize },
    #[snafu(display("{location} - invalid semantic version requirement: {reason}"))]
    Require { location: Location, reason: String },
    #[snafu(display("module not found: could not find file named {name}.bml or directory named {name}.d in any of these paths:\n{}", search_paths.iter().map(|x| x.to_string_lossy().to_string()).collect::<Vec<_>>().join("\n")))]
    Search {
        name: String,
        search_paths: Vec<PathBuf>,
    },
    #[snafu(display("unknown error occurred"))]
    #[default]
    Unknown,
    #[snafu(display("{location} - invalid semantic version: {reason}"))]
    Version { location: Location, reason: String },
}
