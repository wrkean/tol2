use clap::Parser;
use tol2::{args::Args, driver::compile};

fn main() {
    let args = Args::parse();
    if let Err(ewos) = compile(args) {
        for e in ewos.errors {
            match ewos.source_code.as_ref() {
                Some(src) => {
                    eprintln!("{:?}", miette::Report::new(e).with_source_code(src.clone()))
                }
                None => eprintln!("{:?}", miette::Report::new(e)),
            }
        }
        std::process::exit(1);
    }
}
