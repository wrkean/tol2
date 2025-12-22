use clap::Parser;
use tol2::{args::Args, compiler::CompilerContext};

fn main() {
    let args = Args::parse();

    let mut compiler = CompilerContext::new(args);
    compiler.run();
}
