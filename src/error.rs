use std::fmt;

use colored::Colorize;
use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

use crate::lexer::token::TokenKind;

#[derive(Error, Debug, Diagnostic)]
#[allow(unused)]
pub enum CompilerError {
    #[error("{}: {}", "Mali sa lexer".bright_red(), message)]
    Lexer {
        message: String,

        #[source_code]
        src: NamedSource<String>,

        #[label("dito")]
        span: SourceSpan,

        #[help]
        help: Option<String>,
    },

    #[error("{}", "Mali sa I/O".bright_red())]
    IO(#[from] std::io::Error),

    #[error("{}", "Hindi inaasahang pagtatapos ng input".bright_red())]
    #[diagnostic(help("ito ay hindi madalas mangyari, maaaring bug ito sa compiler."))]
    UnexpectedEndOfInput,

    #[error("{}", "Hindi inaasahang token".bright_red())]
    UnexpectedToken {
        expected: String,

        #[source_code]
        src: NamedSource<String>,

        #[label("Umasa ng `{}` ito ang naibigay", &expected)]
        span: SourceSpan,

        #[help]
        help: Option<String>,
    },
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum LexingError {
    InvalidChar {
        character: String,
        span: logos::Span,
    },
    UnterminatedString {
        span: logos::Span,
    },

    #[default]
    Other,
}

impl fmt::Display for LexingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexingError::InvalidChar { .. } => {
                write!(f, "invalid na token")
            }
            LexingError::UnterminatedString { .. } => write!(f, "hindi natapos na string"),
            LexingError::Other => unreachable!(),
        }
    }
}

impl LexingError {
    pub fn span(&self) -> logos::Span {
        match self {
            LexingError::InvalidChar { span, .. } | LexingError::UnterminatedString { span } => {
                span.to_owned()
            }
            LexingError::Other => unreachable!(),
        }
    }

    pub fn help(&self) -> Option<&str> {
        match self {
            LexingError::InvalidChar { .. } => {
                Some("baka ang (mga) karakter ay hindi parte ng sintaks")
            }
            LexingError::UnterminatedString { .. } | LexingError::Other => None,
        }
    }

    pub fn unterminated_string(lexer: &mut logos::Lexer<TokenKind>) -> Result<(), LexingError> {
        let start = lexer.span().start;
        let remainder = lexer.remainder();

        let mut offset = 0;
        for c in remainder.chars() {
            if c == '\n' {
                break;
            }
            offset += c.len_utf8();
        }

        lexer.bump(offset);

        Err(LexingError::UnterminatedString {
            span: start..start + offset,
        })
    }

    pub fn invalid_char(lexer: &mut logos::Lexer<TokenKind>) -> LexingError {
        let start = lexer.span().start;
        let remainder = lexer.remainder();

        let mut characters = String::new();
        let mut offset = 0;

        for c in remainder.chars() {
            if can_start_valid_token(c) {
                break;
            }

            characters.push(c);
            offset += c.len_utf8();
        }

        lexer.bump(offset); // safe bump inside remainder

        LexingError::InvalidChar {
            character: characters,
            span: start..start + offset,
        }
    }
}

fn can_start_valid_token(c: char) -> bool {
    if c == '_' || c.is_ascii_alphabetic() {
        return true;
    }

    if c.is_ascii_digit() {
        return true;
    }

    matches!(
        c,
        '+' | '-' | '*' | '/' | '=' | '!' | '<' | '>' | ':' | ';' | ',' | '(' | ')' | '{' | '}'
    )
}
