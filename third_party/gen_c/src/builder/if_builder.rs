use crate::product::statement::{CStatement, IfBranch};

pub struct IfBuilder {
    initial_branch: IfBranch,
    branches: Vec<IfBranch>,
}

impl IfBuilder {
    pub fn new(initial_cond: String, initial_body: CStatement) -> Self {
        Self {
            initial_branch: IfBranch::new(Some(initial_cond), initial_body),
            branches: Vec::new(),
        }
    }

    pub fn add_elseif_branch(mut self, cond: String, body: CStatement) -> Self {
        self.branches.push(IfBranch::new(Some(cond), body));

        self
    }

    pub fn with_else_branch(mut self, body: CStatement) -> Self {
        self.branches.push(IfBranch::new(None, body));

        self
    }

    pub fn build(self) -> CStatement {
        CStatement::If {
            initial_cond: self.initial_branch.cond.unwrap(),
            initial_block: self.initial_branch.body,
            branches: self.branches,
        }
    }
}
