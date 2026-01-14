#![allow(unused)]

use colored::Colorize;
use miette::{Diagnostic, LabeledSpan, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum CompilerError {
    #[error("{}: {}", "Mali sa lexer".bright_red(), message)]
    Lexer {
        message: String,

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

    #[error("{}", "Walang kapares na bracket".bright_red())]
    UnmatchedBracket {
        bracket: char,

        #[label("Walang kapares ang `{bracket}`")]
        span: SourceSpan,
    },

    #[error("{}", "Hindi naideklarang pangalan".bright_red())]
    UndeclaredSymbol {
        message: String,

        #[label("{message}")]
        span: SourceSpan,
    },

    #[error("{}", "Pagdeklara ulit ng kaparehong pangalan sa kaparehong sakop".bright_red())]
    Redeclaration {
        declared_message: String,
        redeclared_message: String,

        #[label("{declared_message}")]
        declared_span: SourceSpan,

        #[label("{redeclared_message}")]
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
    #[help("Subukang gamitin ang `gawing` (halimbawa: `<expresyon> gawing <tipo>`)")]
    TypeMismatch {
        lhs_type: String,
        rhs_type: String,

        #[label(collection)]
        spans: Vec<LabeledSpan>, // Spans indicate the mismatched types
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
    // #[error("{}", "Hindi wastong numero sa hexadecimal".bright_red())]
    // #[help(
    //     "Ang hexadecimal na literal ay may sakop na `0` hanggang `F` lamang (0, 1, 2, ... 8, 9, A, ... E, F)"
    // )]
    // InvalidHexLiteral {
    //     #[label("ito")]
    //     span: SourceSpan,
    // },
    //
    // #[error("{}", "Hindi wastong numero sa binary".bright_red())]
    // #[help("Ang binary na literal ay may sakop na `0` at `1` lamang")]
    // InvalidBinLiteral {
    //     #[label("ito")]
    //     span: SourceSpan,
    // },
    //
    // #[error("{}", "Hindi wastong numero sa octal".bright_red())]
    // #[help("Ang octal na literal ay may sakop na `0` hanggang `7` lamang")]
    // InvalidOctLiteral {
    //     #[label("ito")]
    //     span: SourceSpan,
    // },
}
