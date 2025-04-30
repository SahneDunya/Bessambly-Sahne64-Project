use crate::parser::{AstNode, Expression};
use crate::semantic_analyzer::SemanticAnalyzer;
use crate::types::{Type, TYPE_USIZE}; // Handle tipi için

impl SemanticAnalyzer {
    pub fn analyze_io(&mut self, ast: &AstNode) {
        match ast {
            AstNode::Program(statements) => {
                for statement in statements {
                    self.analyze_io_statement(statement);
                }
            }
            _ => {}
        }
    }

    fn analyze_io_statement(&mut self, node: &AstNode) {
        match node {
            AstNode::Instruction { opcode, operands } => {
                if opcode == "READ" {
                    self.analyze_read(operands);
                } else if opcode == "WRITE" {
                    self.analyze_write(operands);
                }
            }
            AstNode::SendMessage { handle, message } => {
                self.analyze_send_message(handle, message);
            }
            AstNode::ReceiveMessage { handle, buffer } => {
                self.analyze_receive_message(handle, buffer);
            }
            _ => {}
        }
    }

    fn analyze_read(&self, operands: &Vec<Expression>) {
        if operands.len() != 1 {
            panic!("READ komutu bir operand (hedef değişken) gerektirir");
        }
        if let Some(Expression::Identifier(var_name)) = operands.first() {
            if self.symbol_table.lookup(var_name).filter(|s| matches!(s.symbol_type, SymbolType::Variable(_))).is_none() {
                panic!("Tanımsız değişken '{}' READ komutunda kullanılıyor", var_name);
            }
            // İstenirse değişkenin tipinin uygun olup olmadığı kontrol edilebilir
        } else {
            panic!("READ komutunun operandı bir değişken olmalı");
        }
    }

    fn analyze_write(&self, operands: &Vec<Expression>) {
        if operands.len() != 1 {
            panic!("WRITE komutu bir operand (çıktı değeri veya değişken) gerektirir");
        }
        if let Some(operand) = operands.first() {
            match operand {
                Expression::Identifier(var_name) => {
                    if self.symbol_table.lookup(var_name).filter(|s| matches!(s.symbol_type, SymbolType::Variable(_))).is_none() {
                        panic!("Tanımsız değişken '{}' WRITE komutunda kullanılıyor", var_name);
                    }
                    // İstenirse değişkenin tipinin uygun olup olmadığı kontrol edilebilir
                }
                Expression::Number(_) | Expression::Flag(_) | Expression::StringLiteral(_) => {} // Sabit değerler de yazılabilir
                _ => panic!("WRITE komutunun operandı bir değişken, sayı, flag veya string olmalı"),
            }
        }
    }

    fn analyze_send_message(&self, handle: &Expression, message: &Expression) {
        self.analyze_expression(handle, Some(&TYPE_USIZE)); // Handle usize olmalı
        // Mesajın tipi hakkında daha fazla bilgiye ihtiyaç duyulabilir (yapı, boyut vb.)
        // Şu anda sadece var olup olmadığını kontrol ediyoruz (analyze_expression)
    }

    fn analyze_receive_message(&self, handle: &Expression, buffer: &String) {
        self.analyze_expression(handle, Some(&TYPE_USIZE)); // Handle usize olmalı
        if self.symbol_table.lookup(buffer).filter(|s| matches!(s.symbol_type, SymbolType::Variable(_))).is_none() {
            panic!("Tanımsız değişken '{}' RECV komutunda buffer olarak kullanılıyor", buffer);
        }
        // Buffer'ın yeterli boyutta olup olmadığı gibi ek kontroller yapılabilir (tipine bakarak)
    }

    fn analyze_expression(&self, expression: &Expression, expected_type: Option<&Type>) {
        // Bu fonksiyon semantic_analyzer.rs'den buraya taşınabilir veya ortak bir yerde tutulabilir.
        // Tekrar eden kodu önlemek için bu bir iyileştirme olabilir.
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
            Expression::Flag(_) => {}
            Expression::StringLiteral(_) => {
                if let Some(expected) = expected_type {
                    if !matches!(expected.base, crate::types::BaseType::String) {
                        panic!("Tip uyuşmazlığı: String beklenmiyordu");
                    }
                }
            }
            // Handle ve TaskId kontrolleri
            Expression::Identifier(name) => {
                if let Some(expected) = expected_type {
                    if expected.base == crate::types::BaseType::Handle {
                        if self.symbol_table.lookup(name).filter(|s| s.symbol_type != SymbolType::Handle).is_some() {
                            panic!("Tip uyuşmazlığı: Handle bekleniyordu, '{}' bulundu", name);
                        }
                    } else if expected.base == crate::types::BaseType::TaskId {
                        if self.symbol_table.lookup(name).filter(|s| s.symbol_type != SymbolType::TaskId).is_some() {
                            panic!("Tip uyuşmazlığı: TaskId bekleniyordu, '{}' bulundu", name);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn resolve_variable(&self, name: &str) {
        if self.symbol_table.lookup(name).filter(|s| matches!(s.symbol_type, SymbolType::Variable(_))).is_none() {
            panic!("Tanımsız değişken '{}'", name);
        }
    }

    fn get_variable_type(&self, name: &str) -> Option<&Type> {
        if let Some(symbol) = self.symbol_table.lookup(name) {
            if let SymbolType::Variable(var_type) = &symbol.symbol_type {
                return var_type.as_ref();
            }
        }
        None
    }
}