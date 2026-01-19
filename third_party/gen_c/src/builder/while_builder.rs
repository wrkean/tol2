use crate::product::statement::CStatement;

pub struct WhileBuilder {
    cond: String,
    body: CStatement,
}

impl WhileBuilder {
    pub fn new(cond: String, body: CStatement) -> Self {
        Self { cond, body }
    }

    pub fn build(self) -> CStatement {
        CStatement::While {
            cond: self.cond,
            body: Box::new(self.body),
        }
    }
}
