use crate::{
    error::CompilerError,
    lexer::token::{Token, TokenKind},
    module::{lexed_module::LexedModule, parsed_module::ParsedModule},
    parser::{
        ast::{
            expr::{Expr, ExprKind},
            stmt::{Stmt, StmtKind},
        },
        operators::Associativity,
    },
    toltype::TolType,
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

    pub fn parse(mut self) -> (ParsedModule, Vec<CompilerError>) {
        let mut ast = Vec::new();
        while !self.is_at_end() {
            if self.peek().kind == TokenKind::Eof {
                break;
            }

            match self.parse_statement() {
                Ok(s) => ast.push(s),
                Err(e) => self.errors.push(e),
            };
        }

        (
            ParsedModule {
                ast,
                src_filename: self.src_filename,
            },
            self.errors,
        )
    }

    pub fn parse_statement(&mut self) -> Result<Stmt, CompilerError> {
        match self.peek().kind() {
            TokenKind::Ang => Ok(self.parse_ang()),
            _ => todo!(),
        }
    }

    fn parse_ang(&mut self) -> Stmt {
        let start = self.advance().span.start;
        let id = match self.consume(TokenKind::Identifier, "pangalan pagkatapos ng `ang`") {
            Ok(t) => t.to_owned(),
            Err(e) => {
                self.record(e);
                self.synchronize_to_stmt();
                return Stmt::new_dummy();
            }
        };
        match self.consume(TokenKind::Colon, "`:` pagkatapos ng pangalan") {
            Ok(_) => {}
            Err(e) => {
                self.record(e);
                self.synchronize_to_stmt();
                return Stmt {
                    kind: StmtKind::Ang {
                        id,
                        ttype: TolType::Void,
                        rhs: Expr::new_dummy(),
                    },
                    span: start..self.previous().span.end,
                };
            }
        };
        let ttype = match self.parse_type() {
            Ok(t) => t,
            Err(e) => {
                self.record(e);
                self.synchronize_until(|tk| {
                    tk == &TokenKind::Equal
                        || tk.starts_an_expression()
                        || tk == &TokenKind::Semicolon
                });
                TolType::Void
            }
        };
        match self.consume(TokenKind::Equal, "`=`") {
            Ok(_) => {}
            Err(e) => {
                self.record(e);
                self.synchronize_to_stmt();
                return Stmt {
                    kind: StmtKind::Ang {
                        id,
                        ttype,
                        rhs: Expr::new_dummy(),
                    },
                    span: start..self.previous().span.end,
                };
            }
        };
        let rhs = match self.parse_expression(0) {
            Ok(ex) => ex,
            Err(e) => {
                self.record(e);
                self.synchronize_until(|tk| tk == &TokenKind::Semicolon);
                Expr::new_dummy()
            }
        };
        match self.consume(TokenKind::Semicolon, "`;`") {
            Ok(_) => {}
            Err(e) => {
                self.record(e);
                self.synchronize_to_stmt();
            }
        };

        Stmt {
            kind: StmtKind::Ang { id, ttype, rhs },
            span: start..self.previous().span.end,
        }
    }

    fn parse_type(&mut self) -> Result<TolType, CompilerError> {
        match self.peek().kind() {
            TokenKind::Identifier => match self.peek().lexeme() {
                "u8" => {
                    self.advance();
                    Ok(TolType::U8)
                }
                "u16" => {
                    self.advance();
                    Ok(TolType::U16)
                }
                "u32" => {
                    self.advance();
                    Ok(TolType::U32)
                }
                "u64" => {
                    self.advance();
                    Ok(TolType::U64)
                }
                "usize" => {
                    self.advance();
                    Ok(TolType::USize)
                }
                "i8" => {
                    self.advance();
                    Ok(TolType::I8)
                }
                "i16" => {
                    self.advance();
                    Ok(TolType::I16)
                }
                "i32" => {
                    self.advance();
                    Ok(TolType::I32)
                }
                "i64" => {
                    self.advance();
                    Ok(TolType::I64)
                }
                "isize" => {
                    self.advance();
                    Ok(TolType::ISize)
                }
                "byte" => {
                    self.advance();
                    Ok(TolType::Byte)
                }
                "char" => {
                    self.advance();
                    Ok(TolType::Char)
                }
                "bool" => {
                    self.advance();
                    Ok(TolType::Bool)
                }
                _ => {
                    let name = self.advance().lexeme.clone();
                    Ok(TolType::UnknownIdentifier(name))
                }
            },
            _ => Err(CompilerError::UnexpectedType {
                span: self.peek().span().into(),
                help: None,
            }),
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

    fn record(&mut self, err: CompilerError) {
        self.errors.push(err);
    }

    fn synchronize_until(&mut self, predicate: fn(&TokenKind) -> bool) {
        if self.is_at_eof() {
            return;
        }

        while !self.is_at_eof() && !predicate(self.peek().kind()) {
            self.advance();
        }
    }

    fn synchronize_to_stmt(&mut self) {
        if self.is_at_eof() {
            return;
        }

        while !self.is_at_eof() && !self.peek().kind.starts_a_statement() {
            self.advance();
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

    fn previous(&self) -> &Token {
        if self.current > self.tokens.len() {
            panic!("Compiler bug: tried to get previous but previous not a token")
        }

        &self.tokens[self.current - 1]
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
                    "Umasa ng {} pero nakita ay {}",
                    expected_str,
                    self.peek().lexeme()
                ),
                span: self.peek().span().into(),
                help: None,
            })
        }
    }

    fn is_at_end(&self) -> bool {
        // Excluding EOF token
        self.current >= self.tokens.len()
    }

    fn is_at_eof(&self) -> bool {
        self.tokens[self.current].kind == TokenKind::Eof
    }
}
