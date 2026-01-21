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
    #[semicolon_inferrable]
    Gagawin,

    #[keyword]
    #[stmt_starter]
    Habang,

    #[keyword]
    #[stmt_starter]
    #[semicolon_inferrable]
    Ibalik,

    #[keyword]
    #[stmt_starter]
    Kung,

    #[keyword]
    Sa,

    #[keyword]
    #[semicolon_inferrable]
    Tama,

    #[keyword]
    #[semicolon_inferrable]
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
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
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
    FatArrow,
    ThinArrow,

    // Delimiters
    #[semicolon_inferrable]
    RParen,

    #[semicolon_inferrable]
    RBrace,

    #[semicolon_inferrable]
    RBracket,

    #[stmt_starter]
    Indent,

    LParen,
    LBrace,
    LBracket,
    Comma,
    Colon,
    Semicolon,
    Dedent,

    // Literals
    #[semicolon_inferrable]
    Integer,

    #[semicolon_inferrable]
    HexLiteral,

    #[semicolon_inferrable]
    OctalLiteral,

    #[semicolon_inferrable]
    BinLiteral,

    #[semicolon_inferrable]
    Float,

    #[semicolon_inferrable]
    String,

    #[semicolon_inferrable]
    Identifier,

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

    pub fn op_to_string(&self) -> Option<String> {
        Some(
            match self {
                TokenKind::Plus => "+",
                TokenKind::Minus => "-",
                TokenKind::Star => "*",
                TokenKind::Slash => "/",
                TokenKind::Equal => "=",
                TokenKind::PlusEqual => "+=",
                TokenKind::MinusEqual => "-=",
                TokenKind::StarEqual => "*=",
                TokenKind::SlashEqual => "/=",
                TokenKind::EqualEqual => "==",
                TokenKind::Pipe => "|",
                TokenKind::PipePipe => "||",
                TokenKind::Amper => "&",
                TokenKind::AmperAmper => "&&",
                TokenKind::Bang => "!",
                TokenKind::BangEqual => "!=",
                TokenKind::Less => "<",
                TokenKind::LessEqual => "<=",
                TokenKind::Greater => ">",
                TokenKind::GreaterEqual => ">=",
                _ => return None,
            }
            .to_string(),
        )
    }
}
