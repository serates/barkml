use crate::{Result, error};
use base64::Engine;
use logos::{Lexer, Logos, Skip};
use snafu::ResultExt;
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::ast::Location;

/// Stores the integer with precision retained
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Integer {
    Signed(i64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Unsigned(u64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
}

/// Handles incrementing the line and column counters
fn newline_callback(lex: &mut Lexer<Token>) -> Skip {
    lex.extras.line += 1;
    lex.extras.column = lex.span().end;
    Skip
}

// Creates a source location with detailed information
fn base_callback(lex: &mut Lexer<Token>) -> Location {
    let span = lex.span();
    let source_text = lex.slice().to_string();

    Location {
        module: None,
        line: lex.extras.line,
        column: span.start - lex.extras.column,
        source_text: Some(source_text),
        length: span.end - span.start,
        file_path: None,
    }
}

/// Represents the allowed tokens in a barkml file
#[derive(Logos, Debug, PartialEq, Eq, Clone, Hash)]
#[logos(error = error::Error)]
#[logos(extras = Location)]
#[logos(skip r"[ \t\f\r]+")] // Ignore this regex pattern between tokens
pub enum Token {
    #[regex(r"\n", newline_callback)]
    Newline,

    Error(Location),

    #[token("true", base_callback, priority = 11)]
    #[token("True", base_callback, priority = 11)]
    #[token("yes", base_callback, priority = 11)]
    #[token("Yes", base_callback, priority = 11)]
    #[token("On", base_callback, priority = 11)]
    #[token("on", base_callback, priority = 11)]
    True(Location),
    #[token("false", base_callback, priority = 11)]
    #[token("False", base_callback, priority = 11)]
    #[token("no", base_callback, priority = 11)]
    #[token("No", base_callback, priority = 11)]
    #[token("Off", base_callback, priority = 11)]
    #[token("off", base_callback, priority = 11)]
    False(Location),

    #[token("!", base_callback)]
    Exclaim(Location),
    #[token("$", base_callback)]
    Dollar(Location),

    #[token("[", base_callback)]
    LBracket(Location),
    #[token("]", base_callback)]
    RBracket(Location),
    #[token("{", base_callback)]
    LBrace(Location),
    #[token("}", base_callback)]
    RBrace(Location),
    #[token("(", base_callback)]
    LParen(Location),
    #[token(")", base_callback)]
    RParen(Location),
    #[token("=", base_callback)]
    Assign(Location),
    #[token(":", base_callback)]
    Colon(Location),
    #[token("?", base_callback)]
    Question(Location),
    #[token(",", base_callback)]
    Comma(Location),

    // Keywords
    #[token("null", base_callback, priority = 10)]
    #[token("nil", base_callback, priority = 10)]
    #[token("none", base_callback, priority = 10)]
    KeyNull(Location),
    #[token("bool", base_callback, priority = 10)]
    KeyBool(Location),
    #[token("string", base_callback, priority = 10)]
    KeyString(Location),
    #[token("int", base_callback, priority = 10)]
    KeyInt(Location),
    #[token("uint", base_callback, priority = 10)]
    KeyUInt(Location),
    #[token("i8", base_callback, priority = 10)]
    KeyInt8(Location),
    #[token("u8", base_callback, priority = 10)]
    KeyUInt8(Location),
    #[token("i16", base_callback, priority = 10)]
    KeyInt16(Location),
    #[token("u16", base_callback, priority = 10)]
    KeyUInt16(Location),
    #[token("i32", base_callback, priority = 10)]
    KeyInt32(Location),
    #[token("u32", base_callback, priority = 10)]
    KeyUInt32(Location),
    #[token("i64", base_callback, priority = 10)]
    KeyInt64(Location),
    #[token("u64", base_callback, priority = 10)]
    KeyUInt64(Location),
    #[token("i128", base_callback, priority = 10)]
    KeyInt128(Location),
    #[token("u128", base_callback, priority = 10)]
    KeyUInt128(Location),
    #[token("float", base_callback, priority = 10)]
    KeyFloat(Location),
    #[token("f64", base_callback, priority = 10)]
    KeyFloat64(Location),
    #[token("f32", base_callback, priority = 10)]
    KeyFloat32(Location),
    #[token("bytes", base_callback, priority = 10)]
    KeyBytes(Location),
    #[token("version", base_callback, priority = 10)]
    KeyVersion(Location),
    #[token("require", base_callback, priority = 10)]
    KeyRequire(Location),
    #[token("label", base_callback, priority = 10)]
    KeyLabel(Location),
    #[token("array", base_callback, priority = 10)]
    KeyArray(Location),
    #[token("table", base_callback, priority = 10)]
    KeyTable(Location),
    #[token("section", base_callback, priority = 10)]
    KeySection(Location),
    #[token("block", base_callback, priority = 10)]
    KeyBlock(Location),
    #[token("symbol", base_callback, priority = 10)]
    KeySymbol(Location),

    // Unused but reserved
    #[token("module", base_callback, priority = 10)]
    KeyModule(Location),
    #[token("use", base_callback, priority = 10)]
    KeyUse(Location),
    #[token("as", base_callback, priority = 10)]
    KeyAs(Location),
    #[token("schema", base_callback, priority = 10)]
    KeySchema(Location),

    // Integer
    #[regex(
        r"[+-]?[0-9][0-9_]*(i128|i64|i32|i16|i8|u128|u64|u32|u16|u8)?",
        decimal::integer
    )]
    #[regex(
        r"[+-]?0x[0-9a-fA-F][0-9a-fA-F_]*(i128|i64|i32|i16|i8|u128|u64|u32|u16|u8)?",
        hexadecimal::integer
    )]
    #[regex(
        r"[+-]?0o[0-7][0-7_]*(i128|i64|i32|i16|i8|u128|u64|u32|u16|u8)?",
        octal::integer
    )]
    #[regex(
        r"[+-]?0b[0-1][0-1_]*(i128|i64|i32|i16|i8|u128|u64|u32|u16|u8)?",
        binary::integer
    )]
    Int((Location, Integer)),

    // Floating-point
    #[regex(
        r"[+-]?[0-9][0-9_]*\.[0-9][0-9_]*([eE][+-]?[0-9][0-9_]*)?(f64|f32)?",
        float
    )]
    Float((Location, HashableFloat)),

    #[regex(r"m'[^']*'", macro_string)]
    MacroString((Location, String)),
    #[regex(r"b'[-A-Za-z0-9+/]*={0,3}'", byte_string)]
    ByteString((Location, Vec<u8>)),
    #[regex(r"'[^']*'", quote_string)]
    #[regex(r#""[^"]*""#, quote_string)]
    String((Location, String)),

    #[regex(r"[a-zA-Z][a-zA-Z0-9_\-]*", |x| {
        (base_callback(x), x.slice().to_string()) }, priority = 5
    )]
    Identifier((Location, String)),
    #[regex(r"m\![a-zA-Z][a-zA-Z0-9_\-\.]*", |x| {
        (base_callback(x), x.slice().trim_start_matches("m!").to_string()) }
    , priority = 6)]
    MacroIdentifier((Location, String)),
    #[regex(r"\![a-zA-Z][a-zA-Z0-9_\-]*", |x| {
        (base_callback(x), x.slice().trim_start_matches('!').to_string())
    }, priority = 7)]
    LabelIdentifier((Location, String)),
    #[regex(r"\:[a-zA-Z][a-zA-Z0-9_\-\/]*", |x| {
        (base_callback(x), x.slice().trim_start_matches(':').to_string())
    }, priority = 7)]
    SymbolIdentifier((Location, String)),
    #[regex(r"\$[a-zA-Z][a-zA-Z0-9_\-]*", |x| {
        (base_callback(x), x.slice().trim_start_matches('$').to_string()) }, priority = 8
    )]
    ControlIdentifier((Location, String)),

    #[regex(r"([0-9]+\.){2}[0-9]+([-A-Za-z0-9\.]+)?", version_literal)]
    Version((Location, semver::Version)),

    #[regex(
        r"(=|~|\^|>|<|<=|>=)[0-9]+(\.[0-9]+(\.[0-9]+(\-[a-z0-9]+(\+[a-z0-9]+)?)?)?)?",
        version_require
    )]
    Require((Location, semver::VersionReq)),

    #[regex(r"(#[ \t\f]*[^\n\r]+[\n\r])*", line_comment)]
    LineComment((Location, String)),
    #[regex(r"\/\*[^\/\*]*\*\/", multiline_comment)]
    MultiLineComment((Location, String)),
}

impl Token {
    pub fn location(&self, module: Option<String>) -> Location {
        match self {
            Self::Error(source)
            | Self::True(source)
            | Self::False(source)
            | Self::Exclaim(source)
            | Self::Dollar(source)
            | Self::LBracket(source)
            | Self::RBracket(source)
            | Self::LBrace(source)
            | Self::RBrace(source)
            | Self::LParen(source)
            | Self::RParen(source)
            | Self::Assign(source)
            | Self::Colon(source)
            | Self::Question(source)
            | Self::Comma(source)
            | Self::KeyNull(source)
            | Self::KeyBool(source)
            | Self::KeyString(source)
            | Self::KeyInt(source)
            | Self::KeyInt8(source)
            | Self::KeyInt16(source)
            | Self::KeyInt32(source)
            | Self::KeyInt64(source)
            | Self::KeyUInt8(source)
            | Self::KeyUInt16(source)
            | Self::KeyUInt32(source)
            | Self::KeyUInt64(source)
            | Self::KeyFloat(source)
            | Self::KeyFloat32(source)
            | Self::KeyFloat64(source)
            | Self::KeyBytes(source)
            | Self::KeyVersion(source)
            | Self::KeyRequire(source)
            | Self::KeyLabel(source)
            | Self::KeyArray(source)
            | Self::KeyTable(source)
            | Self::KeySection(source)
            | Self::KeyBlock(source)
            | Self::KeyModule(source)
            | Self::KeyUse(source)
            | Self::KeyAs(source)
            | Self::KeySchema(source)
            | Self::Int((source, ..))
            | Self::Float((source, ..))
            | Self::MacroString((source, ..))
            | Self::LabelIdentifier((source, ..))
            | Self::ByteString((source, ..))
            | Self::String((source, ..))
            | Self::Identifier((source, ..))
            | Self::MacroIdentifier((source, ..))
            | Self::ControlIdentifier((source, ..))
            | Self::Version((source, ..))
            | Self::Require((source, ..))
            | Self::LineComment((source, ..))
            | Self::MultiLineComment((source, ..)) => {
                let mut src = source.clone();
                if let Some(module_name) = module.as_ref() {
                    src.set_module(module_name.as_str());
                }
                src
            }
            _ => Location::default(),
        }
    }

    /// Compares two tokens ignoring their location information
    pub fn eq_kind(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Newline, Self::Newline) => true,
            (Self::Error(_), Self::Error(_)) => true,
            (Self::True(_), Self::True(_)) => true,
            (Self::False(_), Self::False(_)) => true,
            (Self::Exclaim(_), Self::Exclaim(_)) => true,
            (Self::Dollar(_), Self::Dollar(_)) => true,
            (Self::LBracket(_), Self::LBracket(_)) => true,
            (Self::RBracket(_), Self::RBracket(_)) => true,
            (Self::LBrace(_), Self::LBrace(_)) => true,
            (Self::RBrace(_), Self::RBrace(_)) => true,
            (Self::LParen(_), Self::LParen(_)) => true,
            (Self::RParen(_), Self::RParen(_)) => true,
            (Self::Assign(_), Self::Assign(_)) => true,
            (Self::Colon(_), Self::Colon(_)) => true,
            (Self::Question(_), Self::Question(_)) => true,
            (Self::Comma(_), Self::Comma(_)) => true,
            (Self::KeyNull(_), Self::KeyNull(_)) => true,
            (Self::KeyBool(_), Self::KeyBool(_)) => true,
            (Self::KeyString(_), Self::KeyString(_)) => true,
            (Self::KeyInt(_), Self::KeyInt(_)) => true,
            (Self::KeyUInt(_), Self::KeyUInt(_)) => true,
            (Self::KeyInt8(_), Self::KeyInt8(_)) => true,
            (Self::KeyUInt8(_), Self::KeyUInt8(_)) => true,
            (Self::KeyInt16(_), Self::KeyInt16(_)) => true,
            (Self::KeyUInt16(_), Self::KeyUInt16(_)) => true,
            (Self::KeyInt32(_), Self::KeyInt32(_)) => true,
            (Self::KeyUInt32(_), Self::KeyUInt32(_)) => true,
            (Self::KeyInt64(_), Self::KeyInt64(_)) => true,
            (Self::KeyUInt64(_), Self::KeyUInt64(_)) => true,
            (Self::KeyInt128(_), Self::KeyInt128(_)) => true,
            (Self::KeyUInt128(_), Self::KeyUInt128(_)) => true,
            (Self::KeyFloat(_), Self::KeyFloat(_)) => true,
            (Self::KeyFloat32(_), Self::KeyFloat32(_)) => true,
            (Self::KeyFloat64(_), Self::KeyFloat64(_)) => true,
            (Self::KeyBytes(_), Self::KeyBytes(_)) => true,
            (Self::KeyVersion(_), Self::KeyVersion(_)) => true,
            (Self::KeyRequire(_), Self::KeyRequire(_)) => true,
            (Self::KeyLabel(_), Self::KeyLabel(_)) => true,
            (Self::KeyArray(_), Self::KeyArray(_)) => true,
            (Self::KeyTable(_), Self::KeyTable(_)) => true,
            (Self::KeySection(_), Self::KeySection(_)) => true,
            (Self::KeyBlock(_), Self::KeyBlock(_)) => true,
            (Self::KeySymbol(_), Self::KeySymbol(_)) => true,
            (Self::KeyModule(_), Self::KeyModule(_)) => true,
            (Self::KeyUse(_), Self::KeyUse(_)) => true,
            (Self::KeyAs(_), Self::KeyAs(_)) => true,
            (Self::KeySchema(_), Self::KeySchema(_)) => true,
            (Self::Int((_, int1)), Self::Int((_, int2))) => int1 == int2,
            (Self::Float((_, float1)), Self::Float((_, float2))) => float1 == float2,
            (Self::MacroString((_, str1)), Self::MacroString((_, str2))) => str1 == str2,
            (Self::ByteString((_, bytes1)), Self::ByteString((_, bytes2))) => bytes1 == bytes2,
            (Self::String((_, str1)), Self::String((_, str2))) => str1 == str2,
            (Self::Identifier((_, id1)), Self::Identifier((_, id2))) => id1 == id2,
            (Self::MacroIdentifier((_, id1)), Self::MacroIdentifier((_, id2))) => id1 == id2,
            (Self::LabelIdentifier((_, id1)), Self::LabelIdentifier((_, id2))) => id1 == id2,
            (Self::SymbolIdentifier((_, id1)), Self::SymbolIdentifier((_, id2))) => id1 == id2,
            (Self::ControlIdentifier((_, id1)), Self::ControlIdentifier((_, id2))) => id1 == id2,
            (Self::Version((_, ver1)), Self::Version((_, ver2))) => ver1 == ver2,
            (Self::Require((_, req1)), Self::Require((_, req2))) => req1 == req2,
            (Self::LineComment((_, comment1)), Self::LineComment((_, comment2))) => {
                comment1 == comment2
            }
            (Self::MultiLineComment((_, comment1)), Self::MultiLineComment((_, comment2))) => {
                comment1 == comment2
            }
            _ => false,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum HashableFloat {
    Generic(f64),
    Float32(f32),
    Float64(f64),
}

impl Eq for HashableFloat {}

impl Hash for HashableFloat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Float32(float) => state.write(float.to_be_bytes().as_slice()),
            Self::Float64(float) | Self::Generic(float) => {
                state.write(float.to_be_bytes().as_slice())
            }
        }
    }
}

impl From<HashableFloat> for f32 {
    fn from(val: HashableFloat) -> Self {
        match val {
            HashableFloat::Float32(value) => value,
            HashableFloat::Float64(value) | HashableFloat::Generic(value) => value as f32,
        }
    }
}

impl From<HashableFloat> for f64 {
    fn from(val: HashableFloat) -> Self {
        match val {
            HashableFloat::Float32(value) => value as f64,
            HashableFloat::Float64(value) | HashableFloat::Generic(value) => value,
        }
    }
}

fn multiline_comment(lexer: &mut Lexer<Token>) -> (Location, String) {
    let slice = lexer.slice();
    let comment = slice.trim_start_matches("/*").trim_end_matches("*/").trim();
    (base_callback(lexer), comment.to_string())
}

fn line_comment(lexer: &mut Lexer<Token>) -> (Location, String) {
    let slice = lexer.slice();
    let comment = slice
        .lines()
        .map(|x| x.trim_start_matches('#').trim())
        .collect::<Vec<_>>()
        .join("\n");
    (base_callback(lexer), comment.to_string())
}

fn version_require(lexer: &mut Lexer<Token>) -> Result<(Location, semver::VersionReq)> {
    let slice = lexer.slice();
    let location = base_callback(lexer);
    Ok((
        location.clone(),
        semver::VersionReq::parse(slice).map_err(|e| error::Error::Require {
            location: location.clone(),
            reason: e.to_string(),
        })?,
    ))
}

fn version_literal(lexer: &mut Lexer<Token>) -> Result<(Location, semver::Version)> {
    let slice = lexer.slice();
    let location = base_callback(lexer);
    Ok((
        location.clone(),
        semver::Version::parse(slice).map_err(|e| error::Error::Version {
            location: location.clone(),
            reason: e.to_string(),
        })?,
    ))
}

fn quote_string(lexer: &mut Lexer<Token>) -> (Location, String) {
    let slice = lexer.slice();
    let value = slice
        .trim_start_matches('"')
        .trim_start_matches('\'')
        .trim_end_matches('\'')
        .trim_end_matches('"');
    (base_callback(lexer), value.to_string())
}

fn byte_string(lexer: &mut Lexer<Token>) -> Result<(Location, Vec<u8>)> {
    let slice = lexer
        .slice()
        .trim_start_matches("b'")
        .trim_end_matches('\'');
    let location = base_callback(lexer);
    Ok((
        location.clone(),
        base64::engine::general_purpose::STANDARD
            .decode(slice)
            .context(error::Base64Snafu {
                location: location.clone(),
            })?,
    ))
}

fn macro_string(lexer: &mut Lexer<Token>) -> (Location, String) {
    let slice = lexer.slice();
    (
        base_callback(lexer),
        slice
            .trim_start_matches("m'")
            .trim_end_matches('\'')
            .to_string(),
    )
}

fn float(lexer: &mut Lexer<Token>) -> Result<(Location, HashableFloat)> {
    let slice = lexer.slice();
    let location = base_callback(lexer);
    if slice.ends_with("f64") {
        let slice = slice.trim_end_matches("f64");
        return Ok((
            location.clone(),
            HashableFloat::Float64(slice.parse().context(error::FloatSnafu {
                location: location.clone(),
            })?),
        ));
    }
    if slice.ends_with("f32") {
        let slice = slice.trim_end_matches("f32");
        return Ok((
            location.clone(),
            HashableFloat::Float32(slice.parse().context(error::FloatSnafu {
                location: location.clone(),
            })?),
        ));
    }
    Ok((
        location.clone(),
        HashableFloat::Generic(slice.parse().context(error::FloatSnafu {
            location: location.clone(),
        })?),
    ))
}

macro_rules! number {
    ($radix_name: ident : $radix: literal [ $($type: ident as $wrap: ident where $suffix: literal),* ] ) => {
        pub(crate) mod $radix_name {
            use snafu::ResultExt;
            pub fn integer(lexer: &mut logos::Lexer<$crate::syn::Token>) -> $crate::Result<($crate::ast::Location, super::Integer)> {
                let location = $crate::syn::lexer::base_callback(lexer);
                let slice = lexer.slice().replace('_', "");
                let slice = slice.as_str();
                let slice = slice.trim_start_matches("0x").trim_start_matches("0o").trim_start_matches("0b");
                $(
                    if slice.ends_with($suffix) {
                        let slice = slice.trim_end_matches($suffix);
                        let value = $type::from_str_radix(slice, $radix).context($crate::error::IntegerSnafu {
                            location: location.clone(),
                        })?;
                        return Ok((location.clone(), super::Integer::$wrap(value)));
                    }
                )*
                let value = i64::from_str_radix(slice, $radix).context($crate::error::IntegerSnafu {
                    location: location.clone(),
                })?;
                Ok((location.clone(), super::Integer::Signed(value)))
            }
        }
    }
}

number!(decimal: 10 [
    u64 as Unsigned where "uint",
    i128 as I128 where "i128",
    u128 as U128 where "u128",
    i64 as I64 where "i64",
    u64 as U64 where "u64",
    i32 as I32 where "i32",
    u32 as U32 where "u32",
    i16 as I16 where "i16",
    u16 as U16 where "u16",
    i8 as I8 where "i8",
    u8 as U8 where "u8"
]);

number!(hexadecimal: 16 [
    u64 as Unsigned where "uint",
    i128 as I128 where "i128",
    u128 as U128 where "u128",
    i64 as I64 where "i64",
    u64 as U64 where "u64",
    i32 as I32 where "i32",
    u32 as U32 where "u32",
    i16 as I16 where "i16",
    u16 as U16 where "u16",
    i8 as I8 where "i8",
    u8 as U8 where "u8"
]);
number!(octal: 8 [
    u64 as Unsigned where "uint",
    i128 as I128 where "i128",
    u128 as U128 where "u128",
    i64 as I64 where "i64",
    u64 as U64 where "u64",
    i32 as I32 where "i32",
    u32 as U32 where "u32",
    i16 as I16 where "i16",
    u16 as U16 where "u16",
    i8 as I8 where "i8",
    u8 as U8 where "u8"
]);
number!(binary: 2 [
    u64 as Unsigned where "uint",
    i128 as I128 where "i128",
    u128 as U128 where "u128",
    i64 as I64 where "i64",
    u64 as U64 where "u64",
    i32 as I32 where "i32",
    u32 as U32 where "u32",
    i16 as I16 where "i16",
    u16 as U16 where "u16",
    i8 as I8 where "i8",
    u8 as U8 where "u8"
]);

#[cfg(test)]
mod test {
    use super::{HashableFloat, Integer, Token};
    use crate::ast::Location;
    use logos::Logos;

    fn assert_single_token(input: &str, expected: Token) {
        let mut lexer = Token::lexer(input);
        let token = lexer.next().unwrap().unwrap();
        assert!(
            token.eq_kind(&expected),
            "Got a token {:?} but expected {:?}",
            token,
            expected
        );
        assert!(lexer.next().is_none(), "Expected only one token");
    }

    #[test]
    fn test_byte_string() {
        let mut lexer = Token::lexer("b'aGVsbG8='");
        if let Ok(Token::ByteString((_, bytes))) = lexer.next().unwrap() {
            assert_eq!(bytes, b"hello");
        } else {
            panic!("Expected ByteString token");
        }
    }

    #[test]
    fn test_boolean_tokens() {
        // Test all boolean true variants
        assert_single_token("true", Token::True(Location::default()));
        assert_single_token("True", Token::True(Location::default()));
        assert_single_token("yes", Token::True(Location::default()));
        assert_single_token("Yes", Token::True(Location::default()));
        assert_single_token("on", Token::True(Location::default()));
        assert_single_token("On", Token::True(Location::default()));

        // Test all boolean false variants
        assert_single_token("false", Token::False(Location::default()));
        assert_single_token("False", Token::False(Location::default()));
        assert_single_token("no", Token::False(Location::default()));
        assert_single_token("No", Token::False(Location::default()));
        assert_single_token("off", Token::False(Location::default()));
        assert_single_token("Off", Token::False(Location::default()));
    }

    #[test]
    fn test_punctuation_tokens() {
        assert_single_token("!", Token::Exclaim(Location::default()));
        assert_single_token("$", Token::Dollar(Location::default()));
        assert_single_token("[", Token::LBracket(Location::default()));
        assert_single_token("]", Token::RBracket(Location::default()));
        assert_single_token("{", Token::LBrace(Location::default()));
        assert_single_token("}", Token::RBrace(Location::default()));
        assert_single_token("(", Token::LParen(Location::default()));
        assert_single_token(")", Token::RParen(Location::default()));
        assert_single_token("=", Token::Assign(Location::default()));
        assert_single_token(":", Token::Colon(Location::default()));
        assert_single_token("?", Token::Question(Location::default()));
        assert_single_token(",", Token::Comma(Location::default()));
    }

    #[test]
    fn test_keyword_tokens() {
        assert_single_token("null", Token::KeyNull(Location::default()));
        assert_single_token("nil", Token::KeyNull(Location::default()));
        assert_single_token("none", Token::KeyNull(Location::default()));
        assert_single_token("bool", Token::KeyBool(Location::default()));
        assert_single_token("string", Token::KeyString(Location::default()));
        assert_single_token("int", Token::KeyInt(Location::default()));
        assert_single_token("uint", Token::KeyUInt(Location::default()));
        assert_single_token("float", Token::KeyFloat(Location::default()));
        assert_single_token("bytes", Token::KeyBytes(Location::default()));
    }

    #[test]
    fn test_integer_tokens() {
        // Test decimal integers
        let mut lexer = Token::lexer("123");
        if let Ok(Token::Int((_, Integer::Signed(value)))) = lexer.next().unwrap() {
            assert_eq!(value, 123);
        } else {
            panic!("Expected Int token with value 123");
        }

        // Test with type suffix
        let mut lexer = Token::lexer("42i8");
        if let Ok(Token::Int((_, Integer::I8(value)))) = lexer.next().unwrap() {
            assert_eq!(value, 42i8);
        } else {
            panic!("Expected Int token with i8 value 42");
        }

        // Test hexadecimal
        let mut lexer = Token::lexer("0xFF");
        if let Ok(Token::Int((_, Integer::Signed(value)))) = lexer.next().unwrap() {
            assert_eq!(value, 255);
        } else {
            panic!("Expected Int token with hex value 0xFF (255)");
        }

        // Test binary
        let mut lexer = Token::lexer("0b1010");
        if let Ok(Token::Int((_, Integer::Signed(value)))) = lexer.next().unwrap() {
            assert_eq!(value, 10);
        } else {
            panic!("Expected Int token with binary value 0b1010 (10)");
        }

        // Test octal
        let mut lexer = Token::lexer("0o10");
        if let Ok(Token::Int((_, Integer::Signed(value)))) = lexer.next().unwrap() {
            assert_eq!(value, 8);
        } else {
            panic!("Expected Int token with octal value 0o10 (8)");
        }
    }

    #[test]
    fn test_float_tokens() {
        // Test basic float
        let mut lexer = Token::lexer("123.45");
        if let Ok(Token::Float((_, float_val))) = lexer.next().unwrap() {
            match float_val {
                HashableFloat::Generic(value) => assert_eq!(value, 123.45),
                _ => panic!("Expected generic float"),
            }
        } else {
            panic!("Expected Float token");
        }

        // Test with type suffix
        let mut lexer = Token::lexer("42.5f32");
        if let Ok(Token::Float((_, float_val))) = lexer.next().unwrap() {
            match float_val {
                HashableFloat::Float32(value) => assert_eq!(value, 42.5f32),
                _ => panic!("Expected f32 float"),
            }
        } else {
            panic!("Expected Float token with f32 suffix");
        }

        // Test scientific notation
        let mut lexer = Token::lexer("1.5e3");
        if let Ok(Token::Float((_, float_val))) = lexer.next().unwrap() {
            match float_val {
                HashableFloat::Generic(value) => assert_eq!(value, 1500.0),
                _ => panic!("Expected generic float with scientific notation"),
            }
        } else {
            panic!("Expected Float token with scientific notation");
        }
    }

    #[test]
    fn test_string_tokens() {
        // Test single quoted string
        let mut lexer = Token::lexer("'hello'");
        if let Token::String((_, value)) = lexer.next().unwrap().unwrap() {
            assert_eq!(value, "hello");
        } else {
            panic!("Expected String token");
        }

        // Test double quoted string
        let mut lexer = Token::lexer("\"world\"");
        if let Token::String((_, value)) = lexer.next().unwrap().unwrap() {
            assert_eq!(value, "world");
        } else {
            panic!("Expected String token");
        }

        // Test macro string
        let mut lexer = Token::lexer("m'macro'");
        if let Token::MacroString((_, value)) = lexer.next().unwrap().unwrap() {
            assert_eq!(value, "macro");
        } else {
            panic!("Expected MacroString token");
        }
    }

    #[test]
    fn test_identifier_tokens() {
        // Test regular identifier
        let mut lexer = Token::lexer("variable");
        if let Token::Identifier((_, value)) = lexer.next().unwrap().unwrap() {
            assert_eq!(value, "variable");
        } else {
            panic!("Expected Identifier token");
        }

        // Test macro identifier
        let mut lexer = Token::lexer("m!macro_name");
        if let Token::MacroIdentifier((_, value)) = lexer.next().unwrap().unwrap() {
            assert_eq!(value, "macro_name");
        } else {
            panic!("Expected MacroIdentifier token");
        }

        // Test label identifier
        let mut lexer = Token::lexer("!label");
        if let Token::LabelIdentifier((_, value)) = lexer.next().unwrap().unwrap() {
            assert_eq!(value, "label");
        } else {
            panic!("Expected LabelIdentifier token");
        }

        // Test symbol identifier
        let mut lexer = Token::lexer(":symbol");
        if let Token::SymbolIdentifier((_, value)) = lexer.next().unwrap().unwrap() {
            assert_eq!(value, "symbol");
        } else {
            panic!("Expected SymbolIdentifier token");
        }

        // Test control identifier
        let mut lexer = Token::lexer("$control");
        if let Token::ControlIdentifier((_, value)) = lexer.next().unwrap().unwrap() {
            assert_eq!(value, "control");
        } else {
            panic!("Expected ControlIdentifier token");
        }
    }

    #[test]
    fn test_version_tokens() {
        // Test version literal
        let mut lexer = Token::lexer("1.2.3");
        if let Ok(Token::Version((_, version))) = lexer.next().unwrap() {
            assert_eq!(version.major, 1);
            assert_eq!(version.minor, 2);
            assert_eq!(version.patch, 3);
        } else {
            panic!("Expected Version token");
        }

        // Test version requirement
        let mut lexer = Token::lexer("^1.2.3");
        if let Ok(Token::Require((_, req))) = lexer.next().unwrap() {
            // Just check that it parsed successfully
            assert!(req.to_string().contains("^1.2.3"));
        } else {
            panic!("Expected Require token");
        }
    }

    #[test]
    fn test_comment_tokens() {
        // Test line comment
        let mut lexer = Token::lexer("# This is a comment\n");
        if let Token::LineComment((_, comment)) = lexer.next().unwrap().unwrap() {
            assert_eq!(comment, "This is a comment");
        } else {
            panic!("Expected LineComment token");
        }

        // Test multiline comment
        let mut lexer = Token::lexer("/* This is a\nmultiline comment */");
        if let Token::MultiLineComment((_, comment)) = lexer.next().unwrap().unwrap() {
            assert_eq!(comment, "This is a\nmultiline comment");
        } else {
            panic!("Expected MultiLineComment token");
        }
    }

    #[test]
    fn test_whitespace_handling() {
        // Test that whitespace is properly skipped
        let mut lexer = Token::lexer("  true  false  ");
        assert!(
            lexer
                .next()
                .unwrap()
                .unwrap()
                .eq_kind(&Token::True(Location::default()))
        );
        assert!(
            lexer
                .next()
                .unwrap()
                .unwrap()
                .eq_kind(&Token::False(Location::default()))
        );
        assert!(lexer.next().is_none());
    }

    #[test]
    fn test_newline_handling() {
        // Test that newlines are properly recognized
        let mut lexer = Token::lexer("true\nfalse");
        assert!(
            lexer
                .next()
                .unwrap()
                .unwrap()
                .eq_kind(&Token::True(Location::default()))
        );
        // assert_eq!(lexer.next().unwrap().unwrap(), Token::Newline);
        assert!(
            lexer
                .next()
                .unwrap()
                .unwrap()
                .eq_kind(&Token::False(Location::default()))
        );
    }

    #[test]
    fn test_multiple_tokens() {
        // Test a more complex example with multiple tokens
        let input = "section {\n  name: 'test',\n  value: 42\n}";
        let mut lexer = Token::lexer(input);

        // Check each token in sequence
        assert!(
            lexer
                .next()
                .unwrap()
                .unwrap()
                .eq_kind(&Token::KeySection(Location::default()))
        );
        assert!(
            lexer
                .next()
                .unwrap()
                .unwrap()
                .eq_kind(&Token::LBrace(Location::default()))
        );

        if let Token::Identifier((_, name)) = lexer.next().unwrap().unwrap() {
            assert_eq!(name, "name");
        } else {
            panic!("Expected Identifier token");
        }

        assert!(
            lexer
                .next()
                .unwrap()
                .unwrap()
                .eq_kind(&Token::Colon(Location::default()))
        );

        if let Token::String((_, value)) = lexer.next().unwrap().unwrap() {
            assert_eq!(value, "test");
        } else {
            panic!("Expected String token");
        }

        assert!(
            lexer
                .next()
                .unwrap()
                .unwrap()
                .eq_kind(&Token::Comma(Location::default()))
        );

        if let Token::Identifier((_, name)) = lexer.next().unwrap().unwrap() {
            assert_eq!(name, "value");
        } else {
            panic!("Expected Identifier token");
        }

        assert!(
            lexer
                .next()
                .unwrap()
                .unwrap()
                .eq_kind(&Token::Colon(Location::default()))
        );

        if let Ok(Token::Int((_, Integer::Signed(value)))) = lexer.next().unwrap() {
            assert_eq!(value, 42);
        } else {
            panic!("Expected Int token");
        }

        assert!(
            lexer
                .next()
                .unwrap()
                .unwrap()
                .eq_kind(&Token::RBrace(Location::default()))
        );
    }
}
