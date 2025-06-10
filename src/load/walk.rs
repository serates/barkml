use super::{error, Result};
use crate::ast::{Statement, Value};
use indexmap::IndexSet;
use snafu::OptionExt;

/// Walker for navigating and extracting data from BarkML statements and values
///
/// This enum provides a convenient way to traverse the AST and extract values
/// from BarkML documents. It handles both Statement and Value nodes, allowing
/// for flexible navigation through the document structure.
///
/// It is recommended to use this after loading a document with a standard loader,
/// as it provides a more ergonomic API for accessing data compared to working
/// directly with the AST.
///
pub enum Walk<'source> {
    /// A reference to a Statement node in the AST
    Statement(&'source Statement),

    /// A reference to a Value node in the AST
    Value(&'source Value),
}

impl<'source> Walk<'source> {
    /// Create a new walker over the given module statement
    ///
    /// This is typically the entry point for using the Walk API.
    /// You would create a walker from the root module statement
    /// returned by a loader.
    ///
    /// # Arguments
    ///
    /// * `module` - The root module statement to walk
    ///
    /// # Returns
    ///
    /// A new Walk instance for the given module
    pub fn new(module: &'source Statement) -> Self {
        Self::Statement(module)
    }

    /// Create a new walker over a value
    ///
    /// This is useful when you want to start walking from a specific value
    /// rather than a statement.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to walk
    ///
    /// # Returns
    ///
    /// A new Walk instance for the given value
    pub fn from_value(value: &'source Value) -> Self {
        Self::Value(value)
    }

    /// Fetch a child statement by field name
    ///
    /// This method retrieves a child statement from a parent statement.
    /// It's useful for navigating through the statement hierarchy.
    ///
    /// # Arguments
    ///
    /// * `field` - The name of the child statement to retrieve
    ///
    /// # Returns
    ///
    /// A reference to the child statement, or an error if:
    /// - The current node is not a statement with children
    /// - No child with the given name exists
    ///
    pub fn get_child(&self, field: &str) -> Result<&Statement> {
        match self {
            Self::Statement(stmt) => {
                // Get the grouped statements (if any)
                let children = stmt.get_grouped().context(error::NotScopeSnafu {
                    location: stmt.meta.location.clone(),
                })?;

                // Look up the requested field
                children.get(field).context(error::NoFieldSnafu {
                    field,
                    location: stmt.meta.location.clone(),
                })
            }
            Self::Value(value) => error::NotScopeSnafu {
                location: value.meta.location.clone(),
            }
            .fail(),
        }
    }

    /// Fetch and convert a value from a field to the requested type
    ///
    /// This is one of the most powerful methods in the Walk API. It allows you to
    /// extract a value from a field and automatically convert it to the desired type.
    /// The type conversion is handled by the TryFrom implementations for Value.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type to convert the value to (must implement TryFrom<&Value>)
    ///
    /// # Arguments
    ///
    /// * `field` - The name of the field to retrieve
    ///
    /// # Returns
    ///
    /// The value converted to type T, or an error if:
    /// - The field doesn't exist
    /// - The field doesn't contain a value
    /// - The value can't be converted to type T
    ///
    pub fn get<T>(&self, field: &str) -> Result<T>
    where
        T: TryFrom<&'source Value, Error = error::Error>,
    {
        match self {
            Self::Statement(stmt) => stmt
                .get_grouped()
                .or(stmt.get_labeled().map(|x| x.1))
                .context(error::NotScopeSnafu {
                    location: stmt.meta.location.clone(),
                })?
                .get(field)
                .context(error::NoFieldSnafu {
                    location: stmt.meta.location.clone(),
                    field,
                })?
                .get_value()
                .context(error::NoValueSnafu {
                    location: stmt.meta.location.clone(),
                    field,
                })?
                .try_into(),
            Self::Value(value) => value
                .as_table()
                .context(error::NotScopeSnafu {
                    location: value.meta.location.clone(),
                })?
                .get(field)
                .context(error::NoFieldSnafu {
                    location: value.meta.location.clone(),
                    field,
                })?
                .try_into(),
        }
    }

    /// Get the identifier of the current statement
    ///
    /// This method returns the ID of the current statement, if the walker
    /// is currently positioned at a statement. If the walker is positioned
    /// at a value, it returns None.
    ///
    /// # Returns
    ///
    /// The statement ID as a String, or None if the walker is positioned at a value
    ///
    pub fn get_id(&self) -> Option<String> {
        match self {
            Self::Statement(stmt) => Some(stmt.id.clone()),
            Self::Value(_) => None,
        }
    }

    /// Fetch and convert a block label to the requested type
    ///
    /// Block statements in BarkML can have labels, which are values that help
    /// identify or categorize the block. This method retrieves a label by its
    /// index and converts it to the requested type.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type to convert the label to (must implement TryFrom<&Value>)
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the label to retrieve (0-based)
    ///
    /// # Returns
    ///
    /// The label converted to type T, or an error if:
    /// - The current node is not a block statement
    /// - The index is out of bounds
    /// - The label can't be converted to type T
    ///
    pub fn get_label<T>(&self, index: usize) -> Result<T>
    where
        T: TryFrom<&'source Value, Error = error::Error>,
    {
        match self {
            Self::Statement(stmt) => stmt
                .get_labeled()
                .context(error::NotScopeSnafu {
                    location: stmt.meta.location.clone(),
                })?
                .0
                .get(index)
                .context(error::NoElementSnafu {
                    location: stmt.meta.location.clone(),
                    index,
                })?
                .try_into(),
            Self::Value(value) => error::NotScopeSnafu {
                location: value.meta.location.clone(),
            }
            .fail(),
        }
    }

    /// Get all section names in the current scope
    ///
    /// This method returns the names of all sections within the current scope.
    /// Sections are top-level organizational units in BarkML.
    ///
    /// # Returns
    ///
    /// A set of section names, or an error if the current node is not a statement
    /// with children
    ///
    pub fn get_sections(&self) -> Result<IndexSet<String>> {
        match self {
            Self::Statement(stmt) => Ok(stmt
                .get_grouped()
                .context(error::NotScopeSnafu {
                    location: stmt.meta.location.clone(),
                })?
                .iter()
                .filter_map(|(k, s)| {
                    if matches!(s.type_, crate::StatementType::Section(..)) {
                        Some(k.clone())
                    } else {
                        None
                    }
                })
                .collect()),
            Self::Value(value) => error::NotScopeSnafu {
                location: value.meta.location.clone(),
            }
            .fail(),
        }
    }

    /// Get all block names in the current scope, regardless of their ID
    ///
    /// This method returns the fully qualified names of all blocks within the current scope,
    /// regardless of their ID. This is useful when you want to enumerate all blocks
    /// without filtering by type.
    ///
    /// # Returns
    ///
    /// A set of block names, or an error if the current node is not a statement
    /// with children
    ///
    pub fn get_all_blocks(&self) -> Result<IndexSet<String>> {
        match self {
            Self::Statement(stmt) => Ok(stmt
                .get_grouped()
                .context(error::NotScopeSnafu {
                    location: stmt.meta.location.clone(),
                })?
                .iter()
                .filter_map(|(k, s)| {
                    if matches!(s.type_, crate::StatementType::Block { .. }) {
                        Some(k.clone())
                    } else {
                        None
                    }
                })
                .collect()),
            Self::Value(value) => error::NotScopeSnafu {
                location: value.meta.location.clone(),
            }
            .fail(),
        }
    }

    /// Get all blocks with a specific ID in the current scope
    ///
    /// This method returns the fully qualified names of all blocks within the current scope
    /// that have the specified ID. This is useful for finding all blocks of a particular type.
    ///
    /// # Arguments
    ///
    /// * `field` - The ID of the blocks to find
    ///
    /// # Returns
    ///
    /// A set of block names, or an error if the current node is not a statement
    /// with children
    ///
    pub fn get_blocks(&self, field: &str) -> Result<IndexSet<String>> {
        match self {
            Self::Statement(stmt) => Ok(stmt
                .get_grouped()
                .or(stmt.get_labeled().map(|x| x.1))
                .context(error::NotScopeSnafu {
                    location: stmt.meta.location.clone(),
                })?
                .into_iter()
                .filter_map(|(k, s)| {
                    if s.get_labeled().is_some() && s.id == field {
                        Some(k.clone())
                    } else {
                        None
                    }
                })
                .collect()),
            Self::Value(value) => error::NotScopeSnafu {
                location: value.meta.location.clone(),
            }
            .fail(),
        }
    }

    /// Create a new walker for a nested field
    ///
    /// This method allows you to navigate to a nested field and create a new walker
    /// for it. This is essential for traversing the hierarchical structure of a
    /// BarkML document.
    ///
    /// # Arguments
    ///
    /// * `field` - The name of the field to navigate to
    ///
    /// # Returns
    ///
    /// A new walker positioned at the specified field, or an error if:
    /// - The field doesn't exist
    /// - The field is not a statement or value that can be walked
    ///
    pub fn walk(&self, field: &str) -> Result<Self> {
        match self {
            Self::Statement(stmt) => Ok(
                if let Some(children) = stmt.get_grouped().or(stmt.get_labeled().map(|x| x.1)) {
                    Self::new(children.get(field).context(error::NoFieldSnafu {
                        location: stmt.meta.location.clone(),
                        field,
                    })?)
                } else {
                    Self::Value(stmt.get_value().unwrap())
                },
            ),
            Self::Value(value) => Ok(Self::Value(
                value
                    .as_table()
                    .context(error::NotScopeSnafu {
                        location: value.meta.location.clone(),
                    })?
                    .get(field)
                    .context(error::NoFieldSnafu {
                        location: value.meta.location.clone(),
                        field,
                    })?,
            )),
        }
    }
}
