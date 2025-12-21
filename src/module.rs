use std::collections::HashMap;

pub struct Module<'cctx> {
    parent: Option<&'cctx Module<'cctx>>,
    submodules: HashMap<String, &'cctx Module<'cctx>>,
}

impl<'cctx> Module<'cctx> {
    pub fn new(parent: Option<&'cctx Module<'cctx>>) -> Self {
        Self {
            parent,
            submodules: HashMap::new(),
        }
    }
}
