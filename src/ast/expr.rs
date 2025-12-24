use crate::token::Token;

pub enum Expr {
    Integer { lexeme: Token },
    Float { lexeme: Token },
    Boolean { lexeme: Token },
}
