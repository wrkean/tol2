use std::{fs, path::PathBuf, sync::Arc};

use miette::NamedSource;

use crate::{args::Args, compiler::Compiler, error::CompilerError};

pub struct ErrorsWithOptSource {
    pub source_code: Option<NamedSource<Arc<str>>>,
    pub errors: Vec<CompilerError>,
}

pub fn compile(args: Args) -> Result<(), ErrorsWithOptSource> {
    let source_code = fs::read_to_string(args.source_path()).map_err(|e| ErrorsWithOptSource {
        source_code: None,
        errors: vec![e.into()],
    })?;
    let file_name = args
        .source_path()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let compiler = Compiler::new(args);
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
