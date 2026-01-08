use std::ops::Range;

use logos::Logos;

use crate::{
    error::CompilerError,
    lexer::token::{Token, TokenKind},
    module::lexed_module::LexedModule,
};

pub mod token;

#[derive(Default)]
pub struct LexerState {
    bracket_stack: Vec<(char, Range<usize>)>,
}

pub struct Lexer;

impl Lexer {
    pub fn lex(source_code: &str, source_file_name: &str) -> (LexedModule, Vec<CompilerError>) {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();

        let mut kind_iter = TokenKind::lexer(source_code);
        while let Some(res) = kind_iter.next() {
            match res {
                Ok(tk) => {
                    match tk {
                        TokenKind::LParen | TokenKind::LBrace => {
                            kind_iter.extras.bracket_stack.push(match tk {
                                TokenKind::LParen => ('(', kind_iter.span()),
                                TokenKind::LBrace => ('{', kind_iter.span()),
                                _ => unreachable!(),
                            });
                        }
                        TokenKind::RParen | TokenKind::RBrace => {
                            let expected = match tk {
                                TokenKind::RParen => '(',
                                TokenKind::RBrace => '{',
                                _ => unreachable!(),
                            };

                            if let Some((ch, _)) = kind_iter.extras.bracket_stack.pop()
                                && ch != expected
                            {
                                errors.push(CompilerError::UnmatchedDelimiter {
                                    delimiter: kind_iter.slice().to_string(),
                                    span: kind_iter.span().into(),
                                });
                                continue;
                            }
                        }
                        _ => {}
                    }
                    tokens.push(Token {
                        kind: tk,
                        lexeme: kind_iter.slice().to_string(),
                        span: kind_iter.span(),
                    });
                }
                Err(e) => {
                    errors.push(CompilerError::Lexer {
                        message: e.to_string(),
                        span: e.span().into(),
                        help: e.help().map(|s| s.to_string()),
                    });
                }
            }
        }
        tokens.push(Token {
            kind: TokenKind::Eof,
            lexeme: "<EOF>".to_string(),
            span: if tokens.is_empty() {
                0..0
            } else {
                let last_span = tokens.last().unwrap().span.end;
                last_span..last_span + 1
            },
        });

        // Report errors for unmatched brackets
        for (ch, span) in kind_iter.extras.bracket_stack.iter() {
            errors.push(CompilerError::UnmatchedBracket {
                bracket: *ch,
                span: span.to_owned().into(),
            });
        }

        for tok in tokens.iter() {
            println!("{:?} <=> {:?}", tok.lexeme(), tok.kind());
        }

        (
            LexedModule {
                tokens,
                src_filename: source_file_name.to_string(),
            },
            errors,
        )
    }
}
