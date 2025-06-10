//! Module for loading and processing BarkML files
//!
//! This module provides functionality for loading BarkML files from disk,
//! resolving macros, and walking through the resulting AST.

use crate::ast::{Scope, Statement};
use crate::{error, Result};

mod standard;
mod walk;

pub use standard::*;
pub use walk::*;

/// LoaderInterface defines the shared interface for structs that
/// can read and load BarkML files.
///
/// Implementations of this trait provide different strategies for loading
/// BarkML content from various sources and with different configurations.
pub trait Loader {
    /// Checks if macro resolution is enabled for this loader
    ///
    /// When enabled, macros in the loaded content will be resolved and replaced
    /// with their actual values.
    fn is_resolution_enabled(&self) -> bool;
    
    /// Disables macro resolution for this loader
    ///
    /// When disabled, macros in the loaded content will remain as-is.
    fn skip_macro_resolution(&mut self) -> Result<&mut Self>;
    
    /// Reads all BarkML configuration files according to the loader's configuration
    /// and returns the resulting module statement
    ///
    /// This method performs the actual file reading and parsing but does not
    /// resolve macros.
    fn read(&self) -> Result<Statement>;

    /// Loads everything into a module statement and resolves macros if enabled
    ///
    /// This is the main entry point for loading BarkML content. It reads the content,
    /// then optionally resolves macros.
    fn load(&self) -> Result<Statement> {
        let mut module = self.read()?;
        if self.is_resolution_enabled() {
            let mut scope = Scope::new(&module);
            module = scope.apply()?;
        }
        Ok(module)
    }
}
