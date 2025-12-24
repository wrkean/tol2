use std::{collections::HashMap, rc::Rc};

use crate::module::compiled_module::CompiledModule;

pub struct ModuleRegistry<'com> {
    main_module: Option<CompiledModule<'com>>,
    stdlib: Option<CompiledModule<'com>>,
    #[allow(dead_code)]
    cache: HashMap<String, Rc<CompiledModule<'com>>>,
}

impl<'com> ModuleRegistry<'com> {
    pub fn new() -> Self {
        Self {
            main_module: None,
            stdlib: None,
            cache: HashMap::new(),
        }
    }

    pub fn cache(&mut self) {
        todo!()
    }

    pub fn is_main_loaded(&self) -> bool {
        self.main_module.is_some()
    }

    pub fn is_stdlib_loaded(&self) -> bool {
        self.stdlib.is_some()
    }
}

impl<'com> Default for ModuleRegistry<'com> {
    fn default() -> Self {
        Self::new()
    }
}
