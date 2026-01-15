use crate::ast::stmt::Stmt;

pub type Ast = Vec<Stmt>;

pub mod expr;
pub mod stmt;
