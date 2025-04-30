use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::semantic_analyzer::SemanticAnalyzer;
use crate::error_reporter::ErrorReporter;
use crate::memory_manager::MemoryManager;
use crate::codegen::CodeGenerator;
use crate::linker::Linker;
use std::fs;
use std::io;
use std::io::Write;

pub struct Compiler {
    error_reporter: ErrorReporter,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            error_reporter: ErrorReporter::new(),
        }
    }

    pub fn compile(&mut self, input_filename: &str, output_filename: &str) -> Result<(), io::Error> {
        let input_code = fs::read_to_string(input_filename)?;
        let mut lexer = Lexer::new(input_code);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse_program();

        let mut semantic_analyzer = SemanticAnalyzer::new();
        semantic_analyzer.analyze(&ast);
        semantic_analyzer.analyze_control_flow(&ast);
        semantic_analyzer.analyze_functions(&ast);
        semantic_analyzer.analyze_variables(&ast);
        semantic_analyzer.analyze_operators(&ast);
        semantic_analyzer.analyze_io(&ast);

        if self.error_reporter.has_errors() {
            self.error_reporter.print_errors();
            return Ok(());
        }

        let memory_manager = MemoryManager::new(0x1000); // Statik bellek başlangıç adresi
        // Sembol tablosunu kullanarak statik değişkenler ve handle'lar için bellek/kayıt ayır
        if let crate::parser::AstNode::Program(statements) = &ast {
            for statement in statements {
                match statement {
                    AstNode::Instruction { opcode, operands } if opcode == "VAR" && operands.len() == 2 => {
                        if let (Expression::Identifier(name), Expression::Identifier(type_name)) = (&operands[0], &operands[1]) {
                            let size = semantic_analyzer.resolve_type(type_name).size().unwrap_or(4); // Varsayılan 4
                            memory_manager.allocate_static(name, size).unwrap();
                        }
                    }
                    AstNode::AllocateMemory { handle, .. } |
                    AstNode::AcquireResource { handle, .. } |
                    AstNode::GetTaskId { target: handle } |
                    AstNode::GetCoreId { target: handle } |
                    AstNode::GetTotalCores { target: handle } => {
                        memory_manager.allocate_handle(handle).unwrap();
                    }
                    _ => {}
                }
            }
        }

        let code_generator = CodeGenerator::new(semantic_analyzer.symbol_table, memory_manager);
        let generated_code = code_generator.generate_code(&ast);

        let linker = Linker::new();
        let linked_code = linker.link(generated_code);

        let mut output_file = fs::File::create(output_filename)?;
        for line in linked_code {
            writeln!(output_file, "{}", line)?;
        }

        println!("Derleme başarılı. Çıktı dosyası: {}", output_filename);
        Ok(())
    }
}