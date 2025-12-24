use crate::lexer::token::TokenKind;

#[derive(Clone, Copy)]
pub enum Assoc {
    Left,
    Right,
}

pub struct TolOp;

impl TolOp {
    pub fn infix_bp(op: &TokenKind) -> u8 {
        match op {
            TokenKind::Plus => 1,
            TokenKind::Minus => 1,
            TokenKind::Star => 2,
            TokenKind::Slash => 2,
            _ => 0,
        }
    }

    pub fn prefix_bp(op: &TokenKind) -> u8 {
        match op {
            TokenKind::Minus => 3,
            _ => 0,
        }
    }

    pub fn assoc(op: &TokenKind) -> Assoc {
        use Assoc::*;

        match op {
            TokenKind::Plus | TokenKind::Minus | TokenKind::Star | TokenKind::Slash => Left,
            _ => unreachable!("Ensure this is checked by the main parse_expression loop"),
        }
    }
}
