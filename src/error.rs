use colored::Colorize;
use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum CompilerError {
    #[error("{}: {}", "MALI SA LEXER".bright_red(), message)]
    #[diagnostic(help("Baka ang simbolong ito ay di parte ng sintaks"))]
    LexerError {
        message: String,

        #[source_code]
        src: NamedSource<String>,

        #[label("dito")]
        span: SourceSpan,
    },

    #[error("{}", "MALI SA I/O".bright_red())]
    #[diagnostic(help("Siguraduhing tama ang file path at may wastong permisyon ito"))]
    IOError(#[from] std::io::Error),
}
