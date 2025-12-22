use std::{fs, path::PathBuf};

use crate::{args::Args, compiler::Compiler};

pub fn compile(args: Args) {
    // TODO: Use miette and proper error handling.
    let source_code =
        fs::read_to_string(args.source_path()).unwrap_or_else(|e| panic!("Nag error: {e}"));

    let mut compiler = Compiler::new(args, source_code);
    let stdlib_path = resolve_stdlib_path();
    compiler.load_stdlib(&stdlib_path);
    compiler.run();
}

// NOTE: stdlib is searched in the current directory for now
fn resolve_stdlib_path() -> PathBuf {
    todo!()
}
