use crate::{
    analyzer::SemanticAnalyzer, driver::CompilerOptions, error::CompilerError, lexer::Lexer,
    module::module_registry::ModuleRegistry, parser::Parser,
};
use std::path::Path;

#[derive(Default)]
pub struct CompilerCtx {
    pub errors: Vec<CompilerError>,
}

impl CompilerCtx {
    pub fn add_error(&mut self, error: CompilerError) {
        self.errors.push(error);
    }

    pub fn extend_errors(&mut self, iter: impl IntoIterator<Item = CompilerError>) {
        self.errors.extend(iter);
    }
}

pub struct Compiler<'com> {
    opts: CompilerOptions,
    module_registry: ModuleRegistry<'com>,
}

impl<'com> Compiler<'com> {
    pub fn new(opts: CompilerOptions) -> Self {
        Self {
            opts,
            module_registry: ModuleRegistry::new(),
        }
    }

    pub fn run(&mut self, source_code: &'com str) -> CompilerCtx {
        // WARN: Have better handling for this
        let source_file_name = self
            .opts
            .source_path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();

        let mut ctx = CompilerCtx::default();

        let lexer = Lexer::new(source_code, source_file_name);
        let tokens = lexer.lex(&mut ctx);

        for tok in tokens.iter() {
            println!("{} <=> {:?}", tok.lexeme(), tok.kind());
        }

        let parser = Parser::new(&tokens);
        let ast = parser.parse(&mut ctx);

        for stmt in ast.iter() {
            println!("{:#?}", stmt);
        }

        let analyzer = SemanticAnalyzer::new(ast);
        analyzer.analyze(&mut ctx);

        ctx
    }

    pub fn load_stdlib(&mut self, _stdlib_path: &Path) {
        todo!()
    }
}
