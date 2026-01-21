use std::{fmt, ops::Range};

use crate::lexer::token::{Token, TokenKind};

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Range<usize>,
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Integer {
        lexeme: Token,
        suffix: Option<String>,
    },
    Float {
        lexeme: Token,
        suffix: Option<String>,
    },
    Boolean {
        lexeme: Token,
    },
    Identifier {
        lexeme: Token,
    },
    Binary {
        left: Box<Expr>,
        right: Box<Expr>,
        op: TokenKind,
    },
    FnCall {
        callee: Box<Expr>,
        args: Vec<Expr>,
        args_span: Range<usize>,
    },
    Unary {
        op: TokenKind,
        right: Box<Expr>,
    },
    StructLiteral {
        left: Box<Expr>,
        fields: Vec<StructLiteralField>,
    },

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

    pub fn is_lvalue(&self) -> bool {
        matches!(&self.kind, ExprKind::Identifier { .. })
    }

    pub fn span(&self) -> Range<usize> {
        self.span.clone()
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ExprKind::Integer { lexeme, .. }
            | ExprKind::Float { lexeme, .. }
            | ExprKind::Boolean { lexeme }
            | ExprKind::Identifier { lexeme } => write!(f, "{}", lexeme.lexeme()),
            ExprKind::Dummy => write!(f, "<dummy>"),
            ExprKind::StructLiteral { left, fields } => write!(f, "{} {{ {:#?} }}", left, fields),
            ExprKind::Binary { left, right, op } => {
                write!(f, "{} {} {}", left, op.op_to_string().unwrap(), right)
            }
            ExprKind::FnCall { callee, args, .. } => write!(f, "{}({:#?})", callee, args),
            ExprKind::Unary { op, right } => write!(f, "{}{}", op.op_to_string().unwrap(), right),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StructLiteralField(pub String, pub Option<Expr>);
