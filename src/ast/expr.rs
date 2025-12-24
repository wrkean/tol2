use std::ops::Range;

use crate::{lexer::token::Token, visitor::expr_visitor::ExprVisitor};

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Range<usize>,
}

#[derive(Debug)]
pub enum ExprKind {
    Integer { lexeme: Token },
    Float { lexeme: Token },
    Boolean { lexeme: Token },
    Add { left: Box<Expr>, right: Box<Expr> },
}

impl Expr {
    pub fn new(kind: ExprKind) -> Self {
        let span = match &kind {
            ExprKind::Integer { lexeme } => lexeme.span.clone(),
            ExprKind::Float { lexeme } => lexeme.span.clone(),
            ExprKind::Boolean { lexeme } => lexeme.span.clone(),
            ExprKind::Add { left, right } => left.span.start..right.span.end,
        };

        Self { kind, span }
    }

    pub fn accept<V: ExprVisitor>(&self, visitor: &mut V) {
        match &self.kind {
            ExprKind::Integer { .. } => visitor.visit_integer(self),
            ExprKind::Float { .. } => visitor.visit_float(self),
            ExprKind::Boolean { .. } => visitor.visit_boolean(self),
            ExprKind::Add { .. } => visitor.visit_add(self),
        }
    }
}
