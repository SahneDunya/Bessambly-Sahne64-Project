#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Keyword(String),     // JUMP, FLAG, ALLOCATE vb.
    Identifier(String),  // Değişken, etiket, prosedür adı
    Number(i64),
    Flag(String),        // ZF, CF vb.
    Handle(usize),       // Sahne64 Handle (sayısal olarak temsil edilebilir)
    TaskId(usize),       // Sahne64 Task ID (sayısal olarak temsil edilebilir)
    ResourceId(String),  // Kaynak adı (string)
    Colon,               // :
    Comma,               // ,
    OpenParen,           // (
    CloseParen,          // )
    Equals,              // =
    StringLiteral(String), // Örneğin kaynak adları için
    EndOfFile,
    Unknown(char),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

pub struct Lexer {
    input: String,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input,
            position: 0,
            line: 1,
            column: 1,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    fn advance(&mut self) -> Option<char> {
        let current = self.peek();
        if let Some(c) = current {
            self.position += 1;
            self.column += 1;
            Some(c)
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                if c == '\n' {
                    self.line += 1;
                    self.column = 1;
                }
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.peek() {
            Some(c) => {
                match c {
                    ':' => self.single_char_token(TokenType::Colon),
                    ',' => self.single_char_token(TokenType::Comma),
                    '(' => self.single_char_token(TokenType::OpenParen),
                    ')' => self.single_char_token(TokenType::CloseParen),
                    '=' => self.single_char_token(TokenType::Equals),
                    '"' => self.string_literal(),
                    'J' | 'U' | 'M' | 'P' | 'F' | 'L' | 'A' | 'G' | // Temel anahtar kelimeler
                    'A' | 'L' | 'O' | 'C' | 'T' | 'E' |           // ALLOCATE
                    'R' | 'E' | 'L' | 'S' | 'A' | 'E' |           // RELEASE
                    'S' | 'P' | 'A' | 'W' | 'N' |                 // SPAWN
                    'E' | 'X' | 'I' | 'T' |                       // EXIT
                    'S' | 'L' | 'E' | 'E' | 'P' |                 // SLEEP
                    'Y' | 'I' | 'E' | 'L' | 'D' |                 // YIELD
                    'A' | 'C' | 'Q' | 'U' | 'I' | 'R' | 'E' |       // ACQUIRE
                    'C' | 'T' | 'R' | 'L' |                       // CTRL
                    'S' | 'E' | 'N' | 'D' |                       // SEND
                    'R' | 'E' | 'C' | 'V' |                       // RECV
                    'G' | 'E' | 'T' | '_' => {                    // GET_...
                        self.identifier_or_keyword()
                    }
                    'Z' | 'C' | 'S' | 'O' => self.flag(),
                    '0'..='9' | '-' => self.number(),
                    other => {
                        self.advance();
                        Token { token_type: TokenType::Unknown(other), line: self.line, column: self.column - 1 }
                    }
                }
            }
            None => Token { token_type: TokenType::EndOfFile, line: self.line, column: self.column },
        }
    }

    fn single_char_token(&mut self, token_type: TokenType) -> Token {
        let token = Token { token_type, line: self.line, column: self.column };
        self.advance();
        token
    }

    fn identifier_or_keyword(&mut self) -> Token {
        let mut identifier = String::new();
        let start_column = self.column;
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        let token_type = match identifier.as_str() {
            "JUMP" | "FLAG" | "ALLOCATE" | "RELEASE" | "SPAWN" | "EXIT" | "SLEEP" | "YIELD" |
            "ACQUIRE" | "CTRL" | "SEND" | "RECV" | "GET_TASK_ID" | "GET_CORE_ID" | "GET_TOTAL_CORES" => {
                TokenType::Keyword(identifier)
            }
            _ => TokenType::Identifier(identifier),
        };
        Token { token_type, line: self.line, column: start_column }
    }

    fn flag(&mut self) -> Token {
        let mut flag = String::new();
        let start_column = self.column;
        while let Some(ch) = self.peek() {
            if ch.is_alphabetic() {
                flag.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        Token { token_type: TokenType::Flag(flag), line: self.line, column: start_column }
    }

    fn number(&mut self) -> Token {
        let mut number = String::new();
        let start_column = self.column;
        while let Some(ch) = self.peek() {
            if ch.is_digit(10) || ch == '-' {
                number.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        if let Ok(num) = number.parse::<i64>() {
            // Burada Handle veya TaskId olabilecek sayıları ayırt etmek için bağlama duyarlı analiz gerekebilir.
            // Şimdilik sadece Number olarak kabul edelim ve Parser'da daha detaylı kontrol edelim.
            Token { token_type: TokenType::Number(num), line: self.line, column: start_column }
        } else {
            Token { token_type: TokenType::Unknown(number.chars().next().unwrap_or('\0')), line: self.line, column: start_column }
        }
    }

    fn string_literal(&mut self) -> Token {
        self.advance(); // Açılış tırnağını atla
        let mut value = String::new();
        let start_column = self.column;
        while let Some(ch) = self.peek() {
            self.advance();
            if ch == '"' {
                return Token { token_type: TokenType::StringLiteral(value), line: self.line, column: start_column };
            }
            value.push(ch);
        }
        Token { token_type: TokenType::Unknown('"'), line: self.line, column: start_column } // Kapanış tırnağı yok hatası
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_with_sahne64() {
        let input = "ALLOCATE 1024 AS handle1\nSPAWN task_func WITH prio=2\nACQUIRE \"my_resource\" AS res1\nSEND handle1, message";
        let mut lexer = Lexer::new(input.to_string());

        assert_eq!(lexer.next_token().token_type, TokenType::Keyword("ALLOCATE".to_string()));
        assert_eq!(lexer.next_token().token_type, TokenType::Number(1024));
        assert_eq!(lexer.next_token().token_type, TokenType::Keyword("AS".to_string())); // "AS" anahtar kelime olarak eklenebilir
        assert_eq!(lexer.next_token().token_type, TokenType::Identifier("handle1".to_string()));
        assert_eq!(lexer.next_token().token_type, TokenType::Keyword("SPAWN".to_string()));
        assert_eq!(lexer.next_token().token_type, TokenType::Identifier("task_func".to_string()));
        assert_eq!(lexer.next_token().token_type, TokenType::Keyword("WITH".to_string())); // "WITH" anahtar kelime olarak eklenebilir
        assert_eq!(lexer.next_token().token_type, TokenType::Identifier("prio".to_string()));
        assert_eq!(lexer.next_token().token_type, TokenType::Equals);
        assert_eq!(lexer.next_token().token_type, TokenType::Number(2));
        assert_eq!(lexer.next_token().token_type, TokenType::Keyword("ACQUIRE".to_string()));
        assert_eq!(lexer.next_token().token_type, TokenType::StringLiteral("my_resource".to_string()));
        assert_eq!(lexer.next_token().token_type, TokenType::Keyword("AS".to_string()));
        assert_eq!(lexer.next_token().token_type, TokenType::Identifier("res1".to_string()));
        assert_eq!(lexer.next_token().token_type, TokenType::Keyword("SEND".to_string()));
        assert_eq!(lexer.next_token().token_type, TokenType::Identifier("handle1".to_string()));
        assert_eq!(lexer.next_token().token_type, TokenType::Comma);
        assert_eq!(lexer.next_token().token_type, TokenType::Identifier("message".to_string()));
        assert_eq!(lexer.next_token().token_type, TokenType::EndOfFile);
    }
}