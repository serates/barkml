//! Abstract Syntax Tree module for BarkML
//!
//! This module contains the core data structures that represent the parsed BarkML language.
//! The AST is composed of statements, values, types, and scopes that together form
//! a complete representation of a BarkML document.

mod scope;
mod statement;
mod types;
mod value;

// Re-export all public items from submodules
pub use scope::*;
pub use statement::*;
pub use types::*;
pub use value::*;

/// Core AST types for BarkML
pub mod prelude {
    pub use super::{Location, Metadata, Scope, Statement, Value, ValueType};
}
