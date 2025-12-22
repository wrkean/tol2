use std::{collections::HashMap, rc::Rc};

pub struct CompiledModule<'com> {
    parent: Option<&'com Self>,
    submodules: HashMap<String, Rc<Self>>,
}

impl<'com> CompiledModule<'com> {
    pub fn new(parent: Option<&'com Self>) -> Self {
        Self {
            parent,
            submodules: HashMap::new(),
        }
    }
}
