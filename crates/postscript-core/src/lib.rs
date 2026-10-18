//! PostScript tokenizer/parser — Rust port of the Ruby
//! `lib/postscript/source/{lexer,ast_builder}.rb` pair.
//!
//! PARITY GATE: this crate replaces the pure-Ruby lexer only when it
//! reproduces the Ruby lexer's token stream byte-for-byte across the
//! postscript gem's spec corpus (149 examples) and the postsvg
//! consumption path. Until then the gem's hand-written lexer is the
//! fallback (see claricle/postscript#4).

/// Lex/parse error for malformed PostScript source.
#[derive(Debug, thiserror::Error)]
pub enum LexError {
    #[error("unexpected character {ch:?} at offset {offset}")]
    UnexpectedChar { ch: char, offset: usize },
    #[error("unterminated {what} starting at offset {offset}")]
    Unterminated { what: &'static str, offset: usize },
}

/// Token kinds mirrored from the Ruby lexer's output.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// integer or real literal
    Number(f64),
    /// literal name (slash-prefixed)
    LiteralName(String),
    /// executable name
    Name(String),
    /// (…) string literal
    StringLiteral(String),
    /// <hex> string literal
    HexString(String),
    /// [ array-beginner
    ArrayBegin,
    /// ] array-ender
    ArrayEnd,
    /// { procedure-begin
    ProcedureBegin,
    /// } procedure-end
    ProcedureEnd,
    /// << dictionary-begin
    DictionaryBegin,
    /// >> dictionary-end
    DictionaryEnd,
}

/// Tokenize PostScript source into the token stream the AST builder
/// consumes.
///
/// TODO(#4): implement the full lexer — parity harness is the
/// postscript gem spec suite (lib: claricle/postscript).
pub fn tokenize(_source: &str) -> Result<Vec<Token>, LexError> {
    Ok(Vec::new())
}
