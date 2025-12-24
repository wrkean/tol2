use std::ops::Range;

use crate::{lexer::token::Token, visitor::stmt_visitor::StmtVisitor};

pub struct Stmt {
    pub kind: StmtKind,
    pub span: Range<usize>,
}

pub enum StmtKind {
    Paraan {
        id: Token,
        // return_type: TolType,
        // params: Vec<ParamInfo>,
        // block: Expr,
    },
    Ang {
        id: Token,
        // ttype: TolType,
        // rhs: Expr,
    },
    Ibalik {
        id: Token,
        // rhs: Expr,
    },
}

impl Stmt {
    pub fn new(kind: StmtKind) -> Self {
        let span = match &kind {
            // TODO: Change this later when more ast nodes are created
            StmtKind::Ang { id } => id.span.clone(),
            StmtKind::Paraan { id } => id.span.clone(),
            StmtKind::Ibalik { id } => id.span.clone(),
        };
        Self { kind, span }
    }

    pub fn accept<V: StmtVisitor>(&self, visitor: &mut V) {
        match &self.kind {
            StmtKind::Paraan { .. } => visitor.visit_paraan(self),
            StmtKind::Ang { .. } => visitor.visit_ang(self),
            StmtKind::Ibalik { .. } => visitor.visit_ibalik(self),
        }
    }
}
