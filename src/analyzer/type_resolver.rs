use crate::{
    ast::stmt::{Stmt, StmtKind},
    error::CompilerError,
    toltype::TolType,
};

pub struct TypeResolver;

impl TypeResolver {
    pub fn resolve(&mut self, stmt: &mut Stmt) -> Result<(), CompilerError> {
        match &stmt.kind {
            StmtKind::Paraan { .. } => self.resolve_paraan(stmt),
            StmtKind::Ang { .. } | StmtKind::Dapat { .. } => self.resolve_decl(stmt),
            StmtKind::Ibalik { .. } => todo!(),
            StmtKind::Bawat { .. } => todo!(),
            StmtKind::Habang { .. } => todo!(),
            StmtKind::Kung { .. } => todo!(),
            StmtKind::Block { .. } => todo!(),
            StmtKind::Gagawin => todo!(),
            StmtKind::Null => todo!(),
        }
    }

    pub fn resolve_paraan(&self, stmt: &mut Stmt) -> Result<(), CompilerError> {
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
            param.ttype = self.resolve_type(&param.ttype);
        }

        *return_type = self.resolve_type(return_type);

        self.resolve(block)?;

        Ok(())
    }

    pub fn resolve_decl(&self, stmt: &mut Stmt) -> Result<(), CompilerError> {
        let (StmtKind::Ang { ttype, .. } | StmtKind::Dapat { ttype, .. }) = &mut stmt.kind else {
            unreachable!()
        };

        *ttype = self.resolve_type(ttype);

        Ok(())
    }

    pub fn resolve_type(&self, ttype: &TolType) -> TolType {
        match ttype {
            TolType::UnknownIdentifier(_s) => todo!(),
            _ => ttype.to_owned(),
        }
    }
}
