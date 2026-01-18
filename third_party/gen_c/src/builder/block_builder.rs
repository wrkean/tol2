use crate::product::statement::CStatement;

#[derive(Default)]
pub struct BlockBuilder {
    statements: Vec<CStatement>,
}

impl BlockBuilder {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }

    pub fn add_statement(mut self, statement: CStatement) -> Self {
        self.statements.push(statement);

        self
    }

    pub fn build(self) -> CStatement {
        CStatement::Block {
            statements: self.statements,
        }
    }
}
