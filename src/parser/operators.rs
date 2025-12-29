use crate::lexer::token::TokenKind;

#[derive(Copy, Clone)]
pub enum Associativity {
    Left,
    Right,
}

pub struct TolOp {
    assoc: Associativity,
    precedence: u8,
}

impl TolOp {
    pub fn new(assoc: Associativity, precedence: u8) -> Self {
        Self { assoc, precedence }
    }

    pub fn assoc(&self) -> Associativity {
        self.assoc
    }

    pub fn precedence(&self) -> u8 {
        self.precedence
    }
}

use Associativity::*;
pub fn get_infix_op(kind: &TokenKind) -> TolOp {
    match kind {
        TokenKind::Plus => TolOp::new(Left, 1),
        TokenKind::Minus => TolOp::new(Left, 1),
        TokenKind::Star => TolOp::new(Left, 2),
        TokenKind::Slash => TolOp::new(Left, 2),
        _ => TolOp::new(Left, 0),
    }
}

pub fn get_prefix_op(kind: &TokenKind) -> TolOp {
    match kind {
        TokenKind::Minus => TolOp::new(Right, 3),
        _ => TolOp::new(Right, 0),
    }
}
