use crate::{ABOUT, AUTHOR, VERSION, compiler::Compiler, error::CompilerError};
use clap::Parser;
use miette::NamedSource;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

pub struct ErrorsWithOptSource {
    pub source_code: Option<NamedSource<Arc<str>>>,
    pub errors: Vec<CompilerError>,
}

#[derive(Debug, Parser)]
#[command(
    author = AUTHOR,
    version = VERSION,
    about = ABOUT,
)]
pub struct CompilerOptions {
    #[arg(required = true, value_name = "SOURCE_FILE")]
    source_path: PathBuf,

    #[arg(short = 'D', long = "dev-debug", default_value_t = false)]
    dev_debug: bool,
}

impl CompilerOptions {
    pub fn dev_debug(&self) -> bool {
        self.dev_debug
    }

    pub fn source_path(&self) -> &Path {
        &self.source_path
    }
}

pub fn compile(opts: CompilerOptions) -> Result<(), ErrorsWithOptSource> {
    let source_code = fs::read_to_string(opts.source_path()).map_err(|e| ErrorsWithOptSource {
        source_code: None,
        errors: vec![e.into()],
    })?;
    let file_name = opts
        .source_path()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let compiler = Compiler::new(opts);
    compiler
        .run(&source_code)
        .map_err(|ve| ErrorsWithOptSource {
            source_code: Some(NamedSource::new(file_name, Arc::from(source_code))),
            errors: ve,
        })?;

    Ok(())
}

// NOTE: stdlib is searched in the current directory for now
fn resolve_stdlib_path() -> PathBuf {
    todo!()
}
