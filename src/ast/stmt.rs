use std::ops::Range;

use crate::{ast::expr::Expr, lexer::token::Token, toltype::TolType};

#[derive(Debug)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Range<usize>,
}

#[derive(Debug)]
pub enum StmtKind {
    Paraan {
        id: Token,
        return_type: TolType,
        params: Vec<ParamInfo>,
        block: Expr,
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
}

impl Stmt {
    pub fn new(kind: StmtKind, span: Range<usize>) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug)]
pub struct ParamInfo {
    pub id: String,
    pub ttype: TolType,
}
