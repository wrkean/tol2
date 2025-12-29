use std::{fmt, ops::Range};

use crate::{lexer::token::Token, parser::ast::stmt::Stmt};

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Range<usize>,
}

#[derive(Debug)]
pub enum ExprKind {
    Integer {
        lexeme: String,
    },
    Float {
        lexeme: String,
    },
    Boolean {
        lexeme: String,
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
    #[deprecated]
    pub fn new(kind: ExprKind, span: Range<usize>) -> Self {
        Self { kind, span }
    }
}

// Made to be easier for ast to be tested
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ExprKind::Integer { lexeme }
            | ExprKind::Float { lexeme }
            | ExprKind::Boolean { lexeme } => write!(f, "{}", lexeme),
            ExprKind::Add { left, right } => write!(f, "(+ {} {})", left, right),
            ExprKind::Sub { left, right } => write!(f, "(- {} {})", left, right),
            ExprKind::Mult { left, right } => write!(f, "(* {} {})", left, right),
            ExprKind::Div { left, right } => write!(f, "(/ {} {})", left, right),
            ExprKind::Block { stmts, .. } => write!(f, "{{ {:?} }}", stmts),
        }
    }
}
