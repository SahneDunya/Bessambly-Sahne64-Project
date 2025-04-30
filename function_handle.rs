use crate::parser::{AstNode, Expression};
use crate::semantic_analyzer::SemanticAnalyzer;
use crate::symbol_table::{Symbol, SymbolType, Scope};
use crate::types::Type; // Eğer fonksiyon tiplerini tutacaksanız

impl SemanticAnalyzer {
    pub fn analyze_functions(&mut self, ast: &AstNode) {
        match ast {
            AstNode::Program(statements) => {
                for statement in statements {
                    self.analyze_function_statement(statement);
                }
            }
            _ => {}
        }
    }

    fn analyze_function_statement(&mut self, node: &AstNode) {
        match node {
            AstNode::Instruction { opcode, operands } => {
                if opcode == "CALL" {
                    self.resolve_procedure_call(operands);
                }
            }
            // Prosedür tanımlama
            AstNode::Label { name } if name.starts_with("PROCEDURE_") => {
                let proc_name = name.split_at("PROCEDURE_".len()).1;
                self.declare_procedure(proc_name);
                self.enter_scope(Scope::Local(proc_name.to_string()));
                // Prosedür içindeki parametreleri ve yerel değişkenleri analiz et
                // ...
            }
            // Prosedür sonu - AST'ye eklenmesi gerekebilir
            // ...
            AstNode::SpawnTask { function, .. } => {
                // SPAWN komutundaki 'function' bir prosedür olmalı
                self.resolve_procedure(function);
                // Öncelik argümanı zaten semantik analizde kontrol ediliyor
            }
            _ => {}
        }
    }

    fn declare_procedure(&mut self, name: &str) {
        let symbol = Symbol {
            name: name.to_string(),
            symbol_type: SymbolType::Procedure,
            scope: Scope::Global, // Prosedürler genellikle global kapsamdadır
        };
        if self.symbol_table.lookup(name).is_some() {
            panic!("Prosedür '{}' zaten tanımlı", name);
        }
        self.symbol_table.insert(symbol);
    }

    fn resolve_procedure_call(&self, operands: &Vec<Expression>) {
        if let Some(Expression::Identifier(proc_name)) = operands.first() {
            if self.symbol_table.lookup(proc_name).filter(|s| s.symbol_type == SymbolType::Procedure).is_none() {
                panic!("Tanımsız prosedür '{}'", proc_name);
            }
            // Argüman sayısını ve tiplerini kontrol edebilirsiniz (ileride eklenecek)
        } else {
            panic!("CALL komutu geçerli bir prosedür adı almalı");
        }
    }

    fn enter_scope(&mut self, scope: Scope) {
        self.current_scope = scope;
    }

    fn exit_scope(&mut self) {
        self.current_scope = Scope::Global; // Basit kapsam çıkışı
        // Yerel kapsamdaki sembolleri temizleme mantığı eklenebilir
    }
}