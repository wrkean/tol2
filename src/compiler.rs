use std::{path::Path, sync::Arc};

use crate::{
    args::Args, error::CompilerError, lexer::Lexer, module::module_registry::ModuleRegistry,
    parser::Parser,
};

pub struct Compiler<'com> {
    module_registry: ModuleRegistry<'com>,
    config: Args,
}

impl<'com> Compiler<'com> {
    pub fn new(args: Args) -> Self {
        Self {
            module_registry: ModuleRegistry::new(),

            config: args,
        }
    }

    pub fn run(&self, source_code: &str) -> Result<(), Vec<CompilerError>> {
        // This here is guaranteed to be a filename as it is checked by the driver beforehand
        // WARN: Have better handling for this
        let source_file_name = self
            .config
            .source_path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();

        let (lexed_mod, mut errors) = Lexer::lex(source_code, source_file_name);

        let parser = Parser::new(lexed_mod);
        let parsed_mod = {
            let (pmod, pars_errs) = parser.parse();
            errors.extend(pars_errs);

            println!("{:#?}", &pmod.ast);

            pmod
        };

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn load_stdlib(&mut self, stdlib_path: &Path) {
        todo!()
    }
}
