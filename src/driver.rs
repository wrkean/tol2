use std::{fs, path::PathBuf};

use miette::NamedSource;

use crate::{args::Args, compiler::Compiler, error::CompilerError};

/// Handles the compilation pipeline
pub fn compile(args: Args) -> Result<(), CompilerError> {
    let source_code = fs::read_to_string(args.source_path()).map_err(CompilerError::from)?;

    let mut compiler = Compiler::new(args, source_code);
    let stdlib_path = resolve_stdlib_path();
    compiler.load_stdlib(&stdlib_path);
    compiler.run();

    Ok(())
}

// NOTE: stdlib is searched in the current directory for now
fn resolve_stdlib_path() -> PathBuf {
    todo!()
}
