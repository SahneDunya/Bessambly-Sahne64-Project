use crate::parser::{AstNode, Expression};
use crate::symbol_table::{SymbolTable, Symbol, SymbolType, Scope};
use crate::types::{Type, TYPE_DWORD, TYPE_USIZE}; // Örnek tipler

pub struct SemanticAnalyzer {
    pub symbol_table: SymbolTable,
    current_scope: Scope,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            symbol_table: SymbolTable::new(),
            current_scope: Scope::Global,
        }
    }

    pub fn analyze(&mut self, ast: &AstNode) {
        match ast {
            AstNode::Program(statements) => {
                for statement in statements {
                    self.analyze_statement(statement);
                }
            }
            _ => panic!("Beklenmeyen AST kök düğümü"),
        }
    }

    fn analyze_statement(&mut self, node: &AstNode) {
        match node {
            AstNode::Label { name } => {
                self.declare_symbol(name, SymbolType::Label, None);
            }
            AstNode::Assignment { variable, value } => {
                self.resolve_variable(variable);
                self.analyze_expression(value, None); // İsteğe bağlı beklenen tip
                // Burada tip kontrolü yapılabilir (basitlik için atlandı)
            }
            AstNode::JumpStatement { target } => {
                self.resolve_label(target);
            }
            AstNode::AllocateMemory { size, handle } => {
                self.analyze_expression(size, Some(&TYPE_USIZE));
                self.declare_symbol(handle, SymbolType::Handle, Some(&TYPE_USIZE)); // Handle usize olmalı
            }
            AstNode::ReleaseMemory { handle } => {
                self.analyze_expression(handle, Some(&TYPE_USIZE)); // Handle usize olmalı
            }
            AstNode::SpawnTask { function, priority } => {
                self.resolve_procedure(function);
                if let Some(prio) = priority {
                    self.analyze_expression(prio, Some(&TYPE_DWORD)); // Öncelik dword olabilir
                }
            }
            AstNode::ExitTask { code } => {
                if let Some(c) = code {
                    self.analyze_expression(c, Some(&TYPE_DWORD)); // Çıkış kodu dword olabilir
                }
            }
            AstNode::SleepTask { duration } => {
                self.analyze_expression(duration, Some(&TYPE_DWORD)); // Süre dword olabilir
            }
            AstNode::YieldTask => {}
            AstNode::AcquireResource { name, handle } => {
                self.analyze_expression(name, Some(&Type::string())); // Kaynak adı string olmalı
                self.declare_symbol(handle, SymbolType::Handle, Some(&TYPE_USIZE)); // Handle usize olmalı
            }
            AstNode::ControlResource { handle, command } => {
                self.analyze_expression(handle, Some(&TYPE_USIZE)); // Handle usize olmalı
                self.analyze_expression(command, Some(&TYPE_DWORD)); // Komut dword olabilir
            }
            AstNode::SendMessage { handle, message } => {
                self.analyze_expression(handle, Some(&TYPE_USIZE)); // Handle usize olmalı
                self.analyze_expression(message, None); // Mesajın tipi şu an belirsiz
            }
            AstNode::ReceiveMessage { handle, buffer } => {
                self.analyze_expression(handle, Some(&TYPE_USIZE)); // Handle usize olmalı
                self.resolve_variable(buffer); // Buffer bir değişken olmalı
                // Burada buffer'ın yeterli boyutta olup olmadığı kontrol edilebilir
            }
            AstNode::GetTaskId { target } => {
                self.declare_symbol(target, SymbolType::TaskId, Some(&TYPE_USIZE)); // TaskId usize olmalı
            }
            AstNode::GetCoreId { target } => {
                self.declare_symbol(target, SymbolType::TaskId, Some(&TYPE_USIZE)); // CoreId usize olmalı
            }
            AstNode::GetTotalCores { target } => {
                self.declare_symbol(target, SymbolType::TaskId, Some(&TYPE_USIZE)); // TotalCores usize olmalı
            }
            AstNode::Instruction { opcode: _, operands } => {
                for operand in operands {
                    self.analyze_expression(operand, None); // Operandların tipleri bağlama göre kontrol edilebilir
                }
            }
            _ => {}
        }
    }

    fn analyze_expression(&self, expression: &Expression, expected_type: Option<&Type>) {
        match expression {
            Expression::Identifier(name) => {
                self.resolve_variable(name);
                if let Some(expected) = expected_type {
                    if let Some(symbol) = self.symbol_table.lookup(name) {
                        if let SymbolType::Variable(Some(actual)) = &symbol.symbol_type {
                            if actual != expected {
                                panic!("Tip uyuşmazlığı: '{}' bekleniyordu, '{}' bulundu", expected, actual);
                            }
                        }
                    }
                }
            }
            Expression::Number(_) => {
                if let Some(expected) = expected_type {
                    if !matches!(expected.base, crate::types::BaseType::Integer { .. } | crate::types::BaseType::Pointer) {
                        panic!("Tip uyuşmazlığı: Sayı beklenmiyordu");
                    }
                }
            }
            Expression::Flag(_) => {} // Flag'lerin tipleri bağlama göre kontrol edilebilir
            Expression::StringLiteral(_) => {
                if let Some(expected) = expected_type {
                    if !matches!(expected.base, crate::types::BaseType::String) {
                        panic!("Tip uyuşmazlığı: String beklenmiyordu");
                    }
                }
            }
            Expression::Handle(_) => {
                if let Some(expected) = expected_type {
                    if !matches!(expected.base, crate::types::BaseType::Integer { size: crate::types::IntegerSize::QWord, .. }) {
                        panic!("Tip uyuşmazlığı: Handle beklenmiyordu");
                    }
                }
            }
            Expression::TaskId(_) => {
                if let Some(expected) = expected_type {
                    if !matches!(expected.base, crate::types::BaseType::Integer { size: crate::types::IntegerSize::QWord, .. }) {
                        panic!("Tip uyuşmazlığı: TaskId beklenmiyordu");
                    }
                }
            }
        }
    }

    fn declare_symbol(&mut self, name: &str, symbol_type: SymbolType, var_type: Option<&Type>) {
        let symbol = Symbol {
            name: name.clone(),
            symbol_type: match symbol_type {
                SymbolType::Variable(_) => SymbolType::Variable(var_type.cloned()),
                other => other,
            },
            scope: self.current_scope.clone(),
        };
        if self.symbol_table.lookup_in_scope(name, &self.current_scope).is_some() {
            panic!("Sembol '{}' zaten bu kapsamda tanımlı", name);
        }
        self.symbol_table.insert(symbol);
    }

    fn resolve_variable(&self, name: &str) {
        if self.symbol_table.lookup(name).filter(|s| matches!(s.symbol_type, SymbolType::Variable(_))).is_none() {
            panic!("Tanımsız değişken '{}'", name);
        }
        // İstenirse değişkenin türü ve diğer özellikleri burada kontrol edilebilir
    }

    fn resolve_label(&self, name: &str) {
        if self.symbol_table.lookup(name).filter(|s| s.symbol_type == SymbolType::Label).is_none() {
            panic!("Tanımsız etiket '{}'", name);
        }
    }

    fn resolve_procedure(&self, name: &str) {
        if self.symbol_table.lookup(name).filter(|s| s.symbol_type == SymbolType::Procedure).is_none() {
            panic!("Tanımsız prosedür '{}'", name);
        }
    }
}