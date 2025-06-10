use super::lexer::{HashableFloat, Integer, Token};
use super::read::{Read, TokenReader};
use crate::ast::{Location, Metadata, Statement, Value, ValueType};
use crate::{error, Result};
use indexmap::IndexMap;
use logos::Lexer;
use snafu::{ensure, OptionExt};

pub struct Parser<'source> {
    tokens: TokenReader<'source>,
}

impl<'source> Parser<'source> {
    pub fn new(name: &str, lexer: Lexer<'source, Token>) -> Self {
        Self {
            tokens: TokenReader {
                module_name: name.to_string(),
                lexer: lexer.peekable(),
                location: Location {
                    module: Some(name.to_string()),
                    line: 0,
                    column: 0,
                    source_text: None,
                    length: 0,
                    file_path: None,
                },
            },
        }
    }

    /// Create a new parser with file path information
    pub fn with_file_path(name: &str, file_path: &str, lexer: Lexer<'source, Token>) -> Self {
        Self {
            tokens: TokenReader {
                module_name: name.to_string(),
                lexer: lexer.peekable(),
                location: Location {
                    module: Some(name.to_string()),
                    line: 0,
                    column: 0,
                    source_text: None,
                    length: 0,
                    file_path: Some(file_path.to_string()),
                },
            },
        }
    }

    pub fn parse(&mut self) -> Result<Statement> {
        self.module()
    }

    fn metadata(&mut self) -> Result<Metadata> {
        let mut meta = Metadata {
            location: self.tokens.location(),
            comment: None,
            label: None,
        };

        // Process comments
        while let Some(token) = self.tokens.peek()? {
            match token {
                Token::LineComment((_, comment)) | Token::MultiLineComment((_, comment)) => {
                    self.tokens.discard();
                    // If we already have a comment, append this one with a newline
                    if let Some(existing) = meta.comment.as_mut() {
                        existing.push('\n');
                        existing.push_str(&comment);
                    } else {
                        meta.comment = Some(comment.clone());
                    }
                }
                Token::LabelIdentifier((_, label)) => {
                    self.tokens.discard();
                    meta.label = Some(label.clone());
                    break;
                }
                _ => break,
            }
        }

        // If we didn't find a label in the comment processing loop, check again
        if meta.label.is_none() {
            if let Some(Token::LabelIdentifier((_, label))) = self.tokens.peek()? {
                self.tokens.discard();
                meta.label = Some(label.clone());
            }
        }

        Ok(meta)
    }

    fn value_type(&mut self) -> Result<ValueType> {
        let token = self.tokens.next()?.context(error::EofSnafu {
            location: self.tokens.location(),
        })?;
        match token {
            Token::KeyString(_) => Ok(ValueType::String),
            Token::KeyInt(_) => Ok(ValueType::Signed),
            Token::KeyInt8(_) => Ok(ValueType::I8),
            Token::KeyInt16(_) => Ok(ValueType::I16),
            Token::KeyInt32(_) => Ok(ValueType::I32),
            Token::KeyInt64(_) => Ok(ValueType::I64),
            Token::KeyInt128(_) => Ok(ValueType::I128),
            Token::KeyUInt(_) => Ok(ValueType::Unsigned),
            Token::KeyUInt128(_) => Ok(ValueType::U128),
            Token::KeyUInt64(_) => Ok(ValueType::U64),
            Token::KeyUInt32(_) => Ok(ValueType::U32),
            Token::KeyUInt16(_) => Ok(ValueType::U16),
            Token::KeyUInt8(_) => Ok(ValueType::U8),
            Token::KeyNull(_) => Ok(ValueType::Null),
            Token::KeyBool(_) => Ok(ValueType::Bool),
            Token::KeyFloat(_) => Ok(ValueType::Float),
            Token::KeyFloat64(_) => Ok(ValueType::F64),
            Token::KeyFloat32(_) => Ok(ValueType::F32),
            Token::KeyBytes(_) => Ok(ValueType::Bytes),
            Token::KeyVersion(_) => Ok(ValueType::Version),
            Token::KeyRequire(_) => Ok(ValueType::Require),
            Token::KeyLabel(_) => Ok(ValueType::Label),
            Token::KeySymbol(_) => Ok(ValueType::Symbol),
            Token::KeyArray(location) => {
                let mut location = location.clone();
                location.set_module(self.tokens.module_name.as_str());
                let tok = self.tokens.next()?.context(error::EofSnafu { location })?;
                let tok_loc = tok.location(Some(self.tokens.module_name.clone()));
                ensure!(
                    matches!(tok, Token::LBracket(_)),
                    error::ExpectedSnafu {
                        location: tok_loc.clone(),
                        expected: "[",
                        got: tok.clone(),
                        context: "while parsing array type definition".to_string()
                    }
                );
                let mut children = Vec::new();
                while let Some(tok) = self.tokens.peek()? {
                    match tok {
                        Token::Comma(_) => {
                            self.tokens.discard();
                            continue;
                        }
                        Token::RBracket(_) => {
                            self.tokens.discard();
                            break;
                        }
                        _ => {
                            children.push(self.value_type()?);
                        }
                    }
                }
                Ok(ValueType::Array(children))
            }
            Token::KeyTable(location) => {
                let mut location = location.clone();
                location.set_module(self.tokens.module_name.as_str());
                let tok = self.tokens.next()?.context(error::EofSnafu { location })?;
                let tok_loc = tok.location(Some(self.tokens.module_name.clone()));
                ensure!(
                    matches!(tok, Token::LBrace(_)),
                    error::ExpectedSnafu {
                        location: tok_loc.clone(),
                        expected: "{",
                        got: tok.clone(),
                        context: "while parsing table type definition".to_string()
                    }
                );
                let mut children = IndexMap::new();
                while let Some(tok) = self.tokens.peek()? {
                    match tok {
                        Token::Comma(_) => {
                            self.tokens.discard();
                            continue;
                        }
                        Token::RBrace(_) => {
                            self.tokens.discard();
                            break;
                        }
                        _ => {
                            let id = self.tokens.next()?.context(error::EofSnafu {
                                location: self.tokens.location(),
                            })?;
                            let id = match id {
                                Token::Identifier((_, id)) | Token::String((_, id)) => {
                                    Ok(id.clone())
                                }
                                got => error::ExpectedSnafu {
                                    location: got.location(Some(self.tokens.module_name.clone())),
                                    expected: "identifier or string value",
                                    got: got.clone(),
                                    context: "while parsing table field key".to_string(),
                                }
                                .fail(),
                            }?;
                            let eq = self.tokens.next()?.context(error::EofSnafu {
                                location: self.tokens.location(),
                            })?;
                            let eq_loc = eq.location(Some(self.tokens.module_name.clone()));
                            ensure!(
                                matches!(eq, Token::Colon(_)),
                                error::ExpectedSnafu {
                                    location: eq_loc.clone(),
                                    expected: ":",
                                    got: eq.clone(),
                                    context: format!(
                                        "while parsing table field type for key '{}'",
                                        id
                                    )
                                }
                            );
                            let subtype = self.value_type()?;
                            children.insert(id, subtype);
                        }
                    }
                }
                Ok(ValueType::Table(children))
            }
            _ => error::ExpectedSnafu {
                location: self.tokens.location(),
                expected: vec![
                    "string", "int", "i8", "i16", "i32", "i64", "i128", "uint", "u8", "u16", "u32",
                    "u64", "u128", "float", "f32", "f64", "bool", "bytes", "version", "require",
                    "label", "symbol", "null", "array", "table",
                ]
                .join(", "),
                got: token.clone(),
                context: "while parsing value type".to_string(),
            }
            .fail(),
        }
    }

    fn value(&mut self) -> Result<(Value, ValueType)> {
        let meta = self.metadata()?;

        let token = self.tokens.next()?.context(error::EofSnafu {
            location: self.tokens.location(),
        })?;

        match token {
            // Simple value types
            Token::KeyNull(_) => Ok((Value::new_null(meta), ValueType::Null)),
            Token::False(_) => Ok((Value::new_bool(false, meta), ValueType::Bool)),
            Token::True(_) => Ok((Value::new_bool(true, meta), ValueType::Bool)),

            // Integer types with various precisions
            Token::Int((_, Integer::Signed(value))) => {
                Ok((Value::new_int(value, meta), ValueType::Signed))
            }
            Token::Int((_, Integer::Unsigned(value))) => {
                Ok((Value::new_uint(value, meta), ValueType::Unsigned))
            }
            Token::Int((_, Integer::I128(value))) => {
                Ok((Value::new_i128(value, meta), ValueType::I128))
            }
            Token::Int((_, Integer::I64(value))) => {
                Ok((Value::new_i64(value, meta), ValueType::I64))
            }
            Token::Int((_, Integer::I32(value))) => {
                Ok((Value::new_i32(value, meta), ValueType::I32))
            }
            Token::Int((_, Integer::I16(value))) => {
                Ok((Value::new_i16(value, meta), ValueType::I16))
            }
            Token::Int((_, Integer::I8(value))) => Ok((Value::new_i8(value, meta), ValueType::I8)),
            Token::Int((_, Integer::U128(value))) => {
                Ok((Value::new_u128(value, meta), ValueType::U128))
            }
            Token::Int((_, Integer::U64(value))) => {
                Ok((Value::new_u64(value, meta), ValueType::U64))
            }
            Token::Int((_, Integer::U32(value))) => {
                Ok((Value::new_u32(value, meta), ValueType::U32))
            }
            Token::Int((_, Integer::U16(value))) => {
                Ok((Value::new_u16(value, meta), ValueType::U16))
            }
            Token::Int((_, Integer::U8(value))) => Ok((Value::new_u8(value, meta), ValueType::U8)),

            // Float types
            Token::Float((_, HashableFloat::Generic(value))) => {
                Ok((Value::new_float(value, meta), ValueType::Float))
            }
            Token::Float((_, HashableFloat::Float32(value))) => {
                Ok((Value::new_f32(value, meta), ValueType::F32))
            }
            Token::Float((_, HashableFloat::Float64(value))) => {
                Ok((Value::new_f64(value, meta), ValueType::F64))
            }

            // String and identifier types
            Token::String((_, value)) => {
                Ok((Value::new_string(value.clone(), meta), ValueType::String))
            }
            Token::SymbolIdentifier((_, value)) => {
                Ok((Value::new_symbol(value.clone(), meta), ValueType::Symbol))
            }
            Token::MacroString((_, value)) => {
                Ok((Value::new_macro(value.clone(), meta), ValueType::Macro))
            }
            Token::MacroIdentifier((_, value)) => {
                Ok((Value::new_macro(value.clone(), meta), ValueType::Macro))
            }
            Token::ByteString((_, value)) => {
                Ok((Value::new_bytes(value.clone(), meta), ValueType::Bytes))
            }

            // Version types
            Token::Version((_, value)) => {
                Ok((Value::new_version(value.clone(), meta), ValueType::Version))
            }
            Token::Require((_, value)) => {
                Ok((Value::new_require(value.clone(), meta), ValueType::Require))
            }
            // Array parsing
            Token::LBracket(_) => {
                let mut children = Vec::with_capacity(8);
                let mut child_types = Vec::with_capacity(8);

                while let Some(token) = self.tokens.peek()? {
                    match token {
                        Token::Comma(_) => {
                            self.tokens.discard();
                            continue;
                        }
                        Token::RBracket(_) => {
                            self.tokens.discard();
                            break;
                        }
                        _ => {
                            let (value, type_) = self.value()?;
                            children.push(value);
                            child_types.push(type_);
                        }
                    };
                }

                Ok((
                    Value::new_array(children, meta),
                    ValueType::Array(child_types),
                ))
            }
            Token::LBrace(location) => {
                let mut children = IndexMap::new();
                let mut child_types = IndexMap::new();
                while let Some(token) = self.tokens.peek()? {
                    match token {
                        Token::Comma(_) => {
                            self.tokens.discard();
                            continue;
                        }
                        Token::RBrace(_) => {
                            self.tokens.discard();
                            break;
                        }
                        Token::Identifier(_) | Token::String(_) => {
                            let next_token = self.tokens.next()?.context(error::EofSnafu {
                                location: self.tokens.location(),
                            })?;

                            let id = match next_token {
                                Token::Identifier((location, id))
                                | Token::String((location, id)) => {
                                    let mut loc = location.clone();
                                    loc.set_module(self.tokens.module_name.as_str());
                                    (loc, id.clone())
                                }
                                _ => unreachable!(), // We already matched this in the peek
                            };

                            let vtype = if let Some(Token::Colon(_)) = self.tokens.peek()? {
                                self.tokens.discard();
                                Some(self.value_type()?)
                            } else {
                                None
                            };

                            let eq_tok = self.tokens.next()?.context(error::EofSnafu {
                                location: id.0.clone(),
                            })?;

                            let eq_loc = eq_tok.location(Some(self.tokens.module_name.clone()));
                            ensure!(
                                matches!(eq_tok, Token::Assign(_)),
                                error::ExpectedSnafu {
                                    location: eq_loc.clone(),
                                    expected: "=",
                                    got: eq_tok.clone(),
                                    context: format!(
                                        "while parsing table entry for key '{}'",
                                        id.1
                                    )
                                }
                            );

                            let (child, child_type) = self.value()?;
                            children.insert(id.1.clone(), child);
                            child_types.insert(id.1, vtype.unwrap_or(child_type));
                        }
                        _ => {
                            return error::ExpectedSnafu {
                                location,
                                expected: ", } identifier string",
                                got: token.clone(),
                                context: "while parsing table entries".to_string(),
                            }
                            .fail()
                        }
                    }
                }
                Ok((
                    Value::new_table(children, meta),
                    ValueType::Table(child_types),
                ))
            }
            // Error for unexpected tokens
            _ => error::ExpectedSnafu {
                location: token.location(Some(self.tokens.module_name.clone())),
                expected: "value (null, bool, number, string, array, table, etc.)",
                got: token.clone(),
                context: "while parsing a value expression".to_string(),
            }
            .fail(),
        }
    }

    fn statement(&mut self) -> Result<Statement> {
        let meta = self.metadata()?;

        let token = self.tokens.next()?.context(error::EofSnafu {
            location: self.tokens.location(),
        })?;

        match &token {
            Token::ControlIdentifier((location, id)) => {
                // Handle control statements ($identifier)
                let mut location = location.clone();
                location.set_module(self.tokens.module_name.as_str());

                // Check for type annotation
                let type_ = if let Some(Token::Colon(_)) = self.tokens.peek()? {
                    self.tokens.discard();
                    Some(self.value_type()?)
                } else {
                    None
                };

                // Expect assignment operator
                let eq = self.tokens.next()?.context(error::EofSnafu {
                    location: location.clone(),
                })?;

                let eq_loc = eq.location(Some(self.tokens.module_name.clone()));
                ensure!(
                    matches!(eq, Token::Assign(_)),
                    error::ExpectedSnafu {
                        location: eq_loc.clone(),
                        expected: "=",
                        got: eq.clone(),
                        context: format!("while parsing control statement '${}'", id)
                    }
                );

                // Parse value and check type compatibility
                let (value, vtype) = self.value()?;
                if let Some(type_) = type_.as_ref() {
                    ensure!(
                        vtype.can_assign(type_),
                        error::AssignSnafu {
                            location: location.clone(),
                            left: type_.clone(),
                            right: vtype
                        }
                    );
                }

                Ok(Statement::new_control(id.as_str(), type_, value, meta)?)
            }

            Token::Identifier((location, id)) | Token::String((location, id)) => {
                let mut loc = location.clone();
                loc.set_module(self.tokens.module_name.as_str());

                // Check if this is an assignment or a block
                if let Some(token) = self.tokens.peek()? {
                    match token {
                        Token::Colon(_) | Token::Assign(_) => {
                            // This is an assignment

                            // Check for type annotation
                            let type_ = if matches!(token, Token::Colon(_)) {
                                self.tokens.discard();
                                let type_ = self.value_type()?;

                                // Now expect assignment operator
                                let eq = self.tokens.next()?.context(error::EofSnafu {
                                    location: loc.clone(),
                                })?;

                                let eq_loc = eq.location(Some(self.tokens.module_name.clone()));
                                ensure!(
                                    matches!(eq, Token::Assign(_)),
                                    error::ExpectedSnafu {
                                        location: eq_loc.clone(),
                                        expected: "=",
                                        got: eq.clone(),
                                        context: format!("while parsing assignment to '{}'", id)
                                    }
                                );

                                Some(type_)
                            } else {
                                // No type annotation, just consume the assignment operator
                                self.tokens.discard();
                                None
                            };

                            // Parse value and check type compatibility
                            let (value, vtype) = self.value()?;
                            if let Some(type_) = type_.as_ref() {
                                ensure!(
                                    vtype.can_assign(type_),
                                    error::AssignSnafu {
                                        location: loc.clone(),
                                        left: type_.clone(),
                                        right: vtype
                                    }
                                );
                            }

                            Ok(Statement::new_assign(id.as_str(), type_, value, meta)?)
                        }

                        _ => {
                            // This is a block
                            let mut labels = Vec::with_capacity(4);

                            // Parse labels until we hit the opening brace
                            loop {
                                match self.tokens.peek()? {
                                    Some(Token::LBrace(_)) => {
                                        self.tokens.discard();
                                        break;
                                    }
                                    Some(Token::Comma(_)) => {
                                        self.tokens.discard();
                                    }
                                    Some(_) => {
                                        labels.push(self.value()?.0);
                                    }
                                    None => {
                                        return error::ExpectedSnafu {
                                            location: self.tokens.location(),
                                            expected: "{",
                                            got: token.clone(),
                                            context: format!(
                                                "while parsing block statement '{}'",
                                                id
                                            ),
                                        }
                                        .fail();
                                    }
                                }
                            }

                            // Parse block contents
                            let mut children = IndexMap::with_capacity(8);
                            while let Some(stmt) = self.tokens.peek()? {
                                match stmt {
                                    Token::RBrace(_) => {
                                        self.tokens.discard();
                                        break;
                                    }
                                    _ => {
                                        let value = self.statement()?;
                                        children.insert(value.inject_id(), value);
                                    }
                                }
                            }

                            Ok(Statement::new_block(id.as_str(), labels, children, meta))
                        }
                    }
                } else {
                    error::ExpectedSnafu {
                        location: self.tokens.location(),
                        expected: "= or : or block contents",
                        got: token.clone(),
                        context: format!(
                            "while parsing identifier '{}' - expected assignment or block",
                            id
                        ),
                    }
                    .fail()
                }
            }

            value => error::ExpectedSnafu {
                expected: "statement",
                got: value.clone(),
                location: value.location(Some(self.tokens.module_name.clone())),
                context: "Expected a statement (assignment, control statement, or block)"
                    .to_string(),
            }
            .fail(),
        }
    }

    fn module(&mut self) -> Result<Statement> {
        let parent_meta = self.metadata()?;
        let mut children = IndexMap::with_capacity(16); // Pre-allocate with reasonable capacity

        while let Some(token) = self.tokens.peek()? {
            let meta = self.metadata()?;
            match token {
                Token::LBracket(location) => {
                    let mut location = location.clone();
                    location.set_module(self.tokens.module_name.as_str());
                    self.tokens.discard();

                    // Get section identifier
                    let id = self.tokens.next()?.context(error::EofSnafu {
                        location: location.clone(),
                    })?;

                    let id = match id {
                        Token::Identifier((_, id)) | Token::String((_, id)) => Ok(id),
                        value => error::ExpectedSnafu {
                            location: value.location(Some(self.tokens.module_name.clone())),
                            expected: "identifier or string",
                            got: value.clone(),
                            context: "while parsing section name".to_string(),
                        }
                        .fail(),
                    }?;

                    // Ensure closing bracket
                    let close = self.tokens.next()?.context(error::EofSnafu {
                        location: self.tokens.location(),
                    })?;

                    let close_loc = close.location(Some(self.tokens.module_name.clone()));
                    ensure!(
                        matches!(close, Token::RBracket(_)),
                        error::ExpectedSnafu {
                            location: close_loc.clone(),
                            expected: "]",
                            got: close.clone(),
                            context: format!("while parsing section declaration '[{}]'", id)
                        }
                    );

                    // Parse section statements
                    let mut statements = IndexMap::with_capacity(8);
                    while let Some(stmt) = self.tokens.peek()? {
                        match stmt {
                            Token::LBracket(_) => break,
                            _ => {
                                let value = self.statement()?;
                                statements.insert(value.inject_id(), value);
                            }
                        }
                    }

                    let child = Statement::new_section(id.as_str(), statements, meta);
                    children.insert(child.inject_id(), child);
                }
                _ => {
                    let value = self.statement()?;
                    children.insert(value.inject_id(), value);
                }
            }
        }

        Ok(Statement::new_module(".", children, parent_meta))
    }
}

#[cfg(test)]
mod test {
    use super::Parser;
    use crate::ast::Metadata;
    use crate::ast::{Location, Statement, Value, ValueType};
    use crate::syn::lexer::Token;
    use indexmap::IndexMap;
    use logos::Logos;

    macro_rules! parser {
        ($input: expr) => {
            Parser::new("root", Token::lexer($input))
        };
    }

    #[test]
    fn metadata() {
        let mut parser = parser!("# Line Comment\n !Hint");
        let meta = parser.metadata().unwrap();
        assert_eq!(meta.comment, Some("Line Comment".to_string()));
        assert_eq!(meta.label, Some("Hint".to_string()));
    }

    #[test]
    fn values() {
        for (case, expected) in [
            (
                "'hello world'",
                (
                    Value::new_string("hello world".to_string(), Metadata::default()),
                    ValueType::String,
                ),
            ),
            (
                ":hello/world",
                (
                    Value::new_symbol("hello/world".to_string(), Metadata::default()),
                    ValueType::Symbol,
                ),
            ),
            (
                "-3",
                (Value::new_int(-3, Metadata::default()), ValueType::Signed),
            ),
            (
                "-3i64",
                (Value::new_i64(-3, Metadata::default()), ValueType::I64),
            ),
            (
                "-3i32",
                (Value::new_i32(-3, Metadata::default()), ValueType::I32),
            ),
            (
                "-3i16",
                (Value::new_i16(-3, Metadata::default()), ValueType::I16),
            ),
            (
                "-3i8",
                (Value::new_i8(-3, Metadata::default()), ValueType::I8),
            ),
            (
                "3u64",
                (Value::new_u64(3, Metadata::default()), ValueType::U64),
            ),
            (
                "3u32",
                (Value::new_u32(3, Metadata::default()), ValueType::U32),
            ),
            (
                "3u16",
                (Value::new_u16(3, Metadata::default()), ValueType::U16),
            ),
            (
                "3u8",
                (Value::new_u8(3, Metadata::default()), ValueType::U8),
            ),
            (
                "-3.14",
                (
                    Value::new_float(-3.14, Metadata::default()),
                    ValueType::Float,
                ),
            ),
            (
                "-3.14f64",
                (Value::new_f64(-3.14, Metadata::default()), ValueType::F64),
            ),
            (
                "-3.14f32",
                (Value::new_f32(-3.14, Metadata::default()), ValueType::F32),
            ),
            (
                "false",
                (Value::new_bool(false, Metadata::default()), ValueType::Bool),
            ),
            (
                "1.2.3",
                (
                    Value::new_version(semver::Version::new(1, 2, 3), Metadata::default()),
                    ValueType::Version,
                ),
            ),
            (
                ">1.4",
                (
                    Value::new_require(
                        semver::VersionReq::parse(">1.4").unwrap(),
                        Metadata::default(),
                    ),
                    ValueType::Require,
                ),
            ),
            (
                "m'hello'",
                (
                    Value::new_macro("hello".to_string(), Metadata::default()),
                    ValueType::Macro,
                ),
            ),
            (
                "m!hello",
                (
                    Value::new_macro("hello".to_string(), Metadata::default()),
                    ValueType::Macro,
                ),
            ),
            (
                "b'aGVsbG8='",
                (
                    Value::new_bytes(b"hello".to_vec(), Metadata::default()),
                    ValueType::Bytes,
                ),
            ),
            (
                "['hello', 3, true]",
                (
                    (Value::new_array(
                        vec![
                            Value::new_string("hello".to_string(), Metadata::default()),
                            Value::new_int(3, Metadata::default()),
                            Value::new_bool(true, Metadata::default()),
                        ],
                        Metadata::default(),
                    )),
                    ValueType::Array(vec![ValueType::String, ValueType::Signed, ValueType::Bool]),
                ),
            ),
            (
                "{one = 'hello', two = 3, three = true}",
                (
                    Value::new_table(
                        IndexMap::from([
                            (
                                "one".to_string(),
                                Value::new_string("hello".to_string(), Metadata::default()),
                            ),
                            ("two".to_string(), Value::new_int(3, Metadata::default())),
                            (
                                "three".to_string(),
                                Value::new_bool(true, Metadata::default()),
                            ),
                        ]),
                        Metadata::default(),
                    ),
                    ValueType::Table(IndexMap::from([
                        ("one".to_string(), ValueType::String),
                        ("two".to_string(), ValueType::Signed),
                        ("three".to_string(), ValueType::Bool),
                    ])),
                ),
            ),
        ] {
            let mut parser = parser!(case);
            assert_eq!(parser.value().unwrap(), expected);
        }
    }

    #[test]
    fn types() {
        for (case, expected) in [
            ("string", ValueType::String),
            ("int", ValueType::Signed),
            ("i8", ValueType::I8),
            ("i16", ValueType::I16),
            ("i32", ValueType::I32),
            ("i64", ValueType::I64),
            ("u8", ValueType::U8),
            ("u16", ValueType::U16),
            ("u32", ValueType::U32),
            ("u64", ValueType::U64),
            ("symbol", ValueType::Symbol),
            ("float", ValueType::Float),
            ("f32", ValueType::F32),
            ("f64", ValueType::F64),
            ("bool", ValueType::Bool),
            ("version", ValueType::Version),
            ("require", ValueType::Require),
            ("bytes", ValueType::Bytes),
            (
                "array[string, int, bool]",
                ValueType::Array(vec![ValueType::String, ValueType::Signed, ValueType::Bool]),
            ),
            (
                "table{one: string, two: int, three: bool}",
                ValueType::Table(IndexMap::from([
                    ("one".to_string(), ValueType::String),
                    ("two".to_string(), ValueType::Signed),
                    ("three".to_string(), ValueType::Bool),
                ])),
            ),
        ] {
            let mut parser = parser!(case);
            assert_eq!(parser.value_type().unwrap(), expected);
        }
    }

    #[test]
    fn statements() {
        for (case, expected) in [
            (
                "$foo = 3",
                Statement::new_control(
                    "foo",
                    None,
                    Value::new_int(3, Metadata::default()),
                    Metadata::default(),
                )
                .unwrap(),
            ),
            (
                "$foo: f64 = 3.14",
                Statement::new_control(
                    "foo",
                    Some(ValueType::F64),
                    Value::new_f64(3.14, Metadata::default()),
                    Metadata::default(),
                )
                .unwrap(),
            ),
            (
                "# Comment\n$foo: f64 = !Hint 3.14",
                Statement::new_control(
                    "foo",
                    Some(ValueType::F64),
                    Value::new_f64(
                        3.14,
                        Metadata {
                            location: Location {
                                module: None,
                                line: 0,
                                column: 0,
                                file_path: None,
                                length: 0,
                                source_text: None,
                            },
                            comment: None,
                            label: Some("Hint".to_string()),
                        },
                    ),
                    Metadata {
                        location: Location {
                            module: None,
                            line: 0,
                            column: 0,
                            file_path: None,
                            length: 0,
                            source_text: None,
                        },
                        comment: Some("Comment".to_string()),
                        label: None,
                    },
                )
                .unwrap(),
            ),
            (
                "foo = 3",
                Statement::new_assign(
                    "foo",
                    None,
                    Value::new_int(3, Metadata::default()),
                    Metadata::default(),
                )
                .unwrap(),
            ),
            (
                "'foo': f64 = 3.14",
                Statement::new_assign(
                    "foo",
                    Some(ValueType::F64),
                    Value::new_f64(3.14, Metadata::default()),
                    Metadata::default(),
                )
                .unwrap(),
            ),
            (
                "# Comment\n\"foo\": f64 = !Hint 3.14",
                Statement::new_assign(
                    "foo",
                    Some(ValueType::F64),
                    Value::new_f64(
                        3.14,
                        Metadata {
                            location: Location {
                                module: None,
                                line: 0,
                                column: 0,
                                file_path: None,
                                length: 0,
                                source_text: None,
                            },
                            comment: None,
                            label: Some("Hint".to_string()),
                        },
                    ),
                    Metadata {
                        location: Location {
                            module: None,
                            line: 0,
                            column: 0,
                            file_path: None,
                            length: 0,
                            source_text: None,
                        },
                        comment: Some("Comment".to_string()),
                        label: None,
                    },
                )
                .unwrap(),
            ),
            (
                "# Comment\ntest 'foo' 'bar' {\n one = 3\n }\n",
                Statement::new_block(
                    "test",
                    vec![
                        Value::new_string("foo".to_string(), Metadata::default()),
                        Value::new_string("bar".to_string(), Metadata::default()),
                    ],
                    IndexMap::from([(
                        "one".to_string(),
                        Statement::new_assign(
                            "one",
                            None,
                            Value::new_int(3, Metadata::default()),
                            Metadata::default(),
                        )
                        .unwrap(),
                    )]),
                    Metadata {
                        location: Location {
                            module: None,
                            line: 0,
                            column: 0,
                            file_path: None,
                            length: 0,
                            source_text: None,
                        },
                        comment: Some("Comment".to_string()),
                        label: None,
                    },
                ),
            ),
        ] {
            println!("testing: {case}");
            let mut parser = parser!(case);
            assert_eq!(parser.statement().unwrap(), expected);
        }
    }
}
