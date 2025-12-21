use std::path::PathBuf;

use clap::Parser;

use crate::{ABOUT, AUTHOR, VERSION};

#[derive(Debug, Parser)]
#[command(
    author = AUTHOR,
    version = VERSION,
    about = ABOUT,
)]
pub struct Args {
    #[arg(value_name = "SOURCE_FILE")]
    pub source_path: Option<PathBuf>,

    #[arg(short = 'D', long = "dev-debug", default_value_t = false)]
    pub dev_debug: bool,
}
