pub mod analyzer_ctx;
pub mod symbol;
pub mod type_resolver;

use std::{
    collections::{HashMap, hash_map::Entry},
    ops::Range,
};

use miette::LabeledSpan;

use crate::{
    analyzer::{
        analyzer_ctx::AnalyzerContext,
        symbol::{Symbol, SymbolKind},
        type_resolver::TypeResolver,
    },
    ast::{
        Ast, TypedAst,
        expr::{Expr, ExprKind},
        stmt::{Stmt, StmtKind},
        typed_expr::{TypedExpr, TypedExprKind},
        typed_stmt::{TypedKungBranches, TypedStmt, TypedStmtKind},
    },
    compiler::CompilerCtx,
    error::CompilerError,
    lexer::token::{Token, TokenKind},
    toltype::TolType,
};

pub type SymbolId = usize;

pub struct SemanticAnalyzer<'ctx> {
    compiler_ctx: &'ctx mut CompilerCtx,
    analyzer_ctx: AnalyzerContext,
    symbol_ids: Vec<HashMap<String, SymbolId>>,
}

impl<'ctx> SemanticAnalyzer<'ctx> {
    pub fn new(compiler_ctx: &'ctx mut CompilerCtx) -> Self {
        Self {
            compiler_ctx,
            analyzer_ctx: AnalyzerContext::new(),
            symbol_ids: vec![HashMap::new()],
        }
    }

    pub fn analyze(mut self, ast: Ast) -> TypedAst {
        // TODO: Declare symbols first then analyze
        let mut typed_ast = Vec::new();
        for mut stmt in ast {
            if let Err(e) = TypeResolver::resolve_stmt(&mut stmt) {
                self.compiler_ctx.add_error(e);
            }
            match self.analyze_statement(stmt) {
                Ok(ts) => typed_ast.push(ts),
                Err(e) => self.compiler_ctx.add_error(e),
            };
        }

        typed_ast
    }

    pub fn analyze_statement(&mut self, stmt: Stmt) -> Result<TypedStmt, CompilerError> {
        match &stmt.kind {
            StmtKind::Paraan { .. } => self.analyze_paraan(stmt),
            StmtKind::Ang { .. } => self.analyze_decl(stmt),
            StmtKind::Dapat { .. } => self.analyze_decl(stmt),
            StmtKind::Ibalik { .. } => self.analyze_ibalik(stmt),
            StmtKind::Bawat { .. } => self.analyze_bawat(stmt),
            StmtKind::Habang { .. } => self.analyze_habang(stmt),
            StmtKind::Kung { .. } => self.analyze_kung(stmt),
            StmtKind::Block { indent_span, .. } => Err(CompilerError::InvalidIndent {
                span: indent_span.clone().into(),
            }),
            StmtKind::Gagawin => todo!(),
            StmtKind::Null => todo!(),
        }
    }

    fn analyze_paraan(&mut self, stmt: Stmt) -> Result<TypedStmt, CompilerError> {
        let StmtKind::Paraan {
            id,
            return_type,
            params,
            block,
            ..
        } = stmt.kind
        else {
            unreachable!()
        };

        let symbol_id = self.declare_symbol(
            &id,
            SymbolKind::Func {
                param_types: params.iter().map(|pi| pi.ttype.clone()).collect(),
                return_type: return_type.clone(),
            },
        )?;

        self.enter_scope();
        for param in params.iter() {
            self.declare_symbol(
                &param.id,
                SymbolKind::Var {
                    ttype: param.ttype.clone(),
                },
            )?;
        }

        self.analyzer_ctx.enter_fn(return_type.clone());
        let block = self.analyze_block(*block)?;
        self.analyzer_ctx.exit_fn();
        self.exit_scope();

        Ok(TypedStmt::new(TypedStmtKind::Paraan {
            params,
            symbol_id,
            block: Box::new(block),
        }))
    }

    pub fn analyze_decl(&mut self, stmt: Stmt) -> Result<TypedStmt, CompilerError> {
        let (is_ang, id, ttype, rhs) = {
            match stmt.kind {
                StmtKind::Ang { id, ttype, rhs } => (true, id, ttype, rhs),
                StmtKind::Dapat { id, ttype, rhs } => (false, id, ttype, rhs),
                _ => unreachable!(),
            }
        };

        let rhs_span = rhs.span();
        let rhs_typex = self.analyze_expression(rhs)?;

        let ttype = self.infer_type(
            ttype.as_ref(),
            &rhs_typex.ttype,
            id.span(),
            rhs_span,
            id.lexeme(),
        )?;

        let symbol_id = self.declare_symbol(
            &id,
            if is_ang {
                SymbolKind::Var { ttype }
            } else {
                SymbolKind::ConstVar { ttype }
            },
        )?;
        if is_ang {
            Ok(TypedStmt::new(TypedStmtKind::Ang {
                symbol_id,
                rhs: rhs_typex,
            }))
        } else {
            Ok(TypedStmt::new(TypedStmtKind::Dapat {
                symbol_id,
                rhs: rhs_typex,
            }))
        }
    }

    fn analyze_ibalik(&mut self, stmt: Stmt) -> Result<TypedStmt, CompilerError> {
        let stmt_span = stmt.span();
        let StmtKind::Ibalik { rhs } = stmt.kind else {
            unreachable!()
        };

        let cur_fn_return_type = self.analyzer_ctx.cur_fn_return_type();
        if rhs.is_none() && cur_fn_return_type != &TolType::Void {
            return Err(CompilerError::UnexpectedType2 {
                expected: self.analyzer_ctx.cur_fn_return_type().to_string(),
                found: TolType::Void.to_string(),
                span: stmt_span.into(),
            });
        }

        if rhs.is_none() && cur_fn_return_type == &TolType::Void {
            return Ok(TypedStmt::new(TypedStmtKind::Ibalik { rhs: None }));
        }

        let rhs_span = rhs.as_ref().unwrap().span();
        let rhs_typex = self.analyze_expression(rhs.unwrap())?;
        match self
            .analyzer_ctx
            .cur_fn_return_type()
            .coerce(&rhs_typex.ttype)
        {
            Some(_) => {}
            None => {
                return Err(CompilerError::UnexpectedType2 {
                    expected: self.analyzer_ctx.cur_fn_return_type().to_string(),
                    found: rhs_typex.ttype.to_string(),
                    span: rhs_span.into(),
                });
            }
        }

        Ok(TypedStmt::new(TypedStmtKind::Ibalik {
            rhs: Some(rhs_typex),
        }))
    }

    fn analyze_bawat(&mut self, stmt: Stmt) -> Result<TypedStmt, CompilerError> {
        let StmtKind::Bawat { bind, iter, block } = stmt.kind else {
            unreachable!()
        };

        self.enter_scope();
        let iter_typex = self.analyze_expression(iter)?;
        let bind_type = iter_typex.ttype.clone();
        self.declare_symbol(
            &bind,
            SymbolKind::Var {
                ttype: bind_type.clone(),
            },
        )?;

        self.enter_scope();
        let block = self.analyze_block(*block)?;
        self.exit_scope();

        self.exit_scope();

        Ok(TypedStmt::new(TypedStmtKind::Bawat {
            iter: iter_typex,
            bind_type,
            block: Box::new(block),
        }))
    }

    fn analyze_habang(&mut self, stmt: Stmt) -> Result<TypedStmt, CompilerError> {
        let StmtKind::Habang { cond, block } = stmt.kind else {
            unreachable!()
        };
        let cond_span = cond.span();
        let cond_typex = self.analyze_expression(cond)?;

        if cond_typex.ttype != TolType::Bool {
            return Err(CompilerError::UnexpectedType2 {
                expected: TolType::Bool.to_string(),
                found: cond_typex.ttype.to_string(),
                span: cond_span.into(),
            });
        }

        self.enter_scope();
        let block = self.analyze_block(*block)?;
        self.exit_scope();

        Ok(TypedStmt::new(TypedStmtKind::Habang {
            cond: cond_typex,
            block: Box::new(block),
        }))
    }

    fn analyze_kung(&mut self, stmt: Stmt) -> Result<TypedStmt, CompilerError> {
        let StmtKind::Kung { branches } = stmt.kind else {
            unreachable!()
        };

        let mut typed_kung_branches = Vec::new();
        let branches_len = branches.len();
        for (i, branch) in branches.into_iter().enumerate() {
            let cond_typex = match branch.cond {
                Some(e) => {
                    let cond_span = e.span();
                    let cond_typex = self.analyze_expression(e)?;
                    if cond_typex.ttype != TolType::Bool {
                        return Err(CompilerError::UnexpectedType2 {
                            expected: TolType::Bool.to_string(),
                            found: cond_typex.ttype.to_string(),
                            span: cond_span.into(),
                        });
                    }

                    Some(cond_typex)
                }
                None => {
                    if branches_len - 1 != i {
                        return Err(CompilerError::InvalidKungdiBranch {
                            span: branch.span.into(),
                        });
                    }

                    None
                }
            };
            self.enter_scope();
            let block = self.analyze_block(branch.block)?;
            self.exit_scope();
            typed_kung_branches.push(TypedKungBranches {
                cond: cond_typex,
                block: Box::new(block),
            })
        }

        Ok(TypedStmt::new(TypedStmtKind::Kung {
            branches: typed_kung_branches,
        }))
    }

    fn analyze_block(&mut self, stmt: Stmt) -> Result<TypedStmt, CompilerError> {
        let StmtKind::Block { stmts, .. } = stmt.kind else {
            unreachable!()
        };

        let mut typed_stmts = Vec::new();
        for stmt in stmts {
            match self.analyze_statement(stmt) {
                Ok(ts) => typed_stmts.push(ts),
                Err(e) => self.compiler_ctx.add_error(e),
            };
        }

        Ok(TypedStmt::new(TypedStmtKind::Block { stmts: typed_stmts }))
    }

    pub fn analyze_expression(&mut self, expr: Expr) -> Result<TypedExpr, CompilerError> {
        match expr.kind {
            ExprKind::Integer { lexeme } => Ok(TypedExpr::new(
                TypedExprKind::Integer { lexeme },
                TolType::UnsizedInteger,
            )),
            ExprKind::Float { lexeme } => Ok(TypedExpr::new(
                TypedExprKind::Float { lexeme },
                TolType::UnsizedFloat,
            )),
            ExprKind::Boolean { lexeme } => Ok(TypedExpr::new(
                TypedExprKind::Bool { lexeme },
                TolType::Bool,
            )),
            ExprKind::Identifier { .. } => self.analyze_identifier(expr),
            ExprKind::Binary { .. } => self.analyze_binary(expr),
            ExprKind::Unary { .. } => self.analyze_unary(expr),
            ExprKind::FnCall { .. } => self.analyze_fncall(expr),
            ExprKind::StructLiteral { .. } => todo!(),
            ExprKind::Dummy => todo!(),
        }
    }

    fn analyze_identifier(&mut self, expr: Expr) -> Result<TypedExpr, CompilerError> {
        let ExprKind::Identifier { lexeme } = expr.kind else {
            unreachable!()
        };

        let id = self.lookup_symbol(&lexeme)?;
        let ttype = self.compiler_ctx.symbol_table[id].get_type();
        Ok(TypedExpr::new(TypedExprKind::Identifier { lexeme }, ttype))
    }

    fn analyze_binary(&mut self, expr: Expr) -> Result<TypedExpr, CompilerError> {
        let ExprKind::Binary { left, right, op } = expr.kind else {
            unreachable!()
        };

        let left_span = left.span();
        let right_span = right.span();

        let left_typex = self.analyze_expression(*left)?;
        let right_typex = self.analyze_expression(*right)?;

        match &op {
            TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Slash
            | TokenKind::Equal
            | TokenKind::PlusEqual
            | TokenKind::MinusEqual
            | TokenKind::StarEqual
            | TokenKind::SlashEqual => {
                let coerced = left_typex.ttype.coerce_or_mismatch(
                    &right_typex.ttype,
                    left_span,
                    right_span,
                )?;

                Ok(TypedExpr::new(
                    TypedExprKind::Binary {
                        left: Box::new(left_typex),
                        right: Box::new(right_typex),
                        op,
                    },
                    coerced,
                ))
            }
            TokenKind::Less
            | TokenKind::LessEqual
            | TokenKind::Greater
            | TokenKind::GreaterEqual => {
                if left_typex.ttype.is_numeric() && right_typex.ttype.is_numeric() {
                    return Ok(TypedExpr::new(
                        TypedExprKind::Binary {
                            left: Box::new(left_typex),
                            right: Box::new(right_typex),
                            op,
                        },
                        TolType::Bool,
                    ));
                }

                Err(CompilerError::InvalidExpression {
                    spans: vec![
                        LabeledSpan::new(
                            Some(format!(
                                "Umaasa ng numerikong tipo, pero ang nakita ay `{}`",
                                left_typex.ttype
                            )),
                            left_span.start,
                            left_span.end - left_span.start,
                        ),
                        LabeledSpan::new(
                            Some(format!(
                                "Umaasa ng numerikong tipo, pero ang nakita ay `{}`",
                                right_typex.ttype
                            )),
                            right_span.start,
                            right_span.end - right_span.start,
                        ),
                    ],
                    help: Some(format!(
                        "Numerikong tipo lamang ang tinatanggap ng `{}`",
                        op.op_to_string().unwrap()
                    )),
                })
            }
            TokenKind::EqualEqual | TokenKind::BangEqual => {
                if (left_typex.ttype.is_numeric() && right_typex.ttype.is_numeric())
                    || (left_typex.ttype == TolType::Bool && right_typex.ttype == TolType::Bool)
                {
                    return Ok(TypedExpr::new(
                        TypedExprKind::Binary {
                            left: Box::new(left_typex),
                            right: Box::new(right_typex),
                            op,
                        },
                        TolType::Bool,
                    ));
                }
                Err(CompilerError::InvalidExpression { spans: vec![
                        LabeledSpan::new(Some(format!("Ito ay `{}`", left_typex.ttype)), left_span.start, left_span.end - left_span.start),
                        LabeledSpan::new(Some(format!("Ito ay `{}`", right_typex.ttype)), right_span.start, right_span.end - right_span.start),
                        LabeledSpan::new(Some(format!("Magkaiba ang tipo ng kaliwa at kanan: `{}` at `{}`", left_typex.ttype, right_typex.ttype)), left_span.start, right_span.end - left_span.start)
                    ], help: Some("Dapat na magkaparehang tipo ang kaliwa at kanan, kung ang kaliwa ay `bool`, ang kanan din ay `bool`. Kung numeriko naman ay dapat numeriko din ang kabila".to_string()) })
            }
            TokenKind::PipePipe | TokenKind::AmperAmper => {
                if left_typex.ttype != TolType::Bool {
                    return Err(CompilerError::UnexpectedType2 {
                        expected: TolType::Bool.to_string(),
                        found: left_typex.ttype.to_string(),
                        span: left_span.into(),
                    });
                }

                if right_typex.ttype != TolType::Bool {
                    return Err(CompilerError::UnexpectedType2 {
                        expected: TolType::Bool.to_string(),
                        found: right_typex.ttype.to_string(),
                        span: right_span.into(),
                    });
                }

                Ok(TypedExpr::new(
                    TypedExprKind::Binary {
                        left: Box::new(left_typex),
                        right: Box::new(right_typex),
                        op,
                    },
                    TolType::Bool,
                ))
            }
            TokenKind::Pipe => todo!(),
            TokenKind::Amper => todo!(),
            _ => todo!(),
        }
    }

    fn analyze_unary(&mut self, expr: Expr) -> Result<TypedExpr, CompilerError> {
        let ExprKind::Unary { op, right } = expr.kind else {
            unreachable!()
        };

        let right_span = right.span();
        let right_typex = self.analyze_expression(*right)?;

        match &op {
            TokenKind::Bang => {
                if right_typex.ttype != TolType::Bool {
                    return Err(CompilerError::UnexpectedType2 {
                        expected: TolType::Bool.to_string(),
                        found: right_typex.ttype.to_string(),
                        span: right_span.into(),
                    });
                }

                Ok(TypedExpr::new(
                    TypedExprKind::Unary {
                        right: Box::new(right_typex),
                        op,
                    },
                    TolType::Bool,
                ))
            }
            TokenKind::Minus => {
                if let Some(t) = TolType::UnsizedInteger.coerce(&right_typex.ttype) {
                    Ok(TypedExpr::new(
                        TypedExprKind::Unary {
                            right: Box::new(right_typex),
                            op,
                        },
                        t,
                    ))
                } else if let Some(t) = TolType::UnsizedFloat.coerce(&right_typex.ttype) {
                    Ok(TypedExpr::new(
                        TypedExprKind::Unary {
                            right: Box::new(right_typex),
                            op,
                        },
                        t,
                    ))
                } else {
                    Err(CompilerError::InvalidExpression {
                        spans: vec![LabeledSpan::new(
                            Some("Umaasa ng numerikong expresyon".to_string()),
                            right_span.start,
                            right_span.end - right_span.start,
                        )],
                        help: Some(
                            "Ang nasa kanan ng `!` ay maaari lamang na numeriko (e.g. 1, 2, 3, ...)".to_string(),
                        ),
                    })
                }
            }
            _ => unreachable!(),
        }
    }

    fn analyze_fncall(&mut self, expr: Expr) -> Result<TypedExpr, CompilerError> {
        let ExprKind::FnCall { callee, args, .. } = &expr.kind else {
            unreachable!()
        };
        let id = self.lookup_symbol_from_expr(callee)?;
        let sym = self.compiler_ctx.symbol_table[id].clone();
        let callee_typex = self.analyze_expression(callee.as_ref().clone())?;

        match sym.kind() {
            SymbolKind::Func { param_types, .. } => {
                let arg_types = args
                    .clone()
                    .into_iter()
                    .map(|arg| self.analyze_expression(arg))
                    .collect::<Result<Vec<TypedExpr>, _>>()?;

                self.check_call(param_types, &arg_types, sym.span(), expr.span(), args)?;

                Ok(TypedExpr::new(
                    TypedExprKind::FnCall {
                        callee: Box::new(callee_typex),
                        args: arg_types,
                    },
                    sym.get_type(),
                ))
            }
            SymbolKind::Var { .. } | SymbolKind::ConstVar { .. } => {
                Err(CompilerError::InvalidCallExpression {
                    span: callee.span().into(),
                })
            }
        }
    }

    fn infer_type(
        &mut self,
        left: Option<&TolType>,
        right: &TolType,
        left_span: Range<usize>,
        right_span: Range<usize>,
        left_name: &str,
    ) -> Result<TolType, CompilerError> {
        match left {
            Some(t) => Ok(t.coerce_or_mismatch(right, left_span, right_span)?),
            None => match right {
                TolType::UnsizedInteger | TolType::UnsizedFloat => {
                    Err(CompilerError::UninferrableType {
                        help_spans: vec![
                            LabeledSpan::new(Some("Ang expresyong ito ay binubuo ng mga numerong literal".to_string()), right_span.start, right_span.end - right_span.start),
                            LabeledSpan::new(Some("Kailangang sabihin kung ano ang konkretong tipo (i32, f32, atbp.) para sa mga numerong literal".to_string()), left_span.start, left_span.end - left_span.start),
                        ],
                        help: Some(format!("Subukan ang `{} na {} = ...`", left_name, match right {
                            TolType::UnsizedInteger => "i32",
                            TolType::UnsizedFloat => "f64",
                            _ => unreachable!()
                        })),
                    })
                }
                _ => Ok(right.to_owned()),
            },
        }
    }

    pub fn check_call(
        &self,
        param_types: &[TolType],
        arg_typex: &[TypedExpr],
        func_sym_span: Range<usize>,
        call_span: Range<usize>,
        args: &[Expr],
    ) -> Result<(), CompilerError> {
        if args.len() != param_types.len() {
            return Err(CompilerError::InvalidNumberOfArguments {
                arg_len: args.len(),
                expected_len: param_types.len(),
                args_span: call_span.into(),
            });
        }

        for (i, (arg, param)) in param_types.iter().zip(arg_typex).enumerate() {
            param
                .ttype
                .coerce_or_mismatch(arg, func_sym_span.clone(), args[i].span())?;
        }

        Ok(())
    }

    pub fn declare_symbol(
        &mut self,
        name_tok: &Token,
        kind: SymbolKind,
    ) -> Result<usize, CompilerError> {
        let last_scope = self.symbol_ids.last_mut().unwrap();
        let current_id = self.compiler_ctx.symbol_table.len();

        match last_scope.entry(name_tok.lexeme().to_string()) {
            Entry::Vacant(ent) => {
                ent.insert(current_id);
                self.compiler_ctx.symbol_table.push(Symbol::new(
                    name_tok.lexeme(),
                    kind,
                    name_tok.span(),
                ));
                Ok(current_id)
            }
            Entry::Occupied(ent) => {
                let occ_sym = &self.compiler_ctx.symbol_table[*ent.get()];
                Err(CompilerError::Redeclaration {
                    declared_span: occ_sym.span().into(),
                    redeclared_span: name_tok.span().into(),
                })
            }
        }
    }

    fn lookup_symbol(&self, name_tok: &Token) -> Result<usize, CompilerError> {
        for scope in self.symbol_ids.iter().rev() {
            if let Some(id) = scope.get(name_tok.lexeme()) {
                return Ok(*id);
            }
        }

        Err(CompilerError::UndeclaredSymbol {
            span: name_tok.span().into(),
        })
    }

    fn lookup_symbol_from_expr(&self, expr: &Expr) -> Result<usize, CompilerError> {
        match &expr.kind {
            ExprKind::Identifier { lexeme } => self.lookup_symbol(lexeme),
            _ => panic!("Can't lookup from expression `{:?}`", expr.kind),
        }
    }

    fn enter_scope(&mut self) {
        self.symbol_ids.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.symbol_ids.pop();
    }
}
