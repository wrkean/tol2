use std::{collections::HashMap, path::PathBuf};

use crate::{args::Args, module::Module};

pub const AUTHOR: &str = "Keanne Barraca";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const ABOUT: &str = "The TOL programming language compiler";

pub mod args;

mod module;

pub struct CompilerContext<'cctx> {
    main_module: Module<'cctx>,
    stdlib: Module<'cctx>,
    external_modules: HashMap<String, Module<'cctx>>,

    source_file: Option<PathBuf>,
    dev_debug: bool,
}

impl<'cctx> CompilerContext<'cctx> {
    pub fn new(args: Args) -> Self {
        Self {
            main_module: Module::new(None),
            stdlib: Self::load_stdlib(),
            external_modules: HashMap::new(),
            source_file: args.source_path,
            dev_debug: args.dev_debug,
        }
    }

    pub fn run(&mut self) {
        if self.source_file.is_none() {
            return;
        }

        todo!();
    }

    pub fn state(&self) -> String {
        let mut out = format!(
            "source_path={}\n",
            self.source_file.clone().map_or_else(
                || "None".to_string(),
                |path| path.to_string_lossy().to_string()
            )
        );

        out.push_str(&format!(
            "Dev debug={}",
            if self.dev_debug { "ON" } else { "OFF" }
        ));

        out
    }

    fn load_stdlib() -> Module<'cctx> {
        todo!();
    }
}
