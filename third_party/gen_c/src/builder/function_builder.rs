use crate::{ctype::CType, product::statement::CStatement};

pub struct FunctionBuilder {
    return_type: CType,
    modifiers: Vec<String>,
    name: String,
    params: Vec<String>,
    body: Box<CStatement>,
}

impl FunctionBuilder {
    pub fn new(return_type: CType, name: &str) -> Self {
        Self {
            return_type,
            modifiers: Vec::new(),
            name: name.to_string(),
            params: Vec::new(),
            body: Box::new(CStatement::Block {
                statements: Vec::new(),
            }),
        }
    }

    pub fn add_param(mut self, ttype: CType, name: &str) -> Self {
        self.params.push(format!("{} {}", ttype, name));

        self
    }

    pub fn add_statement(mut self, statement: CStatement) -> Self {
        let CStatement::Block { statements } = self.body.as_mut() else {
            unreachable!()
        };
        statements.push(statement);

        self
    }

    pub fn as_static(mut self) -> Self {
        self.modifiers.push("static".to_string());

        self
    }

    pub fn build(self) -> CStatement {
        CStatement::Function {
            modifiers: self.modifiers,
            return_type: self.return_type,
            name: self.name,
            params: self.params,
            body: self.body,
        }
    }
}
