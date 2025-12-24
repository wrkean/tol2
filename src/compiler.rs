use std::{collections::HashMap, path::Path, rc::Rc};

use logos::Logos;
use miette::NamedSource;

use crate::{
    args::Args, error::CompilerError, module::compiled_module::CompiledModule, token::TokenKind,
};

pub struct ModuleRegistry<'com> {
    main_module: Option<CompiledModule<'com>>,
    stdlib: Option<CompiledModule<'com>>,
    #[allow(dead_code)]
    cache: HashMap<String, Rc<CompiledModule<'com>>>,
}

impl<'com> ModuleRegistry<'com> {
    pub fn new() -> Self {
        Self {
            main_module: None,
            stdlib: None,
            cache: HashMap::new(),
        }
    }

    pub fn cache(&mut self) {
        todo!()
    }

    pub fn is_main_loaded(&self) -> bool {
        self.main_module.is_some()
    }

    pub fn is_stdlib_loaded(&self) -> bool {
        self.stdlib.is_some()
    }
}

impl<'com> Default for ModuleRegistry<'com> {
    fn default() -> Self {
        Self::new()
    }
}

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
        let tokens = TokenKind::lexer(&self.source_code);
        let mut errors = Vec::new();

        for tok in tokens {
            match tok {
                Ok(t) => {
                    println!("Lexed: {:?}", t);
                }
                Err(e) => {
                    errors.push(CompilerError::Lexer {
                        message: e.to_string(),
                        src: NamedSource::new("some.tol", self.source_code.clone()),
                        span: e.span().into(),
                        help: e.help().map(|s| s.to_string()),
                    });
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    // pub fn lex(&self) -> Result<LexedModule, CompilerError> {}

    pub fn load_stdlib(&mut self, stdlib_path: &Path) {
        todo!()
    }
}
