use std::collections::HashMap;
use crate::lexer::{Token, TokenType};
use crate::parser::{AstNode, Expression, Operand}; // Makro genişletme sırasında yeni AST düğümleri oluşturmak için

#[derive(Debug, Clone)]
pub struct Macro {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Vec<Token>, // Makro gövdesi token dizisi olarak saklanır
}

pub struct MacroTable {
    macros: HashMap<String, Macro>,
}

impl MacroTable {
    pub fn new() -> Self {
        MacroTable { macros: HashMap::new() }
    }

    pub fn define(&mut self, name: &str, parameters: Vec<String>, body: Vec<Token>) {
        if self.macros.contains_key(name) {
            eprintln!("Uyarı: Makro '{}' yeniden tanımlanıyor", name);
        }
        self.macros.insert(
            name.to_string(),
            Macro {
                name: name.to_string(),
                parameters,
                body,
            },
        );
    }

    pub fn lookup(&self, name: &str) -> Option<&Macro> {
        self.macros.get(name)
    }
}

// Leksikal analiz aşamasında makro tanımlarının toplanması ve genişletilmesi için
use crate::lexer::Lexer;

impl Lexer {
    // Makro tanımlarını ayıklayan bir fonksiyon (basit bir örnek)
    pub fn extract_macros(&mut self, macro_table: &mut MacroTable) {
        let mut temp_tokens = Vec::new();
        while let token = self.next_token() {
            temp_tokens.push(token.clone());
            match &token.token_type {
                TokenType::Keyword(kw) if kw == "MACRO" => {
                    // Basit makro tanımı işleme (parametreler ve gövde)
                    if let Some((name, params, body_tokens)) = self.parse_macro_definition() {
                        macro_table.define(&name, params, body_tokens);
                    }
                }
                _ => {} // Makro tanımı dışında kalan tokenler geçici listeye eklenir
            }
        }
        // Basit bir yeniden tokenleştirme (gerçekte daha karmaşık olabilir)
        self.input = temp_tokens.iter().map(|t| format!("{:?}", t.token_type)).collect::<Vec<_>>().join(" ");
        self.position = 0;
        self.line = 1;
        self.column = 1;
    }

    fn parse_macro_definition(&mut self) -> Option<(String, Vec<String>, Vec<Token>)> {
        let macro_name = match self.next_token().token_type {
            TokenType::Identifier(name) => name,
            _ => {
                eprintln!("Hata: MACRO anahtar kelimesinden sonra makro adı bekleniyor");
                return None;
            }
        };

        let mut parameters = Vec::new();
        if let TokenType::OpenParen = self.next_token().token_type {
            loop {
                match self.next_token().token_type {
                    TokenType::Identifier(param) => parameters.push(param),
                    TokenType::Comma => continue,
                    TokenType::CloseParen => break,
                    TokenType::EndOfFile | TokenType::Unknown(_) => {
                        eprintln!("Hata: Makro parametre listesi beklenmedik şekilde sona erdi");
                        return None;
                    }
                    _ => eprintln!("Hata: Geçersiz makro parametresi"),
                }
            }
        }

        let mut body = Vec::new();
        loop {
            let token = self.next_token();
            match &token.token_type {
                TokenType::Keyword(kw) if kw == "ENDMACRO" => break,
                TokenType::EndOfFile => {
                    eprintln!("Hata: ENDMACRO bekleniyordu");
                    return None;
                }
                _ => body.push(token),
            }
        }

        Some((macro_name, parameters, body))
    }

    // Makro çağrılarını genişleten bir fonksiyon (parser aşamasında yapılabilir)
     pub fn expand_macros(&mut self, ast: &mut AstNode, macro_table: &MacroTable) { ... }
}