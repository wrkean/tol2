use crate::{analyzer::SymbolId, ast::typed_expr::TypedExpr};

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
}

impl TypedStmt {
    pub fn new(kind: TypedStmtKind) -> Self {
        Self { kind }
    }
}
