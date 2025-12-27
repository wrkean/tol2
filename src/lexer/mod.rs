use std::sync::Arc;

use logos::Logos;
use miette::NamedSource;

use crate::{
    error::CompilerError,
    lexer::token::{Token, TokenKind},
    module::lexed_module::LexedModule,
};

pub mod token;

pub struct Lexer;

impl Lexer {
    pub fn lex(source_code: &str, source_file_name: &str) -> (LexedModule, Vec<CompilerError>) {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();

        let mut kind_iter = TokenKind::lexer(source_code);
        while let Some(tk) = kind_iter.next() {
            match tk {
                Ok(t) => tokens.push(Token {
                    kind: t,
                    lexeme: kind_iter.slice().to_string(),
                    span: kind_iter.span(),
                }),
                Err(e) => {
                    errors.push(CompilerError::Lexer {
                        message: e.to_string(),
                        // src: NamedSource::new(source_file_name, source_code),
                        span: e.span().into(),
                        help: e.help().map(|s| s.to_string()),
                    });
                }
            }
        }
        tokens.push(Token {
            kind: TokenKind::Eof,
            lexeme: "<EOF>".to_string(),
            span: 0..0,
        });

        (
            LexedModule {
                tokens,
                src_filename: source_file_name.to_string(),
            },
            errors,
        )
    }
}
