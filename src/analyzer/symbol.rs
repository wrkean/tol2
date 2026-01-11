use std::{fmt, ops::Range};

use crate::toltype::TolType;

#[derive(Debug, Clone)]
pub enum SymbolKind {
    Var {
        ttype: TolType,
    },
    Paraan {
        params_types: Vec<TolType>,
        params_span: Range<usize>,
        return_type: TolType,
    },
}

impl fmt::Display for SymbolKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SymbolKind::Var { .. } => write!(f, "variable/constant"),
            SymbolKind::Paraan { .. } => write!(f, "paraan"),
        }
    }
}

#[derive(Debug)]
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

    pub fn span(&self) -> Range<usize> {
        self.span.clone()
    }

    pub fn kind(&self) -> &SymbolKind {
        &self.kind
    }

    pub fn kind_name(&self) -> String {
        self.kind.to_string()
    }
}
