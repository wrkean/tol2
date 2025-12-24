use clap::Parser;
use tol2::{args::Args, driver::compile};

fn main() {
    let args = Args::parse();
    // compile(args).map_err(miette::Report::new)
    if let Err(ve) = compile(args) {
        for e in ve {
            eprintln!("{:?}", miette::Report::new(e));
        }
        std::process::exit(1);
    }
}
