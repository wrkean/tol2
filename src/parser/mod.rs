use miette::NamedSource;

use crate::{
    ast::{
        expr::{Expr, ExprKind},
        stmt::{Stmt, StmtKind},
    },
    error::CompilerError,
    lexer::token::{Token, TokenKind},
    module::{lexed_module::LexedModule, parsed_module::ParsedModule},
    parser::operator_db::{Assoc, TolOp},
    toltype::TolType,
};

mod operator_db;

pub struct Parser {
    tokens: Vec<Token>,
    src_filename: String,
    source_code: String,
    current: usize,
}

impl Parser {
    pub fn new(lexed_module: LexedModule) -> Self {
        Self {
            tokens: lexed_module.tokens,
            src_filename: lexed_module.src_filename,
            source_code: lexed_module.source_code,
            current: 0,
        }
    }

    pub fn parse(mut self) -> (ParsedModule, Vec<CompilerError>) {
        let mut errors = Vec::new();
        let mut ast = Vec::new();

        while !self.is_at_end() {
            match self.parse_statement() {
                Ok(s) => ast.push(s),
                Err(e) => errors.push(e),
            }
        }

        (
            ParsedModule {
                ast,
                src_filename: self.src_filename,
                source_code: self.source_code,
            },
            errors,
        )
    }

    fn parse_statement(&mut self) -> Result<Stmt, CompilerError> {
        match &self.peek()?.kind {
            TokenKind::Ang => self.parse_ang(),
            TokenKind::Dapat => todo!(),
            TokenKind::Paraan => todo!(),
            _ => todo!(),
        }
    }

    fn parse_ang(&mut self) -> Result<Stmt, CompilerError> {
        let ang_tok = self.consume(("ang", &TokenKind::Ang))?;
        let start = ang_tok.span.start;
        let id = self
            .consume(("<variable>", &TokenKind::Identifier))?
            .clone();
        self.consume((":", &TokenKind::Colon))?;
        let ttype = self.parse_type()?;
        self.consume(("=", &TokenKind::Equal))?;
        let rhs = self.parse_expression(0)?;
        let end = rhs.span.end;

        Ok(Stmt::new(StmtKind::Ang { id, ttype, rhs }, start..end))
    }

    fn parse_type(&self) -> Result<TolType, CompilerError> {
        let current_tok = self.peek()?;

        match &current_tok.kind {
            TokenKind::Identifier => Ok(current_tok.lexeme.as_str().into()),
            _ => Err(CompilerError::UnexpectedToken {
                expected: "tipo".to_string(),
                src: NamedSource::new(&self.src_filename, self.source_code.clone()),
                span: current_tok.span.clone().into(),
                help: None,
            }),
        }
    }

    fn parse_expression(&mut self, prec: u8) -> Result<Expr, CompilerError> {
        let mut left = self.nud()?;

        while !self.is_at_end() {
            let op = self.peek()?.clone();
            if TolOp::infix_bp(&op.kind) <= prec {
                break;
            }

            self.advance()?;
            left = self.led(&op, left)?;
        }

        Ok(left)
    }

    fn nud(&mut self) -> Result<Expr, CompilerError> {
        match &self.peek()?.kind {
            TokenKind::Integer => Ok(Expr::new(ExprKind::Integer {
                lexeme: self.advance()?.clone(),
            })),
            TokenKind::Float => Ok(Expr::new(ExprKind::Float {
                lexeme: self.advance()?.clone(),
            })),
            TokenKind::LParen => {
                self.advance()?;
                let expr = self.parse_expression(0)?;
                self.consume((")", &TokenKind::RParen))?;

                Ok(expr)
            }
            TokenKind::Minus => {
                let op = self.advance()?.clone();
                let rhs = self.parse_expression(TolOp::prefix_bp(&op.kind))?;

                todo!()
                // Expr::new(ExprKind::Negate { rhs })
            }
            _ => todo!(),
        }
    }

    fn led(&mut self, op: &Token, left: Expr) -> Result<Expr, CompilerError> {
        let prec = TolOp::infix_bp(&op.kind);
        let assoc = TolOp::assoc(&op.kind);

        match &op.kind {
            TokenKind::Plus | TokenKind::Minus | TokenKind::Star | TokenKind::Slash => {
                self.parse_arithmetic(op, left, prec, assoc)
            }
            _ => todo!(),
        }
    }

    fn parse_arithmetic(
        &mut self,
        op: &Token,
        left: Expr,
        prec: u8,
        assoc: Assoc,
    ) -> Result<Expr, CompilerError> {
        let left = Box::new(left);
        let right = Box::new(match assoc {
            Assoc::Left => self.parse_expression(prec)?,
            Assoc::Right => self.parse_expression(prec + 1)?,
        });
        match &op.kind {
            TokenKind::Plus => Ok(Expr::new(ExprKind::Add { left, right })),
            TokenKind::Minus => Ok(Expr::new(ExprKind::Add { left, right })),
            TokenKind::Star => Ok(Expr::new(ExprKind::Add { left, right })),
            TokenKind::Slash => Ok(Expr::new(ExprKind::Add { left, right })),
            _ => unreachable!(),
        }
    }

    fn peek(&self) -> Result<&Token, CompilerError> {
        if !self.is_at_end() {
            Ok(&self.tokens[self.current])
        } else {
            Err(CompilerError::UnexpectedEndOfInput)
        }
    }

    fn advance(&mut self) -> Result<&Token, CompilerError> {
        if !self.is_at_end() {
            self.current += 1;
            Ok(&self.tokens[self.current - 1])
        } else {
            Err(CompilerError::UnexpectedEndOfInput)
        }
    }

    fn consume(&mut self, expected: (&str, &TokenKind)) -> Result<&Token, CompilerError> {
        let current_tok = self.peek()?;
        if !self.is_at_end() {
            if &current_tok.kind != expected.1 {
                Err(CompilerError::UnexpectedToken {
                    expected: expected.0.to_string(),
                    src: NamedSource::new(&self.src_filename, self.source_code.clone()),
                    span: current_tok.span.clone().into(),
                    help: None,
                })
            } else {
                self.advance()
            }
        } else {
            Err(CompilerError::UnexpectedEndOfInput)
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::Lexer;

    use super::*;

    fn init_dummy_parser(input: &str) -> Parser {
        let lexmod = Lexer::lex(input, "test.file").0;

        Parser::new(lexmod)
    }

    #[test]
    fn expression_output() {
        let mut parser = init_dummy_parser("10 + 50");
        dbg!(parser.parse_expression(0).unwrap());
    }

    #[test]
    fn parses_add() {
        let mut parser = init_dummy_parser("67 + 41");
        if let ExprKind::Add { left, right } = &parser.parse_expression(0).unwrap().kind {
            match (&left.kind, &right.kind) {
                (ExprKind::Integer { lexeme: lt }, ExprKind::Integer { lexeme: rt }) => {
                    assert_eq!(&lt.lexeme, "67");
                    assert_eq!(&rt.lexeme, "41");
                }
                _ => panic!(
                    "Expected integer to integer but found `{:?}` to `{:?}`",
                    &left.kind, &right.kind
                ),
            }
        } else {
            panic!("Expected add expr")
        }
    }
}
