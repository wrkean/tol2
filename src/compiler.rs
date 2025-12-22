use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};

use logos::Logos;

use crate::{args::Args, module::Module, token::Token};

pub struct CompilerContext<'cctx> {
    main_module: Module<'cctx>,
    stdlib: Module<'cctx>,
    external_modules: HashMap<String, Module<'cctx>>,
    source_code: String,

    source_file: Option<PathBuf>,
    dev_debug: bool,
}

impl<'cctx> CompilerContext<'cctx> {
    pub fn new(args: Args) -> Self {
        Self {
            main_module: Module::new(None),
            stdlib: Self::load_stdlib(),
            external_modules: HashMap::new(),
            source_code: String::new(),

            source_file: args.source_path,
            dev_debug: args.dev_debug,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        if self.source_file.is_none() {
            return Ok(());
        }

        self.source_code = self.read_source()?;

        for lex in Token::lexer(&self.source_code) {
            todo!()
        }

        todo!()
    }

    fn load_stdlib() -> Module<'cctx> {
        todo!();
    }

    fn read_source(&self) -> io::Result<String> {
        if let Some(path) = &self.source_file {
            fs::read_to_string(path)
        } else {
            unreachable!("Ensure self.source_file option is checked before this call");
        }
    }
}
