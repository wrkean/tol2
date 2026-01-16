use crate::toltype::TolType;

pub struct AnalyzerContext {
    fn_return_types: Vec<TolType>,
}

impl AnalyzerContext {
    pub fn new() -> Self {
        Self {
            fn_return_types: vec![TolType::Unknown],
        }
    }

    pub fn enter_fn(&mut self, new_fn_return_type: TolType) {
        self.fn_return_types.push(new_fn_return_type);
    }

    pub fn exit_fn(&mut self) -> TolType {
        self.fn_return_types.pop().unwrap()
    }

    pub fn cur_fn_return_type(&self) -> &TolType {
        self.fn_return_types.last().unwrap()
    }
}
