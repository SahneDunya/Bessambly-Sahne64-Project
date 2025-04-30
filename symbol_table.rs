use std::collections::HashMap;
use crate::types::Type; // Tipleri kullanacağız

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolType {
    Variable(Option<Type>), // Değişkenler isteğe bağlı bir tipe sahip olabilir
    Label,
    Procedure,
    Macro,
    External,
    Global,
    Handle,
    TaskId,
    ResourceId,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub symbol_type: SymbolType,
    pub scope: Scope, // Basit kapsam yönetimi
    // Diğer özellikler (adres vb.) eklenebilir
}

#[derive(Debug, Clone, PartialEq)]
pub enum Scope {
    Global,
    Local(String), // Prosedür adı
}

pub struct SymbolTable {
    symbols: HashMap<String, Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            symbols: HashMap::new(),
        }
    }

    pub fn insert(&mut self, symbol: Symbol) {
        self.symbols.insert(symbol.name.clone(), symbol);
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    pub fn lookup_in_scope(&self, name: &str, scope: &Scope) -> Option<&Symbol> {
        self.symbols.get(name).filter(|s| &s.scope == scope)
    }
}