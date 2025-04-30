use crate::parser::{AstNode, Expression};
use crate::semantic_analyzer::SemanticAnalyzer;
use crate::symbol_table::{Symbol, SymbolType, Scope};
use crate::types::{Type, TYPE_DWORD, TYPE_USIZE, TYPE_STRING, TYPE_HANDLE, TYPE_TASK_ID}; // İlgili tipleri kullanacağız

impl SemanticAnalyzer {
    pub fn analyze_variables(&mut self, ast: &AstNode) {
        match ast {
            AstNode::Program(statements) => {
                for statement in statements {
                    self.analyze_variable_statement(statement);
                }
            }
            _ => {}
        }
    }

    fn analyze_variable_statement(&mut self, node: &AstNode) {
        match node {
            // Değişken tanımlama (örneğin "VAR my_var DWORD") - AST'ye eklenmesi gerekebilir
            AstNode::Instruction { opcode, operands } if opcode == "VAR" => {
                if operands.len() == 2 {
                    if let (Expression::Identifier(var_name), Expression::Identifier(type_name)) = (&operands[0], &operands[1]) {
                        let var_type = self.resolve_type(type_name);
                        self.declare_variable(var_name, var_type);
                    } else {
                        panic!("VAR komutu geçerli bir değişken adı ve tip adı almalı");
                    }
                } else {
                    panic!("VAR komutu iki operand almalı (ad ve tip)");
                }
            }
            AstNode::Assignment { variable, value } => {
                self.resolve_variable(variable);
                self.analyze_expression(value, self.get_variable_type(variable));
                // Tip uyumluluğu kontrolü analyze_expression içinde yapılıyor
            }
            AstNode::Instruction { opcode: _, operands } => {
                for operand in operands {
                    if let Expression::Identifier(var_name) = operand {
                        self.resolve_variable(var_name);
                    }
                }
            }
            AstNode::AllocateMemory { handle, .. } |
            AstNode::AcquireResource { handle, .. } |
            AstNode::GetTaskId { target: handle } |
            AstNode::GetCoreId { target: handle } |
            AstNode::GetTotalCores { target: handle } => {
                // Bu yapılar zaten semantik analizde handle/task_id olarak tanımlanıyor
            }
            AstNode::ReceiveMessage { buffer, .. } => {
                self.resolve_variable(buffer); // Buffer bir değişken olmalı
                // Burada buffer'ın yeterli boyutta olup olmadığı kontrol edilebilir
            }
            _ => {}
        }
    }

    fn declare_variable(&mut self, name: &str, var_type: Type) {
        let symbol = Symbol {
            name: name.to_string(),
            symbol_type: SymbolType::Variable(Some(var_type)),
            scope: self.current_scope.clone(),
            // İleride adres bilgileri eklenebilir
        };
        if self.symbol_table.lookup_in_scope(name, &self.current_scope).is_some() {
            panic!("Değişken '{}' zaten bu kapsamda tanımlı", name);
        }
        self.symbol_table.insert(symbol);
    }

    fn get_variable_type(&self, name: &str) -> Option<&Type> {
        if let Some(symbol) = self.symbol_table.lookup(name) {
            if let SymbolType::Variable(var_type) = &symbol.symbol_type {
                return var_type.as_ref();
            }
        }
        None
    }

    // Basit tip çözümleyici
    fn resolve_type(&self, type_name: &str) -> Type {
        match type_name {
            "DWORD" => TYPE_DWORD.clone(),
            "BYTE" => crate::types::TYPE_BYTE.clone(),
            "WORD" => crate::types::TYPE_WORD.clone(),
            "QWORD" => crate::types::TYPE_QWORD.clone(),
            "PTR" => crate::types::TYPE_POINTER.clone(),
            "STRING" => TYPE_STRING.clone(),
            "HANDLE" => TYPE_HANDLE.clone(),
            "TASK_ID" => TYPE_TASK_ID.clone(),
            "USIZE" => TYPE_USIZE.clone(),
            _ => panic!("Bilinmeyen tip '{}'", type_name),
        }
    }
}