use std::ops::Range;

use crate::{lexer::token::Token, parser::ast::expr::Expr, toltype::TolType};

#[derive(Debug)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Range<usize>,
}

impl Stmt {
    pub fn new_dummy() -> Self {
        Self {
            kind: StmtKind::Dummy,
            span: 0..0,
        }
    }

    pub fn new_null() -> Self {
        Self {
            kind: StmtKind::Null,
            span: 0..0,
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
        id: Token,
        // rhs: Expr,
    },
    Sa {
        cond: Expr,
        bind: Option<Token>,
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
        stmts: Vec<Stmt>,
    },
    Null,

    // Special
    Dummy,
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
}
