use crate::{analyzer::SymbolId, ast::typed_expr::TypedExpr, toltype::TolType};

#[derive(Debug)]
pub struct TypedStmt {
    pub kind: TypedStmtKind,
}

#[derive(Debug)]
pub enum TypedStmtKind {
    Ang { symbol_id: SymbolId, rhs: TypedExpr },
}

impl TypedStmt {
    pub fn new(kind: TypedStmtKind) -> Self {
        Self { kind }
    }
}
