use crate::module::lexed_module::LexedModule;

pub struct Parser {
    lexed_module: LexedModule,
}

impl Parser {
    pub fn new(lexed_module: LexedModule) -> Self {
        Self { lexed_module }
    }
}
