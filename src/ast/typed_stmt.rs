use crate::{analyzer::SymbolId, ast::typed_expr::TypedExpr, toltype::TolType};

#[derive(Debug)]
pub struct TypedStmt {
    pub kind: TypedStmtKind,
}

#[derive(Debug)]
pub enum TypedStmtKind {
    Ang {
        symbol_id: SymbolId,
        rhs: TypedExpr,
    },
    Dapat {
        symbol_id: SymbolId,
        rhs: TypedExpr,
    },
    Paraan {
        symbol_id: SymbolId,
        block: Box<TypedStmt>,
    },
    Block {
        stmts: Vec<TypedStmt>,
    },
    Ibalik {
        rhs: TypedExpr,
    },
    Bawat {
        iter: TypedExpr,
        bind_type: TolType,
        block: Box<TypedStmt>,
    },
    Habang {
        cond: TypedExpr,
        block: Box<TypedStmt>,
    },
    Kung {
        branches: Vec<TypedKungBranches>,
    },
}

impl TypedStmt {
    pub fn new(kind: TypedStmtKind) -> Self {
        Self { kind }
    }
}

#[derive(Debug)]
pub struct TypedKungBranches {
    pub cond: Option<TypedExpr>,
    pub block: Box<TypedStmt>,
}
