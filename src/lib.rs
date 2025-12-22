pub const AUTHOR: &str = "Keanne Barraca";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const ABOUT: &str = "The TOL programming language compiler";

pub mod args;
pub mod compiler;

mod module;
mod token;
