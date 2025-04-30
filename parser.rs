use crate::lexer::{Lexer, Token, TokenType};

#[derive(Debug)]
pub enum AstNode {
    JumpStatement { target: String },
    FlagDeclaration { flag: String },
    Assignment { variable: String, value: Expression },
    AllocateMemory { size: Expression, handle: String },
    ReleaseMemory { handle: Expression },
    SpawnTask { function: String, priority: Option<Expression> },
    ExitTask { code: Option<Expression> },
    SleepTask { duration: Expression },
    YieldTask,
    AcquireResource { name: Expression, handle: String },
    ControlResource { handle: Expression, command: Expression },
    SendMessage { handle: Expression, message: Expression },
    ReceiveMessage { handle: Expression, buffer: String }, // Basit buffer adı
    GetTaskId { target: String },
    GetCoreId { target: String },
    GetTotalCores { target: String },
    Instruction { opcode: String, operands: Vec<Expression> },
    Label { name: String },
    Program(Vec<AstNode>),
}

#[derive(Debug)]
pub enum Expression {
    Identifier(String),
    Number(i64),
    Flag(String),
    StringLiteral(String),
    Handle(usize),
    TaskId(usize),
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer::new(input);
        let first_token = lexer.next_token();
        Parser {
            lexer,
            current_token: first_token,
        }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn consume(&mut self, expected_type: TokenType) {
        if self.current_token.token_type == expected_type {
            self.advance();
        } else {
            panic!(
                "Beklenen token: {:?}, bulunan: {:?} (satır {}, sütun {})",
                expected_type,
                self.current_token.token_type,
                self.current_token.line,
                self.current_token.column
            );
        }
    }

    fn parse_expression(&mut self) -> Expression {
        match &self.current_token.token_type {
            TokenType::Identifier(name) => {
                let expr = Expression::Identifier(name.clone());
                self.advance();
                expr
            }
            TokenType::Number(value) => {
                let expr = Expression::Number(*value);
                self.advance();
                expr
            }
            TokenType::Flag(flag) => {
                let expr = Expression::Flag(flag.clone());
                self.advance();
                expr
            }
            TokenType::StringLiteral(s) => {
                let expr = Expression::StringLiteral(s.clone());
                self.advance();
                expr
            }
            _ => panic!("Beklenen ifade, bulunan: {:?} (satır {}, sütun {})", self.current_token.token_type, self.current_token.line, self.current_token.column),
        }
    }

    fn parse_optional_expression(&mut self) -> Option<Expression> {
        match &self.current_token.token_type {
            TokenType::Identifier(_) | TokenType::Number(_) | TokenType::Flag(_) | TokenType::StringLiteral(_) => {
                Some(self.parse_expression())
            }
            _ => None,
        }
    }

    fn parse_statement(&mut self) -> Option<AstNode> {
        match &self.current_token.token_type {
            TokenType::Keyword(keyword) => match keyword.as_str() {
                "JUMP" => {
                    self.advance();
                    match &self.current_token.token_type {
                        TokenType::Identifier(target) => {
                            let node = AstNode::JumpStatement { target: target.clone() };
                            self.advance();
                            Some(node)
                        }
                        _ => panic!("JUMP komutundan sonra hedef bekleniyor (satır {}, sütun {})", self.current_token.line, self.current_token.column),
                    }
                }
                "FLAG" => {
                    self.advance();
                    match &self.current_token.token_type {
                        TokenType::Flag(flag) => {
                            let node = AstNode::FlagDeclaration { flag: flag.clone() };
                            self.advance();
                            Some(node)
                        }
                        _ => panic!("FLAG komutundan sonra flag bekleniyor (satır {}, sütun {})", self.current_token.line, self.current_token.column),
                    }
                }
                "ALLOCATE" => {
                    self.advance();
                    let size = self.parse_expression();
                    self.consume(TokenType::Keyword("AS".to_string()));
                    match &self.current_token.token_type {
                        TokenType::Identifier(handle) => {
                            let node = AstNode::AllocateMemory { size, handle: handle.clone() };
                            self.advance();
                            Some(node)
                        }
                        _ => panic!("ALLOCATE komutundan sonra AS ve handle bekleniyor (satır {}, sütun {})", self.current_token.line, self.current_token.column),
                    }
                }
                "RELEASE" => {
                    self.advance();
                    let handle = self.parse_expression();
                    Some(AstNode::ReleaseMemory { handle })
                }
                "SPAWN" => {
                    self.advance();
                    match &self.current_token.token_type {
                        TokenType::Identifier(function) => {
                            self.advance();
                            let mut priority
                        }
                    }
                }
            }
        }
    }
}