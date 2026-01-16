pub mod symbol;

use std::{
    collections::{HashMap, hash_map::Entry},
    ops::Range,
};

use crate::{
    analyzer::symbol::{Symbol, SymbolKind},
    ast::{
        Ast, TypedAst,
        expr::{Expr, ExprKind},
        stmt::{Stmt, StmtKind},
        typed_expr::{TypedExpr, TypedExprKind},
        typed_stmt::{TypedStmt, TypedStmtKind},
    },
    compiler::CompilerCtx,
    error::CompilerError,
    lexer::token::Token,
    toltype::TolType,
};

macro_rules! analyze_arithmetic_helper {
    ($analyzer:expr, $left:expr, $right:expr, $kind:ident) => {{
        let left_span = $left.span();
        let right_span = $right.span();
        let left_ex = $analyzer.analyze_expression(*$left)?;
        let right_ex = $analyzer.analyze_expression(*$right)?;
        let coerced_type =
            left_ex
                .ttype
                .coerce_or_mismatch(&right_ex.ttype, left_span, right_span)?;

        Ok(TypedExpr::new(
            TypedExprKind::$kind {
                left: Box::new(left_ex),
                right: Box::new(right_ex),
            },
            coerced_type,
        ))
    }};
}

macro_rules! analyze_comparison_helper {
    ($analyzer:expr, $left:expr, $right:expr, $kind:ident) => {{
        let left_span = $left.span();
        let right_span = $right.span();
        let left_ex = $analyzer.analyze_expression(*$left)?;
        let right_ex = $analyzer.analyze_expression(*$right)?;

        if !left_ex.ttype.is_numeric() {
            return Err(CompilerError::UnexpectedType2 {
                expected: "numero".to_string(),
                found: left_ex.ttype.to_string(),
                span: left_span.into(),
            });
        }

        if !right_ex.ttype.is_numeric() {
            return Err(CompilerError::UnexpectedType2 {
                expected: "numero".to_string(),
                found: right_ex.ttype.to_string(),
                span: right_span.into(),
            });
        }

        Ok(TypedExpr::new(
            TypedExprKind::$kind {
                left: Box::new(left_ex),
                right: Box::new(right_ex),
            },
            TolType::Bool,
        ))
    }};
}

pub type SymbolId = usize;

pub struct SemanticAnalyzer<'ctx> {
    ctx: &'ctx mut CompilerCtx,
    symbol_ids: Vec<HashMap<String, SymbolId>>,
}

impl<'ctx> SemanticAnalyzer<'ctx> {
    pub fn new(ctx: &'ctx mut CompilerCtx) -> Self {
        Self {
            ctx,
            symbol_ids: vec![HashMap::new()],
        }
    }

    pub fn analyze(mut self, ast: Ast) -> TypedAst {
        let mut typed_ast = Vec::new();
        for stmt in ast {
            match self.analyze_statement(stmt) {
                Ok(ts) => typed_ast.push(ts),
                Err(e) => self.ctx.add_error(e),
            };
        }

        typed_ast
    }

    pub fn analyze_statement(&mut self, stmt: Stmt) -> Result<TypedStmt, CompilerError> {
        match &stmt.kind {
            StmtKind::Paraan { .. } => todo!(),
            StmtKind::Ang { .. } => self.analyze_decl(stmt),
            StmtKind::Dapat { .. } => self.analyze_decl(stmt),
            StmtKind::Ibalik { .. } => todo!(),
            StmtKind::Bawat { .. } => todo!(),
            StmtKind::Habang { .. } => todo!(),
            StmtKind::Kung { .. } => todo!(),
            StmtKind::Block { .. } => todo!(),
            StmtKind::Gagawin => todo!(),
            StmtKind::Null => todo!(),
        }
    }

    pub fn analyze_decl(&mut self, stmt: Stmt) -> Result<TypedStmt, CompilerError> {
        let (is_ang, id, ttype, rhs) = {
            match stmt.kind {
                StmtKind::Ang { id, ttype, rhs } => (true, id, ttype, rhs),
                StmtKind::Dapat { id, ttype, rhs } => (false, id, ttype, rhs),
                _ => unreachable!(),
            }
        };

        let ttype = self.resolve_type(ttype);
        let rhs_span = rhs.span();
        let rhs_type = self.analyze_expression(rhs)?;
        ttype.coerce_or_mismatch(&rhs_type.ttype, id.span(), rhs_span)?;

        let symbol_id = self.declare_symbol(&id, SymbolKind::Var { ttype })?;
        if is_ang {
            Ok(TypedStmt::new(TypedStmtKind::Ang {
                symbol_id,
                rhs: rhs_type,
            }))
        } else {
            Ok(TypedStmt::new(TypedStmtKind::Dapat {
                symbol_id,
                rhs: rhs_type,
            }))
        }
    }

    pub fn analyze_expression(&mut self, expr: Expr) -> Result<TypedExpr, CompilerError> {
        match &expr.kind {
            ExprKind::Integer { lexeme } => Ok(TypedExpr::new(
                TypedExprKind::Integer {
                    lexeme: lexeme.to_owned(),
                },
                TolType::UnsizedInteger,
            )),
            ExprKind::Float { lexeme } => Ok(TypedExpr::new(
                TypedExprKind::Float {
                    lexeme: lexeme.to_owned(),
                },
                TolType::UnsizedFloat,
            )),
            ExprKind::Boolean { lexeme } => Ok(TypedExpr::new(
                TypedExprKind::Bool {
                    lexeme: lexeme.to_owned(),
                },
                TolType::Bool,
            )),
            ExprKind::Identifier { .. } => self.analyze_identifier(expr),
            // TODO: Next, analyze arithmetic
            ExprKind::Add { .. }
            | ExprKind::Sub { .. }
            | ExprKind::Mult { .. }
            | ExprKind::Div { .. } => self.analyze_arithmetic(expr),
            ExprKind::Equality { .. }
            | ExprKind::InEquality { .. }
            | ExprKind::Greater { .. }
            | ExprKind::Less { .. }
            | ExprKind::GreaterEqual { .. }
            | ExprKind::LessEqual { .. } => self.analyze_comparison(expr),
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
        let ttype = self.ctx.symbol_table[id].get_type();
        Ok(TypedExpr::new(TypedExprKind::Integer { lexeme }, ttype))
    }

    fn analyze_arithmetic(&mut self, expr: Expr) -> Result<TypedExpr, CompilerError> {
        match expr.kind {
            ExprKind::Add { left, right } => analyze_arithmetic_helper!(self, left, right, Add),
            ExprKind::Sub { left, right } => analyze_arithmetic_helper!(self, left, right, Sub),
            ExprKind::Mult { left, right } => analyze_arithmetic_helper!(self, left, right, Mult),
            ExprKind::Div { left, right } => analyze_arithmetic_helper!(self, left, right, Div),
            _ => unreachable!(),
        }
    }

    fn analyze_comparison(&mut self, expr: Expr) -> Result<TypedExpr, CompilerError> {
        match expr.kind {
            ExprKind::Equality { left, right } => {
                analyze_comparison_helper!(self, left, right, Equality)
            }
            ExprKind::InEquality { left, right } => {
                analyze_comparison_helper!(self, left, right, InEquality)
            }
            ExprKind::Greater { left, right } => {
                analyze_comparison_helper!(self, left, right, Greater)
            }
            ExprKind::Less { left, right } => {
                analyze_comparison_helper!(self, left, right, Less)
            }
            ExprKind::GreaterEqual { left, right } => {
                analyze_comparison_helper!(self, left, right, GreaterEqual)
            }
            ExprKind::LessEqual { left, right } => {
                analyze_comparison_helper!(self, left, right, LessEqual)
            }
            _ => unreachable!(),
        }
    }

    fn analyze_fncall(&mut self, expr: Expr) -> Result<TypedExpr, CompilerError> {
        let ExprKind::FnCall { callee, args, .. } = &expr.kind else {
            unreachable!()
        };
        let id = self.lookup_symbol_from_expr(callee)?;
        let sym = self.ctx.symbol_table[id].clone();

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
                        callee: *callee.clone(),
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

    pub fn resolve_type(&self, ttype: TolType) -> TolType {
        match ttype {
            TolType::UnknownIdentifier(_id) => todo!(),
            TolType::UnsizedInteger => TolType::I32,
            TolType::UnsizedFloat => TolType::F64,
            _ => ttype,
        }
    }

    pub fn declare_symbol(
        &mut self,
        name_tok: &Token,
        kind: SymbolKind,
    ) -> Result<usize, CompilerError> {
        let last_scope = self.symbol_ids.last_mut().unwrap();
        let current_id = self.ctx.symbol_table.len();

        match last_scope.entry(name_tok.lexeme().to_string()) {
            Entry::Vacant(ent) => {
                ent.insert(current_id);
                self.ctx
                    .symbol_table
                    .push(Symbol::new(name_tok.lexeme(), kind, name_tok.span()));
                Ok(current_id)
            }
            Entry::Occupied(ent) => {
                let occ_sym = &self.ctx.symbol_table[*ent.get()];
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
}
