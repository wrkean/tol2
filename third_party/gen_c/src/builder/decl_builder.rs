use crate::{ctype::CType, product::statement::CStatement};

pub struct DeclBuilder {
    modifier: Option<String>,
    ttype: CType,
    name: String,
    rhs: Option<String>,
}

impl DeclBuilder {
    pub fn new(ttype: CType, name: &str) -> Self {
        Self {
            modifier: None,
            ttype,
            name: name.to_string(),
            rhs: None,
        }
    }

    pub fn with_rhs(mut self, rhs: String) -> Self {
        self.rhs = Some(rhs);

        self
    }

    pub fn as_const(mut self, kind: ConstKind) -> Self {
        let modifier = match kind {
            ConstKind::Const => "const".to_string(),
            ConstKind::ConstPtr => "*const".to_string(),
            ConstKind::ConstConstPtr => "const * const".to_string(),
        };
        self.modifier = Some(modifier);

        self
    }

    pub fn build(self) -> CStatement {
        CStatement::Declaration {
            modifier: self.modifier,
            ttype: self.ttype,
            name: self.name,
            rhs: self.rhs,
        }
    }
}

pub enum ConstKind {
    Const,
    ConstPtr,
    ConstConstPtr,
}
