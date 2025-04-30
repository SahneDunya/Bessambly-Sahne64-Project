use crate::parser::{AstNode, Expression};
use crate::semantic_analyzer::SemanticAnalyzer;
use crate::types::Type; // Operatörlerin tip kontrolü için

impl SemanticAnalyzer {
    pub fn analyze_operators(&mut self, ast: &AstNode) {
        match ast {
            AstNode::Program(statements) => {
                for statement in statements {
                    self.analyze_operator_statement(statement);
                }
            }
            _ => {}
        }
    }

    fn analyze_operator_statement(&mut self, node: &AstNode) {
        match node {
            AstNode::Instruction { opcode, operands } => {
                self.analyze_instruction_operators(opcode, operands);
            }
            AstNode::Assignment { variable, value } => {
                self.resolve_variable(variable);
                self.analyze_expression(value, self.get_variable_type(variable));
                // Tip uyumluluğu kontrolü analyze_expression içinde yapılıyor
            }
            // Diğer AST düğümlerinde de operatörler olabilir (ifadelerin içinde)
            _ => {}
        }
    }

    fn analyze_instruction_operators(&mut self, opcode: &str, operands: &Vec<Expression>) {
        match opcode {
            "ADD" | "SUB" | "MUL" | "DIV" => {
                self.check_arithmetic_operands(operands);
            }
            "CMP" => {
                self.check_comparison_operands(operands);
            }
            "AND" | "OR" | "XOR" | "NOT" => {
                self.check_logical_operands(operands);
            }
            // Diğer opcodelar ve ilgili operatör kontrolleri buraya eklenebilir
            _ => {}
        }
    }

    fn check_arithmetic_operands(&self, operands: &Vec<Expression>) {
        if operands.len() != 2 {
            panic!("Aritmetik işlemler iki operand gerektirir");
        }
        for operand in operands {
            self.ensure_is_numeric(operand);
        }
        // İstenirse operand tiplerinin uyumluluğu da kontrol edilebilir
    }

    fn check_comparison_operands(&self, operands: &Vec<Expression>) {
        if operands.len() != 2 {
            panic!("Karşılaştırma işlemleri iki operand gerektirir");
        }
        for operand in operands {
            self.ensure_is_comparable(operand);
        }
        // İstenirse operand tiplerinin uyumluluğu da kontrol edilebilir
    }

    fn check_logical_operands(&self, operands: &Vec<Expression>) {
        if operands.len() != 2 && operands.len() != 1 { // NOT tek operand alır
            panic!("Mantıksal işlemler bir veya iki operand gerektirir");
        }
        for operand in operands {
            self.ensure_is_logical(operand);
        }
        // İstenirse operand tiplerinin uyumluluğu da kontrol edilebilir
    }

    fn ensure_is_numeric(&self, operand: &Expression) {
        match operand {
            Expression::Number(_) | Expression::Identifier(_) => {
                if let Expression::Identifier(name) = operand {
                    if let Some(symbol) = self.symbol_table.lookup(name) {
                        if let Some(var_type) = self.get_variable_type(name) {
                            if !matches!(var_type.base, crate::types::BaseType::Integer { .. } | crate::types::BaseType::Pointer) {
                                panic!("Operand '{}' sayısal bir tipte olmalı", name);
                            }
                        }
                    }
                }
            }
            _ => panic!("Operand sayısal bir değer veya değişken olmalı"),
        }
    }

    fn ensure_is_comparable(&self, operand: &Expression) {
        match operand {
            Expression::Number(_) | Expression::Identifier(_) | Expression::Flag(_) => {
                if let Expression::Identifier(name) = operand {
                    if let Some(var_type) = self.get_variable_type(name) {
                        // Tip kontrolü eklenebilir
                    }
                }
            }
            _ => panic!("Operand karşılaştırılabilir bir değer, değişken veya flag olmalı"),
        }
    }

    fn ensure_is_logical(&self, operand: &Expression) {
        match operand {
            Expression::Number(_) | Expression::Identifier(_) | Expression::Flag(_) => {
                if let Expression::Identifier(name) = operand {
                    if let Some(var_type) = self.get_variable_type(name) {
                        if !matches!(var_type.base, crate::types::BaseType::Integer { .. }) {
                            panic!("Operand '{}' mantıksal bir tipte olmalı (tamsayı)", name);
                        }
                    }
                }
            }
            _ => panic!("Operand mantıksal bir değer, değişken veya flag olmalı (tamsayı)"),
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