use std::ops::Range;

use crate::{
    ast::expr::Expr, lexer::token::Token, toltype::TolType, visitor::stmt_visitor::StmtVisitor,
};

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

    pub fn accept<V: StmtVisitor>(&self, visitor: &mut V) {
        match &self.kind {
            StmtKind::Paraan { .. } => visitor.visit_paraan(self),
            StmtKind::Ang { .. } => visitor.visit_ang(self),
            StmtKind::Ibalik { .. } => visitor.visit_ibalik(self),
            StmtKind::Dapat { .. } => visitor.visit_dapat(self),
        }
    }
}

#[derive(Debug)]
pub struct ParamInfo {
    pub id: String,
    pub ttype: TolType,
}
