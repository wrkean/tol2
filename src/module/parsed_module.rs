use std::sync::Arc;

use crate::{
    error::CompilerError,
    parser::ast::{Ast, stmt::Stmt},
};

pub struct ParsedModule {
    pub ast: Ast,
    pub src_filename: String,
}
