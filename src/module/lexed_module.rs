use std::sync::Arc;

use crate::lexer::token::Token;

pub struct LexedModule {
    pub tokens: Vec<Token>,
    pub src_filename: String,
    pub source_code: Arc<str>,
}
