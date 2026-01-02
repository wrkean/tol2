use bitflags::bitflags;

use crate::{
    error::CompilerError,
    lexer::token::{Token, TokenKind},
    module::{lexed_module::LexedModule, parsed_module::ParsedModule},
    parser::{
        ast::{
            expr::{Expr, ExprKind},
            stmt::{ParamInfo, Stmt, StmtKind},
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
        dbg!(self.peek().kind());
        match self.peek().kind() {
            TokenKind::Ang | TokenKind::Dapat => Ok(self.parse_angdapat()),
            TokenKind::Paraan => Ok(self.parse_paraan()),
            TokenKind::Sa => Ok(self.parse_sa()),
            TokenKind::Habang => Ok(self.parse_habang()),
            TokenKind::Semicolon => {
                self.advance();
                Ok(Stmt::new_null())
            }
            TokenKind::LBrace => {
                self.advance();
                let block = self.parse_block();
                let Some(_) = self.consume_and_synchronize(TokenKind::RBrace, "`}`", SyncSet::STMT)
                else {
                    return Ok(Stmt::new_dummy());
                };

                Ok(block)
            }
            _ => todo!(),
        }
    }

    fn parse_angdapat(&mut self) -> Stmt {
        let kind = match self.peek().kind() {
            k @ (TokenKind::Ang | TokenKind::Dapat) => k.to_owned(),
            _ => return Stmt::new_dummy(),
        };
        let start = if let Some(ang) =
            self.consume_and_synchronize(kind, "`ang` o `dapat`", SyncSet::STMT | SyncSet::RBRACE)
        {
            ang.span.start
        } else {
            return Stmt::new_dummy();
        };

        let Some(id) = self.consume_and_synchronize(
            TokenKind::Identifier,
            "pangalan pagkatapos ng `ang` o `dapat`",
            SyncSet::STMT | SyncSet::RBRACE,
        ) else {
            return Stmt::new_dummy();
        };
        let Some(_) = self.consume_and_synchronize(
            TokenKind::Colon,
            "`:` pagkatapos ng pangalan",
            SyncSet::STMT | SyncSet::RBRACE,
        ) else {
            return Stmt::new_dummy();
        };
        let ttype = match self.parse_type() {
            Ok(t) => t,
            Err(e) => {
                self.record(e);
                self.synchronize(SyncSet::STMT | SyncSet::RBRACE);
                return Stmt::new_dummy();
            }
        };

        let Some(_) =
            self.consume_and_synchronize(TokenKind::Equal, "`=`", SyncSet::STMT | SyncSet::RBRACE)
        else {
            return Stmt::new_dummy();
        };
        let rhs = match self.parse_expression(0) {
            Ok(ex) => ex,
            Err(e) => {
                self.record(e);
                self.synchronize(SyncSet::STMT | SyncSet::RBRACE);
                return Stmt::new_dummy();
            }
        };

        self.consume_and_synchronize(
            TokenKind::Semicolon,
            "`;` pagkatapos ng expresyon",
            SyncSet::STMT | SyncSet::RBRACE,
        );

        Stmt {
            kind: StmtKind::Ang { id, ttype, rhs },
            span: start..self.previous().span.end,
        }
    }

    fn parse_paraan(&mut self) -> Stmt {
        let start = if let Some(paraan) =
            self.consume_and_synchronize(TokenKind::Paraan, "`paraan`", SyncSet::LBRACE)
        {
            paraan.span.start
        } else {
            return Stmt::new_dummy();
        };

        let Some(id) = self.consume_and_synchronize(
            TokenKind::Identifier,
            "pangalan pagkatapos ng `paraan`",
            SyncSet::LBRACE,
        ) else {
            return Stmt::new_dummy();
        };

        let Some(_) = self.consume_and_synchronize(
            TokenKind::LParen,
            "`(` pagkatapos ng pangalan",
            SyncSet::LBRACE,
        ) else {
            return Stmt::new_dummy();
        };
        let params = match self.parse_params() {
            Ok(params) => params,
            Err(e) => {
                self.record(e);
                self.synchronize(SyncSet::LBRACE);
                return Stmt::new_dummy();
            }
        };
        let Some(_) = self.consume_and_synchronize(TokenKind::RParen, "`)`", SyncSet::LBRACE)
        else {
            return Stmt::new_dummy();
        };

        let return_type = if self.peek().kind == TokenKind::LBrace {
            TolType::Void
        } else {
            match self.parse_type() {
                Ok(ty) => ty,
                Err(e) => {
                    self.record(e);
                    self.synchronize(SyncSet::LBRACE);
                    TolType::Void
                }
            }
        };

        let Some(_) = self.consume_and_synchronize(TokenKind::LBrace, "`{`", SyncSet::RBRACE)
        else {
            return Stmt::new_dummy();
        };
        let block = self.parse_block();
        let Some(end_tok) = self.consume_and_synchronize(TokenKind::RBrace, "`}`", SyncSet::STMT)
        else {
            return Stmt::new_dummy();
        };

        Stmt {
            kind: StmtKind::Paraan {
                id,
                return_type,
                params,
                block: Box::new(block),
            },
            span: start..end_tok.span.end,
        }
    }

    fn parse_params(&mut self) -> Result<Vec<ParamInfo>, CompilerError> {
        let mut params = Vec::new();
        while !self.is_at_eof() && self.peek().kind != TokenKind::RParen {
            let id = self.consume(TokenKind::Identifier, "pangalan")?.clone();
            self.consume(TokenKind::Colon, "`:` pagkatapos ng pangalan")?;
            let ttype = self.parse_type()?;
            params.push(ParamInfo {
                id: id.lexeme,
                ttype,
            });

            if self.peek().kind == TokenKind::Comma {
                self.advance();
            } else if self.peek().kind != TokenKind::RParen {
                return Err(CompilerError::UnexpectedToken {
                    expected: "umasa ng `,`".to_string(),
                    span: self.peek().span().into(),
                    help: Some(
                        "Mas mabuti kung lagyan mo ng `,` bago matapos ang listahan ng parametro"
                            .to_string(),
                    ),
                });
            }
        }

        Ok(params)
    }

    fn parse_sa(&mut self) -> Stmt {
        let start = if let Some(sa_tok) =
            self.consume_and_synchronize(TokenKind::Sa, "`sa`", SyncSet::LBRACE)
        {
            sa_tok.span.start
        } else {
            return Stmt::new_dummy();
        };

        let cond = match self.parse_expression(0) {
            Ok(ex) => ex,
            Err(e) => {
                self.record(e);
                self.synchronize(SyncSet::LBRACE);
                return Stmt::new_dummy();
            }
        };

        let bind = if self.peek().kind == TokenKind::Arrow {
            self.advance();
            let Some(t) = self.consume_and_synchronize(
                TokenKind::Identifier,
                "pangalan pagkatapos ng `=>`",
                SyncSet::LBRACE,
            ) else {
                return Stmt::new_dummy();
            };
            Some(t)
        } else {
            None
        };

        let Some(_) = self.consume_and_synchronize(TokenKind::LBrace, "`{`", SyncSet::RBRACE)
        else {
            return Stmt::new_dummy();
        };
        let block = self.parse_block();
        let Some(end_tok) = self.consume_and_synchronize(TokenKind::RBrace, "`}`", SyncSet::STMT)
        else {
            return Stmt::new_dummy();
        };

        Stmt {
            kind: StmtKind::Sa {
                cond,
                bind,
                block: Box::new(block),
            },
            span: start..end_tok.span.end,
        }
    }

    fn parse_habang(&mut self) -> Stmt {
        let Some(start_tok) =
            self.consume_and_synchronize(TokenKind::Habang, "`habang`", SyncSet::LBRACE)
        else {
            return Stmt::new_dummy();
        };
        let cond = match self.parse_expression(0) {
            Ok(ex) => ex,
            Err(e) => {
                self.record(e);
                self.synchronize(SyncSet::LBRACE);
                return Stmt::new_dummy();
            }
        };
        let Some(_) = self.consume_and_synchronize(TokenKind::LBrace, "`{`", SyncSet::RBRACE)
        else {
            return Stmt::new_dummy();
        };
        let block = self.parse_block();
        let Some(end_tok) = self.consume_and_synchronize(TokenKind::RBrace, "`}`", SyncSet::STMT)
        else {
            return Stmt::new_dummy();
        };

        Stmt {
            kind: StmtKind::Habang {
                cond,
                block: Box::new(block),
            },
            span: start_tok.span.start..end_tok.span.end,
        }
    }

    fn parse_block(&mut self) -> Stmt {
        let mut stmts = Vec::new();
        let start = self.peek().span.start;

        while !self.is_at_eof() && self.peek().kind != TokenKind::RBrace {
            stmts.push(self.parse_statement().unwrap());
        }

        Stmt {
            kind: StmtKind::Block { stmts },
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

        while !self.is_at_eof() {
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
            TokenKind::Tama | TokenKind::Mali => {
                self.advance();
                Ok(Expr {
                    kind: ExprKind::Boolean {
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
        let infix = operators::get_infix_op(op.kind());
        let precedence = match infix.assoc() {
            Associativity::Left => infix.precedence(),
            Associativity::Right => infix.precedence() + 1,
        };

        let make_expr =
            |constructor: fn(Box<Expr>, Box<Expr>) -> ExprKind| -> Result<Expr, CompilerError> {
                let right = self.parse_expression(precedence)?;
                let span = left.span.start..right.span.end;
                Ok(Expr {
                    kind: constructor(Box::new(left), Box::new(right)),
                    span,
                })
            };

        match op.kind() {
            TokenKind::Plus => make_expr(|l, r| ExprKind::Add { left: l, right: r }),
            TokenKind::Minus => make_expr(|l, r| ExprKind::Sub { left: l, right: r }),
            TokenKind::Star => make_expr(|l, r| ExprKind::Mult { left: l, right: r }),
            TokenKind::Slash => make_expr(|l, r| ExprKind::Div { left: l, right: r }),
            TokenKind::EqualEqual => make_expr(|l, r| ExprKind::Equality { left: l, right: r }),
            TokenKind::BangEqual => make_expr(|l, r| ExprKind::InEquality { left: l, right: r }),
            TokenKind::Greater => make_expr(|l, r| ExprKind::Greater { left: l, right: r }),
            TokenKind::Less => make_expr(|l, r| ExprKind::Less { left: l, right: r }),
            TokenKind::GreaterEqual => {
                make_expr(|l, r| ExprKind::GreaterEqual { left: l, right: r })
            }
            TokenKind::LessEqual => make_expr(|l, r| ExprKind::LessEqual { left: l, right: r }),
            _ => todo!(),
        }
    }

    fn record(&mut self, err: CompilerError) {
        self.errors.push(err);
    }

    fn synchronize(&mut self, set: SyncSet) {
        while !self.is_at_eof() {
            let p = self.peek().kind();

            if set.contains(SyncSet::SEMI) && p == &TokenKind::Semicolon
                || set.contains(SyncSet::RBRACE) && p == &TokenKind::RBrace
                || set.contains(SyncSet::STMT) && p.starts_a_statement()
                || set.contains(SyncSet::LBRACE) && p == &TokenKind::LBrace
            {
                return;
            }

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
                    "Umasa ng {} pero nakita ay `{}`",
                    expected_str,
                    self.peek().lexeme()
                ),
                span: self.peek().span().into(),
                help: None,
            })
        }
    }

    fn consume_and_synchronize(
        &mut self,
        expected: TokenKind,
        expected_str: &str,
        set: SyncSet,
    ) -> Option<Token> {
        match self.consume(expected, expected_str) {
            Ok(t) => Some(t.to_owned()),
            Err(e) => {
                self.record(e);
                self.synchronize(set);
                None
            }
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

bitflags! {
    struct SyncSet: u32 {
        const SEMI = 1 << 0;
        const RBRACE = 1 << 1;
        const STMT = 1 << 2;
        const LBRACE = 1 << 3;
    }
}
