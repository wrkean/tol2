use std::{collections::HashMap, path::Path, rc::Rc};

use logos::Logos;

use crate::{args::Args, module::compiled_module::CompiledModule, token::Token};

pub struct ModuleRegistry<'com> {
    main_module: Option<CompiledModule<'com>>,
    stdlib: Option<CompiledModule<'com>>,
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

    pub fn run(&self) {
        let tokens = Token::lexer(&self.source_code);

        for tok in tokens {
            todo!()
        }
    }

    pub fn load_stdlib(&mut self, stdlib_path: &Path) {
        todo!()
    }
}
