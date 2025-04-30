use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolBinding {
    Global,
    External,
    Sahne64Api, // Sahne64 API fonksiyonları için özel bir bağlama türü
}

#[derive(Debug, Clone)]
pub struct ExternSymbol {
    pub name: String,
    pub binding: SymbolBinding,
    pub address: Option<usize>, // Bağlama aşamasında çözümlenecek adres (API için sistem çağrı numarası olabilir)
}

pub struct ExternSymbolTable {
    symbols: HashMap<String, ExternSymbol>,
}

impl ExternSymbolTable {
    pub fn new() -> Self {
        ExternSymbolTable {
            symbols: HashMap::new(),
        }
    }

    pub fn declare(&mut self, name: &str, binding: SymbolBinding) {
        if self.symbols.contains_key(name) {
            eprintln!("Uyarı: Sembol '{}' zaten tanımlı", name);
        }
        self.symbols.insert(
            name.to_string(),
            ExternSymbol {
                name: name.to_string(),
                binding,
                address: None,
            },
        );
    }

    pub fn resolve(&mut self, name: &str, address: usize) {
        if let Some(symbol) = self.symbols.get_mut(name) {
            symbol.address = Some(address);
        } else {
            eprintln!("Uyarı: Çözümlenmeye çalışılan tanımsız sembol '{}'", name);
        }
    }

    pub fn lookup(&self, name: &str) -> Option<&ExternSymbol> {
        self.symbols.get(name)
    }
}

// Semantik analiz aşamasında harici sembollerin ve Sahne64 API fonksiyonlarının toplanması için
use crate::parser::AstNode;
use crate::semantic_analyzer::SemanticAnalyzer;

impl SemanticAnalyzer {
    pub fn collect_extern_symbols(&mut self, ast: &AstNode) {
        match ast {
            AstNode::Program(statements) => {
                for statement in statements {
                    self.collect_extern_symbol_statement(statement);
                }
            }
            _ => {}
        }
    }

    fn collect_extern_symbol_statement(&mut self, node: &AstNode) {
        match node {
            AstNode::Instruction { opcode, operands } => {
                if opcode == "GLOBAL" && operands.len() == 1 {
                    if let crate::parser::Expression::Identifier(name) = &operands[0] {
                        self.extern_symbol_table.declare(name, SymbolBinding::Global);
                    } else {
                        eprintln!("Uyarı: GLOBAL direktifi bir tanımlayıcı almalı");
                    }
                } else if opcode == "EXTERN" && operands.len() == 1 {
                    if let crate::parser::Expression::Identifier(name) = &operands[0] {
                        self.extern_symbol_table.declare(name, SymbolBinding::External);
                    } else {
                        eprintln!("Uyarı: EXTERN direktifi bir tanımlayıcı almalı");
                    }
                } else if opcode == "SAHNE64_API" && operands.len() == 2 {
                    if let (crate::parser::Expression::Identifier(api_name), crate::parser::Expression::Number(api_id)) = (&operands[0], &operands[1]) {
                        self.extern_symbol_table.declare(api_name, SymbolBinding::Sahne64Api);
                        self.extern_symbol_table.resolve(api_name, *api_id as usize); // API ID'sini adres olarak kaydet
                    } else {
                        eprintln!("Uyarı: SAHNE64_API direktifi API adı ve ID almalı");
                    }
                }
            }
            _ => {}
        }
    }
}
 use crate::linker::Linker;
//
 impl Linker {
     pub fn resolve_extern_symbols(&mut self, object_code: &Vec<String>, symbol_table: &mut ExternSymbolTable) -> Vec<String> {
         let mut resolved_code = object_code.clone();
         for (i, line) in object_code.iter().enumerate() {
             for (symbol_name, symbol) in &symbol_table.symbols {
                 if let Some(address) = symbol.address {
                     // SAHNE64_API sembollerini sistem çağrı numaralarıyla değiştir
                     if symbol.binding == SymbolBinding::Sahne64Api || symbol.binding == SymbolBinding::External {
                         resolved_code[i] = resolved_code[i].replace(symbol_name, &format!("{}", address));
                     }
                 } else if symbol.binding == SymbolBinding::External {
                     eprintln!("Uyarı: Çözümlenemeyen harici sembol '{}'", symbol_name);
                 }
             }
         }
         resolved_code
     }
 }