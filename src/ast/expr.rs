use std::{fmt, ops::Range};

use crate::{
    lexer::token::{Token, TokenKind},
    visitor::expr_visitor::ExprVisitor,
};

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
    Sub { left: Box<Expr>, right: Box<Expr> },
    Mult { left: Box<Expr>, right: Box<Expr> },
    Div { left: Box<Expr>, right: Box<Expr> },
}

impl Expr {
    pub fn new(kind: ExprKind) -> Self {
        let span = match &kind {
            ExprKind::Integer { lexeme } => lexeme.span.clone(),
            ExprKind::Float { lexeme } => lexeme.span.clone(),
            ExprKind::Boolean { lexeme } => lexeme.span.clone(),
            ExprKind::Add { left, right }
            | ExprKind::Sub { left, right }
            | ExprKind::Mult { left, right }
            | ExprKind::Div { left, right } => left.span.start..right.span.end,
        };

        Self { kind, span }
    }

    pub fn accept<V: ExprVisitor>(&self, visitor: &mut V) {
        match &self.kind {
            ExprKind::Integer { .. } => visitor.visit_integer(self),
            ExprKind::Float { .. } => visitor.visit_float(self),
            ExprKind::Boolean { .. } => visitor.visit_boolean(self),
            ExprKind::Add { .. } => visitor.visit_add(self),
            ExprKind::Sub { .. } => visitor.visit_sub(self),
            ExprKind::Mult { .. } => visitor.visit_mult(self),
            ExprKind::Div { .. } => visitor.visit_div(self),
        }
    }
}

// Made to be easier for ast to be tested
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ExprKind::Integer { lexeme }
            | ExprKind::Float { lexeme }
            | ExprKind::Boolean { lexeme } => write!(f, "{}", &lexeme.lexeme),
            ExprKind::Add { left, right }
            | ExprKind::Sub { left, right }
            | ExprKind::Mult { left, right }
            | ExprKind::Div { left, right } => write!(f, "(+ {} {})", left, right),
        }
    }
}
