use crate::parser::ast::Ast;

pub struct ParsedModule {
    pub ast: Ast,
    pub src_filename: String,
}
