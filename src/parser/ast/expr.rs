use std::{fmt, ops::Range};

use crate::parser::ast::stmt::Stmt;

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Range<usize>,
}

#[derive(Debug)]
pub enum ExprKind {
    Integer { lexeme: String },
    Float { lexeme: String },
    Boolean { lexeme: String },
    Identifier { lexeme: String },
    Add { left: Box<Expr>, right: Box<Expr> },
    Sub { left: Box<Expr>, right: Box<Expr> },
    Mult { left: Box<Expr>, right: Box<Expr> },
    Div { left: Box<Expr>, right: Box<Expr> },
    Equality { left: Box<Expr>, right: Box<Expr> },
    InEquality { left: Box<Expr>, right: Box<Expr> },
    Greater { left: Box<Expr>, right: Box<Expr> },
    Less { left: Box<Expr>, right: Box<Expr> },
    GreaterEqual { left: Box<Expr>, right: Box<Expr> },
    LessEqual { left: Box<Expr>, right: Box<Expr> },
    FnCall { callee: Box<Expr>, args: Vec<Expr> },

    // Special
    Dummy,
}

impl Expr {
    #[deprecated]
    pub fn new(kind: ExprKind, span: Range<usize>) -> Self {
        Self { kind, span }
    }

    pub fn new_dummy() -> Self {
        Self {
            kind: ExprKind::Dummy,
            span: 0..0,
        }
    }
}

// Made to be easier for ast to be tested
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ExprKind::Integer { lexeme }
            | ExprKind::Float { lexeme }
            | ExprKind::Boolean { lexeme }
            | ExprKind::Identifier { lexeme } => write!(f, "{}", lexeme),
            ExprKind::Add { left, right } => write!(f, "(+ {} {})", left, right),
            ExprKind::Sub { left, right } => write!(f, "(- {} {})", left, right),
            ExprKind::Mult { left, right } => write!(f, "(* {} {})", left, right),
            ExprKind::Div { left, right } => write!(f, "(/ {} {})", left, right),
            ExprKind::Equality { left, right } => write!(f, "(== {} {})", left, right),
            ExprKind::InEquality { left, right } => write!(f, "(!= {} {})", left, right),
            ExprKind::Greater { left, right } => write!(f, "(!= {} {})", left, right),
            ExprKind::Less { left, right } => write!(f, "(!= {} {})", left, right),
            ExprKind::GreaterEqual { left, right } => write!(f, "(!= {} {})", left, right),
            ExprKind::LessEqual { left, right } => write!(f, "(!= {} {})", left, right),
            ExprKind::FnCall { callee, args } => write!(f, "{}({:#?})", callee, args),
            ExprKind::Dummy => write!(f, "<dummy>"),
        }
    }
}
