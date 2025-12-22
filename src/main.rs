use clap::Parser;
use tol2::{args::Args, driver::compile};

fn main() {
    let args = Args::parse();
    compile(args);
}
