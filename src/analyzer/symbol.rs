use std::ops::Range;

use crate::toltype::TolType;

#[derive(Debug, Clone)]
pub enum SymbolKind {
    Var {
        ttype: TolType,
    },
    Func {
        param_types: Vec<TolType>,
        return_type: TolType,
    },
}

#[derive(Debug, Clone)]
pub struct Symbol {
    name: String,
    kind: SymbolKind,
    span: Range<usize>,
}

impl Symbol {
    pub fn new(name: &str, kind: SymbolKind, span: Range<usize>) -> Self {
        Self {
            name: name.to_string(),
            kind,
            span,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> &SymbolKind {
        &self.kind
    }

    pub fn span(&self) -> Range<usize> {
        self.span.clone()
    }

    /// ```
    /// SymbolKind::Var => ttype.to_owned(),
    /// SymbolKind::Func => return_type.to_owned(),
    /// ```
    pub fn get_type(&self) -> TolType {
        match self.kind() {
            SymbolKind::Var { ttype } => ttype.to_owned(),
            SymbolKind::Func { return_type, .. } => return_type.to_owned(),
        }
    }
}
