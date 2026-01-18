use crate::product::statement::CStatement;

pub mod builder;
pub mod ctype;
pub mod product;

pub struct CCodeGen {
    statements: Vec<CStatement>,
    indent: usize,
    // includes: Vec<CInclude>,
}

#[allow(clippy::new_without_default)]
impl CCodeGen {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
            indent: 0,
        }
    }

    pub fn add_statement(mut self, statement: CStatement) -> Self {
        self.statements.push(statement);

        self
    }

    pub fn produce_c(self) -> String {
        let mut out = String::new();
        for statement in self.statements {
            out.push_str(&statement.produce_c(self.indent));
            out.push('\n');
        }

        out
    }
}
