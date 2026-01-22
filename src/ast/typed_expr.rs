use crate::{
    ast::expr::StructLiteralField,
    lexer::token::{Token, TokenKind},
    toltype::TolType,
};

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
    Binary {
        left: Box<TypedExpr>,
        right: Box<TypedExpr>,
        op: TokenKind,
    },
    FnCall {
        callee: Box<TypedExpr>,
        args: Vec<TypedExpr>,
    },
    StructLiteral {
        left: Box<TypedExpr>,
        fields: Vec<StructLiteralField>,
    },
    ArrayLiteral {
        elems: Vec<TypedExpr>,
    },
    Unary {
        right: Box<TypedExpr>,
        op: TokenKind,
    },
}
