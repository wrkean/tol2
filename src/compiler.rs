use std::path::Path;

use logos::Logos;
use miette::NamedSource;

use crate::{
    args::Args,
    error::CompilerError,
    module::{lexed_module::LexedModule, module_registry::ModuleRegistry},
    token::{Token, TokenKind},
};

pub struct Compiler<'com> {
    module_registry: ModuleRegistry<'com>,
    source_code: String,

    config: Args,
}

impl<'com> Compiler<'com> {
    pub fn new(args: Args, source_code: String) -> Self {
        Self {
            module_registry: ModuleRegistry::new(),
            source_code,

            config: args,
        }
    }

    pub fn run(&self) -> Result<(), Vec<CompilerError>> {
        let (lexed_mod, mut errors) = self.lex();

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn lex(&self) -> (LexedModule, Vec<CompilerError>) {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();

        let mut kind_iter = TokenKind::lexer(&self.source_code);
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
                        // FIXME: "some.tol is a placeholder, replace it."
                        src: NamedSource::new("some.tol", self.source_code.clone()),
                        span: e.span().into(),
                        help: e.help().map(|s| s.to_string()),
                    });
                }
            }
        }

        (LexedModule { tokens }, errors)
    }

    pub fn load_stdlib(&mut self, stdlib_path: &Path) {
        todo!()
    }
}
