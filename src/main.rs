use clap::Parser;
use tol2::{args::Args, driver::compile};

fn main() -> miette::Result<()> {
    let args = Args::parse();
    compile(args).map_err(miette::Report::new)
}
