use clap::Parser;
use tol2::{CompilerContext, args::Args};

fn main() {
    let args = Args::parse();

    let mut compiler = CompilerContext::new(args);
    compiler.run();
}
