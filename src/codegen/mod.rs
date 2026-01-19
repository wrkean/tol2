use gen_c::{
    CCodeGen,
    builder::{
        block_builder::BlockBuilder,
        decl_builder::{ConstKind, DeclBuilder},
        function_builder::FunctionBuilder,
        return_builder::ReturnBuilder,
        while_builder::WhileBuilder,
    },
    ctype::CType,
    product::statement::CStatement,
};

use crate::{
    analyzer::symbol::Symbol,
    ast::{
        TypedAst,
        typed_expr::{TypedExpr, TypedExprKind},
        typed_stmt::{TypedStmt, TypedStmtKind},
    },
    toltype::TolType,
};

// macro_rules! binary_gen {
//     ($codegen:expr, $left:expr, $right:expr, $kind:expr) => {
//         format!("({} {} {})", $codegen.gen_expr($left), $op, $codegen.gen_expr($right))
//     };
// }

pub struct Codegen<'a> {
    ast: &'a TypedAst,
    symbols: &'a [Symbol],
}

impl<'a> Codegen<'a> {
    pub fn new(ast: &'a TypedAst, symbols: &'a [Symbol]) -> Self {
        Self { ast, symbols }
    }

    pub fn generate_c(&self, mut generator: CCodeGen) -> String {
        for stmt in self.ast.iter() {
            generator = generator.add_statement(self.gen_stmt(stmt));
        }

        generator.produce_c()
    }

    fn gen_stmt(&self, stmt: &TypedStmt) -> CStatement {
        match &stmt.kind {
            TypedStmtKind::Ang { .. } | TypedStmtKind::Dapat { .. } => self.gen_decl(stmt),
            TypedStmtKind::Paraan { .. } => self.gen_paraan(stmt),
            TypedStmtKind::Block { .. } => self.gen_block(stmt),
            TypedStmtKind::Ibalik { .. } => self.gen_ibalik(stmt),
            TypedStmtKind::Bawat { .. } => self.gen_bawat(stmt),
            TypedStmtKind::Habang { .. } => self.gen_habang(stmt),
            TypedStmtKind::Kung { .. } => todo!(),
        }
    }

    fn gen_decl(&self, stmt: &TypedStmt) -> CStatement {
        let (is_ang, symbol_id, rhs) = {
            match &stmt.kind {
                TypedStmtKind::Ang { symbol_id, rhs } => (true, symbol_id, rhs),
                TypedStmtKind::Dapat { symbol_id, rhs } => (false, symbol_id, rhs),
                _ => unreachable!(),
            }
        };

        let sym = self.get_symbol(*symbol_id);
        let rhs_c = self.gen_expr(rhs);
        let mut decl = DeclBuilder::new(self.as_c(&sym.get_type()), sym.name()).with_rhs(rhs_c);

        if !is_ang {
            decl = decl.as_const(ConstKind::Const);
        }

        decl.build()
    }

    fn gen_paraan(&self, stmt: &TypedStmt) -> CStatement {
        let TypedStmtKind::Paraan {
            params,
            symbol_id,
            block,
        } = &stmt.kind
        else {
            unreachable!()
        };

        let sym = self.get_symbol(*symbol_id);
        let mut paraan = FunctionBuilder::new(self.as_c(&sym.get_type()), sym.name());
        for param in params.iter() {
            paraan = paraan.add_param(self.as_c(&param.ttype), param.id.lexeme());
        }

        paraan = self.build_fn_block(block, paraan);

        paraan.build()
    }

    fn build_fn_block(&self, block: &TypedStmt, mut paraan: FunctionBuilder) -> FunctionBuilder {
        let TypedStmtKind::Block { stmts } = &block.kind else {
            unreachable!()
        };

        for stmt in stmts {
            paraan = paraan.add_statement(self.gen_stmt(stmt));
        }

        paraan
    }

    fn gen_block(&self, stmt: &TypedStmt) -> CStatement {
        let TypedStmtKind::Block { stmts } = &stmt.kind else {
            unreachable!()
        };

        let mut block = BlockBuilder::new();
        for stmt in stmts.iter() {
            block = block.add_statement(self.gen_stmt(stmt));
        }

        block.build()
    }

    fn gen_ibalik(&self, stmt: &TypedStmt) -> CStatement {
        let TypedStmtKind::Ibalik { rhs } = &stmt.kind else {
            unreachable!()
        };

        let mut builder = ReturnBuilder::new();
        if let Some(tex) = rhs {
            builder = builder.with_rhs(self.gen_expr(tex));
        }

        builder.build()
    }

    fn gen_bawat(&self, stmt: &TypedStmt) -> CStatement {
        let TypedStmtKind::Bawat {
            iter,
            bind_type,
            block,
        } = &stmt.kind
        else {
            unreachable!()
        };

        todo!("`bawat` statement does not work for now")
    }

    fn gen_habang(&self, stmt: &TypedStmt) -> CStatement {
        let TypedStmtKind::Habang { cond, block } = &stmt.kind else {
            unreachable!()
        };

        WhileBuilder::new(self.gen_expr(cond), self.gen_block(block)).build()
    }

    fn gen_expr(&self, expr: &TypedExpr) -> String {
        match &expr.kind {
            TypedExprKind::Integer { lexeme }
            | TypedExprKind::Float { lexeme }
            | TypedExprKind::Identifier { lexeme } => lexeme.lexeme.clone(),
            TypedExprKind::Bool { lexeme } => match lexeme.lexeme() {
                "tama" => "true".to_string(),
                "mali" => "mali".to_string(),
                _ => unreachable!(),
            },
            TypedExprKind::Add { left, right } => {
                format!("({} + {})", self.gen_expr(left), self.gen_expr(right))
            }
            TypedExprKind::Sub { left, right } => {
                format!("({} - {})", self.gen_expr(left), self.gen_expr(right))
            }
            TypedExprKind::Mult { left, right } => {
                format!("({} * {})", self.gen_expr(left), self.gen_expr(right))
            }
            TypedExprKind::Div { left, right } => {
                format!("({} / {})", self.gen_expr(left), self.gen_expr(right))
            }
            TypedExprKind::Equality { left, right } => {
                format!("({} == {})", self.gen_expr(left), self.gen_expr(right))
            }
            TypedExprKind::InEquality { left, right } => {
                format!("({} != {})", self.gen_expr(left), self.gen_expr(right))
            }
            TypedExprKind::Greater { left, right } => {
                format!("({} > {})", self.gen_expr(left), self.gen_expr(right))
            }
            TypedExprKind::Less { left, right } => {
                format!("({} < {})", self.gen_expr(left), self.gen_expr(right))
            }
            TypedExprKind::GreaterEqual { left, right } => {
                format!("({} >= {})", self.gen_expr(left), self.gen_expr(right))
            }
            TypedExprKind::LessEqual { left, right } => {
                format!("({} <= {})", self.gen_expr(left), self.gen_expr(right))
            }
            TypedExprKind::FnCall { callee, args } => {
                format!(
                    "({}({}))",
                    self.gen_expr(callee),
                    args.iter()
                        .map(|tex| self.gen_expr(tex))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            TypedExprKind::StructLiteral { .. } => todo!(),
        }
    }

    fn as_c(&self, ttype: &TolType) -> CType {
        match ttype {
            TolType::U8 => CType::U8,
            TolType::U16 => CType::U16,
            TolType::U32 => CType::U32,
            TolType::U64 => CType::U64,
            TolType::USize => CType::Size,
            TolType::I8 => CType::I8,
            TolType::I16 => CType::I16,
            TolType::I32 => CType::I32,
            TolType::I64 => CType::I64,
            TolType::ISize => CType::PtrDiff,
            TolType::F32 => CType::Float,
            TolType::F64 => CType::Double,
            TolType::Byte => CType::U8,
            TolType::Char => CType::Char,
            TolType::Bool => CType::Bool,
            TolType::UnknownIdentifier(s) => CType::Custom(s.to_owned()),
            TolType::Void => CType::Void,
            _ => unreachable!(
                "{} is unreachable as it is already checked by the analyzer",
                ttype
            ),
        }
    }

    fn get_symbol(&self, id: usize) -> &Symbol {
        &self.symbols[id]
    }
}
