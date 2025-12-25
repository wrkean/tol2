use std::sync::Arc;

use crate::ast::{Ast, stmt::Stmt};

pub struct ParsedModule {
    pub ast: Ast,
    pub src_filename: String,
    pub source_code: Arc<str>,
}
