use crate::{
    ast::stmt::{Stmt, StmtKind},
    error::CompilerError,
    toltype::TolType,
};

pub struct TypeResolver;

impl TypeResolver {
    pub fn resolve_stmt(stmt: &mut Stmt) -> Result<(), CompilerError> {
        match &stmt.kind {
            StmtKind::Paraan { .. } => Self::resolve_paraan(stmt),
            StmtKind::Ang { .. } | StmtKind::Dapat { .. } => Self::resolve_decl(stmt),
            _ => Ok(()),
        }
    }

    pub fn resolve_paraan(stmt: &mut Stmt) -> Result<(), CompilerError> {
        let StmtKind::Paraan {
            return_type,
            params,
            block,
            ..
        } = &mut stmt.kind
        else {
            unreachable!()
        };

        for param in params.iter_mut() {
            param.ttype = Self::resolve_type(&param.ttype);
        }

        *return_type = Self::resolve_type(return_type);

        Self::resolve_stmt(block)?;

        Ok(())
    }

    pub fn resolve_decl(stmt: &mut Stmt) -> Result<(), CompilerError> {
        let (StmtKind::Ang { ttype, .. } | StmtKind::Dapat { ttype, .. }) = &mut stmt.kind else {
            unreachable!()
        };

        if let Some(t) = ttype {
            *t = Self::resolve_type(t);
        }

        Ok(())
    }

    pub fn resolve_type(ttype: &TolType) -> TolType {
        match ttype {
            TolType::UnknownIdentifier(_s) => todo!(),
            TolType::Array { inner, size } => TolType::Array {
                inner: Box::new(Self::resolve_type(inner)),
                size: size.to_owned(),
            },
            _ => ttype.to_owned(),
        }
    }
}
