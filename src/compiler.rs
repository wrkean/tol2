use std::path::Path;

use crate::{
    args::Args, error::CompilerError, lexer::Lexer, module::module_registry::ModuleRegistry,
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
        // This here is guaranteed to be a filename as it is checked by the driver beforehand
        // WARN: Have better handling for this
        let source_file_name = self
            .config
            .source_path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        let (lexed_mod, mut errors) = Lexer::lex(&self.source_code, source_file_name);

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
