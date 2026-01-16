#![allow(unused)]

use colored::Colorize;
use miette::{Diagnostic, LabeledSpan, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum CompilerError {
    #[error("{}: {}", "Mali sa lexer".bright_red(), message)]
    Lexer {
        message: String,

        #[label("ito")]
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

        #[label("{expected}")]
        span: SourceSpan,

        #[help]
        help: Option<String>,
    },

    #[error("{}", "Hindi inaasahang tipo".bright_red())]
    UnexpectedType {
        found: String,

        #[label("Umasa ng tipo pero nakita ay {found}")]
        span: SourceSpan,

        #[help]
        help: Option<String>,
    },

    #[error("{}", "Maling pagumpisa ng pahayag".bright_red())]
    InvalidStartOfStatement {
        #[label("Hindi ito pwede magumpisa ng pahayag")]
        span: SourceSpan,
    },

    #[error("{}", "Walang kapares na delimiter".bright_red())]
    UnmatchedDelimiter {
        delimiter: String,

        #[label("Walang kapares ang {delimiter}")]
        span: SourceSpan,
    },

    #[error("{}", "Hindi naisaradong bracket".bright_red())]
    UnmatchedBracket {
        bracket: char,

        #[label("Ang `{bracket}` ay hindi naisarado")]
        span: SourceSpan,
    },

    #[error("{}", "Hindi naideklarang pangalan".bright_red())]
    UndeclaredSymbol {
        #[label("Hindi pa ito naideklara")]
        span: SourceSpan,
    },

    #[error("{}", "Pagdeklara ulit ng kaparehong pangalan sa kaparehong sakop".bright_red())]
    Redeclaration {
        #[label("Naideklara na dito")]
        declared_span: SourceSpan,

        #[label("Idineklara ulit dito")]
        redeclared_span: SourceSpan,
    },

    #[error("{}", "Bawal na expresyon".bright_red())]
    InvalidExpression {
        #[label(collection)]
        spans: Vec<LabeledSpan>,

        #[help]
        help: Option<String>,
    },

    #[error("{} Hindi pwede ang `{lhs_type}` at `{rhs_type}`", "Mismatch ng tipo:".bright_red())]
    TypeMismatch {
        lhs_type: String,
        rhs_type: String,

        #[label(collection)]
        spans: Vec<LabeledSpan>,
    },

    #[error("{}", "Hindi inaasahang tipo".bright_red())]
    UnexpectedType2 {
        expected: String,
        found: String,

        #[label("Umasa ng `{expected}` pero ito ay `{found}`")]
        span: SourceSpan,
    },

    #[error("{}", "Maling pag-dedent".bright_red())]
    InvalidDedent {
        #[label("dito")]
        span: SourceSpan,
    },

    #[error("{}", "Hindi naisarang string".bright_red())]
    UnterminatedString {
        #[label("simula ng string")]
        span: SourceSpan,
    },

    #[error("{}", "Hindi kilalang escape character".bright_red())]
    InvalidEscapeCharacter {
        #[label("ito")]
        span: SourceSpan,
    },

    #[error("{}", "Maling bilang ng argumento".bright_red())]
    InvalidNumberOfArguments {
        arg_len: usize,
        expected_len: usize,

        #[label("{arg_len} na argumento, umaasa ng {expected_len}")]
        args_span: SourceSpan,
    },

    #[error("{}", "Tinawag ang hindi natatawag")]
    InvalidCallExpression {
        #[label("Baka hindi ito idineklara bilang isang `paraan`?")]
        span: SourceSpan,
    },
}
