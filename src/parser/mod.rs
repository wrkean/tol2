use crate::{
    error::CompilerError,
    lexer::token::{Token, TokenKind},
    module::lexed_module::LexedModule,
    parser::{
        ast::expr::{Expr, ExprKind},
        operators::Associativity,
    },
};

pub mod ast;
pub mod operators;

pub struct Parser {
    tokens: Vec<Token>,
    errors: Vec<CompilerError>,
    src_filename: String,
    current: usize,
}

impl Parser {
    pub fn new(lexed_mod: LexedModule) -> Self {
        Self {
            tokens: lexed_mod.tokens,
            src_filename: lexed_mod.src_filename,
            errors: Vec::new(),
            current: 0,
        }
    }

    pub fn parse(&mut self) {
        while !self.is_at_end() {
            self.parse_statement();
        }
    }

    pub fn parse_statement(&mut self) -> Expr {
        match self.peek().kind() {
            _ => match self.parse_expression(0) {
                Ok(e) => e,
                Err(e) => {
                    eprintln!("{:?}", miette::Report::new(e));
                    std::process::exit(1);
                }
            },
        }
    }

    fn parse_expression(&mut self, prec: u8) -> Result<Expr, CompilerError> {
        let mut left = self.nud()?;

        while !self.is_at_end() {
            let op = self.peek().clone();
            if operators::get_infix_op(op.kind()).precedence() <= prec {
                break;
            }

            self.advance();
            left = self.led(&op, left)?;
        }

        Ok(left)
    }

    fn nud(&mut self) -> Result<Expr, CompilerError> {
        let current_tok = self.peek().clone();

        match current_tok.kind() {
            TokenKind::Integer => {
                self.advance();
                Ok(Expr {
                    kind: ExprKind::Integer {
                        lexeme: current_tok.lexeme.clone(),
                    },
                    span: current_tok.span(),
                })
            }
            TokenKind::Float => {
                self.advance();
                Ok(Expr {
                    kind: ExprKind::Float {
                        lexeme: current_tok.lexeme.clone(),
                    },
                    span: current_tok.span(),
                })
            }
            TokenKind::LParen => {
                self.advance();
                let expr = self.parse_expression(0)?;
                self.consume(TokenKind::RParen, ")")?;

                Ok(expr)
            }
            _ => todo!(),
        }
    }

    fn led(&mut self, op: &Token, left: Expr) -> Result<Expr, CompilerError> {
        let precedence = match operators::get_infix_op(op.kind()).assoc() {
            Associativity::Left => operators::get_infix_op(op.kind()).precedence(),
            Associativity::Right => operators::get_infix_op(op.kind()).precedence() + 1,
        };

        match op.kind() {
            TokenKind::Plus => {
                let right = self.parse_expression(precedence)?;
                let span = left.span.start..right.span.end;
                Ok(Expr {
                    kind: ExprKind::Add {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    span,
                })
            }
            TokenKind::Minus => {
                let right = self.parse_expression(precedence)?;
                let span = left.span.start..right.span.end;
                Ok(Expr {
                    kind: ExprKind::Sub {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    span,
                })
            }
            TokenKind::Star => {
                let right = self.parse_expression(precedence)?;
                let span = left.span.start..right.span.end;
                Ok(Expr {
                    kind: ExprKind::Mult {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    span,
                })
            }
            TokenKind::Slash => {
                let right = self.parse_expression(precedence)?;
                let span = left.span.start..right.span.end;
                Ok(Expr {
                    kind: ExprKind::Mult {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    span,
                })
            }
            _ => todo!(),
        }
    }

    fn advance(&mut self) -> &Token {
        if self.is_at_end() {
            panic!("Compiler bug: unexpected end of input")
        }

        self.current += 1;
        &self.tokens[self.current - 1]
    }

    fn peek(&self) -> &Token {
        if self.is_at_end() {
            panic!("Compiler bug: unexpected end of input")
        }

        &self.tokens[self.current]
    }

    fn consume(
        &mut self,
        expected: TokenKind,
        expected_str: &str,
    ) -> Result<&Token, CompilerError> {
        if self.is_at_end() {
            panic!("Compiler bug: unexpected end of input")
        }

        if expected == self.peek().kind {
            Ok(self.advance())
        } else {
            Err(CompilerError::UnexpectedToken {
                expected: format!(
                    "Umasa ng `{}` pero nakita ay `{}`",
                    expected_str,
                    self.peek().lexeme()
                ),
                span: self.peek().span().into(),
                help: None,
            })
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() - 1
    }
}
