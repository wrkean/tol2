use crate::product::statement::CStatement;

pub struct ReturnBuilder {
    rhs: Option<String>,
}

impl ReturnBuilder {
    pub fn new() -> Self {
        Self { rhs: None }
    }

    pub fn with_rhs(mut self, rhs: String) -> Self {
        self.rhs = Some(rhs);

        self
    }

    pub fn build(self) -> CStatement {
        CStatement::Return { rhs: self.rhs }
    }
}
