use std::ops::Range;

use bitflags::bitflags;

use crate::{
    ast::{
        Ast,
        expr::{Expr, ExprKind, StructLiteralField},
        stmt::{KungBranch, ParamInfo, Stmt, StmtKind},
    },
    compiler::CompilerCtx,
    error::CompilerError,
    lexer::token::{Token, TokenKind},
    parser::{operators::Associativity, parsing_context::ExprParseContext},
    toltype::TolType,
};

pub mod operators;
mod parsing_context;

macro_rules! consume_stmt_terminator {
    ($parser:expr) => {
        $parser.consume(TokenKind::Semicolon, "`;`")?
    };
}

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
    errors: Vec<CompilerError>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            errors: Vec::new(),
        }
    }

    pub fn parse(mut self, ctx: &mut CompilerCtx) -> Ast {
        let mut ast = Vec::new();
        while !self.is_at_end() {
            if self.peek().kind == TokenKind::Eof {
                break;
            }

            match self.parse_statement() {
                Ok(s) => ast.push(s),
                Err(e) => {
                    self.synchronize();
                    self.record(e);
                }
            };
        }

        ctx.extend_errors(self.errors);

        ast
    }

    pub fn parse_statement(&mut self) -> Result<Stmt, CompilerError> {
        match self.peek().kind() {
            TokenKind::Ang | TokenKind::Dapat => self.parse_angdapat(),
            TokenKind::Paraan => self.parse_paraan(),
            TokenKind::Bawat => self.parse_bawat(),
            TokenKind::Habang => self.parse_habang(),
            TokenKind::Kung => self.parse_kung(),
            TokenKind::Ibalik => self.parse_ibalik(),
            TokenKind::Gagawin => {
                let start = self.peek().span.start;
                self.advance();
                let end = consume_stmt_terminator!(self).span.end;
                Ok(Stmt::new_gagawin(start..end))
            }
            TokenKind::Semicolon => {
                self.advance();
                Ok(Stmt::new_null())
            }
            TokenKind::Indent => {
                let indent_span = self.advance().span();
                let block = self.parse_block(indent_span);
                self.consume(TokenKind::Dedent, "dedent")?;

                block
            }
            _ => {
                let found = self.peek().lexeme().to_string();
                let span = self.peek().span();
                Err(CompilerError::InvalidStartOfStatement {
                    found,
                    span: span.into(),
                })
            }
        }
    }

    fn parse_angdapat(&mut self) -> Result<Stmt, CompilerError> {
        let (start, kind) = {
            let tok = self.consume_many(&[TokenKind::Ang, TokenKind::Dapat], "`ang` o `dapat`")?;

            (tok.span.start, tok.kind.clone())
        };
        let id = self
            .consume(
                TokenKind::Identifier,
                "pangalan pagkatapos ng `ang` o `dapat`",
            )?
            .clone();
        self.consume(TokenKind::Na, "`na` pagkatapos ng pangalan")?;
        let ttype = self.parse_type()?;

        self.consume(TokenKind::Equal, "`=` pagkatapos ng tipo")?;
        let rhs = self.parse_expression(0, ExprParseContext::AngDapatStatement)?;
        let end = consume_stmt_terminator!(self).span.end;

        Ok(Stmt {
            kind: match kind {
                TokenKind::Ang => StmtKind::Ang { id, ttype, rhs },
                TokenKind::Dapat => StmtKind::Dapat { id, ttype, rhs },
                _ => unreachable!(),
            },
            span: start..end,
        })
    }

    fn parse_paraan(&mut self) -> Result<Stmt, CompilerError> {
        let start = self.consume(TokenKind::Paraan, "`paraan`")?.span.start;

        let id = self
            .consume(TokenKind::Identifier, "pangalan pagkatapos ng `paraan`")?
            .clone();
        let param_start = self
            .consume(TokenKind::LParen, "`(` pagkatapos ng pangalan")?
            .span
            .start;
        let params = self.parse_params()?;
        let param_end = self.consume(TokenKind::RParen, "`)`")?.span.end;

        let return_type = if self.peek().kind == TokenKind::ThinArrow {
            self.advance();
            self.parse_type()?
        } else {
            TolType::Void
        };
        self.consume(TokenKind::Colon, "`:`")?;

        let indent_span = self.consume(TokenKind::Indent, "indent")?.span();
        let block = self.parse_block(indent_span)?;
        let end = self.consume(TokenKind::Dedent, "dedent")?.span.end;

        Ok(Stmt {
            kind: StmtKind::Paraan {
                id,
                return_type,
                params,
                block: Box::new(block),
                params_span: param_start..param_end,
            },
            span: start..end,
        })
    }

    fn parse_params(&mut self) -> Result<Vec<ParamInfo>, CompilerError> {
        let mut params = Vec::new();
        while !self.is_at_eof() && self.peek().kind != TokenKind::RParen {
            let param_start = self.peek().span.start;
            let id = self
                .consume(TokenKind::Identifier, "pangalan ng parametro")?
                .clone();
            self.consume(TokenKind::Na, "`na` pagkatapos ng pangalan")?;
            let ttype = self.parse_type()?;
            params.push(ParamInfo {
                id,
                ttype,
                span: param_start..self.previous().span.end,
            });

            if self.peek().kind == TokenKind::Comma {
                self.advance();
            } else if self.peek().kind != TokenKind::RParen {
                return Err(CompilerError::UnexpectedToken {
                    expected: "umasa ng `,`".to_string(),
                    span: self.peek().span().into(),
                    help: Some(
                        "Inirekomenda ko (gumawa ng compiler na to) na lagyan ng `,` sa pinakahuli ng mga parametro"
                            .to_string(),
                    ),
                });
            }
        }

        Ok(params)
    }

    #[allow(unreachable_code)]
    fn parse_bawat(&mut self) -> Result<Stmt, CompilerError> {
        todo!("Hindi pa sinusuportahan ng linggwahe ang `bawat`");
        let start = self.consume(TokenKind::Bawat, "`bawat`")?.span.start;

        let bind = self
            .consume(TokenKind::Identifier, "pangalan pagkatapos ng `bawat`")?
            .clone();
        self.consume(TokenKind::Sa, "`sa` pagkatapos ng pangalan")?;
        let iter_expr = self.parse_expression(0, ExprParseContext::BawatStatement)?;
        self.consume(TokenKind::Colon, "`:`")?;

        let indent_span = self.consume(TokenKind::Indent, "indent")?.span();
        let block = self.parse_block(indent_span)?;
        let end = self.consume(TokenKind::Dedent, "dedent")?.span.end;

        Ok(Stmt {
            kind: StmtKind::Bawat {
                bind,
                iter: iter_expr,
                block: Box::new(block),
            },
            span: start..end,
        })
    }

    fn parse_habang(&mut self) -> Result<Stmt, CompilerError> {
        let start = self.consume(TokenKind::Habang, "`habang`")?.span.start;

        let cond = self.parse_expression(0, ExprParseContext::HabangStatement)?;
        self.consume(TokenKind::Colon, "`:` pagkatapos ng expresyon")?;

        let indent_span = self.consume(TokenKind::Indent, "indent")?.span();
        let block = self.parse_block(indent_span)?;
        let end = self.consume(TokenKind::Dedent, "dedent")?.span.end;

        Ok(Stmt {
            kind: StmtKind::Habang {
                cond,
                block: Box::new(block),
            },
            span: start..end,
        })
    }

    fn parse_kung(&mut self) -> Result<Stmt, CompilerError> {
        let mut branches = Vec::new();

        let start = self.consume(TokenKind::Kung, "`kung`")?.span.start;

        // Parse initial `kung` statement
        let cond = Some(self.parse_expression(0, ExprParseContext::KungStatement)?);
        let cond_end = cond.as_ref().unwrap().span.end;
        self.consume(TokenKind::Colon, "`:` pagkatapos ng expresyon")?;

        let indent_span = self.consume(TokenKind::Indent, "indent")?.span();
        let block = self.parse_block(indent_span)?;
        self.consume(TokenKind::Dedent, "dedent")?;
        branches.push(KungBranch {
            cond,
            block,
            span: start..cond_end,
        });

        // Parse following `kungdi` brannches
        let mut end = 0;
        while self.peek().kind == TokenKind::Kungdi {
            let branch_start_span = self.consume(TokenKind::Kungdi, "`kungdi`")?.span();
            let cond = if self.peek().kind != TokenKind::Colon {
                Some(self.parse_expression(0, ExprParseContext::KungStatement)?)
            } else {
                None
            };
            let cond_end = match cond.as_ref() {
                Some(e) => e.span().end,
                None => branch_start_span.end,
            };
            self.consume(TokenKind::Colon, "`:` pagkatapos ng expresyon")?;

            let indent_span = self.consume(TokenKind::Indent, "indent")?.span();
            let block = self.parse_block(indent_span)?;
            end = self.consume(TokenKind::Dedent, "dedent")?.span().end;

            branches.push(KungBranch {
                cond,
                block,
                span: branch_start_span.start..cond_end,
            });
        }

        Ok(Stmt {
            kind: StmtKind::Kung { branches },
            span: start..end,
        })
    }

    fn parse_ibalik(&mut self) -> Result<Stmt, CompilerError> {
        let start = self.consume(TokenKind::Ibalik, "`ibalik`")?.span.start;
        let rhs = if self.peek().kind == TokenKind::Semicolon {
            None
        } else {
            Some(self.parse_expression(0, ExprParseContext::IbalikStatement)?)
        };
        let end = consume_stmt_terminator!(self).span.end;

        Ok(Stmt {
            kind: StmtKind::Ibalik { rhs },
            span: start..end,
        })
    }

    fn parse_block(&mut self, indent_span: Range<usize>) -> Result<Stmt, CompilerError> {
        let mut stmts = Vec::new();
        let start = self.peek().span.start;

        while !self.is_at_eof() && self.peek().kind != TokenKind::Dedent {
            let stmt = match self.parse_statement() {
                Ok(s) => s,
                Err(e) => {
                    self.record(e);
                    self.synchronize_until(|tk| {
                        tk.starts_a_statement() || tk == &TokenKind::Dedent
                    });
                    continue;
                }
            };

            stmts.push(stmt);
        }

        Ok(Stmt {
            kind: StmtKind::Block { indent_span, stmts },
            span: start..self.previous().span.end,
        })
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
                "f32" => {
                    self.advance();
                    Ok(TolType::F32)
                }
                "f64" => {
                    self.advance();
                    Ok(TolType::F64)
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
                found: self.peek().lexeme.clone(),
                span: self.peek().span().into(),
                help: None,
            }),
        }
    }

    fn parse_expression(&mut self, prec: u8, ctx: ExprParseContext) -> Result<Expr, CompilerError> {
        let mut left = self.nud()?;

        while !self.is_at_eof() {
            let op = self.peek().clone();
            if left.is_lvalue() && op.kind == TokenKind::LBrace && ctx.can_have_struct_lit() {
                return self.parse_struct_literal(left);
            }

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
        let current_tok_span = current_tok.span();

        match current_tok.kind() {
            TokenKind::Integer => {
                self.advance();
                Ok(Expr {
                    kind: ExprKind::Integer {
                        lexeme: current_tok,
                    },
                    span: current_tok_span,
                })
            }
            TokenKind::Float => {
                self.advance();
                Ok(Expr {
                    kind: ExprKind::Float {
                        lexeme: current_tok,
                    },
                    span: current_tok_span,
                })
            }
            TokenKind::Tama | TokenKind::Mali => {
                self.advance();
                Ok(Expr {
                    kind: ExprKind::Boolean {
                        lexeme: current_tok,
                    },
                    span: current_tok_span,
                })
            }
            TokenKind::Identifier => {
                self.advance();
                Ok(Expr {
                    kind: ExprKind::Identifier {
                        lexeme: current_tok,
                    },
                    span: current_tok_span,
                })
            }
            TokenKind::LParen => {
                self.advance();
                let expr = self.parse_expression(0, ExprParseContext::InExpression)?;
                self.consume(TokenKind::RParen, ")")?;

                Ok(expr)
            }
            TokenKind::Minus => {
                self.advance();
                let prec = operators::get_prefix_op(&TokenKind::Minus).precedence();
                let rhs = self.parse_expression(prec, ExprParseContext::InExpression)?;
                let end = rhs.span().end;

                Ok(Expr {
                    kind: ExprKind::UnaryMinus {
                        right: Box::new(rhs),
                    },
                    span: current_tok_span.start..end,
                })
            }
            TokenKind::Hindi => {
                self.advance();
                let prec = operators::get_prefix_op(&TokenKind::Hindi).precedence();
                let rhs = self.parse_expression(prec, ExprParseContext::InExpression)?;
                let end = rhs.span.end;

                Ok(Expr {
                    kind: ExprKind::UnaryNot {
                        right: Box::new(rhs),
                    },
                    span: current_tok_span.start..end,
                })
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

        let mut make_expr = |left: Expr,
                             constructor: fn(Box<Expr>, Box<Expr>) -> ExprKind|
         -> Result<Expr, CompilerError> {
            let right = self.parse_expression(precedence, ExprParseContext::InExpression)?;
            let span = left.span.start..right.span.end;
            Ok(Expr {
                kind: constructor(Box::new(left), Box::new(right)),
                span,
            })
        };

        match op.kind() {
            TokenKind::Plus => make_expr(left, |l, r| ExprKind::Add { left: l, right: r }),
            TokenKind::Minus => make_expr(left, |l, r| ExprKind::Sub { left: l, right: r }),
            TokenKind::Star => make_expr(left, |l, r| ExprKind::Mult { left: l, right: r }),
            TokenKind::Slash => make_expr(left, |l, r| ExprKind::Div { left: l, right: r }),
            TokenKind::EqualEqual => {
                make_expr(left, |l, r| ExprKind::Equality { left: l, right: r })
            }
            TokenKind::BangEqual => {
                make_expr(left, |l, r| ExprKind::InEquality { left: l, right: r })
            }
            TokenKind::Greater => make_expr(left, |l, r| ExprKind::Greater { left: l, right: r }),
            TokenKind::Less => make_expr(left, |l, r| ExprKind::Less { left: l, right: r }),
            TokenKind::GreaterEqual => {
                make_expr(left, |l, r| ExprKind::GreaterEqual { left: l, right: r })
            }
            TokenKind::LessEqual => {
                make_expr(left, |l, r| ExprKind::LessEqual { left: l, right: r })
            }
            TokenKind::LParen => self.parse_fncall(left, op.span.start),
            _ => todo!(),
        }
    }

    fn parse_fncall(&mut self, callee: Expr, args_start: usize) -> Result<Expr, CompilerError> {
        let mut args = Vec::new();
        let start = callee.span.start;

        while !self.is_at_eof() && self.peek().kind != TokenKind::RParen {
            args.push(self.parse_expression(0, ExprParseContext::Argument)?);

            if self.peek().kind == TokenKind::Comma {
                self.advance();
            } else if self.peek().kind != TokenKind::RParen {
                return Err(CompilerError::UnexpectedToken {
                    expected: "umasa ng `,` o `)`".to_string(),
                    span: self.peek().span().into(),
                    help: Some(
                        "mas maganda kung lagyan mo ng `,` sa pinakahuling argumento".to_string(),
                    ),
                });
            }
        }

        let args_end = self.consume(TokenKind::RParen, "`)`")?.span.end;

        Ok(Expr {
            kind: ExprKind::FnCall {
                callee: Box::new(callee),
                args,
                args_span: args_start..args_end,
            },
            span: start..self.peek().span.end,
        })
    }

    fn parse_struct_literal(&mut self, left: Expr) -> Result<Expr, CompilerError> {
        let mut fields = Vec::new();

        let start = left.span.start;

        self.consume(TokenKind::LBrace, "`{`")?;
        while !self.is_at_eof_or_delimiter(TokenKind::RBrace) {
            let id = match self.consume(TokenKind::Identifier, "pangalan") {
                Ok(t) => t.to_owned(),
                Err(e) => {
                    self.record(e);
                    self.synchronize_until(|tk| matches!(tk, TokenKind::RBrace));
                    continue;
                }
            };
            let ex = if self.peek().kind == TokenKind::Colon {
                self.advance();

                Some(
                    match self.parse_expression(0, ExprParseContext::StructLiteralField) {
                        Ok(ex) => ex,
                        Err(e) => {
                            self.record(e);
                            self.synchronize_until(|tk| matches!(tk, TokenKind::RBrace));
                            continue;
                        }
                    },
                )
            } else {
                None
            };

            fields.push(StructLiteralField(id.lexeme, ex));

            if self.peek().kind == TokenKind::Comma {
                self.advance();
            } else if self.peek().kind != TokenKind::RBrace {
                return Err(CompilerError::UnexpectedToken {
                    expected: format!("Umasa ng `}}` pero nakita ay {}", self.peek().lexeme()),
                    span: self.peek().span().into(),
                    help: None,
                });
            }
        }

        let end = self.consume(TokenKind::RBrace, "`}`")?.span.end;

        Ok(Expr {
            kind: ExprKind::StructLiteral {
                left: Box::new(left),
                fields,
            },
            span: start..end,
        })
    }

    fn record(&mut self, err: CompilerError) {
        self.errors.push(err);
    }

    fn synchronize(&mut self) {
        while !self.is_at_eof() {
            if self.peek().kind.starts_a_statement() {
                return;
            }

            self.advance();
        }
    }

    /// Synchronizes until the predicate is true. Predicate performs on the current token
    fn synchronize_until(&mut self, predicate: fn(&TokenKind) -> bool) {
        while !self.is_at_eof() {
            if predicate(self.peek().kind()) {
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

    fn consume_many(
        &mut self,
        kinds: &[TokenKind],
        expected_str: &str,
    ) -> Result<&Token, CompilerError> {
        if self.is_at_end() {
            panic!("Compiler bug: unexpected end of input")
        }

        for kind in kinds {
            if self.peek().kind() == kind {
                return Ok(self.advance());
            }
        }

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

    fn is_at_end(&self) -> bool {
        // Excluding EOF token
        self.current >= self.tokens.len()
    }

    fn is_at_eof(&self) -> bool {
        self.tokens[self.current].kind == TokenKind::Eof
    }

    fn is_at_eof_or_delimiter(&self, delimiter: TokenKind) -> bool {
        self.is_at_eof() || self.peek().kind == delimiter
    }
}

bitflags! {
    struct SyncSet: u32 {
        const SEMI = 1 << 0;
        const RBRACE = 1 << 1;
        const STMT = 1 << 2;
    }
}
