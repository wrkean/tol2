use miette::NamedSource;

use crate::{
    ast::{
        expr::{Expr, ExprKind},
        stmt::{ParamInfo, Stmt, StmtKind},
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
                Err(e) => errors.push(*e),
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

    fn parse_statement(&mut self) -> Result<Stmt, Box<CompilerError>> {
        match &self.peek().kind {
            TokenKind::Ang => self.parse_ang(),
            TokenKind::Dapat => self.parse_dapat(),
            TokenKind::Paraan => self.parse_paraan(),
            _ => todo!(),
        }
    }

    fn parse_ang(&mut self) -> Result<Stmt, Box<CompilerError>> {
        let start = self.consume("ang", TokenKind::Ang)?.span.start;
        let id = self.consume("<variable>", TokenKind::Identifier)?.clone();
        self.consume(":", TokenKind::Colon)?;
        let ttype = self.parse_type()?;
        self.consume("=", TokenKind::Equal)?;
        let rhs = self.parse_expression(0)?;
        let end = self.consume(";", TokenKind::Semicolon)?.span.end;

        Ok(Stmt::new(StmtKind::Ang { id, ttype, rhs }, start..end))
    }

    fn parse_dapat(&mut self) -> Result<Stmt, Box<CompilerError>> {
        let start = self.consume("dapat", TokenKind::Dapat)?.span.start;
        let id = self.consume("<variable>", TokenKind::Identifier)?.clone();
        self.consume(":", TokenKind::Colon)?;
        let ttype = self.parse_type()?;
        self.consume("=", TokenKind::Equal)?;
        let rhs = self.parse_expression(0)?;
        let end = self.consume(";", TokenKind::Semicolon)?.span.end;

        Ok(Stmt::new(StmtKind::Dapat { id, ttype, rhs }, start..end))
    }

    fn parse_paraan(&mut self) -> Result<Stmt, Box<CompilerError>> {
        let start = self.consume("paraan", TokenKind::Paraan)?.span.start;
        let id = self.consume("<variable>", TokenKind::Identifier)?.clone();
        let params = self.parse_params()?;
        let return_type = if self.peek().kind == TokenKind::LBrace {
            TolType::Void
        } else {
            self.parse_type()?
        };
        let block = self.parse_block()?;
        let end = block.span.end;

        Ok(Stmt::new(
            StmtKind::Paraan {
                id,
                return_type,
                params,
                block,
            },
            start..end,
        ))
    }

    fn parse_params(&mut self) -> Result<Vec<ParamInfo>, Box<CompilerError>> {
        let mut params = Vec::new();
        self.consume("(", TokenKind::LParen)?;
        let id = self.consume("<variable>", TokenKind::Identifier)?.clone();
        self.consume(":", TokenKind::Colon)?;
        let ttype = self.parse_type()?;
        params.push(ParamInfo {
            id: id.lexeme,
            ttype,
        });

        while !self.is_at_end() && self.peek().kind == TokenKind::Comma {
            self.advance();

            if self.peek().kind == TokenKind::RParen {
                self.advance();
                break;
            }

            let id = self.consume("<variable>", TokenKind::Identifier)?.clone();
            self.consume(":", TokenKind::Colon)?;
            let ttype = self.parse_type()?;
            params.push(ParamInfo {
                id: id.lexeme,
                ttype,
            });
        }

        self.consume(")", TokenKind::RParen)?;

        Ok(params)
    }

    fn parse_block(&mut self) -> Result<Expr, Box<CompilerError>> {
        let start = self.consume("{", TokenKind::LBrace)?.span.start;

        let mut stmts = Vec::new();
        while !self.is_at_end() && self.peek().kind != TokenKind::RBrace {
            stmts.push(self.parse_statement()?);
        }

        let end = self.consume("}", TokenKind::RBrace)?.span.end;

        // TODO: Value is None for now until after the parsing of the last block exprssion is
        // implmeented
        Ok(Expr::new(
            ExprKind::Block { stmts, value: None },
            start..end,
        ))
    }

    fn parse_type(&self) -> Result<TolType, Box<CompilerError>> {
        let current_tok = self.peek();

        match &current_tok.kind {
            TokenKind::Identifier => Ok(current_tok.lexeme.as_str().into()),
            _ => Err(Box::new(CompilerError::UnexpectedToken {
                expected: "tipo".to_string(),
                src: NamedSource::new(&self.src_filename, self.source_code.clone()),
                span: current_tok.span.clone().into(),
                help: None,
            })),
        }
    }

    fn parse_expression(&mut self, prec: u8) -> Result<Expr, Box<CompilerError>> {
        let mut left = self.nud()?;

        while !self.is_at_end() {
            let op = self.peek().clone();
            if TolOp::infix_bp(&op.kind) <= prec {
                break;
            }

            self.advance();
            left = self.led(&op, left)?;
        }

        Ok(left)
    }

    fn nud(&mut self) -> Result<Expr, Box<CompilerError>> {
        let start = self.peek().span.start;
        match &self.peek().kind {
            TokenKind::Integer => {
                let end = self.advance().span.end;
                Ok(Expr::new(
                    ExprKind::Integer {
                        lexeme: self.peek().clone(),
                    },
                    start..end,
                ))
            }
            TokenKind::Float => {
                let end = self.advance().span.end;
                Ok(Expr::new(
                    ExprKind::Float {
                        lexeme: self.peek().clone(),
                    },
                    start..end,
                ))
            }
            TokenKind::LParen => {
                self.advance();
                let expr = self.parse_expression(0)?;
                self.consume(")", TokenKind::RParen)?;

                Ok(expr)
            }
            TokenKind::Minus => {
                // let op = self.advance()?.clone();
                // let rhs = self.parse_expression(TolOp::prefix_bp(&op.kind))?;

                todo!()
                // Expr::new(ExprKind::Negate { rhs })
            }
            _ => todo!(),
        }
    }

    fn led(&mut self, op: &Token, left: Expr) -> Result<Expr, Box<CompilerError>> {
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
    ) -> Result<Expr, Box<CompilerError>> {
        let left = Box::new(left);
        let start = left.span.start;
        let right = Box::new(match assoc {
            Assoc::Left => self.parse_expression(prec)?,
            Assoc::Right => self.parse_expression(prec + 1)?,
        });
        let end = right.span.end;
        match &op.kind {
            TokenKind::Plus => Ok(Expr::new(ExprKind::Add { left, right }, start..end)),
            TokenKind::Minus => Ok(Expr::new(ExprKind::Sub { left, right }, start..end)),
            TokenKind::Star => Ok(Expr::new(ExprKind::Mult { left, right }, start..end)),
            TokenKind::Slash => Ok(Expr::new(ExprKind::Div { left, right }, start..end)),
            _ => unreachable!(),
        }
    }

    fn peek(&self) -> &Token {
        if !self.is_at_end() {
            &self.tokens[self.current]
        } else {
            panic!("Unexpected end of input")
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
            &self.tokens[self.current - 1]
        } else {
            panic!("Unexpected end of input")
        }
    }

    fn consume(
        &mut self,
        expected_str: &str,
        expected: TokenKind,
    ) -> Result<&Token, Box<CompilerError>> {
        let current_tok = self.peek();
        if !self.is_at_end() {
            if current_tok.kind != expected {
                Err(Box::new(CompilerError::UnexpectedToken {
                    expected: expected_str.to_string(),
                    src: NamedSource::new(&self.src_filename, self.source_code.clone()),
                    span: current_tok.span.clone().into(),
                    help: None,
                }))
            } else {
                Ok(self.advance())
            }
        } else {
            panic!("Unexpected end of input")
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

    #[test]
    fn parses_expression() {
        let mut parser = init_dummy_parser("78 + (10 - 43) / 12 * 2");
        assert_eq!(
            "(+ 78 (* (/ (- 10 43) 12) 2))",
            parser.parse_expression(0).unwrap().to_string()
        );
    }
}
