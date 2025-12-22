use std::path::{Path, PathBuf};

use clap::Parser;

use crate::{ABOUT, AUTHOR, VERSION};

#[derive(Debug, Parser)]
#[command(
    author = AUTHOR,
    version = VERSION,
    about = ABOUT,
)]
pub struct Args {
    #[arg(required = true, value_name = "SOURCE_FILE")]
    source_path: PathBuf,

    #[arg(short = 'D', long = "dev-debug", default_value_t = false)]
    dev_debug: bool,
}

impl Args {
    pub fn dev_debug(&self) -> bool {
        self.dev_debug
    }

    pub fn source_path(&self) -> &Path {
        &self.source_path
    }
}
