use std::sync::Arc;

use crate::{
    ast::{Ast, stmt::Stmt},
    error::CompilerError,
};

pub struct ParsedModule {
    pub ast: Ast,
    pub src_filename: String,
}
