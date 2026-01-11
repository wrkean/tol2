use std::{
    collections::{HashMap, hash_map::Entry},
    ops::Range,
};

use miette::LabeledSpan;

use crate::{
    analyzer::symbol::{Symbol, SymbolKind},
    error::CompilerError,
    module::parsed_module::ParsedModule,
    parser::ast::{
        Ast,
        expr::{Expr, ExprKind},
        stmt::{Stmt, StmtKind},
    },
    toltype::TolType,
};

mod symbol;

pub struct SemanticAnalyzer {
    pub ast: Ast,
    pub src_filename: String,
    pub errors: Vec<CompilerError>,
    pub symbol_table: Vec<HashMap<String, Symbol>>,
}

impl SemanticAnalyzer {
    pub fn new(parsed_mod: ParsedModule) -> Self {
        Self {
            ast: parsed_mod.ast,
            src_filename: parsed_mod.src_filename,
            errors: Vec::new(),
            symbol_table: vec![HashMap::new()],
        }
    }

    pub fn analyze(&mut self) {
        let mut statements = std::mem::take(&mut self.ast);

        for stmt in statements.iter_mut() {
            if let Err(e) = self.analyze_statement(stmt) {
                self.errors.push(e);
            }
        }

        self.ast = statements;
    }

    fn analyze_statement(&mut self, stmt: &mut Stmt) -> Result<(), CompilerError> {
        match &stmt.kind {
            StmtKind::Paraan { .. } => todo!(),
            StmtKind::Ang { .. } => self.analyze_ang(stmt),
            StmtKind::Dapat { id, ttype, rhs } => todo!(),
            StmtKind::Ibalik { id } => todo!(),
            StmtKind::Sa { cond, bind, block } => todo!(),
            StmtKind::Habang { cond, block } => todo!(),
            StmtKind::Kung { branches } => todo!(),
            StmtKind::Block { stmts } => todo!(),
            StmtKind::Null => todo!(),
            StmtKind::Dummy => todo!(),
        }
    }

    fn analyze_ang(&mut self, stmt: &mut Stmt) -> Result<(), CompilerError> {
        let StmtKind::Ang { id, ttype, rhs } = &mut stmt.kind else {
            unreachable!()
        };

        *ttype = self.resolve_type(ttype)?;
        let mut rhs_type = self.analyze_expresion(rhs)?;
        rhs_type = self.resolve_type(&rhs_type)?;
        ttype.coerce_or_mismatch(&rhs_type, id.span(), rhs.span())?;

        self.declare_symbol(
            id.lexeme(),
            SymbolKind::Var {
                ttype: ttype.clone(),
            },
            stmt.span.clone(),
        )?;

        Ok(())
    }

    fn analyze_expresion(&mut self, expr: &mut Expr) -> Result<TolType, CompilerError> {
        let ttype = match &mut expr.kind {
            ExprKind::Integer { .. } => TolType::UnsizedInteger,
            ExprKind::Float { .. } => TolType::UnsizedFloat,
            ExprKind::Boolean { .. } => TolType::Bool,
            ExprKind::Identifier { lexeme } => {
                let sym = self.lookup_symbol(lexeme, expr.span.clone())?;

                match sym.kind() {
                    SymbolKind::Var { ttype } => ttype.clone(),
                    SymbolKind::Paraan { .. } => return Err(CompilerError::InvalidExpression { spans: vec![LabeledSpan::new(
                        Some("Hindi muna sinusuportahan ang pangalan ng paraan bilang isang expresyon".to_string()),
                        expr.span().start,
                        expr.span().end - expr.span().start
                    )], help: None } )
                }
            }
            ExprKind::Add { left, right } => return self.analyze_arithmetic(left, right),
            ExprKind::Sub { left, right } => return self.analyze_arithmetic(left, right),
            ExprKind::Mult { left, right } => return self.analyze_arithmetic(left, right),
            ExprKind::Div { left, right } => return self.analyze_arithmetic(left, right),
            ExprKind::Equality { left, right } => return self.analyze_comparison(left, right),
            ExprKind::InEquality { left, right } => return self.analyze_comparison(left, right),
            ExprKind::Greater { left, right } => return self.analyze_comparison(left, right),
            ExprKind::Less { left, right } => return self.analyze_comparison(left, right),
            ExprKind::GreaterEqual { left, right } => return self.analyze_comparison(left, right),
            ExprKind::LessEqual { left, right } => return self.analyze_comparison(left, right),
            ExprKind::FnCall {
                callee,
                args,
                args_span,
                ..
            } => return self.analyze_fncall(callee, args, args_span.to_owned()),
            ExprKind::StructLiteral { left, fields } => todo!(),
            ExprKind::Dummy => todo!(),
        };

        Ok(ttype)
    }

    fn analyze_arithmetic(
        &mut self,
        left: &mut Expr,
        right: &mut Expr,
    ) -> Result<TolType, CompilerError> {
        let left_type = self.analyze_expresion(left)?;
        let right_type = self.analyze_expresion(right)?;

        left_type.coerce_or_mismatch(&right_type, left.span(), right.span())
    }

    fn analyze_comparison(
        &mut self,
        left: &mut Expr,
        right: &mut Expr,
    ) -> Result<TolType, CompilerError> {
        let left_type = self.analyze_expresion(left)?;
        let right_type = self.analyze_expresion(right)?;

        if !left_type.is_numeric() {
            return Err(CompilerError::UnexpectedType {
                found: "numerikong tipo".to_string(),
                span: left.span().into(),
                help: Some(
                    "Halimbawa ng isang numerikong tipo ay i8, u8, i16, u16, at iba pa..."
                        .to_string(),
                ),
            });
        }

        if !right_type.is_numeric() {
            return Err(CompilerError::UnexpectedType {
                found: "numerikong tipo".to_string(),
                span: right.span().into(),
                help: Some(
                    "Halimbawa ng isang numerikong tipo ay i8, u8, i16, u16, at iba pa..."
                        .to_string(),
                ),
            });
        }

        left_type.coerce_or_mismatch(&right_type, left.span(), right.span())?;

        Ok(TolType::Bool)
    }

    fn analyze_fncall(
        &mut self,
        callee: &mut Expr,
        args: &mut [Expr],
        args_span: Range<usize>,
    ) -> Result<TolType, CompilerError> {
        let sym = match &callee.kind {
            ExprKind::Identifier { lexeme } => self.lookup_symbol(lexeme, callee.span()),
            _ => {
                return Err(CompilerError::InvalidExpression {
                    spans: vec![LabeledSpan::new(
                        Some("Hindi ito pwede tawagin".to_string()),
                        callee.span().start,
                        callee.span().end - callee.span().start,
                    )],
                    help: None,
                });
            }
        }?;

        let kind = sym.kind().to_owned();
        match kind {
            SymbolKind::Paraan {
                params_types,
                params_span,
                return_type,
            } => {
                self.check_call(&params_types, args, params_span, args_span)?;
                Ok(return_type.to_owned())
            }
            SymbolKind::Var { .. } => {
                return Err(CompilerError::InvalidExpression {
                    spans: vec![
                        LabeledSpan::new(
                            Some("Naideklara dito".to_string()),
                            callee.span().start,
                            callee.span().end - callee.span().start,
                        ),
                        LabeledSpan::new(
                            Some(
                                "Ito ay isang variable/constant, hindi ito pwedeng tawagin"
                                    .to_string(),
                            ),
                            sym.span().start,
                            sym.span().end - sym.span().start,
                        ),
                    ],
                    help: None,
                });
            }
        }
    }

    fn check_call(
        &mut self,
        param_types: &[TolType],
        args: &mut [Expr],
        params_span: Range<usize>,
        args_span: Range<usize>,
    ) -> Result<(), CompilerError> {
        let params_len = param_types.len();
        let args_len = args.len();
        if params_len != args_len {
            return Err(CompilerError::InvalidExpression {
                spans: vec![
                    LabeledSpan::new(
                        Some(format!(
                            "Hindi tugma ang bilang ng argumento ({}) at parametro ({})",
                            args_len, params_len
                        )),
                        args_span.start,
                        args_span.end - args_span.start,
                    ),
                    LabeledSpan::new(
                        Some("Mga parametro".to_string()),
                        params_span.start,
                        params_span.end - params_span.start,
                    ),
                ],
                help: None,
            });
        }

        for (arg, param) in args.iter_mut().zip(param_types) {
            let arg_type = self.analyze_expresion(arg)?;

            param.coerce_or_mismatch(&arg_type, params_span.clone(), arg.span())?;
        }

        Ok(())
    }

    fn resolve_type(&self, type_to_resolve: &TolType) -> Result<TolType, CompilerError> {
        match type_to_resolve {
            TolType::UnknownIdentifier(_id) => todo!(),
            TolType::UnsizedInteger => Ok(TolType::I32),
            TolType::UnsizedFloat => Ok(TolType::F64),
            _ => Ok(type_to_resolve.to_owned()),
        }
    }

    fn declare_symbol(
        &mut self,
        name: &str,
        kind: SymbolKind,
        span: Range<usize>,
    ) -> Result<(), CompilerError> {
        let current_scope = self.symbol_table.last_mut().unwrap();
        match current_scope.entry(name.to_string()) {
            Entry::Vacant(entry) => entry.insert(Symbol::new(name, kind, span)),
            Entry::Occupied(entry) => {
                let declared_span = entry.get().span().into();
                let redeclared_span = span.into();
                return Err(CompilerError::Redeclaration {
                    declared_message: "Naideklara dito".to_string(),
                    redeclared_message: "Naideklara ulit dito".to_string(),
                    declared_span,
                    redeclared_span,
                });
            }
        };

        Ok(())
    }

    fn lookup_symbol(&self, name: &str, span: Range<usize>) -> Result<&Symbol, CompilerError> {
        for scope in self.symbol_table.iter().rev() {
            match scope.get(name) {
                Some(sym) => return Ok(sym),
                None => {}
            }
        }

        Err(CompilerError::UndeclaredSymbol {
            message: format!("Hindi mahanap ang `{}`", name),
            span: span.into(),
        })
    }
}
