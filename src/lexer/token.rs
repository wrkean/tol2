use std::ops::Range;

use tokenkind_derive::TolTokenKind;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub span: Range<usize>,
}

impl Token {
    pub fn new_dummy() -> Self {
        Self {
            kind: TokenKind::Dummy,
            lexeme: "".to_string(),
            span: 0..0,
        }
    }
}

impl Token {
    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }

    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }

    pub fn span(&self) -> Range<usize> {
        self.span.clone()
    }
}

#[derive(TolTokenKind, Debug, Clone, PartialEq)]
pub enum TokenKind {
    #[keyword]
    #[stmt_starter]
    Ang,

    #[keyword]
    #[stmt_starter]
    Dapat,

    #[keyword]
    #[stmt_starter]
    Paraan,

    #[keyword]
    #[stmt_starter]
    Babalik,

    #[keyword]
    #[stmt_starter]
    Bawat,

    #[keyword]
    #[stmt_starter]
    Gagawin,

    #[keyword]
    #[stmt_starter]
    Habang,

    #[keyword]
    #[stmt_starter]
    Ibalik,

    #[keyword]
    #[stmt_starter]
    Kung,

    #[keyword]
    Sa,

    #[keyword]
    Tama,

    #[keyword]
    Mali,

    #[keyword]
    Kungdi,

    #[keyword]
    Na,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    EqualEqual,
    Pipe,
    PipePipe,
    Amper,
    AmperAmper,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Arrow,

    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Colon,
    Semicolon,
    Indent,
    Dedent,

    // Literals
    Integer,
    HexLiteral,
    OctalLiteral,
    BinLiteral,
    Float,
    String,
    UnterminatedString,
    Identifier,
    Comment,
    Whitespace,
    Newline,

    Eof,
    Dummy,
}

impl TokenKind {
    pub fn starts_an_expression(&self) -> bool {
        matches!(
            self,
            TokenKind::Integer | TokenKind::Float | TokenKind::Identifier
        )
    }

    pub fn starts_a_type(&self) -> bool {
        matches!(self, TokenKind::Identifier)
    }
}
