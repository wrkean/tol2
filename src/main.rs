use clap::Parser;
use colored::Colorize;
use tol2::driver::{CompilerOptions, compile};

fn main() {
    let opts = CompilerOptions::parse();
    let compilation_result = compile(opts);

    if let Err(ewos) = compilation_result {
        for e in ewos.errors {
            match ewos.source_code.as_ref() {
                Some(src) => {
                    eprintln!(
                        "{}",
                        "========================================================================"
                            .bright_cyan()
                    );
                    eprintln!("{:?}", miette::Report::new(e).with_source_code(src.clone()));
                }
                None => eprintln!("{:?}", miette::Report::new(e)),
            }
        }
        std::process::exit(1);
    }
}
