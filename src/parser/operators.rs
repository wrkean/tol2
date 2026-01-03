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
        TokenKind::PipePipe => TolOp::new(Left, 1),
        TokenKind::AmperAmper => TolOp::new(Left, 2),
        TokenKind::BangEqual => TolOp::new(Left, 3),
        TokenKind::EqualEqual => TolOp::new(Left, 3),
        TokenKind::LessEqual => TolOp::new(Left, 4),
        TokenKind::GreaterEqual => TolOp::new(Left, 4),
        TokenKind::Less => TolOp::new(Left, 4),
        TokenKind::Greater => TolOp::new(Left, 4),
        TokenKind::Plus => TolOp::new(Left, 5),
        TokenKind::Minus => TolOp::new(Left, 5),
        TokenKind::Star => TolOp::new(Left, 6),
        TokenKind::Slash => TolOp::new(Left, 6),
        TokenKind::LParen => TolOp::new(Left, 7),
        _ => TolOp::new(Left, 0),
    }
}

pub fn get_prefix_op(kind: &TokenKind) -> TolOp {
    match kind {
        TokenKind::Minus => TolOp::new(Right, 3),
        _ => TolOp::new(Right, 0),
    }
}
