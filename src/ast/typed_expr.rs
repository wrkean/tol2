use crate::{ast::expr::StructLiteralField, lexer::token::Token, toltype::TolType};

#[derive(Debug)]
pub struct TypedExpr {
    pub kind: TypedExprKind,
    pub ttype: TolType,
}

impl TypedExpr {
    pub fn new(kind: TypedExprKind, ttype: TolType) -> Self {
        Self { kind, ttype }
    }
}

#[derive(Debug)]
pub enum TypedExprKind {
    Integer {
        lexeme: Token,
    },
    Float {
        lexeme: Token,
    },
    Bool {
        lexeme: Token,
    },
    Identifier {
        lexeme: Token,
    },
    Add {
        left: Box<TypedExpr>,
        right: Box<TypedExpr>,
    },
    Sub {
        left: Box<TypedExpr>,
        right: Box<TypedExpr>,
    },
    Mult {
        left: Box<TypedExpr>,
        right: Box<TypedExpr>,
    },
    Div {
        left: Box<TypedExpr>,
        right: Box<TypedExpr>,
    },
    Equality {
        left: Box<TypedExpr>,
        right: Box<TypedExpr>,
    },
    InEquality {
        left: Box<TypedExpr>,
        right: Box<TypedExpr>,
    },
    Greater {
        left: Box<TypedExpr>,
        right: Box<TypedExpr>,
    },
    Less {
        left: Box<TypedExpr>,
        right: Box<TypedExpr>,
    },
    GreaterEqual {
        left: Box<TypedExpr>,
        right: Box<TypedExpr>,
    },
    LessEqual {
        left: Box<TypedExpr>,
        right: Box<TypedExpr>,
    },
    FnCall {
        callee: Box<TypedExpr>,
        args: Vec<TypedExpr>,
    },
    StructLiteral {
        left: Box<TypedExpr>,
        fields: Vec<StructLiteralField>,
    },
}
