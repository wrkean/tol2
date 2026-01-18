use gen_c::CCodeGen;

use crate::{
    analyzer::{SemanticAnalyzer, symbol::Symbol},
    codegen::Codegen,
    driver::CompilerOptions,
    error::CompilerError,
    lexer::Lexer,
    module::module_registry::ModuleRegistry,
    parser::Parser,
};
use std::path::Path;

#[derive(Default)]
pub struct CompilerCtx {
    pub continue_compiling: bool,
    pub errors: Vec<CompilerError>,
    pub symbol_table: Vec<Symbol>,
}

impl CompilerCtx {
    pub fn new() -> Self {
        Self {
            continue_compiling: true,
            errors: Vec::new(),
            symbol_table: Vec::new(),
        }
    }

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

        let mut ctx = CompilerCtx::new();

        let lexer = Lexer::new(source_code, source_file_name);
        let tokens = lexer.lex(&mut ctx);

        for tok in tokens.iter() {
            println!("{} <=> {:?}", tok.lexeme(), tok.kind());
        }

        if !ctx.continue_compiling {
            return ctx;
        }

        let parser = Parser::new(&tokens);
        let ast = parser.parse(&mut ctx);

        for stmt in ast.iter() {
            println!("{:#?}", stmt);
        }

        let analyzer = SemanticAnalyzer::new(&mut ctx);
        let typed_ast = analyzer.analyze(ast);
        println!("{:#?}", typed_ast);
        println!("{:#?}", ctx.symbol_table);

        let codegen = Codegen::new(&typed_ast, &ctx.symbol_table);
        println!("{}", codegen.generate_c(CCodeGen::new()));

        ctx
    }

    pub fn load_stdlib(&mut self, _stdlib_path: &Path) {
        todo!()
    }
}
