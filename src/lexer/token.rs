use std::ops::Range;

use logos::Logos;

use crate::error::LexingError;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub span: Range<usize>,
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip(r"[ \t\f\r\n]+"))]
#[logos(utf8 = true)]
#[logos(error(LexingError, LexingError::invalid_char))]
pub enum TokenKind {
    // Keywords
    #[token("ang")]
    Ang,
    #[token("dapat")]
    Dapat,
    #[token("paraan")]
    Paraan,
    #[token("sa")]
    Sa,
    #[token("habang")]
    Habang,
    #[token("tama")]
    Tama,
    #[token("mali")]
    Mali,

    // Operators
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("=")]
    Equal,
    #[token("==")]
    EqualEqual,
    #[token("!=")]
    BangEqual,
    #[token("<")]
    Less,
    #[token("<=")]
    LessEqual,
    #[token(">")]
    Greater,
    #[token(">=")]
    GreaterEqual,
    #[token("=>")]
    Arrow,

    // Delimiters
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,

    // Literals
    #[regex(r"[0-9]([0-9_]*[0-9])?")]
    Integer,
    #[regex(r"[0-9]([0-9_]*[0-9])?\.[0-9]([0-9_]*[0-9])?")]
    Float,
    #[regex(r#""([^"\\]|\\.)*""#)]
    String,
    #[regex(r#"""#, LexingError::unterminated_string)]
    UnterminatedString,
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
    #[regex(r"//[^\n]*", logos::skip, allow_greedy = true)]
    Comment,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lex_single_token() {
        let mut tokens = TokenKind::lexer("+-*/");

        assert_eq!(tokens.next(), Some(Ok(TokenKind::Plus)));
        assert_eq!(tokens.slice(), "+");

        assert_eq!(tokens.next(), Some(Ok(TokenKind::Minus)));
        assert_eq!(tokens.slice(), "-");

        assert_eq!(tokens.next(), Some(Ok(TokenKind::Star)));
        assert_eq!(tokens.slice(), "*");

        assert_eq!(tokens.next(), Some(Ok(TokenKind::Slash)));
        assert_eq!(tokens.slice(), "/");

        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn lex_keywords() {
        let mut tokens = TokenKind::lexer("ang dapat paraan sa habang");

        assert_eq!(tokens.next(), Some(Ok(TokenKind::Ang)));
        assert_eq!(tokens.slice(), "ang");

        assert_eq!(tokens.next(), Some(Ok(TokenKind::Dapat)));
        assert_eq!(tokens.slice(), "dapat");

        assert_eq!(tokens.next(), Some(Ok(TokenKind::Paraan)));
        assert_eq!(tokens.slice(), "paraan");

        assert_eq!(tokens.next(), Some(Ok(TokenKind::Sa)));
        assert_eq!(tokens.slice(), "sa");

        assert_eq!(tokens.next(), Some(Ok(TokenKind::Habang)));
        assert_eq!(tokens.slice(), "habang");

        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn ignores_comments() {
        let mut tokens = TokenKind::lexer(
            r#"// Compiler, ignore this
// Ignore this too
"#,
        );

        // Comments and whitespace should be skipped entirely
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn lex_identifiers_and_numbers() {
        let mut tokens = TokenKind::lexer("foo bar 123 45.67");

        assert_eq!(tokens.next(), Some(Ok(TokenKind::Identifier)));
        assert_eq!(tokens.slice(), "foo");

        assert_eq!(tokens.next(), Some(Ok(TokenKind::Identifier)));
        assert_eq!(tokens.slice(), "bar");

        assert_eq!(tokens.next(), Some(Ok(TokenKind::Integer)));
        assert_eq!(tokens.slice(), "123");

        assert_eq!(tokens.next(), Some(Ok(TokenKind::Float)));
        assert_eq!(tokens.slice(), "45.67");

        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn lex_mixed_operators_and_delimiters() {
        let mut tokens = TokenKind::lexer("(){}:; => == != <= >= < > + - * /");

        assert_eq!(tokens.next(), Some(Ok(TokenKind::LParen)));
        assert_eq!(tokens.slice(), "(");

        assert_eq!(tokens.next(), Some(Ok(TokenKind::RParen)));
        assert_eq!(tokens.slice(), ")");

        assert_eq!(tokens.next(), Some(Ok(TokenKind::LBrace)));
        assert_eq!(tokens.slice(), "{");

        assert_eq!(tokens.next(), Some(Ok(TokenKind::RBrace)));
        assert_eq!(tokens.slice(), "}");

        assert_eq!(tokens.next(), Some(Ok(TokenKind::Colon)));
        assert_eq!(tokens.slice(), ":");

        assert_eq!(tokens.next(), Some(Ok(TokenKind::Semicolon)));
        assert_eq!(tokens.slice(), ";");

        assert_eq!(tokens.next(), Some(Ok(TokenKind::Arrow)));
        assert_eq!(tokens.slice(), "=>");

        assert_eq!(tokens.next(), Some(Ok(TokenKind::EqualEqual)));
        assert_eq!(tokens.slice(), "==");

        assert_eq!(tokens.next(), Some(Ok(TokenKind::BangEqual)));
        assert_eq!(tokens.slice(), "!=");

        assert_eq!(tokens.next(), Some(Ok(TokenKind::LessEqual)));
        assert_eq!(tokens.slice(), "<=");

        assert_eq!(tokens.next(), Some(Ok(TokenKind::GreaterEqual)));
        assert_eq!(tokens.slice(), ">=");

        assert_eq!(tokens.next(), Some(Ok(TokenKind::Less)));
        assert_eq!(tokens.slice(), "<");

        assert_eq!(tokens.next(), Some(Ok(TokenKind::Greater)));
        assert_eq!(tokens.slice(), ">");

        assert_eq!(tokens.next(), Some(Ok(TokenKind::Plus)));
        assert_eq!(tokens.slice(), "+");

        assert_eq!(tokens.next(), Some(Ok(TokenKind::Minus)));
        assert_eq!(tokens.slice(), "-");

        assert_eq!(tokens.next(), Some(Ok(TokenKind::Star)));
        assert_eq!(tokens.slice(), "*");

        assert_eq!(tokens.next(), Some(Ok(TokenKind::Slash)));
        assert_eq!(tokens.slice(), "/");

        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn lex_strings() {
        let mut tokens = TokenKind::lexer(r#""hello" "escaped\n" "quotes\"inside""#);

        assert_eq!(tokens.next(), Some(Ok(TokenKind::String)));
        assert_eq!(tokens.slice(), r#""hello""#);

        assert_eq!(tokens.next(), Some(Ok(TokenKind::String)));
        assert_eq!(tokens.slice(), r#""escaped\n""#);

        assert_eq!(tokens.next(), Some(Ok(TokenKind::String)));
        assert_eq!(tokens.slice(), r#""quotes\"inside""#);

        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn lex_unterminated_string() {
        let mut tokens = TokenKind::lexer(r#""This is an unterminated string literal"#);

        assert!(matches!(tokens.next(), Some(Err(_))));

        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn lex_invalid_char() {
        let mut tokens = TokenKind::lexer("#?");

        assert!(matches!(tokens.next(), Some(Err(_))));

        assert_eq!(tokens.next(), None);
    }
}
