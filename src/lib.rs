pub const AUTHOR: &str = "Keanne Barraca";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const ABOUT: &str = "The TOL programming language compiler";

pub mod compiler;
pub mod driver;
pub mod error;
pub mod module;
pub mod toltype;

mod ast;
mod lexer;
mod parser;
