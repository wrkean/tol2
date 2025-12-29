use crate::{
    driver::CompilerOptions, error::CompilerError, lexer::Lexer,
    module::module_registry::ModuleRegistry,
};
use std::path::Path;

pub struct Compiler<'com> {
    module_registry: ModuleRegistry<'com>,
    opts: CompilerOptions,
}

impl<'com> Compiler<'com> {
    pub fn new(opts: CompilerOptions) -> Self {
        Self {
            module_registry: ModuleRegistry::new(),

            opts,
        }
    }

    pub fn run(&self, source_code: &str) -> Result<(), Vec<CompilerError>> {
        // WARN: Have better handling for this
        let source_file_name = self
            .opts
            .source_path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();

        let (lexed_mod, errors) = Lexer::lex(source_code, source_file_name);

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn load_stdlib(&mut self, _stdlib_path: &Path) {
        todo!()
    }
}
