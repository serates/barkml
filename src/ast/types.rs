use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Defines the type of a given value
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum ValueType {
    /// String values
    String,

    /// Non-precise signed integer (defaults to 64-bit)
    Signed,
    /// 8-bit signed integer
    I8,
    /// 16-bit signed integer
    I16,
    /// 32-bit signed integer
    I32,
    /// 64-bit signed integer
    I64,
    /// 128-bit signed integer
    I128,

    /// Non-precise unsigned integer (defaults to 64-bit)
    Unsigned,
    /// 8-bit unsigned integer
    U8,
    /// 16-bit unsigned integer
    U16,
    /// 32-bit unsigned integer
    U32,
    /// 64-bit unsigned integer
    U64,
    /// 128-bit unsigned integer
    U128,

    /// Non-precise floating point (defaults to 64-bit)
    Float,
    /// 32-bit floating point
    F32,
    /// 64-bit floating point
    F64,

    /// Byte Array (Vec<u8>)
    Bytes,

    /// Boolean
    Bool,

    /// Semantic Version
    Version,
    /// Semantic Version Requirement
    Require,

    /// Macro string
    Macro,

    /// Label identifier
    Label,

    /// :Symbol identifier
    Symbol,

    /// Null
    Null,

    /// Array
    Array(Vec<Self>),

    /// Table
    Table(IndexMap<String, Self>),
}

impl ValueType {
    /// Checks if the right type can be assigned to the left when a type requirement
    /// is not specified.
    ///
    /// This function implements the type compatibility rules for BarkML, determining
    /// whether a value of one type can be assigned to a variable of another type.
    ///
    /// # Arguments
    ///
    /// * `right` - The type of the value being assigned
    ///
    /// # Returns
    ///
    /// `true` if the assignment is valid, `false` otherwise
    pub fn can_assign(&self, right: &Self) -> bool {
        match self {
            // String types are only compatible with other strings
            Self::String => matches!(right, Self::String),
            
            // I64 can accept signed integers and unsigned integers that fit
            Self::I64 => matches!(right, Self::I64 | Self::Signed | Self::Unsigned),
            
            // Generic signed integer can accept any integer type
            Self::Signed => matches!(
                right,
                Self::I8
                    | Self::I16
                    | Self::I32
                    | Self::I64
                    | Self::Signed
                    | Self::U8
                    | Self::U16
                    | Self::U32
                    | Self::U64
                    | Self::Unsigned
            ),
            
            // U64 can accept unsigned integers
            Self::U64 => matches!(right, Self::U64 | Self::Unsigned),
            
            // F64 can accept F64 or generic float
            Self::F64 => matches!(right, Self::F64 | Self::Float),
            
            // Generic unsigned integer can accept any unsigned integer type
            Self::Unsigned => matches!(
                right,
                Self::U8 | Self::U16 | Self::U32 | Self::U64 | Self::Unsigned
            ),
            
            // Generic float can accept any float type
            Self::Float => matches!(right, Self::Float | Self::F64 | Self::F32),
            
            // All other types require exact match
            value => value == right,
        }
    }
    
    /// Determines if this type is a numeric type
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            Self::Signed | Self::Unsigned | Self::I8 | Self::I16 | Self::I32 | Self::I64 | Self::I128 |
            Self::U8 | Self::U16 | Self::U32 | Self::U64 | Self::U128 | Self::Float | Self::F32 | Self::F64
        )
    }
    
    /// Determines if this type is an integer type
    pub fn is_integer(&self) -> bool {
        matches!(
            self,
            Self::Signed | Self::Unsigned | Self::I8 | Self::I16 | Self::I32 | Self::I64 | Self::I128 |
            Self::U8 | Self::U16 | Self::U32 | Self::U64 | Self::U128
        )
    }
    
    /// Determines if this type is a floating point type
    pub fn is_float(&self) -> bool {
        matches!(self, Self::Float | Self::F32 | Self::F64)
    }
    
    /// Determines if this type is a compound type (array or table)
    pub fn is_compound(&self) -> bool {
        matches!(self, Self::Array(_) | Self::Table(_))
    }
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String => f.write_str("string"),
            Self::Signed => f.write_str("int"),
            Self::I8 => f.write_str("i8"),
            Self::I16 => f.write_str("i16"),
            Self::I32 => f.write_str("i32"),
            Self::I64 => f.write_str("i64"),
            Self::I128 => f.write_str("i128"),
            Self::Unsigned => f.write_str("uint"),
            Self::U8 => f.write_str("u8"),
            Self::U16 => f.write_str("u16"),
            Self::U32 => f.write_str("u32"),
            Self::U64 => f.write_str("u64"),
            Self::U128 => f.write_str("u128"),
            Self::Float => f.write_str("float"),
            Self::F32 => f.write_str("f32"),
            Self::F64 => f.write_str("f64"),
            Self::Bytes => f.write_str("bytes"),
            Self::Bool => f.write_str("bool"),
            Self::Version => f.write_str("version"),
            Self::Require => f.write_str("require"),
            Self::Macro => f.write_str("macro"),
            Self::Label => f.write_str("label"),
            Self::Symbol => f.write_str("symbol"),
            Self::Null => f.write_str("null"),
            Self::Array(children) => f.write_fmt(format_args!(
                "[{}]",
                children
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )),
            Self::Table(children) => f.write_fmt(format_args!(
                "{{ {} }}",
                children
                    .iter()
                    .map(|(k, v)| format!("{k}: {v}"))
                    .collect::<Vec<_>>()
                    .join(",\n")
            )),
        }
    }
}

/// Represents the type of a statement in the BarkML language
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub enum StatementType {
    /// Control statement ($identifier = value)
    /// Stores the expected type of the value
    Control(ValueType),
    
    /// Assignment statement (identifier = value)
    /// Stores the expected type of the value
    Assignment(ValueType),
    
    /// Block statement (identifier labels { statements })
    /// Stores the types of labels and contents
    Block {
        /// Types of the label values
        labels: Vec<ValueType>,
        /// Types of the contained statements
        contents: IndexMap<String, Self>,
    },
    
    /// Section statement ([identifier] statements)
    /// Stores the types of contained statements
    Section(IndexMap<String, Self>),
    
    /// Module statement (top-level container)
    /// Stores the types of contained statements
    Module(IndexMap<String, Self>),
}

/// Stores the metadata associated with a value or statement
///
/// Metadata provides additional information about AST nodes, including source location,
/// comments, and labels. This information is useful for error reporting, documentation
/// generation, and preserving the original structure of the code.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Metadata {
    /// Source location information
    pub location: Location,
    
    /// Optional comment associated with the node (from # or /* */ comments)
    pub comment: Option<String>,
    
    /// Optional label associated with the node (from !label syntax)
    pub label: Option<String>,
}

impl Metadata {
    /// Creates a new Metadata instance with the given location
    pub fn new(location: Location) -> Self {
        Self {
            location,
            comment: None,
            label: None,
        }
    }
    
    /// Creates a new Metadata instance with location, comment, and label
    pub fn with_details(location: Location, comment: Option<String>, label: Option<String>) -> Self {
        Self {
            location,
            comment,
            label,
        }
    }
    
    /// Returns true if this metadata has a comment
    pub fn has_comment(&self) -> bool {
        self.comment.is_some()
    }
    
    /// Returns true if this metadata has a label
    pub fn has_label(&self) -> bool {
        self.label.is_some()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Location {
    /// Module name where the token was found
    pub module: Option<String>,
    /// Line number (0-based)
    pub line: usize,
    /// Column number (0-based)
    pub column: usize,
    /// Original source text for the token
    pub source_text: Option<String>,
    /// Length of the token in characters
    pub length: usize,
    /// File path where the token was found
    pub file_path: Option<String>,
}

impl Location {
    /// Create a new location with basic information
    pub fn new(line: usize, column: usize) -> Self {
        Self {
            module: None,
            line,
            column,
            source_text: None,
            length: 0,
            file_path: None,
        }
    }
    
    /// Create a new location with full information
    pub fn with_details(
        module: Option<String>,
        line: usize,
        column: usize,
        source_text: Option<String>,
        length: usize,
        file_path: Option<String>,
    ) -> Self {
        Self {
            module,
            line,
            column,
            source_text,
            length,
            file_path,
        }
    }
    
    /// Set the module name
    pub fn set_module(&mut self, module: &str) {
        self.module = Some(module.to_string());
    }
    
    /// Set the source text
    pub fn set_source_text(&mut self, text: &str) {
        self.source_text = Some(text.to_string());
        self.length = text.len();
    }
    
    /// Set the file path
    pub fn set_file_path(&mut self, path: &str) {
        self.file_path = Some(path.to_string());
    }
    
    /// Get the human-readable position (1-based for display)
    pub fn position(&self) -> (usize, usize) {
        (self.line + 1, self.column + 1)
    }
    
    /// Get the source context for error reporting
    ///
    /// Returns the source text if available, or an empty string if not.
    /// This is useful for providing context in error messages.
    pub fn context(&self) -> String {
        if let Some(source) = &self.source_text {
            source.clone()
        } else {
            String::new()
        }
    }
    
    /// Get a formatted source context with line numbers for error reporting
    ///
    /// Returns a formatted string with line numbers and the source text,
    /// highlighting the current line if possible.
    pub fn formatted_context(&self) -> String {
        if let Some(source) = &self.source_text {
            let lines: Vec<&str> = source.lines().collect();
            let line_num = self.line;
            
            // Determine the range of lines to show (up to 2 lines before and after)
            let start_line = if line_num > 2 { line_num - 2 } else { 0 };
            let end_line = std::cmp::min(line_num + 2, lines.len());
            
            let mut result = String::new();
            for i in start_line..end_line {
                let line_display = i + 1; // 1-based line numbers for display
                if i == line_num {
                    // Highlight the current line
                    result.push_str(&format!("-> {}: {}\n", line_display, lines[i]));
                    
                    // Add caret pointing to the column
                    let spaces = "   ".len() + line_display.to_string().len() + 2 + self.column;
                    result.push_str(&format!("{}^\n", " ".repeat(spaces)));
                } else {
                    result.push_str(&format!("   {}: {}\n", line_display, lines[i]));
                }
            }
            
            result
        } else {
            String::new()
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (line, column) = self.position();
        
        if let Some(module) = self.module.as_ref() {
            if let Some(file_path) = self.file_path.as_ref() {
                write!(f, "[{}@{}:{}:{}]", module, file_path, line, column)
            } else {
                write!(f, "[{}@{}:{}]", module, line, column)
            }
        } else if let Some(file_path) = self.file_path.as_ref() {
            write!(f, "[{}:{}:{}]", file_path, line, column)
        } else {
            write!(f, "[{}:{}]", line, column)
        }
    }
}
