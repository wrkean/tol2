use std::ops::Range;

use crate::{ast::expr::Expr, lexer::token::Token, toltype::TolType};

#[derive(Debug)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Range<usize>,
}

impl Stmt {
    pub fn new_null() -> Self {
        Self {
            kind: StmtKind::Null,
            span: 0..0,
        }
    }

    pub fn new_gagawin(span: Range<usize>) -> Self {
        Self {
            kind: StmtKind::Gagawin,
            span,
        }
    }

    pub fn span(&self) -> Range<usize> {
        self.span.clone()
    }
}

#[derive(Debug)]
pub enum StmtKind {
    Paraan {
        id: Token,
        return_type: TolType,
        params: Vec<ParamInfo>,
        params_span: Range<usize>,
        block: Box<Stmt>,
    },
    Ang {
        id: Token,
        ttype: TolType,
        rhs: Expr,
    },
    Dapat {
        id: Token,
        ttype: TolType,
        rhs: Expr,
    },
    Ibalik {
        rhs: Option<Expr>,
    },
    Bawat {
        bind: Token,
        iter: Expr,
        block: Box<Stmt>,
    },
    Habang {
        cond: Expr,
        block: Box<Stmt>,
    },
    Kung {
        branches: Vec<KungBranch>,
    },
    Block {
        indent_span: Range<usize>,
        stmts: Vec<Stmt>,
    },
    Gagawin,

    // Special
    Null,
}

#[derive(Debug, Clone)]
pub struct ParamInfo {
    pub id: Token,
    pub ttype: TolType,
    pub span: Range<usize>,
}

#[derive(Debug)]
pub struct KungBranch {
    pub cond: Option<Expr>,
    pub block: Stmt,
    pub span: Range<usize>,
}
