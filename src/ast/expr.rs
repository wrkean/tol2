use std::{fmt, ops::Range};

use crate::{ast::stmt::Stmt, lexer::token::Token};

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Range<usize>,
}

#[derive(Debug)]
pub enum ExprKind {
    Integer {
        lexeme: Token,
    },
    Float {
        lexeme: Token,
    },
    Boolean {
        lexeme: Token,
    },
    Add {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Sub {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Mult {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Div {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Block {
        stmts: Vec<Stmt>,
        value: Option<Box<Expr>>,
    },
}

impl Expr {
    pub fn new(kind: ExprKind, span: Range<usize>) -> Self {
        // let span = match &kind {
        //     ExprKind::Integer { lexeme } => lexeme.span.clone(),
        //     ExprKind::Float { lexeme } => lexeme.span.clone(),
        //     ExprKind::Boolean { lexeme } => lexeme.span.clone(),
        //     ExprKind::Add { left, right }
        //     | ExprKind::Sub { left, right }
        //     | ExprKind::Mult { left, right }
        //     | ExprKind::Div { left, right } => left.span.start..right.span.end,
        //     ExprKind::FnBlock { stmts } => stmts[0].span.start..stmts[stmts.len() - 1].span.end,
        // };
        //
        Self { kind, span }
    }
}

// Made to be easier for ast to be tested
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ExprKind::Integer { lexeme }
            | ExprKind::Float { lexeme }
            | ExprKind::Boolean { lexeme } => write!(f, "{}", &lexeme.lexeme),
            ExprKind::Add { left, right } => write!(f, "(+ {} {})", left, right),
            ExprKind::Sub { left, right } => write!(f, "(- {} {})", left, right),
            ExprKind::Mult { left, right } => write!(f, "(* {} {})", left, right),
            ExprKind::Div { left, right } => write!(f, "(/ {} {})", left, right),
            ExprKind::Block { stmts, .. } => write!(f, "{{ {:?} }}", stmts),
        }
    }
}
