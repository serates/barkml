use std::{
    ffi::OsStr,
    fs::{File, read_dir},
    io::{Read, Seek},
    path::Path,
};

use super::Loader;
use crate::{Result, error};
use crate::{
    StatementData,
    ast::Statement,
    syn::{Parser, Token},
};
use indexmap::IndexMap;
use logos::Logos;
use snafu::{OptionExt, ensure};

/// Standard loader for BarkML files
///
/// This loader supports multiple methodologies for reading and combining BarkML files:
/// - Loading individual files
/// - Loading directories of files
/// - Merging multiple files into a single module
/// - Importing files as separate modules
/// - Auto-discovering files in search paths
///
/// The loader can be configured to handle collisions between modules and to
/// enable or disable macro resolution.
pub struct StandardLoader {
    /// Map of module names to their corresponding statements
    modules: IndexMap<String, Statement>,

    /// Whether to allow collisions between modules (overwrite on conflict)
    collisions: bool,

    /// Whether to resolve macros during loading
    resolve_macros: bool,
}

impl Default for StandardLoader {
    /// Create a new loader with the default settings
    /// which is:
    ///   mode = Single
    ///   allow_collisions = false
    ///   resolve_macros = true
    fn default() -> Self {
        Self {
            modules: IndexMap::new(),
            collisions: false,
            resolve_macros: true,
        }
    }
}

impl StandardLoader {
    /// Merges the contents of the right statement into the left statement
    ///
    /// This function recursively merges two statements, handling different statement types
    /// and respecting the collision policy.
    ///
    /// # Arguments
    ///
    /// * `left` - The target statement to merge into (modified in-place)
    /// * `right` - The source statement to merge from
    /// * `is_collision_allowed` - Whether to allow collisions (overwrite on conflict)
    ///
    /// # Returns
    ///
    /// Ok(()) if the merge was successful, or an error if there was a collision
    /// and collisions are not allowed
    fn merge_into(
        left: &mut Statement,
        right: &Statement,
        is_collision_allowed: bool,
    ) -> Result<()> {
        match &right.data {
            // If the right statement is a group or labeled statement, merge its children
            StatementData::Group(right_stmts) | StatementData::Labeled(_, right_stmts) => {
                match &mut left.data {
                    // If the left statement is also a group or labeled statement, merge recursively
                    StatementData::Group(left_stmts) | StatementData::Labeled(_, left_stmts) => {
                        // Pre-allocate capacity if needed
                        if left_stmts.len() < left_stmts.len() + right_stmts.len() {
                            left_stmts.reserve(right_stmts.len());
                        }

                        // Merge each child statement
                        for (key, value) in right_stmts {
                            if let Some(target) = left_stmts.get_mut(key) {
                                // Recursive merge for existing keys
                                Self::merge_into(target, value, is_collision_allowed)?;
                            } else {
                                // Simple insert for new keys
                                left_stmts.insert(key.clone(), value.clone());
                            }
                        }
                    }
                    // If the left statement is not a group or labeled statement, replace it
                    // if collisions are allowed
                    _ => {
                        ensure!(
                            is_collision_allowed,
                            error::CollisionSnafu {
                                left_id: left.id.clone(),
                                left_location: left.meta.location.clone(),
                                right_id: right.id.clone(),
                                right_location: right.meta.location.clone()
                            }
                        );
                        *left = right.clone();
                    }
                }
            }
            // If the right statement is a single value, replace the left statement
            // if collisions are allowed
            StatementData::Single(_) => {
                ensure!(
                    is_collision_allowed,
                    error::CollisionSnafu {
                        left_id: left.id.clone(),
                        left_location: left.meta.location.clone(),
                        right_id: right.id.clone(),
                        right_location: right.meta.location.clone()
                    }
                );
                *left = right.clone();
            }
        }
        Ok(())
    }

    /// When merging or appending multiple files together
    /// tell the loader to allow collisions and overwrite the first found one with
    /// the next
    #[allow(dead_code)]
    fn allow_collisions(&mut self) -> Result<&mut Self> {
        self.collisions = true;
        Ok(self)
    }

    /// Add a module with the given name to this loader, if a module already
    /// exists by the name the modules will be merged
    pub fn add_module<R>(
        &mut self,
        name: &str,
        code: &mut R,
        filename: Option<String>,
    ) -> Result<&mut Self>
    where
        R: Read + Seek,
    {
        let filename = filename.unwrap_or(name.to_string());
        let mut module_code = String::default();
        code.read_to_string(&mut module_code)
            .map_err(|e| error::Error::Io {
                reason: e.to_string(),
            })?;
        let lexer = Token::lexer(module_code.as_str());
        let mut parser = Parser::new(filename.as_str(), lexer);
        let module = parser.parse()?;
        if let Some(left) = self.modules.get_mut(name) {
            Self::merge_into(left, &module, self.collisions)?;
        } else {
            self.modules.insert(name.to_string(), module);
        }
        Ok(self)
    }

    /// Add a single file to this loader as a new module
    ///
    /// This method reads a .bml file and adds it as a new module with the same name
    /// as the file (without extension).
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the .bml file
    ///
    /// # Returns
    ///
    /// A reference to self for method chaining, or an error if the file
    /// cannot be read or processed
    pub fn import<P>(&mut self, path: P) -> Result<&mut Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let name = basename(path)?;

        // Open and read the file
        let mut file = File::open(path).map_err(|e| error::Error::Io {
            reason: e.to_string(),
        })?;

        // Add the file as a module with its basename
        self.add_module(name.as_str(), &mut file, Some(name.clone()))
    }

    /// Add a single file to this loader and merge it into the main module
    ///
    /// This method reads a .bml file and merges its contents into the main module.
    /// Multiple files can be added this way to build up a composite configuration.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the .bml file
    ///
    /// # Returns
    ///
    /// A reference to self for method chaining, or an error if the file
    /// cannot be read or processed
    pub fn add_file<P>(&mut self, path: P) -> Result<&mut Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let name = basename(path)?;

        // Open and read the file
        let mut file = File::open(path).map_err(|e| error::Error::Io {
            reason: e.to_string(),
        })?;

        // Add the file to the "main" module, preserving the original filename for error reporting
        self.add_module("main", &mut file, Some(name))
    }

    // Add a directory to this loader and import all files as individual modules
    pub fn import_dir<P>(&mut self, path: P) -> Result<&mut Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        ensure!(
            path.try_exists().map_err(|e| error::Error::Io {
                reason: e.to_string(),
            })?,
            error::NotFoundSnafu {
                path: path.to_path_buf()
            }
        );
        let dir_reader = read_dir(path).map_err(|e| error::Error::Io {
            reason: e.to_string(),
        })?;
        let mut files = Vec::new();
        for entry in dir_reader {
            let entry = entry.map_err(|e| error::Error::Io {
                reason: e.to_string(),
            })?;
            let entry_path = entry.path();
            if entry_path.is_file() && entry_path.extension() == Some(OsStr::new("bml")) {
                files.push(entry_path.clone());
            }
        }
        files.sort();
        for file in files.iter() {
            self.import(file)?;
        }
        Ok(self)
    }

    /// Add a directory to this loader and merge all files into the main module
    ///
    /// This method reads all .bml files in the specified directory and merges them
    /// into the main module. Files are processed in alphabetical order.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the directory containing .bml files
    ///
    /// # Returns
    ///
    /// A reference to self for method chaining, or an error if the directory
    /// cannot be read or a file cannot be processed
    pub fn add_dir<P>(&mut self, path: P) -> Result<&mut Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        // Check if the path exists
        ensure!(
            path.try_exists().map_err(|e| error::Error::Io {
                reason: e.to_string(),
            })?,
            error::NotFoundSnafu {
                path: path.to_path_buf()
            }
        );

        // Read the directory
        let dir_reader = read_dir(path).map_err(|e| error::Error::Io {
            reason: e.to_string(),
        })?;

        // Collect all .bml files
        let mut files = Vec::with_capacity(10); // Pre-allocate with reasonable capacity
        for entry in dir_reader {
            let entry = entry.map_err(|e| error::Error::Io {
                reason: e.to_string(),
            })?;
            let entry_path = entry.path();
            if entry_path.is_file() && entry_path.extension() == Some(OsStr::new("bml")) {
                files.push(entry_path);
            }
        }

        // Sort files for deterministic processing order
        files.sort();

        // Process each file
        for file in &files {
            self.add_file(file)?;
        }

        Ok(self)
    }

    // Load the main module as the provided name. The name should not contain .bml
    // and should be auto-discoverable in one of the provided search paths. If
    // the loader finds a <name>.bml file, it will load a single file, however if it finds
    // a <name>.d directory it will load and merge all bml files inside that directory
    pub fn main<P>(&mut self, name: &str, search_paths: Vec<P>) -> Result<&mut Self>
    where
        P: AsRef<Path>,
    {
        // Iterate through the search paths, whichever matches first will be the winner
        for path in search_paths.iter() {
            let file_check = path.as_ref().join(name).with_extension("bml");
            if file_check.exists() && file_check.is_file() {
                return self.add_file(file_check);
            }
            let dir_check = path.as_ref().join(name).with_extension("d");
            if dir_check.exists() && dir_check.is_dir() {
                return self.add_dir(dir_check);
            }
        }
        error::SearchSnafu {
            name: name.to_string(),
            search_paths: search_paths
                .iter()
                .map(|x| x.as_ref().to_path_buf())
                .collect::<Vec<_>>(),
        }
        .fail()
    }
}

impl Loader for StandardLoader {
    /// Check if macro resolution is enabled for this loader
    fn is_resolution_enabled(&self) -> bool {
        self.resolve_macros
    }

    /// Disable macro resolution for this loader
    ///
    /// When disabled, macros in the loaded content will remain as-is.
    fn skip_macro_resolution(&mut self) -> Result<&mut Self> {
        self.resolve_macros = false;
        Ok(self)
    }

    /// Read all the configuration files and return everything as a single module
    ///
    /// This method retrieves the "main" module that has been built up through
    /// calls to add_file, add_dir, or main.
    ///
    /// # Returns
    ///
    /// The main module statement, or an error if no main module has been defined
    fn read(&self) -> Result<Statement> {
        self.modules
            .get("main")
            .cloned()
            .context(error::NoMainSnafu)
    }
}

/// Extracts the basename (filename without extension) from a path
///
/// This function handles the common case of extracting just the filename
/// without its extension from a path.
///
/// # Arguments
///
/// * `path` - The path to extract the basename from
///
/// # Returns
///
/// The basename as a String, or an error if the path is invalid
fn basename<P: AsRef<Path>>(path: P) -> Result<String> {
    let file_name = path.as_ref().file_name().context(error::BasenameSnafu)?;
    let file_name = file_name.to_str().context(error::BasenameSnafu)?;

    // Handle the case where there might not be an extension
    if let Some(extension) = path.as_ref().extension() {
        let extension = extension.to_str().context(error::BasenameSnafu)?;
        // Add a dot to properly handle the extension
        let suffix = format!(".{}", extension);
        Ok(file_name
            .strip_suffix(&suffix)
            .unwrap_or(file_name)
            .to_string())
    } else {
        // No extension
        Ok(file_name.to_string())
    }
}

#[cfg(test)]
mod test {
    use indexmap::IndexMap;
    use semver::Version;

    use super::StandardLoader;
    use crate::ast::{Location, Metadata, Statement, Value};
    use crate::load::Loader;

    #[test]
    pub fn load_single() {
        let expected = Statement::new_module(
            ".",
            IndexMap::from([
                (
                    "tire".into(),
                    Statement::new_control(
                        "tire",
                        None,
                        Value::new_version(
                            Version::new(1, 0, 0),
                            Metadata {
                                location: Location::default(),
                                comment: None,
                                label: Some("Test".into()),
                            },
                        ),
                        Metadata::default(),
                    )
                    .unwrap(),
                ),
                (
                    "section-1".into(),
                    Statement::new_section(
                        "section-1",
                        IndexMap::from([
                            (
                                "number".into(),
                                Statement::new_assign(
                                    "number",
                                    None,
                                    Value::new_int(4, Metadata::default()),
                                    Metadata {
                                        location: Location::default(),
                                        comment: Some("Documentation".into()),
                                        label: None,
                                    },
                                )
                                .unwrap(),
                            ),
                            (
                                "floating".into(),
                                Statement::new_assign(
                                    "floating",
                                    Some(crate::ValueType::F32),
                                    Value::new_f32(3.14, Metadata::default()),
                                    Metadata::default(),
                                )
                                .unwrap(),
                            ),
                            (
                                "versioning".into(),
                                Statement::new_assign(
                                    "versioning",
                                    None,
                                    Value::new_version(
                                        Version::parse("1.2.3-beta.6").unwrap(),
                                        Metadata::default(),
                                    ),
                                    Metadata::default(),
                                )
                                .unwrap(),
                            ),
                            (
                                "requires".into(),
                                Statement::new_assign(
                                    "requires",
                                    None,
                                    Value::new_require(
                                        semver::VersionReq::parse("^1.3.3").unwrap(),
                                        Metadata::default(),
                                    ),
                                    Metadata::default(),
                                )
                                .unwrap(),
                            ),
                            (
                                "strings".into(),
                                Statement::new_assign(
                                    "strings",
                                    None,
                                    Value::new_string("foobar".into(), Metadata::default()),
                                    Metadata::default(),
                                )
                                .unwrap(),
                            ),
                            (
                                "arrays".to_string(),
                                Statement::new_assign(
                                    "arrays",
                                    None,
                                    Value::new_array(
                                        vec![
                                            Value::new_string("hello".into(), Metadata::default()),
                                            Value::new_int(5, Metadata::default()),
                                            Value::new_float(3.14, Metadata::default()),
                                            Value::new_string("single".into(), Metadata::default()),
                                        ],
                                        Metadata::default(),
                                    ),
                                    Metadata::default(),
                                )
                                .unwrap(),
                            ),
                            (
                                "object".into(),
                                Statement::new_assign(
                                    "object",
                                    None,
                                    Value::new_table(
                                        IndexMap::from([
                                            (
                                                "foo".into(),
                                                Value::new_bytes(
                                                    b"binarystring".to_vec(),
                                                    Metadata::default(),
                                                ),
                                            ),
                                            ("bar".into(), Value::new_int(4, Metadata::default())),
                                        ]),
                                        Metadata::default(),
                                    ),
                                    Metadata::default(),
                                )
                                .unwrap(),
                            ),
                            (
                                "blocks.0.3.label-a".into(),
                                Statement::new_block(
                                    "blocks",
                                    vec![
                                        Value::new_float(0.3, Metadata::default()),
                                        Value::new_string(
                                            "label-a".to_string(),
                                            Metadata::default(),
                                        ),
                                    ],
                                    IndexMap::from([(
                                        "simple".into(),
                                        Statement::new_assign(
                                            "simple",
                                            None,
                                            Value::new_string("me".into(), Metadata::default()),
                                            Metadata::default(),
                                        )
                                        .unwrap(),
                                    )]),
                                    Metadata::default(),
                                ),
                            ),
                        ]),
                        Metadata::default(),
                    ),
                ),
            ]),
            Metadata::default(),
        );
        let result = StandardLoader::default()
            .main("example", vec!["examples"])
            .expect("path detection failed")
            .load()
            .expect("load failed");
        assert_eq!(result.data, expected.data);
    }

    #[test]
    pub fn load_multiple() {
        let _expected = Statement::new_module(
            ".",
            IndexMap::from([
                (
                    "tire".into(),
                    Statement::new_control(
                        "tire",
                        None,
                        Value::new_version(
                            Version::new(1, 0, 0),
                            Metadata {
                                location: Location::default(),
                                comment: None,
                                label: Some("Test".into()),
                            },
                        ),
                        Metadata::default(),
                    )
                    .unwrap(),
                ),
                (
                    "section-1".into(),
                    Statement::new_section(
                        "section-1",
                        IndexMap::from([
                            (
                                "number".into(),
                                Statement::new_assign(
                                    "number",
                                    None,
                                    Value::new_int(4, Metadata::default()),
                                    Metadata {
                                        location: Location::default(),
                                        comment: Some("Documentation".into()),
                                        label: None,
                                    },
                                )
                                .unwrap(),
                            ),
                            (
                                "floating".into(),
                                Statement::new_assign(
                                    "floating",
                                    Some(crate::ValueType::F32),
                                    Value::new_f32(3.14, Metadata::default()),
                                    Metadata::default(),
                                )
                                .unwrap(),
                            ),
                            (
                                "versioning".into(),
                                Statement::new_assign(
                                    "versioning",
                                    None,
                                    Value::new_version(
                                        Version::parse("1.2.3-beta.6").unwrap(),
                                        Metadata::default(),
                                    ),
                                    Metadata::default(),
                                )
                                .unwrap(),
                            ),
                            (
                                "requires".into(),
                                Statement::new_assign(
                                    "requires",
                                    None,
                                    Value::new_require(
                                        semver::VersionReq::parse("^1.3.3").unwrap(),
                                        Metadata::default(),
                                    ),
                                    Metadata::default(),
                                )
                                .unwrap(),
                            ),
                            (
                                "strings".into(),
                                Statement::new_assign(
                                    "strings",
                                    None,
                                    Value::new_string("foobar".into(), Metadata::default()),
                                    Metadata::default(),
                                )
                                .unwrap(),
                            ),
                            (
                                "arrays".to_string(),
                                Statement::new_assign(
                                    "arrays",
                                    None,
                                    Value::new_array(
                                        vec![
                                            Value::new_string("hello".into(), Metadata::default()),
                                            Value::new_int(5, Metadata::default()),
                                            Value::new_float(3.14, Metadata::default()),
                                            Value::new_string("single".into(), Metadata::default()),
                                        ],
                                        Metadata::default(),
                                    ),
                                    Metadata::default(),
                                )
                                .unwrap(),
                            ),
                            (
                                "object".into(),
                                Statement::new_assign(
                                    "object",
                                    None,
                                    Value::new_table(
                                        IndexMap::from([
                                            (
                                                "foo".into(),
                                                Value::new_bytes(
                                                    b"binarystring".to_vec(),
                                                    Metadata::default(),
                                                ),
                                            ),
                                            ("bar".into(), Value::new_int(4, Metadata::default())),
                                        ]),
                                        Metadata::default(),
                                    ),
                                    Metadata::default(),
                                )
                                .unwrap(),
                            ),
                            (
                                "blocks.0.3.label-a".into(),
                                Statement::new_block(
                                    "blocks",
                                    vec![
                                        Value::new_float(0.3, Metadata::default()),
                                        Value::new_string(
                                            "label-a".to_string(),
                                            Metadata::default(),
                                        ),
                                    ],
                                    IndexMap::from([(
                                        "simple".into(),
                                        Statement::new_assign(
                                            "simple",
                                            None,
                                            Value::new_string("me".into(), Metadata::default()),
                                            Metadata::default(),
                                        )
                                        .unwrap(),
                                    )]),
                                    Metadata::default(),
                                ),
                            ),
                        ]),
                        Metadata::default(),
                    ),
                ),
                (
                    "section-2".into(),
                    Statement::new_section(
                        "section-2",
                        IndexMap::from([
                            (
                                "number".into(),
                                Statement::new_assign(
                                    "number",
                                    None,
                                    Value::new_int(4, Metadata::default()),
                                    Metadata {
                                        location: Location::default(),
                                        comment: Some("Documentation".into()),
                                        label: None,
                                    },
                                )
                                .unwrap(),
                            ),
                            (
                                "floating".into(),
                                Statement::new_assign(
                                    "floating",
                                    Some(crate::ValueType::F32),
                                    Value::new_f32(3.14, Metadata::default()),
                                    Metadata::default(),
                                )
                                .unwrap(),
                            ),
                            (
                                "versioning".into(),
                                Statement::new_assign(
                                    "versioning",
                                    None,
                                    Value::new_version(
                                        Version::parse("1.2.3-beta.6").unwrap(),
                                        Metadata::default(),
                                    ),
                                    Metadata::default(),
                                )
                                .unwrap(),
                            ),
                            (
                                "requires".into(),
                                Statement::new_assign(
                                    "requires",
                                    None,
                                    Value::new_require(
                                        semver::VersionReq::parse("^1.3.3").unwrap(),
                                        Metadata::default(),
                                    ),
                                    Metadata::default(),
                                )
                                .unwrap(),
                            ),
                            (
                                "strings".into(),
                                Statement::new_assign(
                                    "strings",
                                    None,
                                    Value::new_string("foobar".into(), Metadata::default()),
                                    Metadata::default(),
                                )
                                .unwrap(),
                            ),
                            (
                                "arrays".to_string(),
                                Statement::new_assign(
                                    "arrays",
                                    None,
                                    Value::new_array(
                                        vec![
                                            Value::new_string("hello".into(), Metadata::default()),
                                            Value::new_int(5, Metadata::default()),
                                            Value::new_float(3.14, Metadata::default()),
                                            Value::new_string("single".into(), Metadata::default()),
                                        ],
                                        Metadata::default(),
                                    ),
                                    Metadata::default(),
                                )
                                .unwrap(),
                            ),
                            (
                                "object".into(),
                                Statement::new_assign(
                                    "object",
                                    None,
                                    Value::new_table(
                                        IndexMap::from([
                                            (
                                                "foo".into(),
                                                Value::new_bytes(
                                                    b"binarystring".to_vec(),
                                                    Metadata::default(),
                                                ),
                                            ),
                                            ("bar".into(), Value::new_int(4, Metadata::default())),
                                        ]),
                                        Metadata::default(),
                                    ),
                                    Metadata::default(),
                                )
                                .unwrap(),
                            ),
                            (
                                "blocks.0.3.label-a".into(),
                                Statement::new_block(
                                    "blocks",
                                    vec![
                                        Value::new_float(0.3, Metadata::default()),
                                        Value::new_string(
                                            "label-a".to_string(),
                                            Metadata::default(),
                                        ),
                                    ],
                                    IndexMap::from([(
                                        "simple".into(),
                                        Statement::new_assign(
                                            "simple",
                                            None,
                                            Value::new_string("me".into(), Metadata::default()),
                                            Metadata::default(),
                                        )
                                        .unwrap(),
                                    )]),
                                    Metadata::default(),
                                ),
                            ),
                        ]),
                        Metadata::default(),
                    ),
                ),
            ]),
            Metadata::default(),
        );
        let result = StandardLoader::default()
            .main("append", vec!["examples"])
            .expect("path detection failed")
            .load()
            .expect("load failed");
        assert_eq!(result.data, _expected.data);
    }
}
