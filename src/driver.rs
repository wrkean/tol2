use std::{fs, path::PathBuf};

use crate::{args::Args, compiler::Compiler, error::CompilerError};

/// Handles the compilation pipeline
pub fn compile(args: Args) -> Result<(), Vec<CompilerError>> {
    let source_code = fs::read_to_string(args.source_path())
        .map_err(CompilerError::from)
        .map_err(|e| vec![e])?;

    let compiler = Compiler::new(args, source_code);
    compiler.run()?;

    Ok(())
}

// NOTE: stdlib is searched in the current directory for now
fn resolve_stdlib_path() -> PathBuf {
    todo!()
}
