use crate::ast::{stmt::Stmt, typed_stmt::TypedStmt};

pub type Ast = Vec<Stmt>;
pub type TypedAst = Vec<TypedStmt>;

pub mod expr;
pub mod stmt;
pub mod typed_expr;
pub mod typed_stmt;
