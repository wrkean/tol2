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

    // Special
    Dummy,
}

#[derive(Debug)]
pub struct ParamInfo {
    pub id: String,
    pub ttype: TolType,
}
