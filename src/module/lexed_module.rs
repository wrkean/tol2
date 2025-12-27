use crate::{error::CompilerError, lexer::token::Token};

pub struct LexedModule {
    pub tokens: Vec<Token>,
    pub src_filename: String,
}
